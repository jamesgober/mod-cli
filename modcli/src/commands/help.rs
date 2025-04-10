use crate::command::Command;

/// Prints available commands (placeholder until we inject registry data)
pub struct HelpCommand;

impl HelpCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn aliases(&self) -> &[&str] {
        &["--help", "-h"]
    }

    fn help(&self) -> Option<&str> {
        Some("Displays help information")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if args.len() > 1 {
            Err("Too many arguments. Usage: help [command]".into())
        } else {
            Ok(())
        }
    }
    
    fn execute(&self, _args: &[String]) {
        println!("Help:");
        println!("  help         Displays this message");
        println!("  ping         Responds with pong");
        println!("  echo         Repeats your input");
        println!("  hello        Greets the user");
    }
}
