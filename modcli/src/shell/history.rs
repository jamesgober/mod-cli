//! Shell history utilities (persistence and search).
//!
//! Each program should keep its own history: use [`default_path`] with the
//! program's name, for example `~/.config/myapp/history`, and pass it to
//! [`load`], [`save`], and [`add`]. Passing `None` uses the older shared path
//! `~/.config/modcli/history`, which every program built on mod-cli would
//! read and write.
//!
//! History often holds whatever the user typed, tokens included, so the
//! helpers keep it private:
//! - On Unix, new files are created with mode `0600` and new directories with
//!   mode `0700`.
//! - [`save`] and [`add`] refuse to write through a symbolic link.
//! - [`save`] writes a temporary file and renames it over the old one.
//! - There is no fallback to the current directory. When no home directory is
//!   known, [`save`] and [`add`] return an error and [`load`] returns nothing.

use crate::error::ModCliError;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// The history file for the program `app`: `~/.config/<app>/history`, or
/// `%USERPROFILE%\AppData\Roaming\<app>\history` when `HOME` is not set.
///
/// Returns `None` when neither `HOME` nor `USERPROFILE` is set, or when `app`
/// is empty, `.` or `..`, or holds a path separator.
pub fn default_path(app: &str) -> Option<PathBuf> {
    path_from(
        app,
        std::env::var_os("HOME"),
        std::env::var_os("USERPROFILE"),
    )
}

fn path_from(app: &str, home: Option<OsString>, profile: Option<OsString>) -> Option<PathBuf> {
    let valid = !app.is_empty() && app != "." && app != ".." && !app.contains(['/', '\\']);
    if !valid {
        return None;
    }
    if let Some(home) = home.filter(|h| !h.is_empty()) {
        return Some(Path::new(&home).join(".config").join(app).join("history"));
    }
    if let Some(profile) = profile.filter(|p| !p.is_empty()) {
        return Some(
            Path::new(&profile)
                .join("AppData")
                .join("Roaming")
                .join(app)
                .join("history"),
        );
    }
    None
}

/// Resolve the shared default history path.
///
/// This path is the same for every program built on mod-cli. When no home
/// directory is known it is `.modcli_history` in the current directory, which
/// [`save`] and [`add`] no longer write to.
#[deprecated(
    since = "0.7.0",
    note = "shared by every mod-cli program; use `history::default_path(\"your-app\")`"
)]
pub fn default_history_path() -> PathBuf {
    default_path("modcli").unwrap_or_else(|| PathBuf::from(".modcli_history"))
}

fn resolve(path: Option<&Path>) -> io::Result<PathBuf> {
    match path {
        Some(p) => Ok(p.to_path_buf()),
        None => default_path("modcli").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "no home directory for the history file; pass a path",
            )
        }),
    }
}

/// Load history lines from a file, ignoring IO errors (returns empty on failure).
pub fn load(path: Option<&Path>) -> Vec<String> {
    let Ok(p) = resolve(path) else {
        return Vec::new();
    };
    let file = match File::open(&p) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}

/// Save all history lines to the target file, creating directories if needed.
///
/// The lines go to a new temporary file next to the target, which is then
/// renamed over it. Refuses a target that is a symbolic link.
pub fn save(path: Option<&Path>, entries: &[String]) -> Result<(), ModCliError> {
    let p = resolve(path)?;
    refuse_symlink(&p)?;
    let dir = prepare_dir(&p)?;
    let name = p.file_name().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "history path has no file name")
    })?;
    let mut tmp_name = OsString::from(".");
    tmp_name.push(name);
    tmp_name.push(format!(".tmp-{}", std::process::id()));
    let tmp = dir.join(tmp_name);

    let result = (|| {
        let mut f = private_options().write(true).create_new(true).open(&tmp)?;
        for e in entries {
            writeln!(f, "{e}")?;
        }
        f.sync_all()?;
        drop(f);
        fs::rename(&tmp, &p)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result?;
    Ok(())
}

/// Append a single entry to history, creating files/dirs if necessary.
/// Refuses a target that is a symbolic link.
pub fn add(path: Option<&Path>, line: &str) -> Result<(), ModCliError> {
    let p = resolve(path)?;
    refuse_symlink(&p)?;
    prepare_dir(&p)?;
    let mut f = private_options().create(true).append(true).open(&p)?;
    writeln!(f, "{line}")?;
    Ok(())
}

fn refuse_symlink(p: &Path) -> io::Result<()> {
    match fs::symlink_metadata(p) {
        Ok(meta) if meta.file_type().is_symlink() => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "history file is a symbolic link",
        )),
        _ => Ok(()),
    }
}

/// Creates the parent directory (mode 0700 on Unix) and returns it.
fn prepare_dir(p: &Path) -> io::Result<PathBuf> {
    let dir = match p.parent() {
        Some(d) if !d.as_os_str().is_empty() => d.to_path_buf(),
        _ => PathBuf::from("."),
    };
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&dir)?;
    Ok(dir)
}

/// Open options that create files readable only by the owner on Unix.
fn private_options() -> OpenOptions {
    #[allow(unused_mut)]
    let mut options = OpenOptions::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
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

#[cfg(test)]
mod tests {
    use super::*;

    fn os(s: &str) -> Option<OsString> {
        Some(OsString::from(s))
    }

    #[test]
    fn path_is_per_program() {
        let p = path_from("myapp", os("/home/u"), None).unwrap();
        assert_eq!(
            p,
            Path::new("/home/u")
                .join(".config")
                .join("myapp")
                .join("history")
        );
        let w = path_from("myapp", None, os("C:/Users/u")).unwrap();
        assert!(w.ends_with(Path::new("myapp").join("history")));
    }

    #[test]
    fn no_home_means_no_path() {
        assert_eq!(path_from("myapp", None, None), None);
        assert_eq!(path_from("myapp", os(""), os("")), None);
    }

    #[test]
    fn bad_app_names_are_refused() {
        for app in ["", ".", "..", "a/b", "a\\b", "../x"] {
            assert_eq!(path_from(app, os("/home/u"), None), None, "{app}");
        }
    }

    fn scratch(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("modcli-history-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    #[cfg_attr(miri, ignore)] // touches the file system
    fn save_then_add_then_load() {
        let dir = scratch("roundtrip");
        let file = dir.join("history");
        save(Some(&file), &["one".into(), "two".into()]).unwrap();
        add(Some(&file), "three").unwrap();
        assert_eq!(load(Some(&file)), ["one", "two", "three"]);
        save(Some(&file), &["only".into()]).unwrap();
        assert_eq!(load(Some(&file)), ["only"]);
        // No temporary file is left behind.
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    #[cfg_attr(miri, ignore)] // touches the file system
    fn files_are_private() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("mode");
        let saved = dir.join("saved");
        let added = dir.join("added");
        save(Some(&saved), &["x".into()]).unwrap();
        add(Some(&added), "x").unwrap();
        for f in [&saved, &added] {
            let mode = fs::metadata(f).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode & 0o077, 0, "{f:?} has mode {mode:o}");
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    #[cfg_attr(miri, ignore)] // touches the file system
    fn symlinked_target_is_refused() {
        let dir = scratch("symlink");
        fs::create_dir_all(&dir).unwrap();
        let victim = dir.join("victim");
        fs::write(&victim, "keep me\n").unwrap();
        let link = dir.join("history");
        std::os::unix::fs::symlink(&victim, &link).unwrap();
        assert!(save(Some(&link), &["x".into()]).is_err());
        assert!(add(Some(&link), "x").is_err());
        assert_eq!(fs::read_to_string(&victim).unwrap(), "keep me\n");
        let _ = fs::remove_dir_all(&dir);
    }
}
