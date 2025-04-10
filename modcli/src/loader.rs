use std::collections::HashMap;
use crate::command::Command;

#[cfg(feature = "internal-commands")]
use crate::commands::{PingCommand, EchoCommand, HelloCommand, HelpCommand};

pub mod sources;

use crate::loader::sources::CommandSource;

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

        #[cfg(feature = "internal-commands")]
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
            if command.name() == "help" {
                // Special case for help: render help output with registry context
                if args.len() > 1 {
                    println!("Invalid usage: Too many arguments. Usage: help [command]");
                    return;
                }
    
                if args.len() == 1 {
                    let query = &args[0];
                    if let Some(target) = self.commands.get(query) {
                        if target.hidden() {
                            println!("No help available for '{}'", query);
                        } else {
                            println!(
                                "{} - {}",
                                target.name(),
                                target.help().unwrap_or("No description.")
                            );
                        }
                    } else {
                        println!("Unknown command: {}", query);
                    }
                    return;
                }
    
                println!("Help:");
                for command in self.commands.values() {
                    if !command.hidden() {
                        println!(
                            "  {:<12} {}",
                            command.name(),
                            command.help().unwrap_or("No description")
                        );
                    }
                }
            } else {
                // Normal command execution
                if let Err(err) = command.validate(args) {
                    eprintln!("Invalid usage: {}", err);
                    return;
                }
    
                command.execute(args);
            }
        } else {
            eprintln!("Unknown command: {}", cmd);
        }
    }  

    /// Load built-in internal commands (enabled via feature flag)
    #[cfg(feature = "internal-commands")]
    pub fn load_internal_commands(&mut self) {
        self.register(Box::new(PingCommand));
        self.register(Box::new(EchoCommand));
        self.register(Box::new(HelloCommand));
        self.register(Box::new(HelpCommand::new()));
    }

    /// Load commands dynamically from a source (e.g. JSON, plugin)
    pub fn load_from(&mut self, source: Box<dyn CommandSource>) {
        for cmd in source.load_commands() {
            self.register(cmd);
        }
    }

    /// Number of loaded commands
    pub fn len(&self) -> usize {
        self.commands.len()
    }
}
