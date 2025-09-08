use crossterm::style::{Color, Stylize};

/// Generates a horizontal gradient between two colors.
/// This can be used for printing rainbow text or gradual transitions.
/// This function returns a single string with the gradient applied.
/// The text is split into characters, and each character is colored
/// according to its position in the gradient.
/// The gradient is calculated by interpolating between the start and end colors
/// based on the character's index.
/// The result is a string where each character is styled with its corresponding color.
pub fn generate(text: &str, start: Color, end: Color) -> String {
    let chars: Vec<char> = text.chars().collect();
    let steps = chars.len().max(1);
    let mut result = String::with_capacity(text.len() * 10); // estimate

    for (i, c) in chars.iter().enumerate() {
        let r = interpolate(get_r(&start), get_r(&end), i, steps);
        let g = interpolate(get_g(&start), get_g(&end), i, steps);
        let b = interpolate(get_b(&start), get_b(&end), i, steps);
        let color = Color::Rgb { r, g, b };
        result.push_str(&c.to_string().with(color).to_string());
    }

    /// Common easing modes for gradients
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum Easing {
        Linear,
        EaseIn,
        EaseOut,
        EaseInOut,
    }

    #[allow(dead_code)]
    #[inline(always)]
    fn apply_easing(t: f32, mode: Easing) -> f32 {
        match mode {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }

    /// Multi-color gradient with easing across the full text length.
    /// Easing is applied to the overall position before segment selection.
    #[allow(dead_code)]
    pub fn multi_color_eased(text: &str, colors: Vec<Color>, ease: Easing) -> String {
        let chars: Vec<char> = text.chars().collect();
        let steps = chars.len().max(1);
        let segments = colors.len().saturating_sub(1).max(1);
        let mut result = String::with_capacity(text.len() * 10);

        for (i, c) in chars.iter().enumerate() {
            let t_raw = i as f32 / (steps - 1).max(1) as f32;
            let t = apply_easing(t_raw, ease).clamp(0.0, 1.0);
            let seg_float = t * segments as f32;
            let seg = seg_float.floor() as usize;
            let seg_t = seg_float - seg as f32;

            let from = colors.get(seg).unwrap_or(&colors[0]);
            let to = colors.get(seg + 1).unwrap_or(from);

            let r = interpolate(get_r(from), get_r(to), (seg_t * 100.0) as usize, 100);
            let g = interpolate(get_g(from), get_g(to), (seg_t * 100.0) as usize, 100);
            let b = interpolate(get_b(from), get_b(to), (seg_t * 100.0) as usize, 100);

            let color = Color::Rgb { r, g, b };
            result.push_str(&c.to_string().with(color).to_string());
        }

        result
    }

    /// A small set of palette stops for viridis (approximation)
    #[allow(dead_code)]
    pub fn palette_viridis() -> Vec<Color> {
        vec![
            Color::Rgb { r: 68, g: 1, b: 84 },
            Color::Rgb {
                r: 59,
                g: 82,
                b: 139,
            },
            Color::Rgb {
                r: 33,
                g: 145,
                b: 140,
            },
            Color::Rgb {
                r: 94,
                g: 201,
                b: 97,
            },
            Color::Rgb {
                r: 253,
                g: 231,
                b: 37,
            },
        ]
    }

    /// A small set of palette stops for magma (approximation)
    #[allow(dead_code)]
    pub fn palette_magma() -> Vec<Color> {
        vec![
            Color::Rgb { r: 0, g: 0, b: 4 },
            Color::Rgb {
                r: 73,
                g: 15,
                b: 99,
            },
            Color::Rgb {
                r: 187,
                g: 55,
                b: 84,
            },
            Color::Rgb {
                r: 249,
                g: 142,
                b: 8,
            },
            Color::Rgb {
                r: 252,
                g: 253,
                b: 191,
            },
        ]
    }

    result
}

/// Alias for generate()
/// This function is a convenience function that calls the `generate` function
/// with the provided text and colors.
/// It returns the generated gradient string.
#[inline(always)]
pub fn two_color(text: &str, start: Color, end: Color) -> String {
    generate(text, start, end)
}

/// Generates a 3-color gradient (start -> mid, mid -> end)
/// This function creates a gradient that transitions from the start color to the mid color,
/// and then from the mid color to the end color.
pub fn three_color(text: &str, start: Color, mid: Color, end: Color) -> String {
    let chars: Vec<char> = text.chars().collect();
    let total = chars.len().max(1);
    let midpoint = total / 2;
    let mut result = String::with_capacity(text.len() * 10);

    for (i, c) in chars.iter().enumerate() {
        let (from, to, step, steps) = if i < midpoint {
            (start, mid, i, midpoint)
        } else {
            (mid, end, i - midpoint, total - midpoint)
        };

        let r = interpolate(get_r(&from), get_r(&to), step, steps);
        let g = interpolate(get_g(&from), get_g(&to), step, steps);
        let b = interpolate(get_b(&from), get_b(&to), step, steps);
        let color = Color::Rgb { r, g, b };
        result.push_str(&c.to_string().with(color).to_string());
    }

    result
}

/// Generates a gradient from a vector of colors, distributed across text.
/// This function creates a gradient that transitions through multiple colors.
/// The colors are evenly distributed across the text.
pub fn multi_color(text: &str, colors: Vec<Color>) -> String {
    let chars: Vec<char> = text.chars().collect();
    let steps = chars.len().max(1);
    let segments = colors.len().saturating_sub(1).max(1);
    let mut result = String::with_capacity(text.len() * 10);

    for (i, c) in chars.iter().enumerate() {
        let t = i as f32 / (steps - 1).max(1) as f32;
        let seg_float = t * segments as f32;
        let seg = seg_float.floor() as usize;
        let seg_t = seg_float - seg as f32;

        let from = colors.get(seg).unwrap_or(&colors[0]);
        let to = colors.get(seg + 1).unwrap_or(from);

        let r = interpolate(get_r(from), get_r(to), (seg_t * 100.0) as usize, 100);
        let g = interpolate(get_g(from), get_g(to), (seg_t * 100.0) as usize, 100);
        let b = interpolate(get_b(from), get_b(to), (seg_t * 100.0) as usize, 100);

        let color = Color::Rgb { r, g, b };
        result.push_str(&c.to_string().with(color).to_string());
    }

    result
}

// Internal RGB helpers
/// Gets the red, green, and blue components of a color.
/// These functions extract the respective color components from a Color.
/// If the color is not an RGB color, it returns 255.
/// This is useful for interpolating colors in the gradient.
/// The functions use pattern matching to check the color type.
/// Get the red component of a color.
/// This function extracts the red component from a Color.
#[inline(always)]
fn get_r(c: &Color) -> u8 {
    match c {
        Color::Rgb { r, .. } => *r,
        _ => 255,
    }
}

/// Get the green component of a color.
/// This function extracts the green component from a Color.
#[inline(always)]
fn get_g(c: &Color) -> u8 {
    match c {
        Color::Rgb { g, .. } => *g,
        _ => 255,
    }
}

/// Get the blue component of a color.
/// This function extracts the blue component from a Color.
#[inline(always)]
fn get_b(c: &Color) -> u8 {
    match c {
        Color::Rgb { b, .. } => *b,
        _ => 255,
    }
}

/// Interpolates between two color components.
/// This function calculates the interpolated value between two color components.
/// It takes the start and end values, the current step, and the total number of steps.
/// The interpolation is done using a linear formula.
/// The result is rounded to the nearest integer and returned as a u8.
#[inline(always)]
fn interpolate(start: u8, end: u8, step: usize, total: usize) -> u8 {
    let start_f = start as f32;
    let end_f = end as f32;
    let ratio = step as f32 / (total - 1).max(1) as f32;
    (start_f + (end_f - start_f) * ratio).round() as u8
}
