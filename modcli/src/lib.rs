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

use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock,
};

pub mod args;
pub mod command;
pub mod error;
pub mod input;
pub mod loader;
pub mod output;
pub mod parser;
pub mod shell;
pub mod validate;

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

/// Registers a startup banner from a UTF-8 text file. The contents are read immediately
/// and stored; at runtime the stored text is printed when the banner runs.
/// Returns Err if the file cannot be read.
pub fn set_startup_banner_from_file(path: &str) -> Result<(), crate::error::ModCliError> {
    let data = std::fs::read_to_string(path)?;
    let owned = data.clone();
    set_startup_banner(move || {
        println!("{owned}\n");
    });
    Ok(())
}

// --- Macros ------------------------------------------------------------------

/// Register a simple text banner that prints a single line and a newline.
#[macro_export]
macro_rules! banner_text {
    ($text:expr) => {{
        $crate::set_startup_banner(|| {
            $crate::output::print::line($text);
            println!();
        });
    }};
}

/// Register a banner from a file path (evaluated at runtime).
#[macro_export]
macro_rules! banner_file {
    ($path:expr) => {{
        let _ = $crate::set_startup_banner_from_file($path);
    }};
}

/// Register a banner with custom code using a block. Example:
/// banner!({ println!("Hello"); })
#[macro_export]
macro_rules! banner {
    ($body:block) => {{
        $crate::set_startup_banner(|| $body);
    }};
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
        run_startup_banner_if_enabled();
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

// --- Startup banner hook -----------------------------------------------------

static STARTUP_BANNER: OnceLock<Box<dyn Fn() + Send + Sync>> = OnceLock::new();
static BANNER_RAN: AtomicBool = AtomicBool::new(false);

/// Registers a startup banner callback that will be invoked once, the first time
/// `ModCli::run()` is called in this process. If the environment variable
/// `MODCLI_DISABLE_BANNER` is set to "1" or "true" (case-insensitive), the
/// banner will be suppressed.
///
/// Note: This can only be set once per process.
pub fn set_startup_banner<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let _ = STARTUP_BANNER.set(Box::new(f));
}

fn run_startup_banner_if_enabled() {
    // Ensure one-time run per process
    if BANNER_RAN.swap(true, Ordering::SeqCst) {
        return;
    }
    // Allow disabling via env var
    if let Ok(val) = std::env::var("MODCLI_DISABLE_BANNER") {
        if val == "1" || val.eq_ignore_ascii_case("true") {
            return;
        }
    }
    if let Some(cb) = STARTUP_BANNER.get() {
        cb();
    }
}
