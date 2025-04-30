use crate::config::CliConfig;
use crate::output::input::prompt_text;
use crate::output::print;

pub fn run_interactive_console(config: &CliConfig) {
    // Show welcome message, scroll line-by-line with optional delay
    if let Some(welcome) = &config.welcome {
        let welcome_text = welcome.join("\n");
        print::scroll(&welcome_text, config.line_delay.unwrap_or(0));
    }

    // Set prompt prefix or default to "Mod > "
    let prompt = config.prompt_prefix.as_deref().unwrap_or("Mod > ");

    loop {
        let input = prompt_text(prompt);
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        print::line(&format!("You typed: {}", input), 0);
    }

    // Show goodbye message, scroll line-by-line with optional delay
    if let Some(goodbye) = &config.goodbye {
        let goodbye_text = goodbye.join("\n");
        print::scroll(&goodbye_text, config.line_delay.unwrap_or(0));
    }
}