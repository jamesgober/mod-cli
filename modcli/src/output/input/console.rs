use crate::config::CliConfig;
use crate::output::input::prompt_text;
use crate::output::print::print_multiline;

pub fn run_interactive_console(config: &CliConfig) {
    if let Some(welcome) = &config.welcome {
        print_multiline(
            &welcome.iter().map(String::as_str).collect::<Vec<&str>>(),
            Some(config.line_delay.unwrap_or(0)),
        );
    }

    // Set prompt prefix or default to ">> "
    let prompt = config.prompt_prefix.as_deref().unwrap_or("Mod > ");

    loop {
        let input = prompt_text(prompt);
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        println!("You typed: {}", input);
    }

    if let Some(goodbye) = &config.goodbye {
        print_multiline(
            &goodbye.iter().map(String::as_str).collect::<Vec<&str>>(),
            Some(config.line_delay.unwrap_or(0)),
        );
    }
}