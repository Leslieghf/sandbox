use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::*;
use std::io::{self, Write};
extern crate rand;
use rand::Rng;

#[derive(PartialEq)]
pub enum ThreadAllocationState {
    Free,
    Allocated
}

pub struct ThreadPool {
    free_threads: Vec<Thread>,
    allocated_threads: Vec<Thread>,
    rx_from_worker: mpsc::Receiver<(u32, Duration)>,
}

impl ThreadPool {
    pub fn open() -> Self {
        let mut free_threads = Vec::new();
        let (tx_to_main, rx_from_worker) = mpsc::channel();
        for i in 0..NUM_THREADS {
            free_threads.push(Thread::new(ThreadID::new(i), tx_to_main.clone()));
        }

        Self {
            free_threads,
            allocated_threads: Vec::new(),
            rx_from_worker,
        }
    }

    pub fn close(mut self) -> Result<(), String> {
        if !self.allocated_threads.is_empty() {
            return Err("There are still allocated threads, cannot close!".to_string());
        }
    
        for thread in &mut self.free_threads {
            thread.terminate().unwrap();
        }
    
        Ok(())
    }

    pub fn allocate_thread(&mut self) -> Result<&mut Thread, String> {
        if self.free_threads.len() == 0 {
            return Err("No free threads available!".to_string());
        }
    
        let mut thread = self.free_threads.remove(0);
        if let Ok(mut state) = thread.allocation_state.lock() {
            *state = ThreadAllocationState::Allocated;
        } else {
            return Err("Could not lock allocation state for modification".to_string());
        }
        self.allocated_threads.push(thread);
    
        match self.allocated_threads.last_mut() {
            Some(allocated_thread) => return Ok(allocated_thread),
            None => return Err("Error allocating thread!".to_string())
        }
    }

    pub fn free_thread(&mut self, thread_id: ThreadID) -> Result<(), String> {
        let mut remove_index: Option<usize> = None;
    
        if self.free_threads.iter().any(|thread| thread.id.get_id() == thread_id.get_id()) {
            return Err("Thread is already free!".to_string());
        }
    
        for (i, thread) in self.allocated_threads.iter_mut().enumerate() {
            if thread.id.get_id() == thread_id.get_id() {
                if let Ok(mut thread_allocation_state) = thread.allocation_state.lock() {
                    if *thread_allocation_state == ThreadAllocationState::Free {
                        return Err("Thread has to be currently allocated to be freed!".to_string());
                    }
                    *thread_allocation_state = ThreadAllocationState::Free;
                    remove_index = Some(i);
                    break;
                } else {
                    return Err("Error locking thread allocation state!".to_string());
                }
            }
        }
    
        match remove_index {
            Some(index) => {
                self.allocated_threads.remove(index);
                Ok(())
            },
            None => Err("Thread not found!".to_string())
        }
    }

    pub fn get_thread(&self, id: &ThreadID) -> Result<&Thread, String> {
        for thread in &self.allocated_threads {
            if thread.id.get_id() == id.get_id() {
                return Ok(thread);
            }
        }

        Err("Thread not found!".to_string())
    }

    pub fn get_thread_mut(&mut self, id: &ThreadID) -> Result<&mut Thread, String> {
        for thread in &mut self.allocated_threads {
            if thread.id.get_id() == id.get_id() {
                return Ok(thread);
            }
        }

        Err("Thread not found!".to_string())
    }
}

pub struct Thread {
    id: ThreadID,
    allocation_state: Arc<Mutex<ThreadAllocationState>>,
    handle: Option<thread::JoinHandle<()>>,
    tx_to_worker: mpsc::Sender<()>,
    terminate_flag: Arc<Mutex<bool>>,
}

impl Thread {
    fn new(id: ThreadID, tx_to_main: mpsc::Sender<(u32, Duration)>) -> Self {
        let (tx_to_worker, rx_from_main) = mpsc::channel();
        let terminate_flag = Arc::new(Mutex::new(false));
        let allocation_state = Arc::new(Mutex::new(ThreadAllocationState::Free));
        let terminate_flag_clone = terminate_flag.clone();
        let allocation_state_clone = allocation_state.clone();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
        
            loop {
                match terminate_flag_clone.lock() {
                    Ok(terminate_flag) => {
                        if *terminate_flag {
                            match rx_from_main.try_recv() {
                                Ok(_) => {
                                    panic!("Thread was terminated but still received work!")
                                }
                                Err(mpsc::TryRecvError::Empty) => {
                                    println!("Thread '{}' was terminated with empty work queue.", id.get_id());
                                    break;
                                }
                                Err(mpsc::TryRecvError::Disconnected) => {
                                    println!("Thread '{}' was terminated with disconnected work queue.", id.get_id());
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        println!("Error locking terminate flag: {}", err);
                        continue;
                    }
                }
                
                match allocation_state_clone.lock() {
                    Ok(allocation_state) => {
                        if *allocation_state == ThreadAllocationState::Allocated {
                            println!("Thread '{}' is allocated, waiting for work...", id.get_id());
                            match rx_from_main.recv() {
                                Ok(_) => {
                                    println!("Thread '{}' received work!", id.get_id());
                                },
                                Err(err) => {
                                    println!("Error receiving from main thread: {}", err);
                                    continue;
                                }
                            }
                    
                            let start_time = Instant::now();
                            let random_times = rng.gen_range(1..=FIBONACCI_MAX_ITERATIONS);
                    
                            for _ in 0..random_times {
                                let _ = Self::fibonacci(FIBONACCI_LENGTH);
                            }
                    
                            let elapsed_time = start_time.elapsed();
                    
                            match tx_to_main.send((id.id, elapsed_time)) {
                                Ok(_) => {
                                    println!("Thread '{}' sent result!", id.get_id());
                                },
                                Err(err) => {
                                    println!("Error sending to main thread: {}", err);
                                    continue;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        println!("Error locking thread allocation state: {}", err);
                        continue;
                    }
                }
            }
        });
        

        Self {
            id,
            allocation_state,
            handle: Some(handle),
            tx_to_worker,
            terminate_flag,
        }
    }

    pub fn request_work(&self) -> Result<(), String> {
        if let Ok(allocation_state) = self.allocation_state.lock() {
            if *allocation_state == ThreadAllocationState::Free {
                return Err("Thread has to be allocated to request work!".to_string());
            }

            match self.tx_to_worker.send(()) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(format!("Error sending to worker thread: {}", err))
            }
        } else {
            return Err("Error locking thread allocation state!".to_string());
        }
    }

    pub fn terminate(&mut self) -> Result<(), String> {
        match self.terminate_flag.lock() {
            Ok(mut terminate_flag) => {
                if *terminate_flag {
                    return Err("Thread is already terminated!".to_string());
                }

                *terminate_flag = true;
        
                if let Some(handle) = self.handle.take() {
                    match handle.join() {
                        Ok(_) => {},
                        Err(err) => return Err(format!("Error joining thread: {:?}", err))
                    }
                }
                
                return Ok(())
            },
            Err(err) => return Err(format!("Error locking terminate flag: {}", err))
        }
    }

    fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let (mut a, mut b, mut c) = (0, 1, 0);
                for _ in 2..=n {
                    c = a + b;
                    a = b;
                    b = c;
                }
                c
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ThreadID {
    id: u32,
}

impl ThreadID {
    fn new(id: u32) -> Self {
        Self {
            id
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

const REQUEST_ITERATIONS: u32 = 5;
const NUM_THREADS: u32 = 16;
const FIBONACCI_LENGTH: u32 = 16;
const FIBONACCI_MAX_ITERATIONS: u32 = 1000000;

pub struct ThreadingTest2;

impl ThreadingTest2 {
    pub fn main() {
        let mut sequential_time_average = Duration::from_secs(0);
        let total_start_time = Instant::now();

        println!("Opening thread pool...");
        let mut thread_pool = ThreadPool::open();

        println!("Allocating threads...");
        let thread_ids: Vec<ThreadID> = (0..NUM_THREADS).map(|_| thread_pool.allocate_thread().unwrap().id).collect();

        for i in 0..REQUEST_ITERATIONS {
            println!("Requesting works [Iteration {}]...", i);
            for thread_id in &thread_ids {
                println!("Requesting work for thread '{}'...", thread_id.id);
                match thread_pool.get_thread_mut(thread_id) {
                    Ok(thread) => {
                        match thread.request_work() {
                            Ok(_) => {
                                println!("Requested work for thread '{}'...", thread_id.id);
                            },
                            Err(err) => {
                                panic!("Error requesting work for thread: {}", err);
                            }
                        }
                    },
                    Err(err) => {
                        panic!("Error getting thread: {}", err);
                    }
                }
            }
    
            println!("Receiving results [Iteration {}]...", i);
            for _ in &thread_ids {
                let (thread_id, thread_elapsed_time) = thread_pool.rx_from_worker.recv().unwrap();
                sequential_time_average += thread_elapsed_time.clone();
                println!("Thread '{}'\t took {:?}", thread_id, thread_elapsed_time);
            }
        }

        println!("\n\nFreeing threads...");
        for thread_id in thread_ids {
            thread_pool.free_thread(thread_id).unwrap_or_else(|err| panic!("Error freeing thread: {}", err));
        }

        println!("Closing thread pool...");
        thread_pool.close().unwrap_or_else(|err| panic!("Error closing thread pool: {}", err));

        println!("1");
        let total_sequential_time_estimation = sequential_time_average.clone();

        println!("2");
        sequential_time_average /= (REQUEST_ITERATIONS * NUM_THREADS) as u32;
        
        println!("3");
        let total_elapsed_time = total_start_time.elapsed();

        println!("\n\nSequential time average: \t{:?}", sequential_time_average);
        println!("Total sequential time estimate: \t{:?}", total_sequential_time_estimation);
        println!("Total elapsed time: \t\t\t{:?}", total_elapsed_time);
        println!("Speedup: \t\t\t{:?}", total_sequential_time_estimation.as_secs_f64() / total_elapsed_time.as_secs_f64());
    }
}