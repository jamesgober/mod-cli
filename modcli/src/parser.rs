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
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                // escape next char if present
                if let Some(&next) = chars.peek() {
                    cur.push(next);
                    chars.next();
                } else {
                    // trailing backslash; treat as literal
                    cur.push('\\');
                }
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            '\'' if !in_double => {
                in_single = !in_single;
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
