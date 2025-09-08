//! Full app example demonstrating core features.
//! Build with optional features:
//! - json-loader
//! - plugins

use modcli::command::Command;
use modcli::output::{build, gradient, print, ORANGE, RED};
use modcli::ModCli;

struct Greet;
impl Command for Greet {
    fn name(&self) -> &str {
        "greet"
    }
    fn help(&self) -> Option<&str> {
        Some("Greets the user")
    }
    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Ok(())
    }
    fn execute(&self, args: &[String]) {
        let name = args.get(0).map(|s| s.as_str()).unwrap_or("world");
        let msg = build().part("Hello, ").space().part(name).bold().get();
        print::line(&msg);
    }
}

fn main() {
    let mut cli = ModCli::new();

    // Prefix routing
    cli.set_prefix("app");

    // Register a custom command
    cli.registry.register(Box::new(Greet));

    // Styled + Gradient output demo
    let stylish = build()
        .part("Welcome to ")
        .space()
        .part("ModCLI")
        .bold()
        .get();
    print::line(&stylish);
    let rainbow = gradient::two_color("Gradient demo", RED, ORANGE);
    print::line(&rainbow);


    // Run CLI
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print::line("Try: app:greet James");
    }
    cli.run(args);
}
