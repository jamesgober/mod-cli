use crossterm::style::{Color, Stylize};

/*
    DEPRECATED: This module is deprecated and will be removed in future versions.
    Use the `colors` module for color management.
*/


/// Deprecated: Use `colors::list()` instead.
pub fn list_named_colors() -> Vec<(&'static str, Color)> {
    vec![
        ("Red", Color::Red),
        ("Green", Color::Green),
        ("Blue", Color::Blue),
        ("Yellow", Color::Yellow),
        ("Cyan", Color::Cyan),
        ("Magenta", Color::Magenta),
        ("White", Color::White),
        ("Grey", Color::Grey),
        ("Black", Color::Black),
        ("Orange", Color::Rgb { r: 255, g: 165, b: 0 }),
        ("Pink", Color::Rgb { r: 255, g: 105, b: 180 }),
        ("Purple", Color::Rgb { r: 128, g: 0, b: 128 }),
        ("Teal", Color::Rgb { r: 0, g: 128, b: 128 }),
        ("Brown", Color::Rgb { r: 139, g: 69, b: 19 }),
        ("Light Blue", Color::Rgb { r: 173, g: 216, b: 230 }),
        ("Light Green", Color::Rgb { r: 144, g: 238, b: 144 }),
        ("Light Yellow", Color::Rgb { r: 255, g: 255, b: 224 }),
        ("Light Cyan", Color::Rgb { r: 224, g: 255, b: 255 }),
        ("Light Magenta", Color::Rgb { r: 255, g: 224, b: 255 }),
        ("Light Grey", Color::Rgb { r: 211, g: 211, b: 211 }),
        ("Dark Grey", Color::Rgb { r: 169, g: 169, b: 169 }),
        ("Dark Blue", Color::Rgb { r: 0, g: 0, b: 139 }),
        ("Dark Orange", Color::Rgb { r: 255, g: 140, b: 0 }),
        ("Dark Pink", Color::Rgb { r: 255, g: 20, b: 147 }),
        ("Dark Purple", Color::Rgb { r: 75, g: 0, b: 130 }),
        ("Dark Teal", Color::Rgb { r: 0, g: 139, b: 139 }),
        ("Dark Brown", Color::Rgb { r: 101, g: 67, b: 33 }),
    ]
}

/// Deprecated: Use `colors::print()` instead.
pub fn print_color_swatch() {
    println!("[Deprecated] Color Swatch:");
    for (name, color) in list_named_colors() {
        println!("{}", format!("{:<15}", name).with(color));
    }
}

 /// Deprecated: Use `colors::get(name: &str)` instead.
pub fn get_color_by_name(name: &str) -> Option<Color> {
    list_named_colors()
        .into_iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, c)| c)
}