use crate::command::Command;
use std::time::Instant;

pub struct BenchmarkCommand;

impl BenchmarkCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for BenchmarkCommand {
    fn name(&self) -> &str {
        "benchmark"
    }

    fn aliases(&self) -> &[&str] {
        &["bench"]
    }

    fn help(&self) -> Option<&str> {
        Some("Benchmark the execution time of a command")
    }

    fn validate(&self, args: &[String]) -> Result<(), String> {
        if args.is_empty() {
            Err("Usage: benchmark <command> [args]".into())
        } else {
            Ok(())
        }
    }

    fn execute(&self, args: &[String]) {
        let cmd_name = &args[0];
        let cmd_args = &args[1..];

        use crate::loader::CommandRegistry;
        let mut registry = CommandRegistry::new();
        registry.load_internal_commands(); // Load commands again just for access

        if let Some(command) = registry.get(cmd_name) {
            let start = Instant::now();
            command.execute(cmd_args);
            let elapsed = start.elapsed();
            println!("⏱  Completed in {:?}", elapsed);
        } else {
            eprintln!("Unknown command: {}", cmd_name);
        }
    }
}