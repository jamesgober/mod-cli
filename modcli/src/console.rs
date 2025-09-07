use crate::config::CliConfig;
use crate::input::prompt_text;
use crate::output::print;
use crate::shell_commands::dispatch;
use crate::shell_extensions::dispatch_shell_command;
use crate::ModCli;

pub fn run_shell(config: &CliConfig) {
    // Get shell configuration
    let sconf = if let Some(sconf) = &config.modcli.shell {
        sconf
    } else {
        panic!("Shell configuration is missing");
    };

    // Set prompt prefix or default to "Mod > "
    let prompt = sconf.prompt.as_deref().unwrap_or("Mod > ");

    // Show welcome message, scroll line-by-line with optional delay
    if let Some(welcome) = &sconf.welcome {
        let delay = &config.modcli.delay.unwrap_or(0);
        let lines: Vec<&str> = welcome.iter().flat_map(|s| s.lines()).collect();
        print::scroll(&lines, *delay);
    }

    // Show goodbye message, scroll line-by-line with optional delay
    let goodbye_message = if let Some(goodbye) = &sconf.goodbye {
        let delay = &config.modcli.delay.unwrap_or(0);
        let lines: Vec<&str> = goodbye.iter().flat_map(|s| s.lines()).collect();
        Some((lines, *delay))
    } else {
        None
    };

    // Initialize ModCLI
    let mut cli = ModCli::new();

    // Optionally re-load commands if needed
    // let source = crate::loader::sources::JsonFileSource::new("examples/commands.json");
    // cli.registry.load_from(Box::new(source));

    // Loop for shell commands
    loop {
        // Get input
        let input = prompt_text(&prompt);
        let trimmed = input.trim();

        // Check for exit commands
        if matches!(trimmed, "exit" | "quit") {
            break;
        }

        // Check internal shell commands
        if dispatch_shell_command(trimmed, config) {
            continue;
        }

        // Check custom shell commands
        if dispatch(trimmed) {
            continue;
        }

        // Parse input into command and args
        let parts: Vec<String> = trimmed.split_whitespace().map(String::from).collect();

        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0].clone();
        let args = parts[1..].to_vec();

        // Execute command
        cli.run([cmd].into_iter().chain(args).collect());
    }

    if let Some((lines, delay)) = goodbye_message {
        print::scroll(&lines, delay);
    }
}
