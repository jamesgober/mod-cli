/// Minimal Markdown -> ANSI renderer for help text.
/// Supported:
/// - Headings: #, ##, ### (bold)
/// - Lists: lines starting with "- " become bullets
/// - Inline: **bold**, *italic*, `code`
/// - Paragraphs: preserved
pub fn render_markdown(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 32);
    for line in input.lines() {
        let trimmed = line.trim_start();
        let styled_line = if let Some(rest) = trimmed.strip_prefix("### ") {
            format!("\x1b[1m{}\x1b[0m", render_inline(rest))
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            format!("\x1b[1m{}\x1b[0m", render_inline(rest))
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            format!("\x1b[1m{}\x1b[0m", render_inline(rest))
        } else if let Some(rest) = trimmed.strip_prefix("- ") {
            format!(" • {}", render_inline(rest))
        } else {
            render_inline(line)
        };
        out.push_str(&styled_line);
        out.push('\n');
    }
    out
}

fn render_inline(s: &str) -> String {
    // Replace code spans first to avoid conflicts with bold/italic
    let mut out = String::new();
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'`' {
            if let Some(j) = find_next(bytes, i + 1, b'`') {
                out.push_str("\x1b[7m"); // inverse
                out.push_str(&s[i + 1..j]);
                out.push_str("\x1b[0m");
                i = j + 1;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    // Bold **...** and italic *...*
    let s2 = out;
    let s3 = replace_enclosed(&s2, "**", "\x1b[1m", "\x1b[0m");
    replace_enclosed(&s3, "*", "\x1b[3m", "\x1b[0m")
}

fn find_next(bytes: &[u8], mut i: usize, ch: u8) -> Option<usize> {
    while i < bytes.len() {
        if bytes[i] == ch {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn replace_enclosed(s: &str, token: &str, start: &str, end: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    let tlen = token.len();
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if i + tlen <= bytes.len() && &s[i..i + tlen] == token {
            if let Some(j) = find_token(&s[i + tlen..], token) {
                out.push_str(start);
                out.push_str(&s[i + tlen..i + tlen + j]);
                out.push_str(end);
                i = i + tlen + j + tlen;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn find_token(hay: &str, token: &str) -> Option<usize> {
    hay.find(token)
}
