use crate::command::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &str {
        "echo"
    }

    fn aliases(&self) -> &[&str] {
        &["say", "repeat"]
    }

    fn help(&self) -> Option<&str> {
        Some("Echoes the arguments provided")
    }

    fn execute(&self, args: &[String]) {
        println!("{}", args.join(" "));
    }
}
