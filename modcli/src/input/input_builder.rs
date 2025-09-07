use rpassword::read_password;
use std::io::{stdin, stdout, Write};

/// Prompt for plain text input with optional default fallback
pub fn prompt_text(prompt: &str, default: Option<&str>) -> String {
    print!(
        "{prompt}{} ",
        default.map_or(String::new(), |d| format!(" [{d}]"))
    );
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    if trimmed.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        trimmed.to_string()
    }
}

/// Prompt for a yes/no confirmation
pub fn confirm(prompt: &str, default_yes: bool) -> bool {
    let yes_hint = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{prompt} {yes_hint} ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_lowercase();

    match trimmed.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        "" => default_yes,
        _ => default_yes, // Fallback to default if unrecognized
    }
}

/// Prompt for a hidden password
pub fn prompt_password(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();
    read_password().unwrap_or_default()
}
