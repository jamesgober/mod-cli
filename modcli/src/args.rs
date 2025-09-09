//! Lightweight argument helper functions for common patterns.
//!
//! Supports forms like:
//! --flag
//! --flag=true|false
//! --key value
//! --key=value

use crate::error::ModCliError;

/// Return true if a boolean flag is present or set to true.
/// Matches `--flag`, `--flag=true`, `--flag=1`, `--flag=yes` (case-insensitive).
pub fn flag(args: &[String], name: &str) -> bool {
    if !name.starts_with("--") {
        return false;
    }
    let key = name.trim_start_matches('-');
    for a in args {
        let a = a.as_str();
        if a == name {
            return true;
        }
        if let Some((k, v)) = a.strip_prefix("--").and_then(|s| s.split_once('=')) {
            if k == key {
                let v = v.to_ascii_lowercase();
                return matches!(v.as_str(), "1" | "true" | "yes" | "y");
            }
        }
    }
    false
}

/// Get a string value for `--key` from either `--key value` or `--key=value`.
pub fn get_string(args: &[String], name: &str) -> Option<String> {
    if !name.starts_with("--") {
        return None;
    }
    let key = name.trim_start_matches('-');
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a == name {
            if let Some(next) = args.get(i + 1) {
                return Some(next.clone());
            }
            return None;
        }
        if let Some((k, v)) = a.strip_prefix("--").and_then(|s| s.split_once('=')) {
            if k == key {
                return Some(v.to_string());
            }
        }
        i += 1;
    }
    None
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
                "expected numeric value for {name}, got '{s}'"
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
        let v = s.to_ascii_lowercase();
        return Ok(matches!(v.as_str(), "1" | "true" | "yes" | "y"));
    }
    Err(ModCliError::InvalidUsage(format!(
        "missing required argument: {name}"
    )))
}
