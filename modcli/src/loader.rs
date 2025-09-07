// pub mod custom; // feature = "custom-commands"
// use crate::custom::CustomCommand; // feature = "custom-commands"
#[cfg(feature = "plugins")]
pub mod plugins;

#[cfg(feature = "plugins")]
use crate::loader::plugins::PluginLoader;

#[cfg(feature = "internal-commands")]
use crate::commands::{FrameworkCommand, HelloCommand, HelpCommand, PingCommand, ShellCommand};
use crate::output::hook;

use crate::command::Command;
use crate::error::ModCliError;
use std::collections::HashMap;

#[cfg(feature = "json-loader")]
use crate::loader::sources::CommandSource;

#[cfg(feature = "json-loader")]
pub mod sources;

/// Registry for commands and optional alias/prefix routing.
///
/// # Example
/// ```no_run
/// use modcli::loader::CommandRegistry;
/// use modcli::command::Command;
///
/// struct Echo;
/// impl Command for Echo {
///     fn name(&self) -> &str { "echo" }
///     fn execute(&self, args: &[String]) { println!("{}", args.join(" ")) }
/// }
///
/// let mut reg = CommandRegistry::new();
/// reg.register(Box::new(Echo));
/// reg.execute("echo", &["hi".into()]);
/// ```
pub struct CommandRegistry {
    prefix: String,
    commands: HashMap<String, Box<dyn Command>>,
    aliases: HashMap<String, String>,
    #[cfg(feature = "dispatch-cache")]
    cache: std::sync::Mutex<Option<(String, String)>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    /// Creates a new command registry
    pub fn new() -> Self {
        let mut reg = Self {
            prefix: String::new(),
            commands: HashMap::new(),
            aliases: HashMap::new(),
            #[cfg(feature = "dispatch-cache")]
            cache: std::sync::Mutex::new(None),
        };

        #[cfg(feature = "custom-commands")]
        reg.load_custom_commands();

        #[cfg(feature = "internal-commands")]
        reg.load_internal_commands();

        reg
    }

    /// Sets the command prefix
    /// Sets an optional prefix used for routing commands of the form `prefix:cmd`.
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }

    /// Gets the command prefix
    /// Returns the configured prefix (empty string if not set).
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    /// Gets a command by name
    /// Gets a command by its primary name.
    #[inline(always)]
    pub fn get(&self, name: &str) -> Option<&dyn Command> {
        self.commands.get(name).map(|b| b.as_ref())
    }

    /// Gets a command by name with prefix
    /// Registers a command and records its aliases for reverse lookup.
    #[inline(always)]
    pub fn register(&mut self, cmd: Box<dyn Command>) {
        // capture name before moving the command
        let name = cmd.name().to_string();
        self.commands.insert(name.clone(), cmd);

        // map each alias -> primary name without intermediate Vec allocations
        for &alias in self.commands[&name].aliases() {
            // avoid alias clobbering existing command names
            if !self.commands.contains_key(alias) {
                // store alias as owned String
                self.aliases.insert(alias.to_string(), name.clone());
            }
        }
    }

    /// Returns all registered commands (read-only)
    /// Returns an iterator over all registered commands.
    pub fn all(&self) -> impl Iterator<Item = &Box<dyn Command>> {
        self.commands.values()
    }

    /// Registers a command with an alias
    #[cfg(feature = "plugins")]
    pub fn load_plugins(&mut self, path: &str) {
        let loader = PluginLoader::new(path);
        for plugin in loader.load_plugins() {
            self.register(plugin);
        }
    }

    /// Resolves and executes a command by name or alias, with optional prefix routing.
    ///
    /// Behavior:
    /// - Applies optional prefix routing (e.g., `tool:hello`).
    /// - Resolves aliases to primary command names.
    /// - Validates args via `Command::validate()` and logs a themed error on failure.
    /// - Executes the command via `execute_with()`.
    /// - Prints user-facing messages via `output::hook` and does not return an error.
    ///
    /// Example (illustrative):
    /// ```ignore
    /// use modcli::loader::CommandRegistry;
    /// let reg = CommandRegistry::new();
    /// // Will log an unknown command message via output hooks
    /// reg.execute("does-not-exist", &vec![]);
    /// ```
    #[inline(always)]
    pub fn execute(&self, cmd: &str, args: &[String]) {
        if let Err(err) = self.try_execute(cmd, args) {
            match err {
                ModCliError::InvalidUsage(msg) => hook::error(&format!("Invalid usage: {msg}")),
                ModCliError::UnknownCommand(name) => hook::unknown(&format!(
                    "[{name}]. Type `help` or `--help` for a list of available commands."
                )),
                other => hook::error(&format!("{other}")),
            }
        }
    }

    /// Resolves and executes a command by name or alias, with optional prefix routing.
    /// Returns a structured error instead of printing/logging directly.
    ///
    /// Error mapping:
    /// - `InvalidUsage(String)`: when `validate()` returns an error string.
    /// - `UnknownCommand(String)`: command not found after alias/prefix resolution.
    ///
    /// Examples (illustrative):
    ///
    /// ```ignore
    /// use modcli::loader::CommandRegistry;
    /// // Assume `reg` has commands registered
    /// let reg = CommandRegistry::new();
    /// // Success
    /// let _ = reg.try_execute("help", &vec![]);
    /// // Error mapping (unknown)
    /// match reg.try_execute("does-not-exist", &vec![]) {
    ///     Err(modcli::error::ModCliError::UnknownCommand(name)) => assert_eq!(name, "does-not-exist"),
    ///     _ => {}
    /// }
    /// ```
    #[inline(always)]
    pub fn try_execute(&self, cmd: &str, args: &[String]) -> Result<(), ModCliError> {
        // Strip optional prefix `<prefix>:` without intermediate allocations
        let token: &str = if !self.prefix.is_empty() && cmd.len() > self.prefix.len() + 1 {
            let (maybe_prefix, rest_with_colon) = cmd.split_at(self.prefix.len());
            if maybe_prefix == self.prefix && rest_with_colon.as_bytes().first() == Some(&b':') {
                &rest_with_colon[1..]
            } else {
                cmd
            }
        } else {
            cmd
        };

        #[cfg(feature = "dispatch-cache")]
        if let Ok(guard) = self.cache.lock() {
            if let Some((ref t, ref p)) = *guard {
                if t == token {
                    if let Some(command) = self.commands.get(p.as_str()) {
                        if let Err(err) = command.validate(args) {
                            return Err(ModCliError::InvalidUsage(err));
                        }
                        command.execute_with(args, self);
                        return Ok(());
                    }
                }
            }
        }

        // Resolve command by direct name or alias with at most two lookups
        if let Some(command) = self.commands.get(token) {
            if let Err(err) = command.validate(args) {
                return Err(ModCliError::InvalidUsage(err));
            }
            command.execute_with(args, self);
            #[cfg(feature = "dispatch-cache")]
            if let Ok(mut guard) = self.cache.lock() {
                *guard = Some((token.to_string(), token.to_string()));
            }
            return Ok(());
        }

        if let Some(primary) = self.aliases.get(token) {
            if let Some(command) = self.commands.get(primary.as_str()) {
                if let Err(err) = command.validate(args) {
                    return Err(ModCliError::InvalidUsage(err));
                }
                command.execute_with(args, self);
                #[cfg(feature = "dispatch-cache")]
                if let Ok(mut guard) = self.cache.lock() {
                    *guard = Some((token.to_string(), primary.clone()));
                }
                return Ok(());
            }
        }

        Err(ModCliError::UnknownCommand(cmd.to_string()))
    }

    #[cfg(feature = "internal-commands")]
    pub fn load_internal_commands(&mut self) {
        self.register(Box::new(PingCommand));
        self.register(Box::new(HelloCommand));
        self.register(Box::new(ShellCommand));
        self.register(Box::new(FrameworkCommand));
        self.register(Box::new(HelpCommand::new()));
    }

    #[cfg(feature = "json-loader")]
    pub fn load_from(&mut self, source: Box<dyn CommandSource>) {
        for cmd in source.load_commands() {
            self.register(cmd);
        }
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    #[cfg(feature = "custom-commands")]
    pub fn load_custom_commands(&mut self) {
        //self.register(Box::new(CustomCommand));
    }
}
