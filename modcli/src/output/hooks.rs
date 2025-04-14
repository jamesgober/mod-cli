use crate::config::CliConfig;
use std::sync::OnceLock;

static CONFIG: OnceLock<CliConfig> = OnceLock::new();

fn get_theme() -> String {
    CONFIG.get_or_init(|| CliConfig::load("examples/config.json"))
          .theme.clone().unwrap_or_else(|| "default".into())
}

fn styled(label: &str, message: &str, color: &str) {
    match get_theme().as_str() {
        "monochrome" => println!("[{}] {}", label, message),
        _ => println!("\x1b[{}m[{}]\x1b[0m {}", color, label, message),
    }
}

pub fn print_info(msg: &str) {
    styled("INFO", msg, "36") // Cyan
}

pub fn print_warn(msg: &str) {
    styled("WARN", msg, "33") // Yellow
}

pub fn print_error(msg: &str) {
    styled("ERROR", msg, "31") // Red
}

pub fn print_success(msg: &str) {
    styled("SUCCESS", msg, "32") // Green
}

pub fn print_status(msg: &str) {
    styled("...", msg, "34") // Blue
}
