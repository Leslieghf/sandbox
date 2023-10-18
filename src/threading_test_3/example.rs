use core::panic;
use std::time::Duration;
use std::{sync::mpsc::*, thread};

use futures::channel::mpsc::SendError;

use super::thread::*;
use super::task::*;

pub enum NetworkingTaskInput {
    ConnectToServer(ConnectToServerTaskInput),
}

pub struct ConnectToServerTaskInput;

impl TaskInput for ConnectToServerTaskInput {}

pub enum NetworkingTaskOutput {
    ConnectToServer(ConnectToServerTaskOutput),
}

pub struct ConnectToServerTaskOutput;

impl TaskOutput for ConnectToServerTaskOutput {}

pub enum NetworkingTask {
    ConnectToServer(ConnectToServerTask),
}

pub struct ConnectToServerTask {
    input: ConnectToServerTaskInput,
}

impl Task for ConnectToServerTask {
    type Input = ConnectToServerTaskInput;
    type Output = ConnectToServerTaskOutput;

    fn new(input: ConnectToServerTaskInput) -> Self{
        Self { input }
    }

    fn execute(&mut self) -> Result<ConnectToServerTaskOutput, String> {
        thread::sleep(Duration::from_secs(5));
        Ok(ConnectToServerTaskOutput)
    }
}

pub struct NetworkingThread {
    request_sender: Sender<NetworkingThreadRequest>,
    request_receiver: Receiver<NetworkingThreadRequest>,
    result_sender: Sender<NetworkingThreadResult>,
    result_receiver: Receiver<NetworkingThreadResult>,
    handle: thread::JoinHandle<()>,
}

impl NetworkingThread {
    pub fn new() -> Self {
        let (request_sender, request_receiver) = channel::<NetworkingThreadRequest>();
        let (result_sender, result_receiver) = channel::<NetworkingThreadResult>();

        let handle = std::thread::spawn(move || {
            loop {
                match request_receiver.recv() {
                    Ok(request) => match request {
                        NetworkingThreadRequest::Command(command) => match command {
                            CommandRequest::Terminate => {
                                match result_sender.send(NetworkingThreadResult::Command(CommandResult::TerminateOk)) {
                                    Ok(_) => {
                                        break;
                                    }
                                    Err(error) => {
                                        panic!("[NetworkingThread] Error sending result: {}", error);
                                    }
                                }
                            }
                        },
                        NetworkingThreadRequest::Task(task) => match task {
                            NetworkingTask::ConnectToServer(task) => {
                                let result = task.execute();
                                match result_sender.send(NetworkingThreadResult::Task(NetworkingTaskOutput::ConnectToServer(result))) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        panic!("[NetworkingThread] Error sending result: {}", error);
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        panic!("[NetworkingThread] Error receiving request: {}", error);
                    }
                }
            }
        });

        Self {
            request_sender,
            request_receiver,
            result_sender,
            result_receiver,
            handle,
        }
    }

    pub fn send_task_request(&mut self, task: NetworkingTask) -> Result<(), SendError>
    {
        match self.request_sender.send(NetworkingThreadRequest::Task(task)) {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => {
                return error;
            }
        }
    }

    pub fn receive_result(&mut self) -> Result<NetworkingThreadResult, RecvError>
    {
        match self.result_receiver.recv() {
            Ok(result) => {
                return Ok(result);
            }
            Err(error) => {
                return error;
            }
        }
    }
}

impl Thread for NetworkingThread {
    fn send_command_request(&mut self, command: CommandRequest) -> Result<(), SendError> {
        match self.request_sender.send(NetworkingThreadRequest::Command(command)) {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}

pub enum NetworkingThreadRequest {
    Command(CommandRequest),
    Task(NetworkingTask),
}

pub enum NetworkingThreadResult {
    Command(CommandResult),
    Task(NetworkingTaskOutput),
}