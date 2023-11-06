pub mod example;

use example::*;

pub struct ThreadingTest3;

impl ThreadingTest3 {
    pub fn main() {
        let mut thread = NetworkingThread::new();

        match thread.send_task_request(NetworkingTask::ConnectToServer(ConnectToServerTask {
            parameters: ConnectToServerTaskParameters {
                server_address: "lauchsreborn.net".to_string(),
                server_port: 25565,
            },
        })) {
            Ok(_) => println!("Sent task request"),
            Err(_) => println!("Failed to send task request"),
        }

        match thread.send_task_request(NetworkingTask::ConnectToServer(ConnectToServerTask {
            parameters: ConnectToServerTaskParameters {
                server_address: "lauchsreborn.net".to_string(),
                server_port: 25565,
            },
        })) {
            Ok(_) => println!("Sent task request"),
            Err(_) => println!("Failed to send task request"),
        }

        match thread.send_command_request(NetworkingCommand::Terminate) {
            Ok(_) => println!("Sent command request"),
            Err(_) => println!("Failed to send command request"),
        }

        loop {
            match thread.receive_result() {
                Ok(result) => match result {
                    NetworkingThreadResult::Task(task_result) => match task_result {
                        NetworkingTaskResult::ConnectToServerOk(_) => {
                            println!("Connected to server");
                            continue;
                        }
                        NetworkingTaskResult::ConnectToServerError(error) => match error {
                            ConnectToServerTaskError::UnexpectedError(error) => {
                                println!("Failed to connect to server: {}", error);
                                break;
                            }
                        },
                    },
                    NetworkingThreadResult::Command(command_result) => match command_result {
                        NetworkingCommandResult::TerminateOk => {
                            println!("Terminated");
                            break;
                        }
                        NetworkingCommandResult::TerminateError(error) => match error {
                            TerminateCommandError::UnexpectedError(error) => {
                                println!("Failed to terminate: {}", error);
                                break;
                            }
                        },
                    },
                },
                Err(_) => {
                    println!("Failed to receive result");
                    break;
                }
            }
        }
    }
}
