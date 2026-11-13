use crate::output::print;
use rpassword::read_password;
use std::io;

/// Prompts the user for a password (no echo).
///
/// Returns the password exactly as typed, without the line ending. Leading
/// and trailing spaces are kept. If the password cannot be read (for example
/// when there is no terminal), prints an error and returns an empty string.
/// Use [`try_prompt_password`] to get the error instead.
pub fn prompt_password(message: &str) -> String {
    prompt_password_with_validation(message, |_| Ok(()))
}

/// Prompts the user for a password (no echo) and returns the read error, if
/// any, instead of printing it.
pub fn try_prompt_password(message: &str) -> io::Result<String> {
    try_prompt_password_with_validation(message, |_| Ok(()))
}

/// Prompts for a password with validation.
///
/// Behavior:
/// - On validation failure, shows a status message and asks again.
/// - If the password cannot be read (for example when there is no terminal),
///   prints an error and returns an empty string. It does not ask again,
///   because a read that failed once fails the same way every time.
/// - The password is passed to the validator and returned exactly as typed,
///   without the line ending.
pub fn prompt_password_with_validation<F>(message: &str, validator: F) -> String
where
    F: Fn(&str) -> Result<(), &str>,
{
    match try_prompt_password_with_validation(message, validator) {
        Ok(p) => p,
        Err(e) => {
            print::error(&format!("Failed to read password: {e}"));
            String::new()
        }
    }
}

/// Prompts for a password with validation, and returns the read error, if
/// any, instead of printing it. Asks again only when the validator refuses
/// the password.
pub fn try_prompt_password_with_validation<F>(message: &str, validator: F) -> io::Result<String>
where
    F: Fn(&str) -> Result<(), &str>,
{
    prompt_loop(message, validator, read_password)
}

fn prompt_loop<F, R>(message: &str, validator: F, mut read: R) -> io::Result<String>
where
    F: Fn(&str) -> Result<(), &str>,
    R: FnMut() -> io::Result<String>,
{
    loop {
        print!("{message}: ");
        if let Err(e) = io::Write::flush(&mut io::stdout()) {
            print::warn(&format!("flush failed: {e}"));
        }
        let password = read()?;
        match validator(&password) {
            Ok(()) => return Ok(password),
            Err(err) => print::status(&format!("Invalid password: {err}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::prompt_loop;
    use std::cell::Cell;
    use std::io;

    #[test]
    fn read_error_stops_instead_of_retrying() {
        let calls = Cell::new(0);
        let result = prompt_loop(
            "pw",
            |_| Ok(()),
            || {
                calls.set(calls.get() + 1);
                assert!(calls.get() < 5, "kept retrying a failed read");
                Err(io::Error::other("no terminal"))
            },
        );
        assert!(result.is_err());
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn password_is_not_trimmed() {
        let seen = Cell::new(false);
        let result = prompt_loop(
            "pw",
            |p| {
                assert_eq!(p, " secret ");
                seen.set(true);
                Ok(())
            },
            || Ok(" secret ".to_string()),
        );
        assert_eq!(result.unwrap(), " secret ");
        assert!(seen.get());
    }

    #[test]
    fn validator_refusal_asks_again() {
        let calls = Cell::new(0);
        let result = prompt_loop(
            "pw",
            |p| {
                if p.len() >= 3 {
                    Ok(())
                } else {
                    Err("too short")
                }
            },
            || {
                calls.set(calls.get() + 1);
                Ok(if calls.get() == 1 { "ab" } else { "abc" }.to_string())
            },
        );
        assert_eq!(result.unwrap(), "abc");
        assert_eq!(calls.get(), 2);
    }
}
