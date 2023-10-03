use std::sync::mpsc;
use std::thread;

pub struct ThreadingTest1;

impl ThreadingTest1 {
    pub fn main() {
        let (tx_to_worker, rx_from_main) = mpsc::channel();
        let (tx_to_main, rx_from_worker) = mpsc::channel();

        // Spawn a new thread for generating Perlin noise
        thread::spawn(move || loop {
            if let Ok(message) = rx_from_main.recv() {
                if message == "compute" {
                    let result = Self::perform_heavy_computation();
                    tx_to_main.send(result).unwrap();
                }
            }
        });

        for _ in 0..32 {
            tx_to_worker.send("compute").unwrap();
        }

        // Main game loop
        loop {
            // Check for the noise data
            if let Ok(noise) = rx_from_worker.recv() {
                println!("Received result: {:?}", noise);
            }
        }
    }

    fn perform_heavy_computation() -> Vec<f32> {
        std::thread::sleep(std::time::Duration::from_millis(250));
        vec![0.1, 0.2, 0.3]
    }
}
