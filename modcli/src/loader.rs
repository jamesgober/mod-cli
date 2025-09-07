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
}
impl CommandRegistry {

    /// Creates a new command registry
    pub fn new() -> Self {
        let mut reg = Self {
            prefix: String::new(),
            commands: HashMap::new(),
            aliases: HashMap::new(),
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
    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }

    /// Gets a command by name with prefix
    /// Registers a command and records its aliases for reverse lookup.
    pub fn register(&mut self, cmd: Box<dyn Command>) {
        // capture name/aliases before moving the command
        let name = cmd.name().to_string();
        let alias_list: Vec<String> = cmd
            .aliases()
            .iter()
            .map(|a| a.to_string())
            .collect();

        self.commands.insert(name.clone(), cmd);

        // map each alias -> primary name
        for alias in alias_list {
            // avoid alias clobbering existing command names
            if !self.commands.contains_key(&alias) {
                self.aliases.insert(alias, name.clone());
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
    pub fn execute(&self, cmd: &str, args: &[String]) {
        // Handle optional prefix routing: `<prefix>:<command>`
        let mut token = cmd.to_string();
        if !self.prefix.is_empty() {
            let expect = format!("{}:", self.prefix);
            if token.starts_with(&expect) {
                token = token[expect.len()..].to_string();
            }
        }

        // resolve command by direct name or alias
        let resolved_name = if self.commands.contains_key(&token) {
            Some(token.clone())
        } else {
            self.aliases.get(&token).cloned()
        };

        if let Some(name) = resolved_name {
            let command = &self.commands[&name];
            // Validate before execute
            if let Err(err) = command.validate(args) {
                let err_msg = format!("Invalid usage: {}", err);
                hook::error(&err_msg);
                return;
            }
            // Execute with registry context (help and others can leverage it)
            command.execute_with(args, self);
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



    #[cfg(feature = "json-loader")]
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
