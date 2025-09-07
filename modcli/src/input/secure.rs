use rpassword::read_password;
use std::io::{self, Write};
use crate::output::hook;

/// Prompts for a secure password (no echo)
pub fn prompt_password(prompt: &str) -> String {
    print!("{prompt}: ");
    if let Err(e) = io::stdout().flush() { hook::warn(&format!("flush failed: {e}")); }

    match read_password() {
        Ok(password) => password,
        Err(e) => {
            hook::error(&format!("failed to read password: {e}"));
            String::new()
        }
    }
}
