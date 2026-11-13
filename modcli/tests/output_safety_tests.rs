//! Argument helpers, exports, and markdown on untrusted input.

use modcli::args;
use modcli::output::markdown::render_markdown;
use modcli::output::table::{render_table_csv, render_table_csv_safe, render_table_json};
use proptest::prelude::*;

fn argv(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn helpers_stop_at_double_dash() {
    let a = argv(&[
        "--name=x",
        "--",
        "--force",
        "--out=/etc/passwd",
        "--name",
        "y",
    ]);
    assert!(!args::flag(&a, "--force"));
    assert_eq!(args::get_string(&a, "--out"), None);
    assert_eq!(args::get_string(&a, "--name").as_deref(), Some("x"));
    assert!(args::get_bool(&a, "--force").is_err());
}

#[test]
fn last_occurrence_wins() {
    let a = argv(&["--port=1", "--port", "2", "--debug", "--debug=false"]);
    assert_eq!(args::get_int::<u16>(&a, "--port").unwrap(), 2);
    assert!(!args::flag(&a, "--debug"));
}

#[test]
fn parse_never_reads_a_value_as_an_option() {
    let a = argv(&["--message", "--out=/etc/passwd", "--", "--force"]);
    let p = args::parse(&a, &["--message", "--out"]);
    assert_eq!(p.get("--message"), Some("--out=/etc/passwd"));
    assert_eq!(p.get("--out"), None);
    assert!(!p.flag("--force"));
    assert_eq!(p.positionals(), ["--force"]);

    let b = argv(&["build", "--out", "--force", "-v", "x"]);
    let p = args::parse(&b, &["--out"]);
    assert_eq!(p.get("--out"), Some("--force"));
    assert!(!p.flag("--force"));
    assert_eq!(p.positionals(), ["build", "-v", "x"]);
}

#[test]
fn get_int_error_escapes_the_value() {
    let a = argv(&["--port", "1\x1b]52;c;eA==\x07"]);
    let msg = args::get_int::<u16>(&a, "--port").unwrap_err().to_string();
    assert!(!msg.contains('\x1b') && !msg.contains('\x07'), "{msg:?}");
}

#[test]
fn json_export_escapes_backslash_and_controls() {
    let out = render_table_json(
        &["a", "b\"q"],
        &[vec!["x\\", "y"], vec!["l1\nl2\r\t\u{1}", "z"]],
    );
    assert_eq!(
        out,
        r#"[{"a":"x\\","b\"q":"y"},{"a":"l1\nl2\r\t\u0001","b\"q":"z"}]"#
    );
}

#[test]
fn csv_quotes_carriage_return() {
    let out = render_table_csv(&["a"], &[vec!["x\ry"]]);
    assert_eq!(out, "a\n\"x\ry\"\n");
}

#[test]
fn csv_safe_neutralises_formulas() {
    let out = render_table_csv_safe(
        &["h"],
        &[
            vec!["=HYPERLINK(\"http://x\",\"y\")"],
            vec!["+1"],
            vec!["-1"],
            vec!["@SUM(A1)"],
            vec!["plain"],
        ],
    );
    assert_eq!(
        out,
        "h\n\"'=HYPERLINK(\"\"http://x\"\",\"\"y\"\")\"\n'+1\n'-1\n'@SUM(A1)\nplain\n"
    );
    // The plain export leaves cells alone.
    assert_eq!(render_table_csv(&["h"], &[vec!["+1"]]), "h\n+1\n");
}

proptest! {
    #![proptest_config(ProptestConfig {
        // Miri runs a few cases only; it is far slower.
        cases: if cfg!(miri) { 4 } else { 2000 },
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    /// Any Unicode text renders without a panic.
    #[test]
    fn markdown_never_panics(s in any::<String>()) {
        let _ = render_markdown(&s);
    }

    /// Markers mixed with any Unicode text render without a panic.
    #[test]
    fn markdown_with_markers_never_panics(
        parts in proptest::collection::vec(
            prop_oneof![
                any::<String>(),
                Just("*".to_string()),
                Just("**".to_string()),
                Just("`".to_string()),
                Just("# ".to_string()),
                Just("- ".to_string()),
                Just("\n".to_string()),
            ],
            0..24,
        )
    ) {
        let _ = render_markdown(&parts.concat());
    }

    /// Text without markers comes back unchanged, one line at a time.
    #[test]
    fn markdown_keeps_plain_text(s in any::<String>()) {
        let plain: String = s
            .chars()
            .filter(|c| !matches!(c, '*' | '`' | '#' | '-' | '\r'))
            .collect();
        let mut expected = String::new();
        for line in plain.lines() {
            expected.push_str(line);
            expected.push('\n');
        }
        prop_assert_eq!(render_markdown(&plain), expected);
    }
}
