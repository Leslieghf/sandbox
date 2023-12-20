use std::any::Any;
use std::sync::{Arc, Mutex};
pub trait Task: Any + Send + Sync {
    type SuccessType: Any + Send + Sync;
    type FailureType: Any + Send + Sync;

    fn execute(&self) -> Result<Self::SuccessType, Self::FailureType>;
}

type SuccessCallback<ReturnType> = Box<dyn FnOnce(ReturnType) + Send + Sync>;
type FailureCallback<ErrorType> = Box<dyn FnOnce(ErrorType) + Send + Sync>;

struct InternalTask<T: Task> {
    task: Box<dyn Task<SuccessType = T::SuccessType, FailureType = T::FailureType>>,
    success_callback: Option<SuccessCallback<T::SuccessType>>,
    failure_callback: Option<FailureCallback<T::FailureType>>,
}

pub struct TaskManager<T> 
where
    T: Task,
{
    tasks: Arc<Mutex<Vec<InternalTask<T>>>>,
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

        tasks.push(InternalTask { task, success_callback, failure_callback });
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