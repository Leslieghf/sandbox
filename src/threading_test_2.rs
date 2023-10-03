use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::*;
extern crate rand;
use rand::Rng;

#[derive(PartialEq)]
pub enum ThreadAllocationState {
    Free,
    Allocated
}

pub struct ThreadPool {
    free_threads: Vec<Thread>,
    allocated_threads: Vec<Thread>
}

impl ThreadPool {
    pub fn open() -> Self {
        let mut free_threads = Vec::new();
        for i in 0..NUM_THREADS {
            free_threads.push(Thread::new(ThreadID::new(i)));
        }

        Self {
            free_threads,
            allocated_threads: Vec::new()
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
        thread.allocation_state = ThreadAllocationState::Allocated;
        self.allocated_threads.push(thread);

        Ok(self.allocated_threads.last_mut().unwrap())
    }

    pub fn free_thread(&mut self, thread_id: ThreadID) -> Result<(), String> {
        let mut remove_index: Option<usize> = None;
    
        if self.free_threads.iter().any(|thread| thread.id.get_id() == thread_id.get_id()) {
            return Err("Thread is already free!".to_string());
        }
    
        for (i, thread) in self.allocated_threads.iter_mut().enumerate() {
            if thread.id.get_id() == thread_id.get_id() {
                if thread.allocation_state == ThreadAllocationState::Free {
                    return Err("Thread has to be allocated to be freed!".to_string());
                }
                thread.allocation_state = ThreadAllocationState::Free;
                remove_index = Some(i);
                break;
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
    allocation_state: ThreadAllocationState,
    handle: Option<thread::JoinHandle<()>>,
    tx_to_worker: mpsc::Sender<()>,
    rx_from_worker: mpsc::Receiver<()>,
    terminate_flag: Arc<Mutex<bool>>,
    elapsed_time: Arc<Mutex<Duration>>,
}

impl Thread {
    fn new(id: ThreadID) -> Self {
        let (tx_to_worker, rx_from_main) = mpsc::channel();
        let (tx_to_main, rx_from_worker) = mpsc::channel();
        let terminate_flag = Arc::new(Mutex::new(false));
        let flag = terminate_flag.clone();
        let elapsed_time = Arc::new(Mutex::new(Duration::from_secs(0)));
        let elapsed_time_clone = elapsed_time.clone();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            
            loop {
                {
                    if *flag.lock().unwrap() {
                        break;
                    }
                }
                
                let _ = rx_from_main.recv().unwrap();
        
                let start_time = Instant::now();
                let random_times = rng.gen_range(1..=FIBONACCI_MAX_ITERATIONS);
                for _ in 0..random_times {
                    let _ = Self::fibonacci(FIBONACCI_LENGTH);
                }
                let elapsed_time = start_time.elapsed();
                
                *elapsed_time_clone.lock().unwrap() += elapsed_time;
        
                tx_to_main.send(()).unwrap();
            }
        });

        Self {
            id,
            allocation_state: ThreadAllocationState::Free,
            handle: Some(handle),
            tx_to_worker,
            rx_from_worker,
            terminate_flag,
            elapsed_time
        }
    }

    pub fn request_work(&self) -> Result<(), String> {
        if self.allocation_state == ThreadAllocationState::Free {
            return Err("Thread has to be allocated to request work!".to_string());
        }

        self.tx_to_worker.send(()).unwrap();

        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), String> {
        *self.terminate_flag.lock().unwrap() = true;
        
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }

        Ok(())
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
const NUM_THREADS: u32 = 32;
const NUM_ALLOCATIONS: u32 = 32;
const FIBONACCI_LENGTH: u32 = 64;
const FIBONACCI_MAX_ITERATIONS: u32 = 1000000;

pub struct ThreadingTest2;

impl ThreadingTest2 {
    pub fn main() {
        let mut thread_pool = ThreadPool::open();
        let thread_ids: Vec<ThreadID> = (0..NUM_ALLOCATIONS).map(|_| thread_pool.allocate_thread().unwrap().id).collect();

        for _ in 0..REQUEST_ITERATIONS {
            for thread_id in &thread_ids {
                let thread = thread_pool.get_thread(thread_id).unwrap();
    
                for _ in 0..REQUEST_ITERATIONS {
                    thread.request_work().unwrap();
                }
            }
    
            let start_time = Instant::now();
            let mut elapsed_time_sequential = Duration::from_secs(0);
    
            for thread_id in &thread_ids {
                let thread = thread_pool.get_thread_mut(thread_id).unwrap();
                thread.rx_from_worker.recv().unwrap();
                println!("Thread '{}'\t took {:?}", thread_id.id, thread.elapsed_time.lock().unwrap());
                elapsed_time_sequential += *thread.elapsed_time.lock().unwrap();
            }
            let elapsed_time_parallel = start_time.elapsed();
            let efficiency = elapsed_time_sequential.as_secs_f64() / elapsed_time_parallel.as_secs_f64();
    
            println!("Total time sequential: {:?}", elapsed_time_sequential);
            println!("Total time parallel: {:?}", elapsed_time_parallel);
            println!("Efficiency: {}\n\n\n", efficiency);
        }

        for thread_id in thread_ids {
            thread_pool.free_thread(thread_id).unwrap();
        }

        thread_pool.close().unwrap();
    }
}