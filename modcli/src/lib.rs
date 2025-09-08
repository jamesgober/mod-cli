//! ModCLI — a lightweight, modular CLI framework for Rust.
//!
//! # Quick Start
//! ```no_run
//! use modcli::ModCli;
//! let args: Vec<String> = std::env::args().skip(1).collect();
//! let mut cli = ModCli::new();
//! cli.run(args);
//! ```
//!
//! # Features
//! - Custom commands via the `Command` trait
//! - Styled output, gradients, progress, tables
//! - Optional internal helper commands
//!
//! Note: Runtime plugins and JSON/config loaders have been removed from core for
//! security and performance. Configure your CLI directly in code.

pub mod command;
pub mod error;
pub mod input;
pub mod loader;
pub mod output;
pub mod parser;

pub use crate::command::Command as CliCustom;
use crate::loader::CommandRegistry;

#[cfg(feature = "internal-commands")]
pub mod commands;

#[cfg(feature = "custom-commands")]
pub mod custom;

/// Represents a CLI application and provides command registration and dispatch.
///
/// Typical usage:
/// ```no_run
/// use modcli::ModCli;
/// let args: Vec<String> = std::env::args().skip(1).collect();
/// let mut cli = ModCli::new();
/// cli.run(args);
/// ```
pub struct ModCli {
    pub registry: CommandRegistry,
}

impl Default for ModCli {
    fn default() -> Self {
        Self::new()
    }
}

impl ModCli {
    /// Creates a new ModCli instance.
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

    /// Sets the command prefix used for prefix routing (e.g., `tool:hello`).
    pub fn set_prefix(&mut self, prefix: &str) {
        self.registry.set_prefix(prefix);
    }

    /// Gets the current command prefix.
    pub fn get_prefix(&self) -> &str {
        self.registry.get_prefix()
    }

    /// Runs the CLI by dispatching the first arg as the command and the rest as arguments.
    /// Prints an error if no command is provided.
    pub fn run(&mut self, args: Vec<String>) {
        if args.is_empty() {
            crate::output::hook::status("No command provided. Try `help`.");
            return;
        }

        let command = &args[0];
        let rest = &args[1..];

        self.registry.execute(command, rest);
    }
}

/// Returns the version of the ModCLI framework (from `modcli/Cargo.toml`).
///
/// Useful for surfacing framework version from applications.
pub fn modcli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
