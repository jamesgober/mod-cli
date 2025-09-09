use crate::command::Command;
use crate::error::ModCliError;

pub struct HelloCommand;

impl Command for HelloCommand {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn help(&self) -> Option<&str> {
        Some("Greets the user")
    }

    fn validate(&self, args: &[String]) -> Result<(), ModCliError> {
        if args.len() > 1 {
            Err(ModCliError::InvalidUsage(
                "Hello takes at most one argument (your name).".into(),
            ))
        } else {
            Ok(())
        }
    }

    fn execute(&self, args: &[String]) {
        if let Some(name) = args.first() {
            println!("Hello, {name}!");
        } else {
            println!("Hello!");
        }
    }
}
