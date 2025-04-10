use crate::command::Command;

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn help(&self) -> Option<&str> {
        Some("Responds with pong")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if !args.is_empty() {
            Err("Ping does not accept any arguments.".into())
        } else {
            Ok(())
        }
    }

    fn execute(&self, _args: &[String]) {
        println!("Pong!");
    }
}
