/// Minimal Markdown -> ANSI renderer for help text.
/// Supported:
/// - Headings: #, ##, ### (bold)
/// - Lists: lines starting with "- " become bullets
/// - Inline: **bold**, *italic*, `code`
/// - Paragraphs: preserved
///
/// Any UTF-8 text is accepted. Text outside the markers is copied unchanged.
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
    let code = replace_enclosed(s, "`", "\x1b[7m", "\x1b[0m");
    // Bold **...** and italic *...*
    let bold = replace_enclosed(&code, "**", "\x1b[1m", "\x1b[0m");
    replace_enclosed(&bold, "*", "\x1b[3m", "\x1b[0m")
}

/// Wraps each `token ... token` pair in `start ... end`. An unpaired token is
/// kept as text.
///
/// The tokens are ASCII, so every offset `find` returns is a char boundary and
/// the text between markers is copied as whole slices.
fn replace_enclosed(s: &str, token: &str, start: &str, end: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let tlen = token.len();
    let mut rest = s;
    while let Some(open) = rest.find(token) {
        let after = &rest[open + tlen..];
        let Some(close) = after.find(token) else {
            break;
        };
        out.push_str(&rest[..open]);
        out.push_str(start);
        out.push_str(&after[..close]);
        out.push_str(end);
        rest = &after[close + tlen..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_ascii_text_is_kept() {
        assert_eq!(render_markdown("caf\u{e9}"), "caf\u{e9}\n");
        assert_eq!(
            render_markdown("Deploy \u{2014} ship it"),
            "Deploy \u{2014} ship it\n"
        );
        assert_eq!(
            render_markdown("**\u{e9}t\u{e9}** and *\u{65e5}\u{672c}* `\u{1f600}`"),
            "\x1b[1m\u{e9}t\u{e9}\x1b[0m and \x1b[3m\u{65e5}\u{672c}\x1b[0m \x1b[7m\u{1f600}\x1b[0m\n"
        );
        assert_eq!(render_markdown("- \u{e9}l\u{e8}ve"), " • \u{e9}l\u{e8}ve\n");
    }

    #[test]
    fn markers() {
        assert_eq!(render_markdown("# Title"), "\x1b[1mTitle\x1b[0m\n");
        assert_eq!(render_markdown("a `b` c"), "a \x1b[7mb\x1b[0m c\n");
        assert_eq!(render_markdown("**b**"), "\x1b[1mb\x1b[0m\n");
        assert_eq!(render_markdown("*i*"), "\x1b[3mi\x1b[0m\n");
        // Unpaired markers stay as text.
        assert_eq!(render_markdown("a ` b"), "a ` b\n");
        assert_eq!(render_markdown("2 * 3 = 6"), "2 * 3 = 6\n");
    }
}
