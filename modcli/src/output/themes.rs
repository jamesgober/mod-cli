/*

 */
use std::io::{stdout, Write};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor, ResetColor};


// Color constants for various colors
pub const RED: Color = Color::Rgb { r: 255, g: 0, b: 0 };
pub const BLUE: Color = Color::Rgb { r: 0, g: 0, b: 255 };
pub const GREEN: Color = Color::Rgb { r: 0, g: 255, b: 0 };
pub const YELLOW: Color = Color::Rgb { r: 255, g: 255, b: 0 };
pub const CYAN: Color = Color::Rgb { r: 0, g: 255, b: 255 };
pub const MAGENTA: Color = Color::Rgb { r: 255, g: 0, b: 255 };
pub const WHITE: Color = Color::Rgb { r: 255, g: 255, b: 255 };
pub const BLACK: Color = Color::Rgb { r: 0, g: 0, b: 0 };
pub const GREY: Color = Color::Rgb { r: 128, g: 128, b: 128 };
pub const PINK: Color = Color::Rgb { r: 255, g: 105, b: 180 };
pub const PURPLE: Color = Color::Rgb { r: 128, g: 0, b: 128 };
pub const TEAL: Color = Color::Rgb { r: 0, g: 128, b: 128 };
pub const ORANGE: Color = Color::Rgb { r: 255, g: 165, b: 0 };
pub const BROWN: Color = Color::Rgb { r: 165, g: 42, b: 42 };
pub const LIGHT_BLUE: Color = Color::Rgb { r: 173, g: 216, b: 230 };
pub const LIGHT_GREEN: Color = Color::Rgb { r: 144, g: 238, b: 144 };
pub const LIGHT_YELLOW: Color = Color::Rgb { r: 255, g: 255, b: 224 };
pub const LIGHT_CYAN: Color = Color::Rgb { r: 224, g: 255, b: 255 };
pub const LIGHT_MAGENTA: Color = Color::Rgb { r: 255, g: 224, b: 255 };
pub const LIGHT_GREY: Color = Color::Rgb { r: 211, g: 211, b: 211 };
pub const DARK_GREY: Color = Color::Rgb { r: 169, g: 169, b: 169 };
pub const DARK_BLUE: Color = Color::Rgb { r: 0, g: 0, b: 139 }; 
pub const DARK_ORANGE: Color = Color::Rgb { r: 255, g: 140, b: 0 };
pub const DARK_PINK: Color = Color::Rgb { r: 255, g: 20, b: 147 };
pub const DARK_PURPLE: Color = Color::Rgb { r: 75, g: 0, b: 130 };
pub const DARK_TEAL: Color = Color::Rgb { r: 0, g: 139, b: 139 };
pub const DARK_BROWN: Color = Color::Rgb { r: 101, g: 67, b: 33 };
pub const DARK_LIGHT_BLUE: Color = Color::Rgb { r: 70, g: 130, b: 180 };
pub const DARK_LIGHT_GREEN: Color = Color::Rgb { r: 50, g: 205, b: 50 };
pub const DARK_LIGHT_YELLOW: Color = Color::Rgb { r: 255, g: 255, b: 224 };
pub const DARK_LIGHT_CYAN: Color = Color::Rgb { r: 224, g: 255, b: 255 };
pub const DARK_LIGHT_MAGENTA: Color = Color::Rgb { r: 255, g: 224, b: 255 };
pub const DARK_LIGHT_GREY: Color = Color::Rgb { r: 211, g: 211, b: 211 };

// Color constants for different log levels
pub const COLOR_ERROR: Color = RED;
pub const COLOR_WARNING: Color = ORANGE;
pub const COLOR_SUCCESS: Color = GREEN;
pub const COLOR_DEBUG: Color = GREY;
pub const COLOR_INFO: Color = BLUE;
pub const COLOR_TRACE: Color = LIGHT_GREY;
pub const COLOR_DEFAULT: Color = WHITE;

// Apply a CLI color theme
pub fn apply_theme(theme: &str) {
    let mut stdout = stdout();

    match theme {
        "monochrome" => {
            let _ = write!(stdout, "{}{}", SetForegroundColor(Color::Grey), SetBackgroundColor(Color::Black));
        }
        "default" => {
            let _ = write!(stdout, "{}{}", SetForegroundColor(Color::White), SetBackgroundColor(Color::Black));
        }
        "inverted" => {
            let _ = write!(stdout, "{}{}", SetForegroundColor(Color::Black), SetBackgroundColor(Color::White));
        }
        "blue" => {
            let _ = write!(stdout, "{}{}", SetForegroundColor(Color::White), SetBackgroundColor(Color::Blue));
        }
        "green" => {
            let _ = write!(stdout, "{}{}", SetForegroundColor(Color::Black), SetBackgroundColor(Color::Green));
        }
        _ => {
            // Unknown theme
        }
    }

    let _ = stdout.flush();
}

// Resets to terminal default colors
pub fn reset_theme() {
    let _ = write!(stdout(), "{}", ResetColor);
}