use crossterm::style::{Color, Stylize};

/// Generates a horizontal gradient between two colors over N steps.
/// This can be used for printing rainbow text or gradual transitions.
pub fn generate_gradient(text: &str, start: Color, end: Color) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let steps = chars.len().max(1);
    let mut result = Vec::with_capacity(steps);

    for (i, c) in chars.iter().enumerate() {
        let r = interpolate(get_r(&start), get_r(&end), i, steps);
        let g = interpolate(get_g(&start), get_g(&end), i, steps);
        let b = interpolate(get_b(&start), get_b(&end), i, steps);

        let color = Color::Rgb { r, g, b };
        result.push(c.to_string().with(color).to_string());
    }

    result
}

/// Print gradient text directly to stdout.
pub fn print_gradient_line(text: &str, start: Color, end: Color) {
    let gradient = generate_gradient(text, start, end);
    for part in gradient {
        print!("{}", part);
    }
    println!();
}

// ---- Internal RGB utilities ----

fn get_r(c: &Color) -> u8 {
    match c {
        Color::Rgb { r, .. } => *r,
        _ => 255,
    }
}

fn get_g(c: &Color) -> u8 {
    match c {
        Color::Rgb { g, .. } => *g,
        _ => 255,
    }
}

fn get_b(c: &Color) -> u8 {
    match c {
        Color::Rgb { b, .. } => *b,
        _ => 255,
    }
}

fn interpolate(start: u8, end: u8, step: usize, total: usize) -> u8 {
    let start_f = start as f32;
    let end_f = end as f32;
    let ratio = step as f32 / (total - 1).max(1) as f32;
    (start_f + (end_f - start_f) * ratio).round() as u8
}
