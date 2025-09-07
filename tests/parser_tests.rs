use modcli::parser::parse_line;

#[test]
fn parse_empty_and_whitespace() {
    let (cmd, args) = parse_line("");
    assert_eq!(cmd, "");
    assert!(args.is_empty());

    let (cmd, args) = parse_line("   \t   ");
    assert_eq!(cmd, "");
    assert!(args.is_empty());
}

#[test]
fn parse_simple_tokens() {
    let (cmd, args) = parse_line("hello world there");
    assert_eq!(cmd, "hello");
    assert_eq!(args, vec!["world", "there"]);
}

#[test]
fn parse_quoted_tokens() {
    let (cmd, args) = parse_line("say \"hello world\" 'and universe'");
    assert_eq!(cmd, "say");
    assert_eq!(args, vec!["hello world", "and universe"]);
}

#[test]
fn parse_escaped_spaces_and_quotes() {
    let (cmd, args) = parse_line("run path\ with\ spaces \"quote\" ");
    assert_eq!(cmd, "run");
    assert_eq!(args, vec!["path with spaces", "quote"]);
}

#[test]
fn parse_mixed_quotes_and_text() {
    let (cmd, args) = parse_line("cmd pre\"fix \"mid dle\" suf'fix' end");
    assert_eq!(cmd, "cmd");
    assert_eq!(args, vec!["pre\"fix", "mid dle", "suffix", "end"]);
}
