use crate::config::CliConfig;
use crate::output::{build, hook, print};
use std::io::{self, Write};

pub fn dispatch_shell_command(input: &str, config: &CliConfig) -> bool {
    match input.trim() {
        "clear" => {
            clear_screen();
            true
        }
        "project" => {
            let project = config.modcli.name.as_deref().unwrap_or("Unknown");
            let msg = build()
                .part("Current Project: ")
                .bold()
                .part(project)
                .underline()
                .get();
            print::line(&msg);
            true
        }
        "?" | "shell help" => {
            print::info("Shell Commands:");
            print::line("  clear     - Clear the terminal");
            print::line("  project   - Show active project name");
            print::line("  exit/quit - Leave shell");
            true
        }
        _ => false,
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    if let Err(e) = io::stdout().flush() {
        hook::warn(&format!("flush failed: {e}"));
    }
}
