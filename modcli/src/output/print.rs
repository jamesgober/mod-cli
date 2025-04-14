use crossterm::style::Stylize;
use std::{thread, time::Duration};

/// Prints multi-line text with optional delay
pub fn print_multiline(lines: &[&str], delay_ms: Option<u64>) {
    for line in lines {
        println!("{}", line);
        if let Some(ms) = delay_ms {
            thread::sleep(Duration::from_millis(ms));
        }
    }
}

/// Prints a success message
pub fn print_success(msg: &str) {
    println!("{}", msg.green().bold());
}

/// Prints a warning message
pub fn print_warning(msg: &str) {
    println!("{}", msg.yellow().bold());
}

/// Prints an error message
pub fn print_error(msg: &str) {
    println!("{}", msg.red().bold());
}

/// Prints a status/info message
pub fn print_status(msg: &str) {
    println!("{}", msg.cyan());
}
