/// Parses a list of pre-split arguments into `(command, args)`.
/// Provided for compatibility when arguments are already split by the caller.
pub fn parse_args(args: &[String]) -> (String, Vec<String>) {
    if args.is_empty() {
        return (String::new(), Vec::new());
    }
    let cmd = args[0].clone();
    let rest = args[1..].to_vec();
    (cmd, rest)
}

/// Parse a single command line into `(command, args)` with shell-like rules:
/// - Whitespace separates tokens
/// - Double quotes ("...") and single quotes ('...') preserve whitespace within
/// - Supports escaping of quotes and spaces with backslash (e.g., \" or \ )
/// - Mixed quoting is supported; escapes are processed within quoted segments
/// - Empty or whitespace-only input returns ("", vec![])
///
/// # Examples
///
/// Basic splitting:
/// ```
/// use modcli::parser::parse_line;
/// let (cmd, args) = parse_line("hello world there");
/// assert_eq!(cmd, "hello");
/// assert_eq!(args, vec!["world", "there"]);
/// ```
///
/// Quoted segments preserved:
/// ```
/// use modcli::parser::parse_line;
/// let (cmd, args) = parse_line("say \"hello world\" 'and universe'");
/// assert_eq!(cmd, "say");
/// assert_eq!(args, vec!["hello world", "and universe"]);
/// ```
///
/// Escaped spaces and quotes:
/// ```
/// use modcli::parser::parse_line;
/// let (cmd, args) = parse_line("run path\\ with\\ spaces \"quote\"");
/// assert_eq!(cmd, "run");
/// assert_eq!(args, vec!["path with spaces", "quote"]);
/// ```
pub fn parse_line(input: &str) -> (String, Vec<String>) {
    let tokens = tokenize(input);
    parse_args_slice(&tokens)
}

#[inline(always)]
fn parse_args_slice(tokens: &[String]) -> (String, Vec<String>) {
    if tokens.is_empty() {
        return (String::new(), Vec::new());
    }
    (tokens[0].clone(), tokens[1..].to_vec())
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::with_capacity(8);
    let mut cur = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                // Backslash handling differs by context
                if in_single {
                    // Inside single quotes, treat backslash as escaping the next char (including quotes)
                    if let Some(&next) = chars.peek() {
                        cur.push(next);
                        chars.next();
                    } else {
                        cur.push('\\');
                    }
                } else if in_double {
                    if let Some(&next) = chars.peek() {
                        if next == '"' {
                            // escape double quote inside double quotes
                            cur.push('"');
                            chars.next();
                        } else {
                            // keep literal backslash, unless escaping whitespace or backslash
                            if next.is_whitespace() {
                                cur.push(next);
                                chars.next();
                            } else if next == '\\' {
                                cur.push('\\');
                                chars.next();
                            } else {
                                cur.push('\\');
                            }
                        }
                    } else {
                        cur.push('\\');
                    }
                } else {
                    // outside quotes: support escapes
                    if let Some(&next) = chars.peek() {
                        if next.is_whitespace() {
                            // escaped space becomes literal space in current token
                            cur.push(next);
                            chars.next();
                        } else if next == '\\' {
                            // escaped backslash becomes single backslash
                            cur.push('\\');
                            chars.next();
                        } else if next == '"' {
                            // treat \" outside as starting a double-quoted segment (do not include quote)
                            in_double = true;
                            chars.next();
                        } else if next == '\'' {
                            // treat \' outside as starting a single-quoted segment (do not include quote)
                            in_single = true;
                            chars.next();
                        } else {
                            // keep backslash literally for other cases
                            cur.push('\\');
                        }
                    } else {
                        cur.push('\\');
                    }
                }
            }
            '"' if !in_single => {
                if in_double {
                    // inside double quotes: allow doubled quotes as literal
                    if let Some('"') = chars.peek().copied() {
                        cur.push('"');
                        chars.next();
                    } else {
                        // closing
                        in_double = false;
                        // if empty quoted segment and next is whitespace/end, push empty arg
                        if cur.is_empty() {
                            match chars.peek().copied() {
                                Some(c) if c.is_whitespace() => tokens.push(String::new()),
                                None => tokens.push(String::new()),
                                _ => {}
                            }
                        }
                    }
                } else {
                    // If inside a running token, treat a double-quote as literal
                    if !cur.is_empty() {
                        cur.push('"');
                    } else {
                        // opening quoted segment
                        in_double = true;
                    }
                }
            }
            '\'' if !in_double => {
                if in_single {
                    in_single = false;
                    if cur.is_empty() {
                        match chars.peek().copied() {
                            Some(c) if c.is_whitespace() => tokens.push(String::new()),
                            None => tokens.push(String::new()),
                            _ => {}
                        }
                    }
                } else {
                    in_single = true;
                }
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }

    if !cur.is_empty() {
        tokens.push(cur);
    }

    tokens
}
