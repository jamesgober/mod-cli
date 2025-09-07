use std::io::{stdin, stdout, Write};
use crate::output::print;

/// Prompts for free-form text and returns the trimmed input.
///
/// Behavior:
/// - On I/O failure, logs an error via `output::print` and retries.
/// - If a validator is provided (see `prompt_text_with_validation`), it will retry on validation errors.
pub fn prompt_text(message: &str) -> String {
    prompt_text_with_validation(message, |_| Ok(()))
}

/// Prompts for text input with a validator.
///
/// Behavior:
/// - On I/O failure, logs an error via `output::print` and retries.
/// - On validation failure, shows a status message and retries.
pub fn prompt_text_with_validation<F>(message: &str, validator: F) -> String
where
    F: Fn(&str) -> Result<(), &str>,
{
    let mut input = String::new();
    loop {
        print!("{message}: ");
        if let Err(e) = stdout().flush() { print::warn(&format!("flush failed: {e}")); }
        input.clear();
        if let Err(e) = stdin().read_line(&mut input) {
            print::error(&format!("Error reading input: {e}. Try again."));
            continue;
        }
        let trimmed = input.trim();

        match validator(trimmed) {
            Ok(_) => return trimmed.to_string(),
            Err(err) => print::status(&format!("Invalid input: {err}")),
        }
    }
}
