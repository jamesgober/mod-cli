use std::io::{self, Write};
use crate::output::print;

/// Prompts the user to confirm an action (yes/no).
pub fn prompt_confirm(question: &str) -> bool {
    let mut input = String::new();
    loop {
        print!("{question} [y/n]: ");
        if let Err(e) = io::stdout().flush() { print::warn(&format!("flush failed: {e}")); }

        input.clear();
        if let Err(e) = io::stdin().read_line(&mut input) {
            print::error(&format!("Error reading input: {e}. Try again."));
            continue;
        }

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => print::status("Please enter 'y' or 'n'."),
        }
    }
}
