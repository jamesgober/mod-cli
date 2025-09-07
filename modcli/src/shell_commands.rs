use std::sync::{Mutex, OnceLock};

/// Shell command struct with metadata
#[derive(Clone)]
pub struct ShellCommand {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub help: &'static str,
    pub handler: fn(input: &str) -> bool,
}

static HOOKS: OnceLock<Mutex<Vec<ShellCommand>>> = OnceLock::new();

/// Register a new shell-only command from a parent application
pub fn register(cmd: ShellCommand) {
    HOOKS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .push(cmd);
}

/// Dispatches shell input to registered handlers.
/// Returns true if any handler accepted the input.
pub fn dispatch(input: &str) -> bool {
    let input = input.trim();
    if let Some(cmds) = HOOKS.get() {
        for cmd in cmds.lock().unwrap().iter() {
            if cmd.name == input || cmd.aliases.contains(&input) {
                return (cmd.handler)(input);
            }
        }
    }
    false
}

/// Returns all registered shell commands (for dynamic help)
pub fn list() -> Vec<ShellCommand> {
    if let Some(cmds) = HOOKS.get() {
        cmds.lock().unwrap().clone()
    } else {
        vec![]
    }
}
