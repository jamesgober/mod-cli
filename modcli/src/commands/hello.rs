use crate::command::Command;
use crate::output::hook;

pub struct HelloCommand;

impl Command for HelloCommand {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn help(&self) -> Option<&str> {
        Some("Greets the user")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if args.len() > 1 {
            Err("Hello takes at most one argument (your name).".into())
        } else {
            Ok(())
        }
    }

    fn execute(&self, args: &[String]) {
        hook::info("CLI started");
        hook::status("Checking mood...");
        hook::success("You seem ready!");
        hook::warn("But don’t get cocky.");
        hook::error("Just kidding. You're good. 😎");
        hook::unknown("Unknown command. Try 'help'.");
        hook::deprecated("This command is deprecated.");
        if let Some(name) = args.get(0) {
            println!("Hello, {}!", name);
        } else {
            println!("Hello!");
        }
    }
}
