use futures::channel::mpsc::SendError;

pub trait Thread: Send {
    fn send_command_request(&mut self, command: CommandRequest) -> Result<(), SendError>;
}

pub enum CommandRequest {
    Terminate,
}

pub enum CommandResult {
    TerminateOk,
    TerminateError(String),
}