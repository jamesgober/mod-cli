use crate::command::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn help(&self) -> Option<&str> {
        Some("Repeats your input")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if args.is_empty() {
            Err("You must provide at least one word to echo.".into())
        } else {
            Ok(())
        }
    }

    fn execute(&self, args: &[String]) {
        println!("{}", args.join(" "));
    }
}
