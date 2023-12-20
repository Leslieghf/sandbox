pub mod example;
pub mod task;

use example::*;
use task::*;

fn main() {
    let mut task_api = TaskAPI::new();

    task_api.register_task_type::<MyExampleTask>();

    let my_example_task_manager = match task_api.get_task_manager::<MyExampleTask>() {
        Ok(task_manager) => task_manager,
        Err(error) => {
            panic!("{}", error);
        },
    };

    my_example_task_manager.add_task(
        Box::new(MyExampleTask::DoOperationA), 
        Some(Box::new(|_| {
            println!("Operation A succeeded");
        })),
        Some(Box::new(|failure| {
            println!("Operation A failed: {}", failure);
        })),
    );

    my_example_task_manager.add_task(
        Box::new(MyExampleTask::DoOperationB), 
        Some(Box::new(|_| {
            println!("Operation B succeeded");
        })),
        Some(Box::new(|failure| {
            println!("Operation B failed: {}", failure);
        })),
    );

    my_example_task_manager.add_task(
        Box::new(MyExampleTask::DoOperationC), 
        Some(Box::new(|_| {
            println!("Operation C succeeded");
        })),
        Some(Box::new(|failure| {
            println!("Operation C failed: {}", failure);
        })),
    );

    my_example_task_manager.execute_tasks();
}