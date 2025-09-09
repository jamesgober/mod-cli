//! Lightweight validation helpers for common CLI patterns.

use crate::args;
use crate::error::ModCliError;
use std::path::Path;
use std::str::FromStr;

/// Ensure a key is present: `--key` (flag) or `--key <val>` / `--key=<val>`.
pub fn require(argsv: &[String], name: &str) -> Result<(), ModCliError> {
    if args::flag(argsv, name) || args::get_string(argsv, name).is_some() {
        Ok(())
    } else {
        Err(ModCliError::InvalidUsage(format!(
            "missing required argument: {name}"
        )))
    }
}

/// Parse a typed value from `--key`, ensuring it falls in an optional inclusive range.
pub fn parse_in_range<T>(
    argsv: &[String],
    name: &str,
    min: Option<T>,
    max: Option<T>,
) -> Result<T, ModCliError>
where
    T: FromStr + PartialOrd + Copy + std::fmt::Display,
{
    let val: T = args::get_int(argsv, name)?; // uses FromStr
    if let Some(lo) = min {
        if val < lo {
            return Err(ModCliError::InvalidUsage(format!(
                "{name} below minimum ({val} < {lo})"
            )));
        }
    }
    if let Some(hi) = max {
        if val > hi {
            return Err(ModCliError::InvalidUsage(format!(
                "{name} above maximum ({val} > {hi})"
            )));
        }
    }
    Ok(val)
}

/// Validate that a path exists (file or dir).
pub fn path_exists(p: &str) -> Result<(), ModCliError> {
    if Path::new(p).exists() {
        Ok(())
    } else {
        Err(ModCliError::InvalidUsage(format!(
            "path does not exist: {p}"
        )))
    }
}

/// Validate that a path is a file.
pub fn path_is_file(p: &str) -> Result<(), ModCliError> {
    let path = Path::new(p);
    if path.is_file() {
        Ok(())
    } else {
        Err(ModCliError::InvalidUsage(format!("not a file: {p}")))
    }
}

/// Validate that a path is a directory.
pub fn path_is_dir(p: &str) -> Result<(), ModCliError> {
    let path = Path::new(p);
    if path.is_dir() {
        Ok(())
    } else {
        Err(ModCliError::InvalidUsage(format!("not a directory: {p}")))
    }
}
