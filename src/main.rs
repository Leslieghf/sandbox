pub mod example;
pub mod task;

use example::*;
use task::*;

fn main() {
    let mut task_manager: TaskManager<MyExampleTask> = TaskManager::new();

    task_manager.add_task(
        Box::new(MyExampleTask::DoOperationA), 
        Some(Box::new(|success| {
            println!("Operation A succeeded: {:?}", success);
        })),
        Some(Box::new(|failure| {
            println!("Operation A failed: {:?}", failure);
        })),
    );

    task_manager.add_task(
        Box::new(MyExampleTask::DoOperationB), 
        Some(Box::new(|success| {
            println!("Operation B succeeded: {:?}", success);
        })),
        Some(Box::new(|failure| {
            println!("Operation B failed: {:?}", failure);
        })),
    );

    task_manager.add_task(
        Box::new(MyExampleTask::DoOperationC), 
        Some(Box::new(|success| {
            println!("Operation C succeeded: {:?}", success);
        })),
        Some(Box::new(|failure| {
            println!("Operation C failed: {:?}", failure);
        })),
    );

    task_manager.execute_tasks();
}