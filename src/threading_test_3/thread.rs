pub trait Thread: Send {
    fn send_command_request(&mut self, command: CommandRequest) -> Result<(), String>;
}

pub enum CommandRequest {
    Terminate,
}

pub enum CommandResult {
    TerminateOk,
    TerminateError(String),
}