use std::any::Any;
use std::sync::{Arc, Mutex};

// Task trait with success and failure types
trait Task: Any + Send + Sync {
    type SuccessType: Any + Send + Sync;
    type FailureType: Any + Send + Sync;

    fn execute(&self) -> Result<Self::SuccessType, Self::FailureType>;
}

// Callback types
type SuccessCallback<ReturnType> = Box<dyn FnOnce(ReturnType) + Send + Sync>;
type FailureCallback<ErrorType> = Box<dyn FnOnce(ErrorType) + Send + Sync>;

// Struct to hold a task and its callbacks
struct TaskWithCallbacks<T: Task> {
    task: Box<dyn Task<SuccessType = T::SuccessType, FailureType = T::FailureType>>,
    success_callback: Option<SuccessCallback<T::SuccessType>>,
    failure_callback: Option<FailureCallback<T::FailureType>>,
}

// TaskManager that takes a TaskType
struct TaskManager<T> 
where
    T: Task,
{
    tasks: Arc<Mutex<Vec<TaskWithCallbacks<T>>>>,
}

impl<T> TaskManager<T> 
where
    T: Task,
{
    pub fn new() -> Self {
        TaskManager {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_task(
        &mut self, 
        task: Box<dyn Task<SuccessType = T::SuccessType, FailureType = T::FailureType>>, 
        success_callback: Option<SuccessCallback<T::SuccessType>>, 
        failure_callback: Option<FailureCallback<T::FailureType>>) 
    {
        let mut tasks = self.tasks.lock().expect("Failed to lock tasks");

        tasks.push(TaskWithCallbacks { task, success_callback, failure_callback });
    }

    pub fn execute_tasks(&mut self) {
        let mut tasks = self.tasks.lock().expect("Failed to lock tasks");

        for task_with_callbacks in tasks.iter_mut() {
            let result = task_with_callbacks.task.execute();

            match result {
                Ok(success) => {
                    if let Some(success_callback) = task_with_callbacks.success_callback.take() {
                        success_callback(success);
                    }
                },
                Err(failure) => {
                    if let Some(failure_callback) = task_with_callbacks.failure_callback.take() {
                        failure_callback(failure);
                    }
                },
            }
        }
    }
}

enum MyExampleTask {
    DoOperationA,
    DoOperationB,
    DoOperationC
}

impl Task for MyExampleTask {
    type SuccessType = ();
    type FailureType = String;

    fn execute(&self) -> Result<Self::SuccessType, Self::FailureType> {
        match self {
            MyExampleTask::DoOperationA => {
                Err("Operation A not implemented.".to_string())
            },
            MyExampleTask::DoOperationB => {
                Err("Operation B not implemented.".to_string())
            },
            MyExampleTask::DoOperationC => {
                Err("Operation C not implemented.".to_string())
            },
        }
    }
}

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