use crate::command::Command;
use crate::config::CliConfig;
use crate::console::run_shell;
use crate::error::ModCliError;

pub struct ShellCommand;

impl Command for ShellCommand {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn help(&self) -> Option<&str> {
        Some("Launch interactive shell")
    }

    fn validate(&self, args: &[String]) -> Result<(), ModCliError> {
        if !args.is_empty() {
            Err(ModCliError::InvalidUsage("Shell does not accept any arguments.".into()))
        } else {
            Ok(())
        }
    }

    fn execute(&self, _args: &[String]) {
        let config = CliConfig::load(None);
        let _ = run_shell(config);
    }
}
