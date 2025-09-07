use crate::command::Command;
use crate::loader::CommandRegistry;
use crate::output::hook;

/// Built-in help command (execution handled by registry internally)
pub struct HelpCommand;

impl Default for HelpCommand {
    fn default() -> Self {
        Self::new()
    }
}

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

    fn execute(&self, _args: &[String]) {}

    fn execute_with(&self, args: &[String], registry: &CommandRegistry) {
        // validate() already ensures args.len() <= 1
        if args.len() == 1 {
            let query = &args[0];
            if let Some(target) = registry.get(query) {
                if target.hidden() {
                    println!("No help available for '{query}'");
                } else {
                    println!(
                        "{} - {}",
                        target.name(),
                        target.help().unwrap_or("No description.")
                    );
                }
            } else {
                let unknown =
                    format!("[{query}]. Type `help` or `--help` for a list of available commands.");
                hook::unknown(&unknown);
            }
            return;
        }

        println!("Help:");
        for command in registry.all() {
            if !command.hidden() {
                println!(
                    "  {:<12} {}",
                    command.name(),
                    command.help().unwrap_or("No description")
                );
            }
        }
    }
}
