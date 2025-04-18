use std::io::{self, Write};
use rpassword::read_password;

/// Prompts for a secure password (no echo)
pub fn prompt_password(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    match read_password() {
        Ok(password) => password,
        Err(_) => {
            println!("\n[ERROR] Failed to read password.");
            String::new()
        }
    }
}
