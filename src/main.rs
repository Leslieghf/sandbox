use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Spawn some CPU-bound tasks using `spawn_blocking`
    for _ in 0..30 {
        task::spawn_blocking(|| {
            // Simulate a CPU-intensive task
            heavy_computation();
        });
    }

    // And handle many I/O-bound tasks
    for _ in 0..100 {
        tokio::spawn(async {
            // Simulate an I/O-bound task (e.g., network call)
            io_bound_operation().await;
        });
    }

    sleep(Duration::from_secs(10)).await; // Wait for a while to let tasks progress
}

fn heavy_computation() {
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
    println!("Sum is {}", sum);
}

async fn io_bound_operation() {
    // Simulate an I/O-bound task with a delay
    sleep(Duration::from_secs(1)).await;
    println!("I/O task completed");
}
