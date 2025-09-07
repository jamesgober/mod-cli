use crate::output::print;
use rpassword::read_password;

/// Prompts the user for a password (no echo).
///
/// Behavior:
/// - On I/O failure, logs an error via `output::print` and retries when using
///   `prompt_password_with_validation`. The simple `prompt_password` variant
///   returns an empty string on error.
pub fn prompt_password(message: &str) -> String {
    prompt_password_with_validation(message, |_| Ok(()))
}

/// Prompts for a password with validation.
///
/// Behavior:
/// - On I/O failure, logs an error via `output::print` and retries.
/// - On validation failure, shows a status message and retries.
pub fn prompt_password_with_validation<F>(message: &str, validator: F) -> String
where
    F: Fn(&str) -> Result<(), &str>,
{
    loop {
        print!("{message}: ");
        if let Err(e) = std::io::Write::flush(&mut std::io::stdout()) {
            print::warn(&format!("flush failed: {e}"));
        }
        let password = match read_password() {
            Ok(p) => p,
            Err(e) => {
                print::error(&format!("Failed to read password: {e}. Try again."));
                continue;
            }
        };

        match validator(password.trim()) {
            Ok(_) => return password.trim().to_string(),
            Err(err) => print::status(&format!("Invalid password: {err}")),
        }
    }
}
