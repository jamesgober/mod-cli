use crate::output::style::build;
use crate::output::themes::current_theme;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    thread,
    time::Duration,
};

/// Prints a single line with optional delay (ms)
#[inline(always)]
pub fn line(text: &str) {
    println!("{text}");
}

/// Prints a clickable hyperlink using OSC 8 sequences when enabled.
///
/// By default, this function falls back to printing `text (url)` to ensure
/// compatibility with terminals that do not support OSC 8. To enable OSC 8
/// output, set the environment variable `ENABLE_OSC8=true`.
///
/// Example:
/// ```rust
/// use modcli::output::print;
/// print::link("mod-cli docs", "https://docs.rs/mod-cli");
/// ```
pub fn link(text: &str, url: &str) {
    let osc8_enabled = osc8_supported();
    if osc8_enabled {
        // OSC 8: ESC ] 8 ; ; url BEL text ESC ] 8 ; ; BEL
        // Use \x1b (ESC) and \x07 (BEL)
        print!("\x1b]8;;{url}\x07{text}\x1b]8;;\x07");
        println!();
    } else {
        println!("{text} ({url})");
    }
}

/// Detect whether OSC 8 hyperlinks should be enabled.
///
/// Priority:
/// - If ENABLE_OSC8 is explicitly set to true/false, honor it.
/// - Otherwise auto-enable for common terminals that support OSC 8.
fn osc8_supported() -> bool {
    if let Ok(val) = std::env::var("ENABLE_OSC8") {
        let v = val.to_ascii_lowercase();
        if v == "true" || v == "1" {
            return true;
        }
        if v == "false" || v == "0" {
            return false;
        }
    }

    // Auto-detect common terminals with OSC 8 support
    let has = |k: &str| std::env::var_os(k).is_some();
    let term_program = std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let term = std::env::var("TERM")
        .unwrap_or_default()
        .to_ascii_lowercase();

    // WezTerm
    if has("WEZTERM_EXECUTABLE") || term_program.contains("wezterm") {
        return true;
    }
    // iTerm2
    if term_program.contains("iterm") {
        return true;
    }
    // Kitty
    if has("KITTY_WINDOW_ID") || term.contains("kitty") {
        return true;
    }
    // VTE-based (many Linux terminals)
    if has("VTE_VERSION") {
        return true;
    }
    // Windows Terminal
    if has("WT_SESSION") {
        return true;
    }

    false
}

/// Prints text without newline
#[inline(always)]
pub fn write(text: &str) {
    print!("{text}");
}

/// Prints just a newline
#[inline(always)]
pub fn newline() {
    println!();
}

/// Prints just a newline
#[inline(always)]
pub fn end() {
    println!();
}

/// Scrolls through a multi-line string with optional delay
pub fn scroll(multiline: &[&str], delay_ms: u64) {
    let delay = if delay_ms > 0 {
        Some(Duration::from_millis(delay_ms))
    } else {
        None
    };
    for text_line in multiline {
        line(text_line);
        if let Some(d) = delay {
            thread::sleep(d);
        }
    }
}

/// Prints each line from a file with optional delay.
///
/// Behavior:
/// - On open/read failure, logs a themed error via `print::error` and returns (no panic).
/// - `delay_ms` controls a fixed delay between lines.
///
/// Example (ignore in doctest):
/// ```ignore
/// use modcli::output::print;
/// print::file("./examples/banner.txt", 0);
/// ```
pub fn file(path: &str, delay_ms: u64) {
    if let Ok(lines) = read_lines(path) {
        for text_line in lines.map_while(Result::ok) {
            line(&text_line);
            if delay_ms > 0 {
                thread::sleep(Duration::from_millis(delay_ms));
            }
        }
    } else {
        error("Failed to open file");
    }
}

// Reads lines from a file, returns an iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// --- Message Shortcodes ---

pub fn debug(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Debug:")
        .color(theme.get_log_color("debug"))
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn info(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Info:")
        .color(theme.get_log_color("info"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn warn(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Warning:")
        .color(theme.get_log_color("warn"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn error(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Error:")
        .color(theme.get_log_color("error"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn success(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Success:")
        .color(theme.get_log_color("success"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn status(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Status:")
        .color(theme.get_log_color("status"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn deprecated(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Deprecated:")
        .color(theme.get_log_color("notice"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}

pub fn unknown(msg: &str) {
    let theme = current_theme();
    let styled = build()
        .part("Unknown Command:")
        .color(theme.get_log_color("notice"))
        .bold()
        .space()
        .part(msg)
        .get();
    line(&styled);
}
