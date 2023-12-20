use std::any::Any;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub trait Task: Any + Send + Sync {
    type SuccessType: Any + Send + Sync;
    type FailureType: Any + Error + Send + Sync;

    fn execute(&self) -> Result<Self::SuccessType, Self::FailureType>;
}

type SuccessCallback<ReturnType> = Box<dyn FnOnce(ReturnType) + Send + Sync>;
type FailureCallback<ErrorType> = Box<dyn FnOnce(ErrorType) + Send + Sync>;

struct InternalTaskRepresentation<T: Task> {
    task: Box<dyn Task<SuccessType = T::SuccessType, FailureType = T::FailureType>>,
    success_callback: Option<SuccessCallback<T::SuccessType>>,
    failure_callback: Option<FailureCallback<T::FailureType>>,
}

pub struct TaskManager<T> 
where
    T: Task,
{
    tasks: Arc<Mutex<Vec<InternalTaskRepresentation<T>>>>,
}

pub struct TaskAPI {
    task_managers: Vec<Box<dyn Any>>,
}

impl<T> TaskManager<T> 
where
    T: Task,
{
    fn new() -> Self {
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

        tasks.push(InternalTaskRepresentation { task, success_callback, failure_callback });
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

        tasks.clear();
    }
}

impl TaskAPI {
    pub fn new() -> Self {
        TaskAPI {
            task_managers: Vec::new(),
        }
    }

    pub fn register_task_type<T: Task>(&mut self) {
        self.task_managers.push(Box::new(TaskManager::<T>::new()));
    }

    pub fn unregister_task_type<T: Task>(&mut self) {
        self.task_managers.retain(|task_manager| {
            !task_manager.is::<TaskManager<T>>()
        });
    }

    pub fn get_task_manager<T: Task>(&mut self) -> Result<&mut TaskManager<T>, String> {
        for task_manager in self.task_managers.iter_mut() {
            if let Some(task_manager) = task_manager.downcast_mut::<TaskManager<T>>() {
                return Ok(task_manager);
            }
        }

        Err(format!("Task type '{}' is not registered", std::any::type_name::<T>()))
    }
}