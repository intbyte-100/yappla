use std::{fmt::Display, process::Command};




pub trait MenuItemModel {
    fn name<'a>(&'a self) -> &'a String;
    fn run_action(&self) -> Result<(), ActionError>;
    
    fn run(&self) {
        let _ =self.run_action();
        std::process::exit(0);
    }
}


pub struct ActionError {
    cause: std::io::Error,
    error: String,
    command: String,
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  Command: {}\n  Cause: {}", self.error, self.command, self.cause)
    }
}



pub struct Application {
    pub name: String,
    pub description: String,
    exec: String,
}

impl Application {
    pub fn new(name: String, description: String, exec: String) -> Self {
        Application {
            name,
            description,
            exec,
        }
    }
}


impl MenuItemModel for Application {
    fn name<'a>(&'a self) -> &'a String {
        &self.name
    }

    fn run_action(&self) -> Result<(), ActionError> {
        Command::new(self.exec.clone())
            .spawn()
            .map(|_| ())
            .map_err(|error| ActionError {
                cause: error,
                error: format!("Failed to start application '{}'", self.name),
                command: self.exec.clone()
            })
    }
}


pub struct ShellCommand {
    pub exec: String,
}

impl ShellCommand {
    pub fn new(exec: String) -> Self {
        ShellCommand { exec }
    }
}


impl MenuItemModel for ShellCommand {
    fn name<'a>(&'a self) -> &'a String {
        &self.exec
    }

    fn run_action(&self) -> Result<(), ActionError> {
        Command::new(self.exec.clone()).spawn().map(|_| ())
            .map_err(|error| ActionError {
                cause: error,
                error: format!("Failed to execute command"),
                command: self.exec.clone()
            })
    }

}

