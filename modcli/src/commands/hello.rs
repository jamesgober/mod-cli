use crate::command::Command;

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
        if let Some(name) = args.get(0) {
            println!("Hello, {}!", name);
        } else {
            println!("Hello!");
        }
    }
}
