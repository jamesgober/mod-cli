use crate::output::{colors, gradient};
use crossterm::style::Color;

/// Gradient helpers that accept named colors. Requires `features=["gradients"]`.
/// These are thin wrappers over the existing gradient module.

/// Two-color gradient using named colors.
pub fn two_named(text: &str, from: &str, to: &str) -> String {
    let c1 = colors::get(from);
    let c2 = colors::get(to);
    gradient::two_color(text, c1, c2)
}

/// Multi-color gradient using a list of named colors.
pub fn multi_named(text: &str, names: &[&str]) -> String {
    let stops: Vec<Color> = names.iter().map(|n| colors::get(n)).collect();
    gradient::multi_color(text, stops)
}
