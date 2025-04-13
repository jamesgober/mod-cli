use terminal_size::{Width, terminal_size};

/// Renders a table with optional header and rows.
/// Truncates columns if they exceed terminal width.
pub fn render_table(headers: &[&str], rows: &[Vec<&str>]) {
    let term_width = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80);
    let col_count = headers.len();
    let col_width = (term_width - col_count - 1) / col_count; // -1 for borders

    // Render headers
    let header_line: Vec<String> = headers.iter()
        .map(|h| format!("{:^width$}", h, width = col_width))
        .collect();
    println!("{}", header_line.join(" | "));

    // Divider
    println!("{}", "-".repeat(term_width.min(100)));

    // Render rows
    for row in rows {
        let row_line: Vec<String> = row.iter()
            .map(|cell| format!("{:<width$}", cell, width = col_width))
            .collect();
        println!("{}", row_line.join(" | "));
    }
}