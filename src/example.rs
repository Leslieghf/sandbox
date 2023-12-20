use super::task::*;

pub enum MyExampleTask {
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