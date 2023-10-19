use core::panic;
use std::time::Duration;
use std::{sync::mpsc::*, thread};

pub enum NetworkingCommand {
    Terminate,
}

pub enum NetworkingCommandResult {
    TerminateOk,
    TerminateError(TerminateCommandError),
}

pub enum TerminateCommandError {
    UnexpectedError(String),
}

pub enum NetworkingTask {
    ConnectToServer(ConnectToServerTask),
}

pub enum NetworkingTaskParameters {
    ConnectToServer(ConnectToServerTaskParameters),
}

pub struct ConnectToServerTaskParameters {
    pub server_address: String,
    pub server_port: u16,
}

pub enum NetworkingTaskResult {
    ConnectToServerOk(ConnectToServerTaskResult),
    ConnectToServerError(ConnectToServerTaskError),
}

pub enum ConnectToServerTaskError {
    UnexpectedError(String),
}

pub struct ConnectToServerTaskResult;

pub struct ConnectToServerTask {
    pub parameters: ConnectToServerTaskParameters,
}

impl ConnectToServerTask {
    fn execute(&mut self) -> Result<ConnectToServerTaskResult, ConnectToServerTaskError> {
        thread::sleep(Duration::from_secs(5));
        Ok(ConnectToServerTaskResult)
    }
}

pub enum NetworkingThreadRequest {
    Command(NetworkingCommand),
    Task(NetworkingTask),
}

pub enum NetworkingThreadResult {
    Command(NetworkingCommandResult),
    Task(NetworkingTaskResult),
}

pub struct NetworkingThread {
    request_sender: Sender<NetworkingThreadRequest>,
    result_receiver: Receiver<NetworkingThreadResult>,
    handle: thread::JoinHandle<()>,
}
impl NetworkingThread {
    pub fn new() -> Self {
        let (request_sender, request_receiver) = channel::<NetworkingThreadRequest>();
        let (result_sender, result_receiver) = channel::<NetworkingThreadResult>();

        let handle = std::thread::spawn(move || loop {
            match request_receiver.recv() {
                Ok(request) => match request {
                    NetworkingThreadRequest::Command(command) => match command {
                        NetworkingCommand::Terminate => {
                            match result_sender.send(NetworkingThreadResult::Command(
                                NetworkingCommandResult::TerminateOk,
                            )) {
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
                            let mut task = task;
                            match task.execute() {
                                Ok(result) => {
                                    match result_sender.send(NetworkingThreadResult::Task(
                                        NetworkingTaskResult::ConnectToServerOk(result),
                                    )) {
                                        Ok(_) => {}
                                        Err(error) => {
                                            panic!(
                                                "[NetworkingThread] Error sending result: {}",
                                                error
                                            );
                                        }
                                    }
                                }
                                Err(error) => {
                                    match result_sender.send(NetworkingThreadResult::Task(
                                        NetworkingTaskResult::ConnectToServerError(error),
                                    )) {
                                        Ok(_) => {}
                                        Err(error) => {
                                            panic!(
                                                "[NetworkingThread] Error sending result: {}",
                                                error
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    },
                },
                Err(error) => {
                    panic!("[NetworkingThread] Error receiving request: {}", error);
                }
            }
        });

        Self {
            request_sender,
            result_receiver,
            handle,
        }
    }

    pub fn send_task_request(
        &mut self,
        task: NetworkingTask,
    ) -> Result<(), SendError<NetworkingThreadRequest>> {
        match self
            .request_sender
            .send(NetworkingThreadRequest::Task(task))
        {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    pub fn send_command_request(
        &mut self,
        command: NetworkingCommand,
    ) -> Result<(), SendError<NetworkingThreadRequest>> {
        match self
            .request_sender
            .send(NetworkingThreadRequest::Command(command))
        {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    pub fn receive_result(&mut self) -> Result<NetworkingThreadResult, RecvError> {
        match self.result_receiver.recv() {
            Ok(result) => {
                return Ok(result);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}
