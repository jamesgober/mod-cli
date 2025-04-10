use std::collections::HashMap;
use crate::command::Command;

#[cfg(feature = "internal-commands")]
use crate::commands::{PingCommand, EchoCommand, HelloCommand, HelpCommand};

/// Registry for available CLI commands
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    /// Create a new command registry and register internal commands (if enabled)
    pub fn new() -> Self {
        let mut reg = Self {
            commands: HashMap::new(),
        };

        reg.load_internal_commands();
        reg
    }

    /// Register a new command
    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    /// Execute a command if it exists, passing args
    pub fn execute(&self, cmd: &str, args: &[String]) {
        if let Some(command) = self.commands.get(cmd) {
            if let Err(reason) = command.validate(args) {
                eprintln!("Invalid usage: {}", reason);
            } else {
                command.execute(args);
            }
        } else {
            eprintln!("Unknown command: {}", cmd);
        }
    }

    /// Returns the number of registered commands
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Load built-in internal commands if the feature is enabled
    fn load_internal_commands(&mut self) {
        #[cfg(feature = "internal-commands")]
        {
            self.register(Box::new(PingCommand));
            self.register(Box::new(EchoCommand));
            self.register(Box::new(HelloCommand));
            self.register(Box::new(HelpCommand::new()));
        }
    }

}
