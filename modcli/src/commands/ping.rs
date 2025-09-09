use crate::command::Command;
use crate::error::ModCliError;

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn help(&self) -> Option<&str> {
        Some("Responds with pong")
    }

    fn validate(&self, args: &[String]) -> Result<(), ModCliError> {
        if !args.is_empty() {
            Err(ModCliError::InvalidUsage(
                "Ping does not accept any arguments.".into(),
            ))
        } else {
            Ok(())
        }
    }

    fn execute(&self, _args: &[String]) {
        println!("Pong!");
    }
}
