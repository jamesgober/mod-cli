use crate::command::Command;
use crate::config::CliConfig;
use crate::console::run_shell;

pub struct ShellCommand;

impl Command for ShellCommand {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn help(&self) -> Option<&str> {
        Some("Launch interactive shell")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if !args.is_empty() {
            Err("Shell does not accept any arguments.".into())
        } else {
            Ok(())
        }
    }

    fn execute(&self, _args: &[String]) {
        let config = CliConfig::load(None);
        run_shell(config);
    }
}
