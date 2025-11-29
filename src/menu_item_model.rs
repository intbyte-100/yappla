use std::{fmt::Display, io, process::Command};

pub enum MenuItemModel {
    Application(Application),
    Command(ShellCommand),
    String(String),
    None,
}

pub struct ActionError {
    cause: io::Error,
    error: String,
    command: String,
}


impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  Command: {}\n  Cause: {}", self.error, self.command, self.cause)
    }
}


impl MenuItemModel {
    pub fn run_action(&self) -> Result<(), ActionError> {
        match self {
            MenuItemModel::Application(application) => application.launch(),
            MenuItemModel::Command(shell_command) => shell_command.launch(),
            MenuItemModel::String(string) => {
                println!("{}", string);
                Ok(())
            }
            MenuItemModel::None => panic!("LaucherItem::None cannot be launched."),
        }
    }

    pub fn name(&self) -> &String {
        match self {
            MenuItemModel::Application(application) => &application.name,
            MenuItemModel::Command(shell_command) => &shell_command.exec,
            MenuItemModel::String(string) => string,
            MenuItemModel::None => panic!("You can't get name from LauncherItem::None"),
        }
    }
}

impl Default for MenuItemModel {
    fn default() -> Self {
        MenuItemModel::None
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

    fn launch(&self) -> Result<(), ActionError> {
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

    fn launch(&self) -> Result<(), ActionError> {
        Command::new(self.exec.clone()).spawn().map(|_| ())
            .map_err(|error| ActionError {
                cause: error,
                error: format!("Failed to execute command"),
                command: self.exec.clone()
            })
    }
}

impl Into<MenuItemModel> for Application {
    fn into(self) -> MenuItemModel {
        MenuItemModel::Application(self)
    }
}

impl Into<MenuItemModel> for ShellCommand {
    fn into(self) -> MenuItemModel {
        MenuItemModel::Command(self)
    }
}

impl Into<MenuItemModel> for String {
    fn into(self) -> MenuItemModel {
        MenuItemModel::String(self)
    }
}
