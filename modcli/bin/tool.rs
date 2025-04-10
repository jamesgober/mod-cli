use modcli::ModCli;
use modcli::config::CliConfig;
use modcli::loader::sources::JsonFileSource;

fn main() {
    println!("Tool binary is running!");

    let args: Vec<String> = std::env::args().skip(1).collect();

    let _config = CliConfig::load("examples/config.json");



    if args.is_empty() {
        println!("No args provided.");
        return;
    }

    // Initialize CLI
    let mut cli = ModCli::new();

    // Load JSON commands from file located at modcli/examples/commands.json
    let source = JsonFileSource::new("examples/commands.json");
    cli.registry.load_from(Box::new(source));

    // Run CLI with remaining arguments
    cli.run(args);
}
