use std::{fmt::Display, io, process::Command};

pub enum LauncherItem {
    Application(Application),
    Command(ShellCommand),
    String(String),
    None,
}

pub struct LaunchError {
    cause: io::Error,
    error: String,
    command: String,
}


impl Display for LaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  Command: {}\n  Cause: {}", self.error, self.command, self.cause)
    }
}
impl LauncherItem {
    pub fn launch(&self) -> Result<(), LaunchError> {
        match self {
            LauncherItem::Application(application) => application.launch(),
            LauncherItem::Command(shell_command) => shell_command.launch(),
            LauncherItem::String(string) => {
                println!("{}", string);
                Ok(())
            }
            LauncherItem::None => panic!("LaucherItem::None cannot be launched."),
        }
    }

    pub fn name(&self) -> &String {
        match self {
            LauncherItem::Application(application) => &application.name,
            LauncherItem::Command(shell_command) => &shell_command.exec,
            LauncherItem::String(string) => string,
            LauncherItem::None => panic!("You can't get name from LauncherItem::None"),
        }
    }
}

impl Default for LauncherItem {
    fn default() -> Self {
        LauncherItem::None
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

    fn launch(&self) -> Result<(), LaunchError> {
        Command::new(self.exec.clone())
            .spawn()
            .map(|_| ())
            .map_err(|error| LaunchError {
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

    fn launch(&self) -> Result<(), LaunchError> {
        Command::new(self.exec.clone()).spawn().map(|_| ())
            .map_err(|error| LaunchError {
                cause: error,
                error: format!("Failed to execute command"),
                command: self.exec.clone()
            })
    }
}

impl Into<LauncherItem> for Application {
    fn into(self) -> LauncherItem {
        LauncherItem::Application(self)
    }
}

impl Into<LauncherItem> for ShellCommand {
    fn into(self) -> LauncherItem {
        LauncherItem::Command(self)
    }
}

impl Into<LauncherItem> for String {
    fn into(self) -> LauncherItem {
        LauncherItem::String(self)
    }
}
