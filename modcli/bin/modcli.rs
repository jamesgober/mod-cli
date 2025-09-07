use modcli::ModCli;
use modcli::error::ModCliError;

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

    // Auto-load plugins from ./plugins when feature enabled
    #[cfg(feature = "plugins")]
    {
        cli.registry.load_plugins("./plugins");
    }

    // Skip program name and pass only actual arguments
    let cli_args = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        vec![]
    };

    if cli_args.is_empty() {
        eprintln!("No command provided.");
        std::process::exit(2);
    }

    let cmd = cli_args[0].clone();
    let rest = cli_args[1..].to_vec();

    match cli.registry.try_execute(&cmd, &rest) {
        Ok(()) => {}
        Err(ModCliError::InvalidUsage(msg)) => {
            eprintln!("Invalid usage: {msg}");
            std::process::exit(2);
        }
        Err(ModCliError::UnknownCommand(name)) => {
            eprintln!("Unknown command: {name}");
            std::process::exit(127);
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
