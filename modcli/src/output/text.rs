//! Helpers for printing text that came from the user or another program.
//!
//! A command name, an argument, or a file name can hold terminal control
//! sequences. Printed as they are, those can clear the screen, move the
//! cursor, set the window title, or write to the clipboard. [`sanitize`]
//! makes such text safe to print by showing every control character as a
//! visible escape instead.

use std::borrow::Cow;
use std::fmt::Write as _;

/// Returns `true` for the characters [`sanitize`] escapes: C0 controls other
/// than newline and tab, DEL, and C1 controls.
#[inline]
fn is_unsafe(c: char) -> bool {
    matches!(c, '\u{0}'..='\u{8}' | '\u{b}'..='\u{1f}' | '\u{7f}'..='\u{9f}')
}

/// Makes text safe to print to a terminal.
///
/// Every C0 control character except newline and tab, DEL, and every C1
/// control character is replaced with a visible escape: `\x1b` for ESC,
/// `\u{9b}` for a C1 control. Everything else, including all printable
/// Unicode, is left alone. Text with nothing to escape is returned borrowed.
///
/// mod-cli applies this to user input it echoes on its own behalf, such as an
/// unknown command name. Use it for anything else you print that you did not
/// write yourself.
///
/// # Example
/// ```
/// use modcli::output::text::sanitize;
/// assert_eq!(sanitize("plain"), "plain");
/// assert_eq!(sanitize("a\x1b[2Jb"), "a\\x1b[2Jb");
/// ```
pub fn sanitize(s: &str) -> Cow<'_, str> {
    if !s.chars().any(is_unsafe) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        if is_unsafe(c) {
            let n = c as u32;
            if n < 0x80 {
                let _ = write!(out, "\\x{n:02x}");
            } else {
                let _ = write!(out, "\\u{{{n:x}}}");
            }
        } else {
            out.push(c);
        }
    }
    Cow::Owned(out)
}

/// Removes every control character (all of C0 including newline and tab, DEL,
/// and C1). Used where an escape would change the meaning, such as inside a
/// terminal hyperlink.
pub(crate) fn strip_controls(s: &str) -> Cow<'_, str> {
    if !s.chars().any(char::is_control) {
        return Cow::Borrowed(s);
    }
    Cow::Owned(s.chars().filter(|c| !c.is_control()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_plain_text_borrowed() {
        assert!(matches!(
            sanitize("caf\u{e9} \u{2014} ok\n\t"),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn escapes_c0_del_and_c1() {
        assert_eq!(sanitize("\x1b]52;c;x\x07"), "\\x1b]52;c;x\\x07");
        assert_eq!(sanitize("a\x7fb"), "a\\x7fb");
        assert_eq!(sanitize("a\u{9b}2J"), "a\\u{9b}2J");
        assert_eq!(sanitize("line\nnext\ttab"), "line\nnext\ttab");
    }

    #[test]
    fn strip_controls_drops_everything_control() {
        assert_eq!(
            strip_controls("https://a\x07\x1b]52;c;x\x07"),
            "https://a]52;c;x"
        );
        assert_eq!(strip_controls("a\nb\u{9b}"), "ab");
    }
}
