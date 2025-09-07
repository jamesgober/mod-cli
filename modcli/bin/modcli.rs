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

    cli.run(cli_args);
}
