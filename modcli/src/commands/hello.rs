use crate::command::Command;
use crate::output::hooks::*;

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
        print_info("CLI started");
        print_status("Checking mood...");
        print_success("You seem ready!");
        print_warn("But don’t get cocky.");
        print_error("Just kidding. You're good. 😎");
        if let Some(name) = args.get(0) {
            println!("Hello, {}!", name);
        } else {
            println!("Hello!");
        }
    }
}
