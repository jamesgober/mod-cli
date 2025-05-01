pub mod loader;
pub mod config;
pub mod input;
pub mod output;
pub mod command;
pub mod parser;
pub mod console;
pub mod shell_extensions;
pub mod shell_commands;

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
    /// Creates a new ModCli instance
    ///
    /// # Example
    /// ```
    /// use modcli::ModCli;
    /// let cli = ModCli::new();
    /// ```
    ///
    /// # Arguments
    /// * `args` - A vector of command-line arguments
    ///
    /// # Returns
    /// A new instance of `ModCli`
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
        }
    }

    /// Sets the command prefix
    pub fn set_prefix(&mut self, prefix: &str) {
        self.registry.set_prefix(prefix);
    }

    /// Gets the command prefix
    pub fn get_prefix(&self) -> &str {
        self.registry.get_prefix()
    }

    /// Preferred constructor: sets config path before CLI boot
    pub fn with_config(path: &str) -> Self {
        config::set_path(path);
        Self::new()
    }

    pub fn run(&mut self, args: Vec<String>) {
        if args.is_empty() {
            eprintln!("No command provided.");
            return;
        }

        let command = &args[0];
        let rest = &args[1..];

        self.registry.execute(command, rest);
    }
}

/// Returns the version of the ModCLI framework (from modcli/Cargo.toml)
pub fn modcli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}