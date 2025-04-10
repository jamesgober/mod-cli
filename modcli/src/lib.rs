pub mod command;
pub mod parser;
pub mod loader;
pub mod output;

#[cfg(feature = "internal-commands")]
pub mod commands;

use loader::CommandRegistry;

/// Main CLI framework interface
pub struct ModCli {
    pub registry: CommandRegistry,
}

impl ModCli {
    /// Creates a new CLI instance with registered commands
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
        }
    }

    /// Runs the CLI logic with given args
    pub fn run(&mut self, args: Vec<String>) {
        if args.is_empty() {
            eprintln!("No command provided.");
            return;
        }

        let cmd = &args[0];
        let cmd_args = &args[1..];
        self.registry.execute(cmd, cmd_args);
    }
}
