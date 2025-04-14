pub mod loader;
pub mod config;
pub mod input;
pub mod output;
pub mod command;
pub mod parser;
use crate::loader::CommandRegistry;
pub use crate::command::Command as CliCustom;

#[cfg(feature = "internal-commands")]
pub mod commands;

#[cfg(feature = "custom-commands")]
pub mod custom;


/// Represents a CLI application
pub struct ModCli {
    pub registry: CommandRegistry,
}

impl ModCli {

    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
        }
    }


    pub fn run(&self, args: Vec<String>) {
        if args.is_empty() {
            eprintln!("No command provided.");
            return;
        }

        let cmd = &args[0];
        let cmd_args = &args[1..];
        self.registry.execute(cmd, cmd_args);
    }
}

/// Returns the version of the ModCLI framework (from modcli/Cargo.toml)
pub fn modcli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}