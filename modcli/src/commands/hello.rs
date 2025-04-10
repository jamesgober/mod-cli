use crate::command::Command;

pub struct HelloCommand;

impl Command for HelloCommand {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn help(&self) -> Option<&str> {
        Some("Greets the user")
    }

    fn execute(&self, args: &[String]) {
        let name = args.get(0).cloned().unwrap_or_else(|| "World".to_string());
        println!("Hello, {}!", name);
    }
}
