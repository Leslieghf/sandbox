use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Spawn some CPU-bound tasks using `spawn_blocking`
    for i in 0..30 {
        let index = i;
        task::spawn_blocking(move || {
            // Simulate a CPU-intensive task
            heavy_computation(index);
        });
    }

    // And handle many I/O-bound tasks
    for i in 0..100 {
        let index = i;
        tokio::spawn(async move {
            // Simulate an I/O-bound task (e.g., network call)
            io_bound_operation(index).await;
        });
    }

    sleep(Duration::from_secs(10)).await; // Wait for a while to let tasks progress
}

fn heavy_computation(index: i32) {
    let mut sum = 0i32;
    for i in 0..1_000_000 {
        sum = match sum.checked_add(i) {
            Some(value) => value,
            None => {
                println!("Overflow!");
                return;
            }
        };
        sum = match sum.checked_sub(i) {
            Some(value) => value,
            None => {
                println!("Underflow!");
                return;
            }
        };
    }
    println!("Index {}: Sum is {}", index, sum);
}

async fn io_bound_operation(index: i32) {
    // Simulate an I/O-bound task with a delay
    sleep(Duration::from_secs(1)).await;
    println!("Index {}: I/O task completed", index);
}
