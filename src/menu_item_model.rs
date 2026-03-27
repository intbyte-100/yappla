use std::fmt::Display;

pub trait MenuItemModel {
    fn name<'a>(&'a self) -> &'a String;
    fn run_action(&self) -> Result<(), ActionError>;
    
    fn run(&self) {
        let _ =self.run_action();
        std::process::exit(0);
    }
}


pub struct ActionError {
    pub(crate)cause: std::io::Error,
    pub(crate)error: String,
    pub(crate) command: String,
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  Command: {}\n  Cause: {}", self.error, self.command, self.cause)
    }
}