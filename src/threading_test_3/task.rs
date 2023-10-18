pub trait TaskInput: Send {}
pub trait TaskOutput: Send {}

pub trait Task: Send {
    type Input: TaskInput;
    type Output: TaskOutput;

    fn new(input: Self::Input) -> Result<Self, String>;

    fn execute(&mut self) -> Result<Self::Output, String>;
}