use modcli::error::ModCliError;
use modcli::output::hook;
use modcli::ModCli;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Show app version
    if args.len() == 2 && args[1] == "--version" {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Show ModCLI version & internal info
    if args.len() == 2 && args[1] == "--modcli" {
        let cli = ModCli::new();
        println!("ModCLI Framework v{}", modcli::modcli_version());

        #[cfg(feature = "internal-commands")]
        println!("Internal Commands: Enabled");

        #[cfg(not(feature = "internal-commands"))]
        println!("Internal Commands: Disabled");

        println!("Loaded Commands: {}", cli.registry.len());
        return;
    }

    // Default command execution
    #[allow(unused_mut)]
    let mut cli = ModCli::new();

    // Skip program name and pass only actual arguments
    let cli_args = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        vec![]
    };

    if cli_args.is_empty() {
        hook::status("No command provided. Try `help`.");
        std::process::exit(0);
    }

    let cmd = cli_args[0].clone();
    let rest = cli_args[1..].to_vec();

    match cli.registry.try_execute(&cmd, &rest) {
        Ok(()) => {}
        Err(ModCliError::InvalidUsage(msg)) => {
            hook::error(&format!("Invalid usage: {msg}"));
            std::process::exit(2);
        }
        Err(ModCliError::UnknownCommand(name)) => {
            hook::unknown(&format!("{name}"));
            std::process::exit(127);
        }
        Err(e) => {
            hook::error(&e.to_string());
            std::process::exit(1);
        }
    }
}
