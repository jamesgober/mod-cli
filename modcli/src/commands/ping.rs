use crate::command::Command;

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn help(&self) -> Option<&str> {
        Some("Responds with pong")
    }

    fn execute(&self, _args: &[String]) {
        println!("Pong!");
    }
}
