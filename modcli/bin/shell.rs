use modcli::ModCli;
use std::io::{self, Write};


struct Config {
    shell_name: Option<String>,
}

fn main() {

    let config = Config {
        shell_name: Some("Mod".to_string()),
    };

    let shell_name: &str = config.shell_name.as_deref().unwrap_or("cli");

    println!("{} Shell (type 'exit' to quit)", shell_name);

    let mut cli = ModCli::new();

    loop {
        print!("{}> ", shell_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        let args: Vec<String> = trimmed
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if args.is_empty() {
            continue;
        }

        cli.run(args);
    }

    println!("Goodbye.");
}