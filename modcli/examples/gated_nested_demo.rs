use modcli::command::Command;
use modcli::ModCli;
use std::env;

// Top-level group marker: "ops". Typically a no-op; serves as a namespace.
struct OpsGroup;
impl Command for OpsGroup {
    fn name(&self) -> &str {
        "ops"
    }
    fn help(&self) -> Option<&str> {
        Some("Parent group; see `help ops`")
    }
    fn execute(&self, _args: &[String]) {
        println!("Use: help ops  |  ops <child> ...");
    }
}

// Child requiring admin role: "ops:restart"
struct Restart;
impl Command for Restart {
    fn name(&self) -> &str {
        "ops:restart"
    }
    fn help(&self) -> Option<&str> {
        Some("Admin-only restart command")
    }
    fn required_caps(&self) -> &[&str] {
        &["role:admin"]
    }
    fn execute(&self, _args: &[String]) {
        println!("restart: OK")
    }
}

// Child requiring user:james
struct Private;
impl Command for Private {
    fn name(&self) -> &str {
        "ops:private"
    }
    fn help(&self) -> Option<&str> {
        Some("Visible only to user:james")
    }
    fn required_caps(&self) -> &[&str] {
        &["user:james"]
    }
    fn execute(&self, _args: &[String]) {
        println!("private: OK")
    }
}

fn main() {
    // Build CLI and grant capabilities from env var for demo
    // Example: MODCLI_CAPS="role:admin,user:james"
    let caps = env::var("MODCLI_CAPS").unwrap_or_default();

    let mut cli = ModCli::new();
    // Configure capabilities on the registry
    if !caps.is_empty() {
        let list = caps
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        cli.registry.set_caps(list);
    }

    // Register group and children
    cli.registry.register(Box::new(OpsGroup));
    cli.registry.register(Box::new(Restart));
    cli.registry.register(Box::new(Private));

    // Dispatch from process args
    let args: Vec<String> = std::env::args().skip(1).collect();
    cli.run(args);
}
