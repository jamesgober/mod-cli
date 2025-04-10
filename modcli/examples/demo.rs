use modcli::loader::CommandRegistry;
use modcli::command::Command;

struct HelloCommand;

impl Command for HelloCommand {
    fn name(&self) -> &str {
        "hello"
    }

    fn execute(&self, args: &[String]) {
        println!("Hello, {}!", args.get(0).unwrap_or(&"world".to_string()));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut registry = CommandRegistry::new();
    registry.register(Box::new(HelloCommand));

    if let Some((cmd, cmd_args)) = args.split_first() {
        registry.execute(cmd, cmd_args);
    } else {
        println!("No command given");
    }
}
