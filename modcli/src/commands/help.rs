use crate::command::Command;

/// Built-in help command (execution handled by registry internally)
pub struct HelpCommand;

impl HelpCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn aliases(&self) -> &[&str] {
        &["--help", "-h"]
    }

    fn help(&self) -> Option<&str> {
        Some("Displays help information")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if args.len() > 1 {
            Err("Too many arguments. Usage: help [command]".into())
        } else {
            Ok(())
        }
    }

    fn execute(&self, _args: &[String]) {
        // Do nothing — real logic is handled inside registry.execute()
    }
}
