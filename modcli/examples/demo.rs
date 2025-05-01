use modcli::loader::CommandRegistry;
use modcli::command::Command;

struct HelloCommand;

impl Command for HelloCommand {
    fn name(&self) -> &str {
        "hello"
    }

    fn help(&self) -> Option<&str> {
        Some("Prints a greeting. Usage: hello [name]")
    }

    fn execute(&self, args: &[String]) {
        let target = args.get(0).map(String::as_str).unwrap_or("world");
        println!("👋 Hello, {}!", target);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut registry = CommandRegistry::new();
    registry.register(Box::new(HelloCommand));

    match args.split_first() {
        Some((cmd, rest)) => {
            if cmd == "help" {
                println!("Available commands:");
                for command in registry.all() {
                    if !command.hidden() {
                        println!(
                            "  {:<10} {}",
                            command.name(),
                            command.help().unwrap_or("No description")
                        );
                    }
                }
            } else {
                registry.execute(cmd, rest);
            }
        }
        None => {
            println!("No command given. Try: `demo hello` or `demo help`");
        }
    }
}
