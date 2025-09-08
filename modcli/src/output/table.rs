use console::measure_text_width;
use crossterm::style::{Color, Stylize};
use terminal_size::{terminal_size, Width};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[cfg(feature = "table-presets")]
/// Preset sugar: heavy borders, cyan header, separators on by default.
pub fn render_table_preset_heavy_cyan_separators(
    headers: &[&str],
    rows: &[Vec<&str>],
    mode: TableMode,
    alignments: Option<&[Align]>,
    trunc_modes: Option<&[TruncateMode]>,
    row_separators: bool,
) -> String {
    render_table_with_opts_styled(
        headers,
        rows,
        mode,
        TableStyle::Heavy,
        alignments,
        trunc_modes,
        true,
        row_separators,
        Some(crate::output::CYAN),
        Some(crate::output::DARK_BLUE),
    )
}

#[cfg(feature = "table-presets")]
/// Preset sugar: minimal ASCII borders, magenta header, light grey zebra rows.
pub fn render_table_preset_minimal_magenta_grey_zebra(
    headers: &[&str],
    rows: &[Vec<&str>],
    mode: TableMode,
    alignments: Option<&[Align]>,
    trunc_modes: Option<&[TruncateMode]>,
    row_separators: bool,
) -> String {
    render_table_with_opts_styled(
        headers,
        rows,
        mode,
        TableStyle::Ascii,
        alignments,
        trunc_modes,
        true,
        row_separators,
        Some(crate::output::MAGENTA),
        Some(crate::output::LIGHT_GREY),
    )
}

/// Write Markdown table to a file path. Returns Result for error handling.
pub fn write_table_markdown(
    path: &str,
    headers: &[&str],
    rows: &[Vec<&str>],
) -> std::io::Result<()> {
    std::fs::write(path, render_table_markdown(headers, rows))
}

/// Write CSV table to a file path. Returns Result for error handling.
pub fn write_table_csv(path: &str, headers: &[&str], rows: &[Vec<&str>]) -> std::io::Result<()> {
    std::fs::write(path, render_table_csv(headers, rows))
}

/// Styled variant: allow optional header foreground color and zebra row background color.
#[allow(clippy::too_many_arguments)]
pub fn render_table_with_opts_styled(
    headers: &[&str],
    rows: &[Vec<&str>],
    mode: TableMode,
    style: TableStyle,
    alignments: Option<&[Align]>,
    trunc_modes: Option<&[TruncateMode]>,
    zebra: bool,
    row_separators: bool,
    header_fg: Option<Color>,
    zebra_bg: Option<Color>,
) -> String {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
    let col_count = headers.len().max(1);
    let padding: usize = 1;
    let total_padding = (col_count - 1) * padding;

    let col_width = match mode {
        TableMode::Fixed(width) => width,
        TableMode::Full => {
            let border_space = col_count + 1;
            let usable = term_width.saturating_sub(border_space);
            usable / col_count
        }
        TableMode::Flex => {
            let content_max = headers
                .iter()
                .map(|h| measure_text_width(h))
                .chain(
                    rows.iter()
                        .flat_map(|r| r.iter().map(|c| measure_text_width(c))),
                )
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

    let mut out = String::with_capacity(128);

    // Top Border
    out.push(border.top_left);
    for i in 0..col_count {
        out.push_str(&border.horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            out.push(border.top_cross);
        }
    }
    out.push(border.top_right);
    out.push('\n');

    // Header Row (with optional color)
    out.push(border.vertical);
    for h in headers.iter() {
        let a = pick_align(0, alignments);
        let t = pick_trunc(0, trunc_modes);
        let mut cell = pad_cell_with(h, col_width, a, t);
        if let Some(color) = header_fg {
            cell = cell.with(color).bold().to_string();
        }
        out.push_str(&cell);
        out.push(border.vertical);
    }
    out.push('\n');

    // Mid Border
    out.push(border.mid_left);
    for i in 0..col_count {
        out.push_str(&border.inner_horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            out.push(border.mid_cross);
        }
    }
    out.push(border.mid_right);
    out.push('\n');

    // Body Rows (optional zebra bg)
    for (ri, row) in rows.iter().enumerate() {
        out.push(border.vertical);
        for (ci, cell) in row.iter().enumerate() {
            let a = pick_align(ci, alignments);
            let t = pick_trunc(ci, trunc_modes);
            let base = pad_cell_with(cell, col_width, a, t);
            let styled = if zebra && (ri % 2 == 1) {
                if let Some(bg) = zebra_bg {
                    base.on(bg).to_string()
                } else {
                    base
                }
            } else {
                base
            };
            out.push_str(&styled);
            out.push(border.vertical);
        }
        out.push('\n');

        if row_separators && ri < rows.len() - 1 {
            out.push(border.mid_left);
            for i in 0..col_count {
                out.push_str(&border.inner_horizontal.to_string().repeat(col_width));
                if i < col_count - 1 {
                    out.push(border.mid_cross);
                }
            }
            out.push(border.mid_right);
            out.push('\n');
        }
    }

    // Bottom Border
    out.push(border.bottom_left);
    for i in 0..col_count {
        out.push_str(&border.horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            out.push(border.bottom_cross);
        }
    }
    out.push(border.bottom_right);
    out.push('\n');

    out
}

// --- Export adapters ---

/// Render as GitHub-flavored Markdown table.
pub fn render_table_markdown(headers: &[&str], rows: &[Vec<&str>]) -> String {
    let mut out = String::new();
    // header
    out.push('|');
    for h in headers {
        out.push(' ');
        out.push_str(&escape_md(h));
        out.push(' ');
        out.push('|');
    }
    out.push('\n');
    // separator
    out.push('|');
    for _ in headers {
        out.push_str(" --- |");
    }
    out.push('\n');
    // rows
    for row in rows {
        out.push('|');
        for cell in row {
            out.push(' ');
            out.push_str(&escape_md(cell));
            out.push(' ');
            out.push('|');
        }
        out.push('\n');
    }
    out
}

/// Render as CSV. Minimal escaping for quotes and commas/newlines.
pub fn render_table_csv(headers: &[&str], rows: &[Vec<&str>]) -> String {
    let mut out = String::new();
    out.push_str(&join_csv(headers.iter().copied()));
    out.push('\n');
    for row in rows {
        out.push_str(&join_csv(row.iter().copied()));
        out.push('\n');
    }
    out
}

/// Render as JSON array of objects mapping header->value. Not streaming, small tables only.
pub fn render_table_json(headers: &[&str], rows: &[Vec<&str>]) -> String {
    use std::fmt::Write as _;
    let mut out = String::from("[");
    for (ri, row) in rows.iter().enumerate() {
        if ri > 0 {
            out.push(',');
        }
        out.push('{');
        for (ci, h) in headers.iter().enumerate() {
            if ci > 0 {
                out.push(',');
            }
            let _ = write!(
                out,
                "\"{}\":{}",
                escape_json(h),
                json_string(row.get(ci).copied().unwrap_or(""))
            );
        }
        out.push('}');
    }
    out.push(']');
    out
}

fn escape_md(s: &str) -> String {
    s.replace('|', "\\|")
}

fn join_csv<'a, I: IntoIterator<Item = &'a str>>(iter: I) -> String {
    let mut first = true;
    let mut s = String::new();
    for field in iter {
        if !first {
            s.push(',');
        } else {
            first = false;
        }
        s.push_str(&csv_field(field));
    }
    s
}

fn csv_field(s: &str) -> String {
    let need_quotes = s.contains(',') || s.contains('"') || s.contains('\n');
    if need_quotes {
        let escaped = s.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        s.to_string()
    }
}

fn escape_json(s: &str) -> String {
    s.replace('"', "\\\"")
}
fn json_string(s: &str) -> String {
    format!("\"{}\"", escape_json(s))
}

#[derive(Clone, Copy)]
pub enum TruncateMode {
    End,
    Middle,
    Start,
}

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

#[cfg(feature = "table-presets")]
impl TableStyle {
    #[inline]
    pub fn ascii_preset() -> Self {
        TableStyle::Ascii
    }
    #[inline]
    pub fn rounded_preset() -> Self {
        TableStyle::Rounded
    }
    #[inline]
    pub fn heavy_preset() -> Self {
        TableStyle::Heavy
    }
}

pub fn render_table(
    headers: &[&str],
    rows: &[Vec<&str>],
    mode: TableMode,
    style: TableStyle,
) -> String {
    render_table_with(headers, rows, mode, style, None, None)
}

/// Advanced renderer allowing per-column alignment and truncation modes.
pub fn render_table_with(
    headers: &[&str],
    rows: &[Vec<&str>],
    mode: TableMode,
    style: TableStyle,
    alignments: Option<&[Align]>,
    trunc_modes: Option<&[TruncateMode]>,
) -> String {
    render_table_with_opts(
        headers,
        rows,
        mode,
        style,
        alignments,
        trunc_modes,
        false,
        false,
    )
}

/// Advanced renderer with options: per-column alignment/truncation, zebra stripes, and row separators.
#[allow(clippy::too_many_arguments)]
pub fn render_table_with_opts(
    headers: &[&str],
    rows: &[Vec<&str>],
    mode: TableMode,
    style: TableStyle,
    alignments: Option<&[Align]>,
    trunc_modes: Option<&[TruncateMode]>,
    zebra: bool,
    row_separators: bool,
) -> String {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
    let col_count = headers.len().max(1);
    let padding: usize = 1;
    let total_padding = (col_count - 1) * padding;

    let col_width = match mode {
        TableMode::Fixed(width) => width,
        TableMode::Full => {
            let border_space = col_count + 1; // ┏┃┃┃┓ = 4 columns + 2 sides = 5 chars
            let usable = term_width.saturating_sub(border_space);
            usable / col_count
        }
        TableMode::Flex => {
            let content_max = headers
                .iter()
                .map(|h| measure_text_width(h))
                .chain(
                    rows.iter()
                        .flat_map(|r| r.iter().map(|c| measure_text_width(c))),
                )
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

    let mut out = String::with_capacity(128);

    // Top Border
    out.push(border.top_left);
    for i in 0..col_count {
        out.push_str(&border.horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            out.push(border.top_cross);
        }
    }
    out.push(border.top_right);
    out.push('\n');

    // Header Row
    out.push(border.vertical);
    for h in headers.iter() {
        let a = pick_align(0, alignments);
        let t = pick_trunc(0, trunc_modes);
        out.push_str(&pad_cell_with(h, col_width, a, t));
        out.push(border.vertical);
    }
    out.push('\n');

    // Mid Border
    out.push(border.mid_left);
    for i in 0..col_count {
        out.push_str(&border.inner_horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            out.push(border.mid_cross);
        }
    }
    out.push(border.mid_right);
    out.push('\n');

    // Body Rows
    for (ri, row) in rows.iter().enumerate() {
        out.push(border.vertical);
        for (ci, cell) in row.iter().enumerate() {
            let a = pick_align(ci, alignments);
            let t = pick_trunc(ci, trunc_modes);
            let mut cell_s = pad_cell_with(cell, col_width, a, t);
            if zebra && (ri % 2 == 1) {
                // lightweight zebra: replace spaces in padding with middle dot for visibility
                // keeps width identical
                cell_s = cell_s.replace(' ', "·");
            }
            out.push_str(&cell_s);
            out.push(border.vertical);
        }
        out.push('\n');

        if row_separators && ri < rows.len() - 1 {
            // Inner separator line between rows
            out.push(border.mid_left);
            for i in 0..col_count {
                out.push_str(&border.inner_horizontal.to_string().repeat(col_width));
                if i < col_count - 1 {
                    out.push(border.mid_cross);
                }
            }
            out.push(border.mid_right);
            out.push('\n');
        }
    }

    // Bottom Border
    out.push(border.bottom_left);
    for i in 0..col_count {
        out.push_str(&border.horizontal.to_string().repeat(col_width));
        if i < col_count - 1 {
            out.push(border.bottom_cross);
        }
    }
    out.push(border.bottom_right);
    out.push('\n');

    out
}

/// Column-specific width specification.
#[derive(Clone, Copy)]
pub enum ColWidth {
    Fixed(usize),
    Percent(u16),
    Auto,
}

/// Render with explicit per-column widths.
#[allow(clippy::too_many_arguments)]
pub fn render_table_with_columns(
    headers: &[&str],
    rows: &[Vec<&str>],
    style: TableStyle,
    columns: &[ColWidth],
    alignments: Option<&[Align]>,
    trunc_modes: Option<&[TruncateMode]>,
    zebra: bool,
    row_separators: bool,
) -> String {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
    let col_count = headers.len().max(1);
    let padding: usize = 1;
    let gaps_total = padding.saturating_mul(col_count.saturating_sub(1));

    // Compute column widths
    let mut widths = vec![0usize; col_count];
    let mut fixed_total = 0usize;
    let mut pct_total = 0u16;
    let mut auto_count = 0usize;
    for (i, spec) in columns.iter().enumerate().take(col_count) {
        match spec {
            ColWidth::Fixed(w) => {
                widths[i] = *w;
                fixed_total = fixed_total.saturating_add(*w);
            }
            ColWidth::Percent(p) => {
                pct_total = pct_total.saturating_add(*p);
            }
            ColWidth::Auto => {
                auto_count += 1;
            }
        }
    }

    let base_rem = term_width.saturating_sub(fixed_total + gaps_total);
    // Assign percent columns proportional to base_rem
    for (i, spec) in columns.iter().enumerate().take(col_count) {
        if let ColWidth::Percent(p) = spec {
            let w = ((base_rem as u128) * (*p as u128) / 100u128) as usize;
            widths[i] = w;
        }
    }
    // Remaining space goes to autos evenly
    let used_except_auto: usize = widths.iter().sum();
    let remaining = term_width.saturating_sub(used_except_auto + gaps_total);
    let auto_share = if auto_count > 0 {
        remaining / auto_count
    } else {
        0
    };
    for (i, spec) in columns.iter().enumerate().take(col_count) {
        if matches!(spec, ColWidth::Auto) {
            widths[i] = auto_share;
        }
    }

    let border = match style {
        TableStyle::Ascii => BorderSet::ascii(),
        TableStyle::Rounded => BorderSet::rounded(),
        TableStyle::Heavy => BorderSet::heavy(),
    };
    let mut out = String::with_capacity(128);

    // Top
    out.push(border.top_left);
    for (i, w) in widths.iter().enumerate() {
        out.push_str(&border.horizontal.to_string().repeat(*w));
        if i < widths.len() - 1 {
            out.push(border.top_cross);
        }
    }
    out.push(border.top_right);
    out.push('\n');

    // Header
    out.push(border.vertical);
    for (ci, h) in headers.iter().enumerate() {
        let a = pick_align(ci, alignments);
        let t = pick_trunc(ci, trunc_modes);
        out.push_str(&pad_cell_with(h, widths[ci].max(1), a, t));
        out.push(border.vertical);
    }
    out.push('\n');

    // Mid
    out.push(border.mid_left);
    for (i, w) in widths.iter().enumerate() {
        out.push_str(&border.inner_horizontal.to_string().repeat(*w));
        if i < widths.len() - 1 {
            out.push(border.mid_cross);
        }
    }
    out.push(border.mid_right);
    out.push('\n');

    // Rows
    for (ri, row) in rows.iter().enumerate() {
        out.push(border.vertical);
        for (ci, cell) in row.iter().enumerate() {
            let a = pick_align(ci, alignments);
            let t = pick_trunc(ci, trunc_modes);
            let mut cell_s = pad_cell_with(cell, widths[ci].max(1), a, t);
            if zebra && (ri % 2 == 1) {
                cell_s = cell_s.replace(' ', "·");
            }
            out.push_str(&cell_s);
            out.push(border.vertical);
        }
        out.push('\n');

        if row_separators && ri < rows.len() - 1 {
            out.push(border.mid_left);
            for (i, w) in widths.iter().enumerate() {
                out.push_str(&border.inner_horizontal.to_string().repeat(*w));
                if i < widths.len() - 1 {
                    out.push(border.mid_cross);
                }
            }
            out.push(border.mid_right);
            out.push('\n');
        }
    }

    // Bottom
    out.push(border.bottom_left);
    for (i, w) in widths.iter().enumerate() {
        out.push_str(&border.horizontal.to_string().repeat(*w));
        if i < widths.len() - 1 {
            out.push(border.bottom_cross);
        }
    }
    out.push(border.bottom_right);
    out.push('\n');

    out
}

/// Helper to pick alignment for a given column index with fallback.
fn pick_align(idx: usize, aligns: Option<&[Align]>) -> Align {
    aligns
        .and_then(|arr| arr.get(idx).copied())
        .unwrap_or(Align::Left)
}

/// Helper to pick truncate mode for a given column index with fallback.
fn pick_trunc(idx: usize, truncs: Option<&[TruncateMode]>) -> TruncateMode {
    truncs
        .and_then(|arr| arr.get(idx).copied())
        .unwrap_or(TruncateMode::End)
}

/// Truncates the cell to fit `width` characters visually, then pads according to `align`.
fn pad_cell_with(cell: &str, width: usize, align: Align, trunc: TruncateMode) -> String {
    let truncated = truncate_to_width_mode(cell, width, trunc);
    let visual = measure_text_width(&truncated);
    let pad = width.saturating_sub(visual);
    match align {
        Align::Left => format!("{truncated}{}", " ".repeat(pad)),
        Align::Right => format!("{}{truncated}", " ".repeat(pad)),
        Align::Center => {
            let left = pad / 2;
            let right = pad - left;
            format!("{}{}{}", " ".repeat(left), truncated, " ".repeat(right))
        }
    }
}

/// Best-effort truncate that respects visual width using `console::measure_text_width`.
/// If the content exceeds `width`, it trims to `width-1` and appends '…'.
fn truncate_to_width_mode(cell: &str, width: usize, mode: TruncateMode) -> String {
    if width == 0 {
        return String::new();
    }
    let visual = measure_text_width(cell);
    if visual <= width {
        return cell.to_string();
    }

    // Reserve room for ellipsis
    let target = width.saturating_sub(1);
    // Work with grapheme clusters to avoid splitting emojis or accents
    let g = UnicodeSegmentation::graphemes(cell, true).collect::<Vec<&str>>();
    match mode {
        TruncateMode::End => {
            let mut out = String::new();
            for gr in &g {
                let next = format!("{out}{gr}");
                if measure_text_width(&next) > target {
                    break;
                }
                out.push_str(gr);
            }
            out.push('…');
            out
        }
        TruncateMode::Start => {
            let mut tail_rev: Vec<&str> = Vec::new();
            for gr in g.iter().rev() {
                let candidate = tail_rev
                    .iter()
                    .cloned()
                    .rev()
                    .chain(std::iter::once(*gr))
                    .collect::<String>();
                if measure_text_width(&candidate) > target {
                    break;
                }
                tail_rev.push(gr);
            }
            let tail: String = tail_rev.into_iter().rev().collect();
            format!("…{tail}")
        }
        TruncateMode::Middle => {
            let mut head = String::new();
            let mut tail_rev: Vec<&str> = Vec::new();
            let mut left_i = 0usize;
            let mut right_i = g.len();
            loop {
                let current = format!(
                    "{head}…{}",
                    tail_rev.iter().rev().cloned().collect::<String>()
                );
                if measure_text_width(&current) > width {
                    break;
                }
                // Try extend head first
                if left_i < right_i {
                    let next = format!("{head}{}", g[left_i]);
                    let cur2 = format!(
                        "{next}…{}",
                        tail_rev.iter().rev().cloned().collect::<String>()
                    );
                    if measure_text_width(&cur2) <= width {
                        head.push_str(g[left_i]);
                        left_i += 1;
                        continue;
                    }
                }
                // Then try extend tail
                if right_i > left_i {
                    let cand_tail = {
                        let mut tmp = tail_rev.clone();
                        if right_i > 0 {
                            tmp.push(g[right_i - 1]);
                        }
                        tmp
                    };
                    let cur2 = format!(
                        "{head}…{}",
                        cand_tail.iter().rev().cloned().collect::<String>()
                    );
                    if measure_text_width(&cur2) <= width {
                        tail_rev.push(g[right_i - 1]);
                        right_i -= 1;
                        continue;
                    }
                }
                break;
            }
            format!(
                "{head}…{}",
                tail_rev.iter().rev().cloned().collect::<String>()
            )
        }
    }
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
