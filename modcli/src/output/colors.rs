use crossterm::style::{Color, Stylize};

/// Default fallback color if a named color is not found.
pub const DEFAULT: Color = Color::Black;

/// Core color constants
pub const RED:     Color = Color::Rgb { r: 255, g: 0, b: 0 };
pub const BLUE:    Color = Color::Rgb { r: 0, g: 0, b: 255 };
pub const GREEN:   Color = Color::Rgb { r: 0, g: 255, b: 0 };
pub const YELLOW:  Color = Color::Rgb { r: 255, g: 255, b: 0 };
pub const CYAN:    Color = Color::Rgb { r: 0, g: 255, b: 255 };
pub const MAGENTA: Color = Color::Rgb { r: 255, g: 0, b: 255 };
pub const WHITE:   Color = Color::Rgb { r: 255, g: 255, b: 255 };
pub const BLACK:   Color = Color::Rgb { r: 0, g: 0, b: 0 };
pub const GREY:    Color = Color::Rgb { r: 128, g: 128, b: 128 };
pub const PINK:    Color = Color::Rgb { r: 255, g: 105, b: 180 };
pub const PURPLE:  Color = Color::Rgb { r: 128, g: 0, b: 128 };
pub const TEAL:    Color = Color::Rgb { r: 0, g: 128, b: 128 };
pub const ORANGE:  Color = Color::Rgb { r: 255, g: 165, b: 0 };
pub const BROWN:   Color = Color::Rgb { r: 165, g: 42, b: 42 };
pub const LIGHT_BLUE:    Color = Color::Rgb { r: 0, g: 180, b: 255 };
pub const LIGHT_GREEN:   Color = Color::Rgb { r: 144, g: 238, b: 144 };
pub const LIGHT_YELLOW:  Color = Color::Rgb { r: 255, g: 255, b: 224 };
pub const LIGHT_CYAN:    Color = Color::Rgb { r: 224, g: 255, b: 255 };
pub const LIGHT_MAGENTA: Color = Color::Rgb { r: 255, g: 224, b: 255 };
pub const LIGHT_GREY:    Color = Color::Rgb { r: 211, g: 211, b: 211 };
pub const DARK_GREY:     Color = Color::Rgb { r: 169, g: 169, b: 169 };
pub const DARK_BLUE:     Color = Color::Rgb { r: 0, g: 0, b: 139 };
pub const DARK_ORANGE:   Color = Color::Rgb { r: 255, g: 140, b: 0 };
pub const DARK_PINK:     Color = Color::Rgb { r: 255, g: 20, b: 147 };
pub const DARK_PURPLE:   Color = Color::Rgb { r: 75, g: 0, b: 130 };
pub const DARK_TEAL:     Color = Color::Rgb { r: 0, g: 139, b: 139 };
pub const DARK_BROWN:    Color = Color::Rgb { r: 101, g: 67, b: 33 };

/// Color constants for different log levels
pub const COLOR_DEBUG:   Color = GREY;
pub const COLOR_STATUS:  Color = CYAN;
pub const COLOR_INFO:    Color = LIGHT_BLUE;
pub const COLOR_WARNING: Color = ORANGE;
pub const COLOR_ERROR:   Color = RED;
pub const COLOR_TRACE:   Color = LIGHT_GREY;
pub const COLOR_SUCCESS: Color = GREEN;
pub const COLOR_NOTICE:  Color = Color::Rgb { r: (255), g: (90), b: (0) };

/// Returns a list of named colors
pub fn list() -> Vec<(&'static str, Color)> {
    vec![
        ("Red", RED),
        ("Green", GREEN),
        ("Blue", BLUE),
        ("Yellow", YELLOW),
        ("Cyan", CYAN), 
        ("Magenta", MAGENTA), 
        ("White", WHITE), 
        ("Grey", GREY),
        ("Black", BLACK), 
        ("Orange", ORANGE), 
        ("Pink", PINK), 
        ("Purple", PURPLE),
        ("Teal", TEAL), 
        ("Brown", BROWN), 
        ("Light Blue", LIGHT_BLUE),
        ("Light Green", LIGHT_GREEN), 
        ("Light Yellow", LIGHT_YELLOW),
        ("Light Cyan", LIGHT_CYAN), 
        ("Light Magenta", LIGHT_MAGENTA),
        ("Light Grey", LIGHT_GREY), 
        ("Dark Grey", DARK_GREY), 
        ("Dark Blue", DARK_BLUE),
        ("Dark Orange", DARK_ORANGE), 
        ("Dark Pink", DARK_PINK),
        ("Dark Purple", DARK_PURPLE), 
        ("Dark Teal", DARK_TEAL), 
        ("Dark Brown", DARK_BROWN)
    ]
}

/// Returns a color by name (case-insensitive), or DEFAULT color if not found.
pub fn get(name: &str) -> Color {
    list()
        .into_iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, c)| c)
        .unwrap_or(DEFAULT)
}

/// Prints a swatch of all named colors
pub fn print() {
    println!("Available Colors:");
    for (name, color) in list() {
        println!("{}", format!("{:<20}", name).with(color));
    }
}