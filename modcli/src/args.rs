//! Lightweight argument helper functions for common patterns.
//!
//! Supports forms like:
//! --flag
//! --flag=true|false
//! --key value
//! --key=value
//!
//! Every helper stops at `--`: tokens after it are positional and never read
//! as options. When an option is given more than once, the last one wins.
//!
//! The single-option helpers ([`flag`], [`get_string`], [`get_int`],
//! [`get_bool`]) only know about the option they are asked for. They cannot
//! tell that another option's value, such as the text after `--message`, is a
//! value and not an option. When values can come from someone other than the
//! person typing the command (a script, a file, an issue title), use
//! [`parse`] and list every option that takes a value:
//!
//! ```
//! use modcli::args;
//! let argv: Vec<String> = ["--message", "--out=/etc/passwd", "--", "--force"]
//!     .iter().map(|s| s.to_string()).collect();
//! let parsed = args::parse(&argv, &["--message", "--out"]);
//! assert_eq!(parsed.get("--message"), Some("--out=/etc/passwd"));
//! assert_eq!(parsed.get("--out"), None);
//! assert!(!parsed.flag("--force"));
//! assert_eq!(parsed.positionals(), ["--force"]);
//! ```

use crate::error::ModCliError;
use crate::output::text::sanitize;

/// Arguments split into options and positionals by [`parse`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Parsed<'a> {
    /// `(name without the leading --, value)` in command-line order.
    options: Vec<(&'a str, Option<&'a str>)>,
    positionals: Vec<&'a str>,
}

impl<'a> Parsed<'a> {
    fn last(&self, name: &str) -> Option<Option<&'a str>> {
        let key = name.strip_prefix("--")?;
        self.options
            .iter()
            .rev()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
    }

    /// `true` if `--name` is present on its own or set to a true value
    /// (`1`, `true`, `yes`, `y`, any case). The last occurrence wins.
    pub fn flag(&self, name: &str) -> bool {
        match self.last(name) {
            Some(None) => true,
            Some(Some(v)) => truthy(v),
            None => false,
        }
    }

    /// The value of the last `--name value` or `--name=value`.
    pub fn get(&self, name: &str) -> Option<&'a str> {
        self.last(name).flatten()
    }

    /// Tokens that are not options or option values, including every token
    /// after `--`.
    pub fn positionals(&self) -> &[&'a str] {
        &self.positionals
    }
}

/// Splits `args` into options and positionals in one pass.
///
/// `value_options` lists the options that take a value (for example
/// `&["--out", "--message"]`): for those, `--key value` takes the next token
/// as the value whatever it looks like, and that token is never read as an
/// option. `--key=value` works for any option. Parsing stops at `--`; every
/// later token is positional.
pub fn parse<'a>(args: &'a [String], value_options: &[&str]) -> Parsed<'a> {
    let mut parsed = Parsed::default();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--" {
            parsed
                .positionals
                .extend(args[i + 1..].iter().map(String::as_str));
            break;
        }
        match a.strip_prefix("--").filter(|body| !body.is_empty()) {
            Some(body) => {
                if let Some((k, v)) = body.split_once('=') {
                    parsed.options.push((k, Some(v)));
                } else if value_options.contains(&a) {
                    let value = args.get(i + 1).map(String::as_str);
                    if value.is_some() {
                        i += 1;
                    }
                    parsed.options.push((body, value));
                } else {
                    parsed.options.push((body, None));
                }
            }
            None => parsed.positionals.push(a),
        }
        i += 1;
    }
    parsed
}

fn truthy(v: &str) -> bool {
    matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "y")
}

/// Return true if a boolean flag is present or set to true.
/// Matches `--flag`, `--flag=true`, `--flag=1`, `--flag=yes` (case-insensitive).
/// Stops at `--`; the last occurrence wins.
pub fn flag(args: &[String], name: &str) -> bool {
    if !name.starts_with("--") {
        return false;
    }
    parse(args, &[]).flag(name)
}

/// Get a string value for `--key` from either `--key value` or `--key=value`.
/// Stops at `--`; the last occurrence wins.
pub fn get_string(args: &[String], name: &str) -> Option<String> {
    if !name.starts_with("--") {
        return None;
    }
    parse(args, &[name]).get(name).map(str::to_string)
}

/// Get an integer value for `--key`.
/// Returns ModCliError::InvalidUsage on parse failure.
pub fn get_int<T>(args: &[String], name: &str) -> Result<T, ModCliError>
where
    T: std::str::FromStr,
{
    if let Some(s) = get_string(args, name) {
        match s.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(ModCliError::InvalidUsage(format!(
                "expected numeric value for {name}, got '{}'",
                sanitize(&s)
            ))),
        }
    } else {
        Err(ModCliError::InvalidUsage(format!(
            "missing required argument: {name}"
        )))
    }
}

/// Get a boolean value for `--key`.
/// If the key is present with no value (i.e., `--key`), returns true.
/// If provided as `--key=value`, parses truthy/falsey strings.
pub fn get_bool(args: &[String], name: &str) -> Result<bool, ModCliError> {
    if flag(args, name) {
        return Ok(true);
    }
    if let Some(s) = get_string(args, name) {
        return Ok(truthy(&s));
    }
    Err(ModCliError::InvalidUsage(format!(
        "missing required argument: {name}"
    )))
}
