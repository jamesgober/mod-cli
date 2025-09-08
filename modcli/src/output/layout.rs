use console::measure_text_width;
use terminal_size::{terminal_size, Width};

#[derive(Clone, Copy, Debug)]
pub enum WidthSpec {
    Fixed(usize),
    Percent(u16),
    Auto,
}

impl Default for WidthSpec {
    fn default() -> Self {
        WidthSpec::Auto
    }
}

#[derive(Debug, Default)]
pub struct Column {
    pub width: WidthSpec,
    pub gap: usize,
    pub content: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Row {
    pub cols: Vec<Column>,
}

#[derive(Debug, Default)]
pub struct Layout {
    pub rows: Vec<Row>,
    pub hgap: usize,
    pub vgap: usize,
    pub border: bool,
}

pub struct Builder {
    layout: Layout,
    current: Row,
}

pub fn build() -> Builder {
    Builder {
        layout: Layout::default(),
        current: Row::default(),
    }
}

impl Builder {
    pub fn row(mut self) -> Self {
        self.current = Row::default();
        self
    }

    pub fn col_fixed(mut self, width: usize) -> Self {
        self.current.cols.push(Column {
            width: WidthSpec::Fixed(width),
            gap: 1,
            content: Vec::new(),
        });
        self
    }

    pub fn col_percent(mut self, pct: u16) -> Self {
        self.current.cols.push(Column {
            width: WidthSpec::Percent(pct.min(100)),
            gap: 1,
            content: Vec::new(),
        });
        self
    }

    pub fn col_auto(mut self) -> Self {
        self.current.cols.push(Column {
            width: WidthSpec::Auto,
            gap: 1,
            content: Vec::new(),
        });
        self
    }

    pub fn content<I: IntoIterator<Item = String>>(mut self, lines: I) -> Self {
        if let Some(col) = self.current.cols.last_mut() {
            col.content.extend(lines);
        }
        self
    }

    pub fn hgap(mut self, gap: usize) -> Self {
        self.layout.hgap = gap;
        self
    }
    pub fn vgap(mut self, gap: usize) -> Self {
        self.layout.vgap = gap;
        self
    }
    pub fn border(mut self, yes: bool) -> Self {
        self.layout.border = yes;
        self
    }

    pub fn end_row(mut self) -> Self {
        if !self.current.cols.is_empty() {
            self.layout.rows.push(std::mem::take(&mut self.current));
        }
        self
    }

    pub fn finish(mut self) -> Layout {
        if !self.current.cols.is_empty() {
            self.layout.rows.push(self.current);
        }
        self.layout
    }
}

pub fn render(layout: &Layout) -> String {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
    let mut out = String::new();

    for (ri, row) in layout.rows.iter().enumerate() {
        if ri > 0 {
            out.push_str(&"\n".repeat(layout.vgap.max(0)));
        }

        // Compute column widths
        let mut fixed_total = 0usize;
        let mut pct_total = 0u16;
        let mut auto_count = 0usize;
        for c in &row.cols {
            match c.width {
                WidthSpec::Fixed(w) => fixed_total += w,
                WidthSpec::Percent(p) => pct_total = pct_total.saturating_add(p),
                WidthSpec::Auto => auto_count += 1,
            }
        }
        let gaps_total = layout.hgap.saturating_mul(row.cols.len().saturating_sub(1));
        let base_rem = term_width.saturating_sub(fixed_total + gaps_total);
        let _pct_pixels = ((base_rem as u128) * (pct_total as u128) / 100u128) as usize;
        let mut widths: Vec<usize> = Vec::with_capacity(row.cols.len());

        // First pass: assign fixed + percent
        for c in &row.cols {
            match c.width {
                WidthSpec::Fixed(w) => widths.push(w),
                WidthSpec::Percent(p) => {
                    widths.push(((base_rem as u128) * (p as u128) / 100u128) as usize)
                }
                WidthSpec::Auto => widths.push(0),
            }
        }
        // Remaining for autos
        let used_except_auto: usize = widths.iter().sum();
        let remaining = term_width.saturating_sub(used_except_auto + gaps_total);
        let auto_share = if auto_count > 0 {
            remaining / auto_count
        } else {
            0
        };
        for (i, c) in row.cols.iter().enumerate() {
            if matches!(c.width, WidthSpec::Auto) {
                widths[i] = auto_share;
            }
        }

        // Prepare columns as wrapped lines
        let mut prepared: Vec<Vec<String>> = Vec::with_capacity(row.cols.len());
        let mut max_lines = 0usize;
        for (i, c) in row.cols.iter().enumerate() {
            let w = widths[i].max(1);
            let mut lines: Vec<String> = Vec::new();
            for line in &c.content {
                lines.extend(wrap_to_width(line, w));
            }
            max_lines = max_lines.max(lines.len());
            prepared.push(lines);
        }
        // Pad shorter cols
        for lines in prepared.iter_mut() {
            while lines.len() < max_lines {
                lines.push(String::new());
            }
        }

        // Optional border top
        if layout.border {
            out.push_str(&render_border_line(&widths, '┌', '┬', '┐', '─'));
            out.push('\n');
        }

        // Emit lines
        for li in 0..max_lines {
            if layout.border {
                out.push('│');
            }
            for (ci, w) in widths.iter().enumerate() {
                let cell = prepared[ci][li].clone();
                out.push_str(&pad_right(&cell, *w));
                if ci < widths.len() - 1 {
                    if layout.border {
                        out.push('│');
                    }
                    out.push_str(&" ".repeat(layout.hgap));
                    if layout.border {
                        out.push('│');
                    }
                }
            }
            if layout.border {
                out.push('│');
            }
            out.push('\n');
        }

        // Optional border bottom
        if layout.border {
            out.push_str(&render_border_line(&widths, '└', '┴', '┘', '─'));
        }
    }

    out
}

fn render_border_line(widths: &[usize], left: char, cross: char, right: char, h: char) -> String {
    let mut s = String::new();
    s.push(left);
    for (i, w) in widths.iter().enumerate() {
        s.push_str(&h.to_string().repeat(*w));
        if i < widths.len() - 1 {
            s.push(cross);
        }
    }
    s.push(right);
    s
}

fn pad_right(s: &str, width: usize) -> String {
    let vis = measure_text_width(s);
    let pad = width.saturating_sub(vis);
    format!("{s}{}", " ".repeat(pad))
}

fn wrap_to_width(s: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![String::new()];
    }
    let mut lines = Vec::new();
    let mut cur = String::new();
    for ch in s.chars() {
        let next = format!("{cur}{ch}");
        if measure_text_width(&next) > width {
            if cur.is_empty() {
                lines.push(ch.to_string());
            } else {
                lines.push(std::mem::take(&mut cur));
                cur.push(ch);
            }
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}
