//! Shell history utilities (persistence and search).
//!
//! Default path: ~/.config/modcli/history (Unix/macOS)
//!
//! These helpers are optional and can be used by applications embedding mod-cli.

use crate::error::ModCliError;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Resolve a default history path (best-effort, cross-platform fallback).
pub fn default_history_path() -> PathBuf {
    // Prefer XDG on Unix/macOS
    if let Ok(home) = std::env::var("HOME") {
        let base = Path::new(&home).join(".config").join("modcli");
        return base.join("history");
    }
    // Windows USERPROFILE
    if let Ok(home) = std::env::var("USERPROFILE") {
        let base = Path::new(&home)
            .join("AppData")
            .join("Roaming")
            .join("modcli");
        return base.join("history");
    }
    // Fallback: current directory
    PathBuf::from(".modcli_history")
}

/// Load history lines from a file, ignoring IO errors (returns empty on failure).
pub fn load(path: Option<&Path>) -> Vec<String> {
    let p: PathBuf = path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(default_history_path);
    let file = match File::open(&p) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}

/// Save all history lines to the target file, creating directories if needed.
pub fn save(path: Option<&Path>, entries: &[String]) -> Result<(), ModCliError> {
    let p: PathBuf = path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(default_history_path);
    if let Some(dir) = p.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut f = File::create(&p)?;
    for e in entries {
        writeln!(f, "{e}")?;
    }
    Ok(())
}

/// Append a single entry to history, creating files/dirs if necessary.
pub fn add(path: Option<&Path>, line: &str) -> Result<(), ModCliError> {
    let p: PathBuf = path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(default_history_path);
    if let Some(dir) = p.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut f = OpenOptions::new().create(true).append(true).open(&p)?;
    writeln!(f, "{line}")?;
    Ok(())
}

/// Simple case-insensitive substring search returning up to `limit` matches (most recent last).
pub fn search(entries: &[String], query: &str, limit: usize) -> Vec<String> {
    if query.is_empty() {
        return entries
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
    }
    let q = query.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    for e in entries.iter().rev() {
        if e.to_ascii_lowercase().contains(&q) {
            out.push(e.clone());
            if out.len() >= limit {
                break;
            }
        }
    }
    out.reverse();
    out
}
