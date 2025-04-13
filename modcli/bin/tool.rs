use modcli::ModCli;
use modcli::config::CliConfig;
use modcli::loader::sources::JsonFileSource;
// use modcli::output::themes::apply_theme;
use modcli::output::print_multiline;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Load config file
    let config = CliConfig::load("examples/config.json");

    // Apply theme if defined
    if let Some(_theme) = &config.theme {
        // apply_theme(theme.as_str());
    }

    // Print startup banner
    if let Some(banner) = &config.banner {
        let lines: Vec<&str> = banner.lines().collect();
        print_multiline(&lines, None);
    }

    if args.is_empty() {
        println!("No args provided.");
        return;
    }

    // Init CLI
    let mut cli = ModCli::new();

    // Load commands from external JSON source
    let source = JsonFileSource::new("examples/commands.json");
    cli.registry.load_from(Box::new(source));

    // Enforce strict argument count if enabled in config
    if let Some(strict) = config.strict_args {
        if strict && args.len() > 1 {
            eprintln!("Too many arguments. Strict mode is enabled.");
            return;
        }
    }

    cli.run(args);
}
