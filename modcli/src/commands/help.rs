use crate::command::Command;
use crate::error::ModCliError;
use crate::loader::CommandRegistry;
use crate::output::hook;
use crate::output::markdown;
use crate::output::messages;
use crate::output::text::sanitize;

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

    fn validate(&self, args: &[String]) -> Result<(), ModCliError> {
        if args.len() > 1 {
            Err(ModCliError::InvalidUsage(
                "Too many arguments. Usage: help [command]".into(),
            ))
        } else {
            Ok(())
        }
    }

    fn execute(&self, _args: &[String]) {}

    fn execute_with(&self, args: &[String], registry: &CommandRegistry) {
        // validate() already ensures args.len() <= 1
        if args.len() == 1 {
            let query = &args[0];
            // The query is user input: escape control characters before echoing it.
            let shown = sanitize(query);
            // If a direct command matches and is visible, show its help
            if let Some(target) = registry.get(query) {
                if registry.is_visible(target) {
                    let name_line = target.name().to_string();
                    println!("{name_line}");
                    let body = target.help().unwrap_or("No description.");
                    let rendered = markdown::render_markdown(body);
                    print!("{rendered}");
                } else {
                    println!("No help available for '{shown}'");
                }
                return;
            }

            // Namespace help: list children of `query:` that are visible
            let ns = format!("{query}:");
            let mut any = false;
            let ns_header_fallback = format!("Help ({shown}):");
            let header = messages::message_or_default("help.ns_header", &ns_header_fallback);
            println!("{header}");
            for command in sorted(registry) {
                let name = command.name();
                if name.starts_with(&ns) && registry.is_visible(command) {
                    println!(
                        "  {:<20} {}",
                        name,
                        markdown::render_markdown(command.help().unwrap_or("No description"))
                    );
                    any = true;
                }
            }
            if !any {
                let unknown =
                    format!("[{shown}]. Type `help` or `--help` for a list of available commands.");
                hook::unknown(&unknown);
            }
            return;
        }

        let header = messages::message_or_default("help.header", "Help:");
        println!("{header}");
        for command in sorted(registry) {
            let name = command.name();
            let top_level = !name.contains(':');
            if top_level && registry.is_visible(command) {
                println!(
                    "  {:<12} {}",
                    name,
                    markdown::render_markdown(command.help().unwrap_or("No description"))
                );
            }
        }
        if let Some(footer) = messages::get_message("help.footer") {
            println!("{footer}");
        }
    }
}

/// Registered commands in name order, so help lists them the same way on every run.
fn sorted(registry: &CommandRegistry) -> Vec<&dyn Command> {
    let mut commands: Vec<&dyn Command> = registry.all().map(|c| c.as_ref()).collect();
    commands.sort_unstable_by(|a, b| a.name().cmp(b.name()));
    commands
}
