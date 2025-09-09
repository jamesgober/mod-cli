// pub mod custom; // feature = "custom-commands"
// use crate::custom::CustomCommand; // feature = "custom-commands"

#[cfg(feature = "internal-commands")]
use crate::commands::{FrameworkCommand, HelloCommand, HelpCommand, PingCommand};
use crate::output::hook;

#[cfg(feature = "async")]
use crate::command::AsyncCommand;
use crate::command::Command;
#[allow(unused_imports)]
use crate::error::ModCliError;
use std::collections::{HashMap, HashSet};

// Reduce type complexity for registry hooks and error formatter
type PreHookFn = dyn Fn(&str, &[String]) + Send + Sync;
type PostHookFn = dyn Fn(&str, &[String], Result<(), &str>) + Send + Sync;
type ErrorFmtFn = dyn Fn(&crate::error::ModCliError) -> String + Send + Sync;
type VisibilityPolicyFn = dyn Fn(&dyn Command, &HashSet<String>) -> bool + Send + Sync;
type AuthorizePolicyFn =
    dyn Fn(&dyn Command, &HashSet<String>, &[String]) -> Result<(), String> + Send + Sync;

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
    #[cfg(feature = "async")]
    async_commands: HashMap<String, Box<dyn AsyncCommand>>, // separate store for async commands
    #[cfg(feature = "async")]
    async_aliases: HashMap<String, String>,
    caps: HashSet<String>,
    visibility_policy: Option<Box<VisibilityPolicyFn>>,
    authorize_policy: Option<Box<AuthorizePolicyFn>>,
    pre_hook: Option<Box<PreHookFn>>,   // before dispatch
    post_hook: Option<Box<PostHookFn>>, // after dispatch
    error_formatter: Option<Box<ErrorFmtFn>>,
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
            #[cfg(feature = "async")]
            async_commands: HashMap::new(),
            #[cfg(feature = "async")]
            async_aliases: HashMap::new(),
            caps: HashSet::new(),
            visibility_policy: None,
            authorize_policy: None,
            pre_hook: None,
            post_hook: None,
            error_formatter: None,
            #[cfg(feature = "dispatch-cache")]
            cache: std::sync::Mutex::new(None),
        };

        #[cfg(feature = "custom-commands")]
        reg.load_custom_commands();

        #[cfg(feature = "internal-commands")]
        reg.load_internal_commands();

        reg
    }

    /// Register an async command (feature: "async")
    #[cfg(feature = "async")]
    pub fn register_async(&mut self, cmd: Box<dyn AsyncCommand>) {
        let name = cmd.name().to_string();
        self.async_commands.insert(name.clone(), cmd);
        for &alias in self.async_commands[&name].aliases() {
            if !self.async_commands.contains_key(alias) {
                self.async_aliases.insert(alias.to_string(), name.clone());
            }
        }
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

    /// Returns all registered async commands (read-only)
    #[cfg(feature = "async")]
    pub fn all_async(&self) -> impl Iterator<Item = &Box<dyn AsyncCommand>> {
        self.async_commands.values()
    }

    // --- ASYNC DISPATCH (feature: "async") ---------------------------------
    #[cfg(feature = "async")]
    #[inline(always)]
    pub async fn try_execute_async(&self, cmd: &str, args: &[String]) -> Result<(), ModCliError> {
        if let Some(ref pre) = self.pre_hook {
            pre(cmd, args);
        }

        // Strip optional prefix from the incoming token
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

        // Direct name
        if let Some(command) = self.async_commands.get(token) {
            if let Err(e) = self.is_authorized_async(args) {
                return Err(ModCliError::InvalidUsage(e));
            }
            command.execute_async(args).await?;
            if let Some(ref post) = self.post_hook {
                post(cmd, args, Ok(()));
            }
            return Ok(());
        }

        // Alias
        if let Some(primary) = self.async_aliases.get(token) {
            if let Some(command) = self.async_commands.get(primary.as_str()) {
                if let Err(e) = self.is_authorized_async(args) {
                    return Err(ModCliError::InvalidUsage(e));
                }
                command.execute_async(args).await?;
                if let Some(ref post) = self.post_hook {
                    post(cmd, args, Ok(()));
                }
                return Ok(());
            }
        }

        // Two-token nested: parent child -> parent:child
        if !args.is_empty() {
            let combined = format!("{token}:{}", args[0]);
            if let Some(command) = self.async_commands.get(combined.as_str()) {
                let rest = &args[1..];
                if let Err(e) = self.is_authorized_async(rest) {
                    return Err(ModCliError::InvalidUsage(e));
                }
                command.execute_async(rest).await?;
                if let Some(ref post) = self.post_hook {
                    post(cmd, args, Ok(()));
                }
                return Ok(());
            }
        }

        if let Some(ref post) = self.post_hook {
            post(cmd, args, Err("unknown"));
        }
        Err(ModCliError::UnknownCommand(cmd.to_string()))
    }

    /// Execute async and print user-friendly messages
    #[cfg(feature = "async")]
    #[inline(always)]
    pub async fn execute_async(&self, cmd: &str, args: &[String]) {
        if let Err(err) = self.try_execute_async(cmd, args).await {
            if let Some(ref fmt) = self.error_formatter {
                hook::error(&fmt(&err));
            } else {
                match err {
                    ModCliError::InvalidUsage(msg) => hook::error(&format!("Invalid usage: {msg}")),
                    ModCliError::UnknownCommand(name) => hook::unknown(&format!(
                        "[{name}]. Type `help` or `--help` for a list of available commands."
                    )),
                    other => hook::error(&format!("{other}")),
                }
            }
        }
    }

    // Authorization shim to reuse existing policy contract for async commands
    #[cfg(feature = "async")]
    #[inline(always)]
    fn is_authorized_async(&self, args: &[String]) -> Result<(), String> {
        if let Some(ref pol) = self.authorize_policy {
            struct Dummy;
            impl Command for Dummy {
                fn name(&self) -> &str {
                    "__async_dummy__"
                }
                fn execute(&self, _args: &[String]) {}
            }
            return pol(&Dummy, &self.caps, args);
        }
        Ok(())
    }

    // --- Capabilities API -----------------------------------------------------
    pub fn grant_cap<S: Into<String>>(&mut self, cap: S) {
        self.caps.insert(cap.into());
    }
    pub fn revoke_cap(&mut self, cap: &str) {
        self.caps.remove(cap);
    }
    pub fn has_cap(&self, cap: &str) -> bool {
        self.caps.contains(cap)
    }
    pub fn set_caps<I, S>(&mut self, caps: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.caps.clear();
        for c in caps {
            self.caps.insert(c.into());
        }
    }

    pub fn set_visibility_policy<F>(&mut self, f: F)
    where
        F: Fn(&dyn Command, &HashSet<String>) -> bool + Send + Sync + 'static,
    {
        self.visibility_policy = Some(Box::new(f));
    }

    pub fn set_authorize_policy<F>(&mut self, f: F)
    where
        F: Fn(&dyn Command, &HashSet<String>, &[String]) -> Result<(), String>
            + Send
            + Sync
            + 'static,
    {
        self.authorize_policy = Some(Box::new(f));
    }

    pub fn set_pre_hook<F>(&mut self, f: F)
    where
        F: Fn(&str, &[String]) + Send + Sync + 'static,
    {
        self.pre_hook = Some(Box::new(f));
    }

    pub fn set_post_hook<F>(&mut self, f: F)
    where
        F: Fn(&str, &[String], Result<(), &str>) + Send + Sync + 'static,
    {
        self.post_hook = Some(Box::new(f));
    }

    pub fn set_error_formatter<F>(&mut self, f: F)
    where
        F: Fn(&crate::error::ModCliError) -> String + Send + Sync + 'static,
    {
        self.error_formatter = Some(Box::new(f));
    }

    #[inline(always)]
    pub fn is_visible(&self, cmd: &dyn Command) -> bool {
        if let Some(ref pol) = self.visibility_policy {
            return pol(cmd, &self.caps);
        }
        if cmd.hidden() {
            return false;
        }
        cmd.required_caps().iter().all(|c| self.caps.contains(*c))
    }

    #[inline(always)]
    pub fn is_authorized(&self, cmd: &dyn Command, args: &[String]) -> Result<(), String> {
        if let Some(ref pol) = self.authorize_policy {
            return pol(cmd, &self.caps, args);
        }
        if cmd.required_caps().iter().all(|c| self.caps.contains(*c)) {
            Ok(())
        } else {
            Err("Not authorized".into())
        }
    }

    // Note: runtime plugin loading has been removed from core for security/perf.

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
            if let Some(ref fmt) = self.error_formatter {
                hook::error(&fmt(&err));
            } else {
                match err {
                    ModCliError::InvalidUsage(msg) => hook::error(&format!("Invalid usage: {msg}")),
                    ModCliError::UnknownCommand(name) => hook::unknown(&format!(
                        "[{name}]. Type `help` or `--help` for a list of available commands."
                    )),
                    other => hook::error(&format!("{other}")),
                }
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
        if let Some(ref pre) = self.pre_hook {
            pre(cmd, args);
        }
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
                        command.validate(args)?;
                        command.execute_with(args, self);
                        return Ok(());
                    }
                }
            }
        }

        // Try direct name
        if let Some(command) = self.commands.get(token) {
            if let Err(err) = self.is_authorized(command.as_ref(), args) {
                return Err(ModCliError::InvalidUsage(err));
            }
            command.validate(args)?;
            command.execute_with(args, self);
            #[cfg(feature = "dispatch-cache")]
            if let Ok(mut guard) = self.cache.lock() {
                *guard = Some((token.to_string(), token.to_string()));
            }
            if let Some(ref post) = self.post_hook {
                post(cmd, args, Ok(()));
            }
            return Ok(());
        }

        // Try alias mapping
        if let Some(primary) = self.aliases.get(token) {
            if let Some(command) = self.commands.get(primary.as_str()) {
                if let Err(err) = self.is_authorized(command.as_ref(), args) {
                    return Err(ModCliError::InvalidUsage(err));
                }
                command.validate(args)?;
                command.execute_with(args, self);
                #[cfg(feature = "dispatch-cache")]
                if let Ok(mut guard) = self.cache.lock() {
                    *guard = Some((token.to_string(), primary.clone()));
                }
                if let Some(ref post) = self.post_hook {
                    post(cmd, args, Ok(()));
                }
                return Ok(());
            }
        }

        // Two-token nested dispatch: "parent child ..." -> "parent:child"
        if !args.is_empty() {
            let combined = format!("{token}:{}", args[0]);
            if let Some(command) = self.commands.get(combined.as_str()) {
                let rest = &args[1..];
                if let Err(err) = self.is_authorized(command.as_ref(), rest) {
                    return Err(ModCliError::InvalidUsage(err));
                }
                command.validate(rest)?;
                command.execute_with(rest, self);
                if let Some(ref post) = self.post_hook {
                    post(cmd, args, Ok(()));
                }
                return Ok(());
            }
        }
        let err = ModCliError::UnknownCommand(cmd.to_string());
        if let Some(ref post) = self.post_hook {
            post(cmd, args, Err("unknown"));
        }
        Err(err)
    }

    #[cfg(feature = "internal-commands")]
    pub fn load_internal_commands(&mut self) {
        self.register(Box::new(PingCommand));
        self.register(Box::new(HelloCommand));
        self.register(Box::new(FrameworkCommand));
        self.register(Box::new(HelpCommand::new()));
    }

    // Note: JSON loader has been removed from core. Use code registration.

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
