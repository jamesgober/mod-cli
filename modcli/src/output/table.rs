use terminal_size::{terminal_size, Width};
use console::measure_text_width;

pub enum TableMode {
    Flex,
    Fixed(usize),
    Full,
}

pub enum TableStyle {
    Ascii,
    Rounded,
    Heavy,
}

pub fn render_table(headers: &[&str], rows: &[Vec<&str>], mode: TableMode, style: TableStyle) {
    let term_width = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80);
    let col_count = headers.len().max(1);
    let padding = 1;
    let total_padding = (col_count - 1) * padding;

    let col_width = match mode {
        TableMode::Fixed(width) => width,
        TableMode::Full => {
            let border_space = col_count + 1; // ┏┃┃┃┓ = 4 columns + 2 sides = 5 chars
            let usable = term_width.saturating_sub(border_space);
            usable / col_count
        },
        TableMode::Flex => {
            let content_max = headers.iter()
                .map(|h| measure_text_width(h))
                .chain(rows.iter().flat_map(|r| r.iter().map(|c| measure_text_width(c))))
                .max()
                .unwrap_or(10);
            content_max.min((term_width.saturating_sub(total_padding)) / col_count)
        }
    };

    let border = match style {
        TableStyle::Ascii => BorderSet::ascii(),
        TableStyle::Rounded => BorderSet::rounded(),
        TableStyle::Heavy => BorderSet::heavy(),
    };

    // Top Border
    print!("{}", border.top_left);
    for i in 0..col_count {
        print!("{}", border.horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            print!("{}", border.top_cross);
        }
    }
    println!("{}", border.top_right);

    // Header Row
    print!("{}", border.vertical);
    for (i, h) in headers.iter().enumerate() {
        print!("{}{}", pad_cell(h, col_width), border.vertical);
    }
    println!();

    // Mid Border
    print!("{}", border.mid_left);
    for i in 0..col_count {
        print!("{}", border.inner_horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            print!("{}", border.mid_cross);
        }
    }
    println!("{}", border.mid_right);

    // Body Rows
    for row in rows {
        print!("{}", border.vertical);
        for cell in row {
            print!("{}{}", pad_cell(cell, col_width), border.vertical);
        }
        println!();
    }

    // Bottom Border
    print!("{}", border.bottom_left);
    for i in 0..col_count {
        print!("{}", border.horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            print!("{}", border.bottom_cross);
        }
    }
    println!("{}", border.bottom_right);
}

fn pad_cell(cell: &str, width: usize) -> String {
    let visual = measure_text_width(cell);
    let pad = width.saturating_sub(visual);
    format!("{}{}", cell, " ".repeat(pad))
}

struct BorderSet {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    top_cross: char,
    bottom_cross: char,
    mid_cross: char,
    mid_left: char,
    mid_right: char,
    horizontal: char,
    inner_horizontal: char,
    vertical: char,
}

impl BorderSet {
    fn ascii() -> Self {
        Self {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            top_cross: '+',
            bottom_cross: '+',
            mid_cross: '+',
            mid_left: '+',
            mid_right: '+',
            horizontal: '-',
            inner_horizontal: '-',
            vertical: '|',
        }
    }

    fn rounded() -> Self {
        Self {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            top_cross: '┬',
            bottom_cross: '┴',
            mid_cross: '┼',
            mid_left: '├',
            mid_right: '┤',
            horizontal: '─',
            inner_horizontal: '─',
            vertical: '│',
        }
    }

    fn heavy() -> Self {
        Self {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            top_cross: '┳',
            bottom_cross: '┻',
            mid_cross: '╋',
            mid_left: '┣',
            mid_right: '┫',
            horizontal: '━',
            inner_horizontal: '━',
            vertical: '┃',
        }
    }
}