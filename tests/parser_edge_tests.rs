use modcli::parser::parse_line;

#[test]
fn parse_multiple_whitespace_and_tabs() {
    let (cmd, args) = parse_line("   cmd\t\targ1   arg2\t \targ3   ");
    assert_eq!(cmd, "cmd");
    assert_eq!(args, vec!["arg1", "arg2", "arg3"]);
}

#[test]
fn parse_empty_quoted_segments() {
    let (cmd, args) = parse_line("run '' \"\"");
    assert_eq!(cmd, "run");
    assert_eq!(args, vec!["", ""]);
}

#[test]
fn parse_escaped_quotes_inside_quotes() {
    let (cmd, args) = parse_line("say \"he\"\"llo\" 'it\'s fine'");
    assert_eq!(cmd, "say");
    assert_eq!(args, vec!["he\"llo", "it's fine"]);
}

#[test]
fn parse_escaped_backslash_and_space() {
    let (cmd, args) = parse_line("cp foo\\bar path\\ with\\ space");
    assert_eq!(cmd, "cp");
    assert_eq!(args, vec!["foo\\bar", "path with space"]);
}

#[test]
fn parse_double_quotes_containing_single_quotes() {
    let (cmd, args) = parse_line("say \"a 'b' c\"");
    assert_eq!(cmd, "say");
    assert_eq!(args, vec!["a 'b' c"]);
}

#[test]
fn parse_single_quotes_containing_double_quotes() {
    let (cmd, args) = parse_line("say 'a \"b\" c'");
    assert_eq!(cmd, "say");
    assert_eq!(args, vec!["a \"b\" c"]);
}
