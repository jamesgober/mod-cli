use std::io::{self, Write};

/// Prompts the user to confirm an action (yes/no).
pub fn prompt_confirm(question: &str) -> bool {
    let mut input = String::new();
    loop {
        print!("{question} [y/n]: ");
        io::stdout().flush().unwrap();

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Try again.");
            continue;
        }

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please enter 'y' or 'n'.");
            }
        }
    }
}
