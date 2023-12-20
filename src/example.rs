use super::task::*;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyExampleTaskError {
    OperationANotImplemented,
    OperationBNotImplemented,
    OperationCNotImplemented
}

pub enum MyExampleTask {
    DoOperationA,
    DoOperationB,
    DoOperationC
}

impl Display for MyExampleTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyExampleTaskError::OperationANotImplemented => {
                write!(f, "Operation A not implemented.")
            },
            MyExampleTaskError::OperationBNotImplemented => {
                write!(f, "Operation B not implemented.")
            },
            MyExampleTaskError::OperationCNotImplemented => {
                write!(f, "Operation C not implemented.")
            },
        }
    }
}

impl Error for MyExampleTaskError {}

impl Task for MyExampleTask {
    type SuccessType = ();
    type FailureType = MyExampleTaskError;

    fn execute(&self) -> Result<Self::SuccessType, Self::FailureType> {
        match self {
            MyExampleTask::DoOperationA => {
                Err(MyExampleTaskError::OperationANotImplemented)
            },
            MyExampleTask::DoOperationB => {
                Err(MyExampleTaskError::OperationBNotImplemented)
            },
            MyExampleTask::DoOperationC => {
                Err(MyExampleTaskError::OperationCNotImplemented)
            },
        }
    }
}