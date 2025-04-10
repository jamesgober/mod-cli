use modcli::ModCli;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Handle --version (app version)
    if args.len() == 2 && args[1] == "--version" {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Handle --modcli (ModCLI version)
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

    // Normal execution
    let cli = ModCli::new();
    cli.run(args[1..].to_vec());
}