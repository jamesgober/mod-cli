/*
use crate::command::Command;

/// An example custom hardcoded command.
pub struct CustomCommand;

impl Command for CustomCommand {
    fn name(&self) -> &str {
        "custom"
    }

    fn aliases(&self) -> &[&str] {
        &["c"]
    }

    fn help(&self) -> Option<&str> {
        Some("An example hardcoded custom command")
    }

    fn execute(&self, args: &[String]) {
        println!("You executed a custom command!");
        if !args.is_empty() {
            println!("With args: {:?}", args);
        }
    }
}
*/