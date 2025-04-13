use std::thread::sleep;
use std::time::Duration;

/// Print multiple lines with optional delay between lines
pub fn print_multiline(lines: &[&str], delay_ms: Option<u64>) {
    for line in lines {
        println!("{}", line);
        if let Some(ms) = delay_ms {
            sleep(Duration::from_millis(ms));
        }
    }
}