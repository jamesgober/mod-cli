//use modcli::output::hook;
use modcli::config::CliConfig;
use modcli::ModCli;
//use modcli::loader::sources::JsonFileSource;
use modcli::config::MessageConfig;
use modcli::console::run_shell;
use modcli::output::{print, themes::apply_theme};

fn main() {
    // Load config file
    let config = CliConfig::load(None);

    // Grab CLI settings
    let _cli_name = config.modcli.name.as_deref().unwrap_or("mod-cli");
    let cli_prefix = config.modcli.prefix.as_deref().unwrap_or("mod");
    let force_shell = config.modcli.force_shell.unwrap_or(false);

    // Grab CLI args
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Apply theme if defined
    if let Some(theme) = &config.modcli.theme {
        apply_theme(theme);
    }

    // Show banner if defined
    if let Some(banner) = &config.modcli.banner {
        let delay = config.modcli.delay.unwrap_or(0);
        print::scroll(&banner.lines().collect::<Vec<&str>>(), delay);
    }

    // CLI messages
    let default_msg_config = MessageConfig::default();
    let msg_config = config
        .modcli
        .messages
        .as_ref()
        .unwrap_or(&default_msg_config);
    let msg_no_command = msg_config
        .no_command
        .as_deref()
        .unwrap_or("⚠️ No command given. Try `help`.");

    // No args and not forcing shell? Show no-command message
    if args.is_empty() && !force_shell {
        print::status(msg_no_command);
        return;
    }

    // Create and configure CLI
    let mut cli = ModCli::new();
    cli.set_prefix(cli_prefix);

    // Load external JSON commands
    //let source = JsonFileSource::new("examples/commands.json");
    //cli.registry.load_from(Box::new(source));

    // If force_shell is enabled OR user explicitly passed "shell"
    if force_shell || (args.len() == 1 && args[0] == "shell") {
        let _ = run_shell(config);
        return;
    }

    // Enforce strict argument mode (1 command only)
    if let Some(true) = config.modcli.strict {
        if args.len() > 1 {
            eprintln!("Too many arguments. Strict mode is enabled.");
            return;
        }
    }

    // Now safely run the CLI with args
    cli.run(args);
}
