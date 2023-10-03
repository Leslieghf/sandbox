use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::*;

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
    pub fn new(num_threads: u8) -> Self {
        let mut free_threads = Vec::new();
        for i in 0..num_threads {
            free_threads.push(Thread::new(ThreadID::new(i)));
        }

        Self {
            free_threads,
            allocated_threads: Vec::new()
        }
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

    pub fn free_thread(&mut self, thread: &mut Thread) -> Result<(), String> {
        if thread.allocation_state == ThreadAllocationState::Free {
            return Err("Thread has to be allocated to be freed!".to_string());
        }

        thread.allocation_state = ThreadAllocationState::Free;
        self.free_threads.push(self.allocated_threads.remove(0));

        Ok(())
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
    handle: thread::JoinHandle<()>,
    tx_to_worker: mpsc::Sender<()>,
    rx_from_worker: mpsc::Receiver<()>
}

impl Thread {
    fn new(id: ThreadID) -> Self {
        let (tx_to_worker, rx_from_main) = mpsc::channel();
        let (tx_to_main, rx_from_worker) = mpsc::channel();

        let rx_from_main = Arc::new(Mutex::new(rx_from_main));
        let tx_to_main = Arc::new(Mutex::new(tx_to_main));

        let handle = thread::spawn(move || {
            loop {
                let _ = rx_from_main.lock().unwrap().recv().unwrap();
                thread::sleep(Duration::from_millis(1000));
                tx_to_main.lock().unwrap().send(()).unwrap();
            }
        });

        Self {
            id,
            allocation_state: ThreadAllocationState::Free,
            handle,
            tx_to_worker,
            rx_from_worker
        }
    }

    pub fn request_work(&self) -> Result<(), String> {
        if self.allocation_state == ThreadAllocationState::Free {
            return Err("Thread has to be allocated to request work!".to_string());
        }

        self.tx_to_worker.send(()).unwrap();

        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ThreadID {
    id: u8,
}

impl ThreadID {
    fn new(id: u8) -> Self {
        Self {
            id
        }
    }

    fn get_id(&self) -> u8 {
        self.id
    }
}

pub struct ThreadingTest2;

impl ThreadingTest2 {
    pub fn main() {
        let num_threads = 8;
        let num_allocations = 4;
        let num_requests = 4;


        let mut thread_pool = ThreadPool::new(num_threads);
        let threads: Vec<ThreadID> = (0..num_allocations).map(|_| thread_pool.allocate_thread().unwrap().id).collect();

        for thread in &threads {
            let thread = thread_pool.get_thread(thread).unwrap();

            for _ in 0..num_requests {
                thread.request_work().unwrap();
            }
        }

        loop {
            for thread in &threads {
                let _ = thread_pool.get_thread_mut(thread).unwrap().rx_from_worker.recv().unwrap();
                println!("Received result from thread {}", thread.get_id());
            }
        }
    }
}