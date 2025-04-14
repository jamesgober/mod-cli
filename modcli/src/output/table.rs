use terminal_size::{Width, terminal_size};


pub fn render_table(headers: &[&str], rows: &[Vec<&str>]) {
    let term_width = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80);
    let col_count = headers.len().max(1);
    let padding = 3; // space for " | "
    let col_width = ((term_width - (col_count - 1) * padding).saturating_sub(1)) / col_count;

    // Render headers
    let header_line: Vec<String> = headers
        .iter()
        .map(|h| format!("{:^width$}", h, width = col_width))
        .collect();
    println!("{}", header_line.join(" | "));

    // Divider
    println!("{}", "-".repeat(term_width.min(col_count * (col_width + padding))));

    // Render rows
    for row in rows {
        let row_line: Vec<String> = row
            .iter()
            .map(|cell| {
                if cell.len() > col_width {
                    format!("{:.width$}", cell, width = col_width - 1).to_owned() + "…"
                } else {
                    format!("{:<width$}", cell, width = col_width)
                }
            })
            .collect();
        println!("{}", row_line.join(" | "));
    }
}
