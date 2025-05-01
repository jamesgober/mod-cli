#[cfg(feature = "custom-commands")]
//pub mod custom;

#[cfg(feature = "custom-commands")]
//use crate::custom::CustomCommand;

#[cfg(feature = "plugins")]
pub mod plugins;

#[cfg(feature = "plugins")]
use crate::loader::plugins::PluginLoader;

#[cfg(feature = "internal-commands")]
use crate::commands::{
    PingCommand, 
    HelloCommand, 
    ShellCommand,
    HelpCommand,
    FrameworkCommand
};
use crate::output::hook;

use std::collections::HashMap;
use crate::command::Command;
use crate::loader::sources::CommandSource;
pub mod sources;

pub struct CommandRegistry {
    prefix: String,
    commands: HashMap<String, Box<dyn Command>>,
}
impl CommandRegistry {

    /// Creates a new command registry
    pub fn new() -> Self {
        let mut reg = Self {
            prefix: String::new(),
            commands: HashMap::new(),
        };

        #[cfg(feature = "custom-commands")]
        reg.load_custom_commands();

        #[cfg(feature = "internal-commands")]
        reg.load_internal_commands();

        reg
    }

    /// Sets the command prefix
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }

    /// Gets the command prefix
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
 
    /// Gets a command by name
    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }

    /// Gets a command by name with prefix
    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    /// Registers a command with an alias
    #[cfg(feature = "plugins")]
    pub fn load_plugins(&mut self, path: &str) {
        let loader = PluginLoader::new(path);
        for plugin in loader.load_plugins() {
            self.register(plugin);
        }
    }

    pub fn execute(&self, cmd: &str, args: &[String]) {
        if let Some(command) = self.commands.get(cmd) {
            if command.name() == "help" {
                // Special case for help: render help output with registry context
                if args.len() > 1 {
                    hook::error("Invalid usage: Too many arguments. Usage: help [command]");
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
                        let unknown = format!("[{}]. Type `help` or `--help` for a list of available commands.", query);
                        hook::unknown(&unknown);
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
                    let err_msg = format!("Invalid usage: {}", err);
                    hook::error(&err_msg);
                    command.execute(args);
                    return;
                }

                command.execute(args);
            }
        } else {
            let unknown = format!("[{}]. Type `help` or `--help` for a list of available commands.", cmd);
            hook::unknown(&unknown);
        }
    }


    #[cfg(feature = "internal-commands")]
    pub fn load_internal_commands(&mut self) {
        self.register(Box::new(PingCommand));
        self.register(Box::new(HelloCommand));
        self.register(Box::new(ShellCommand));
        self.register(Box::new(FrameworkCommand));
        self.register(Box::new(HelpCommand::new()));
    }



    pub fn load_from(&mut self, source: Box<dyn CommandSource>) {
        for cmd in source.load_commands() {
            self.register(cmd);
        }
    }


    pub fn len(&self) -> usize {
        self.commands.len()
    }

    #[cfg(feature = "custom-commands")]
    pub fn load_custom_commands(&mut self) {
       //self.register(Box::new(CustomCommand));
    }

}
