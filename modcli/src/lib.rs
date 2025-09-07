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
//! - Interactive shell (via built-in `shell` command)
//! - Optional JSON command loading (`json-loader`)
//! - Optional plugin loading (`plugins`)
//!
//! ## JSON Loader (feature: `json-loader`)
//! ```no_run
//! use modcli::ModCli;
//! #[cfg(feature = "json-loader")]
//! use modcli::loader::sources::JsonFileSource;
//! let mut cli = ModCli::new();
//! #[cfg(feature = "json-loader")]
//! {
//!     let source = JsonFileSource::new("modcli/examples/commands.json");
//!     cli.registry.load_from(Box::new(source));
//! }
//! let args: Vec<String> = std::env::args().skip(1).collect();
//! cli.run(args);
//! ```
//!
//! ## Plugins (feature: `plugins`)
//! ```no_run
//! use modcli::ModCli;
//! let mut cli = ModCli::new();
//! #[cfg(feature = "plugins")]
//! {
//!     cli.registry.load_plugins("./plugins");
//! }
//! let args: Vec<String> = std::env::args().skip(1).collect();
//! cli.run(args);
//! ```

pub mod command;
pub mod config;
pub mod console;
pub mod input;
pub mod loader;
pub mod output;
pub mod parser;
pub mod shell_commands;
pub mod shell_extensions;
pub mod error;

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
    config: Option<config::CliConfig>,
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
            config: None,
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

    /// Preferred constructor: sets config path before CLI boot.
    pub fn with_config(path: &str) -> Self {
        config::set_path(path);
        Self::new()
    }

    /// Construct with an owned configuration (non-global). Prefer this in library usage/tests.
    pub fn with_owned_config(cfg: config::CliConfig) -> Self {
        let mut s = Self::new();
        s.apply_config(&cfg);
        s.config = Some(cfg);
        s
    }

    fn apply_config(&mut self, cfg: &config::CliConfig) {
        if let Some(prefix) = cfg.modcli.prefix.as_deref() {
            self.set_prefix(prefix);
        }
    }

    /// Runs the CLI by dispatching the first arg as the command and the rest as arguments.
    /// Prints an error if no command is provided.
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

/// Returns the version of the ModCLI framework (from `modcli/Cargo.toml`).
///
/// Useful for surfacing framework version from applications.
pub fn modcli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
