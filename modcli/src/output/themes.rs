use crate::output::colors::*;
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
#[cfg(feature = "theme-config")]
use serde::Deserialize;
use std::collections::HashMap;
#[cfg(feature = "theme-config")]
use std::fs;
use std::io::{stdout, Write};
#[cfg(feature = "theme-config")]
use std::path::Path;
use std::sync::OnceLock;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub fg: Color,
    pub bg: Color,
    pub log_styles: HashMap<&'static str, Color>,
}

impl Theme {
    pub fn apply(&self) {
        let _ = write!(
            stdout(),
            "{}{}",
            SetForegroundColor(self.fg),
            SetBackgroundColor(self.bg)
        );
        let _ = stdout().flush();
    }

    pub fn reset() {
        let _ = write!(stdout(), "{ResetColor}");
        let _ = stdout().flush();
    }

    pub fn get_log_color(&self, key: &str) -> Color {
        self.log_styles.get(key).copied().unwrap_or(self.fg)
    }
}

static THEME: OnceLock<Theme> = OnceLock::new();

fn log_defaults(base: Color) -> HashMap<&'static str, Color> {
    let mut map = HashMap::new();
    map.insert("error", COLOR_ERROR);
    map.insert("warn", COLOR_WARNING);
    map.insert("success", COLOR_SUCCESS);
    map.insert("debug", COLOR_DEBUG);
    map.insert("info", COLOR_INFO);
    map.insert("trace", COLOR_TRACE);
    map.insert("notice", COLOR_NOTICE);
    map.insert("status", COLOR_STATUS);
    map.insert("default", base);
    // Menu theming keys (used by raw paged builders)
    // Selected background defaults to status color, selected foreground defaults to BLACK for contrast,
    // stripe foreground defaults to DARK_GREY.
    map.insert("menu_selected_bg", COLOR_STATUS);
    map.insert("menu_selected_fg", BLACK);
    map.insert("menu_stripe_fg", DARK_GREY);
    map
}

pub fn apply_theme(name: &str) {
    let theme = match name.to_lowercase().as_str() {
        "monochrome" => Theme {
            name: "monochrome".into(),
            fg: GREY,
            bg: BLACK,
            log_styles: log_defaults(GREY),
        },
        "inverted" => Theme {
            name: "inverted".into(),
            fg: BLACK,
            bg: WHITE,
            log_styles: log_defaults(BLACK),
        },
        "blue" => Theme {
            name: "blue".into(),
            fg: WHITE,
            bg: BLUE,
            log_styles: log_defaults(WHITE),
        },
        "green" => Theme {
            name: "green".into(),
            fg: BLACK,
            bg: GREEN,
            log_styles: log_defaults(BLACK),
        },
        _ => Theme {
            name: "default".into(),
            fg: WHITE,
            bg: BLACK,
            log_styles: log_defaults(WHITE),
        },
    };

    let _ = THEME.set(theme.clone()); // only sets once
    theme.apply();
}

pub fn current_theme() -> Theme {
    THEME.get().cloned().unwrap_or_else(|| Theme {
        name: "default".into(),
        fg: WHITE,
        bg: BLACK,
        log_styles: log_defaults(WHITE),
    })
}

/// RAII guard that applies a theme and resets on drop.
/// Does not mutate global THEME; it only changes terminal colors during the guard's lifetime.
pub struct ThemeGuard {
    reset: bool,
}

impl ThemeGuard {
    pub fn apply(name: &str) -> Self {
        apply_theme(name);
        Self { reset: true }
    }

    pub fn disable_reset(mut self) -> Self {
        self.reset = false;
        self
    }
}

impl Drop for ThemeGuard {
    fn drop(&mut self) {
        if self.reset {
            Theme::reset();
        }
    }
}

#[cfg(feature = "theme-config")]
#[derive(Deserialize)]
struct ThemeFile {
    name: Option<String>,
    fg: Option<String>,
    bg: Option<String>,
    log_styles: Option<HashMap<String, String>>,
}

/// Load a theme from a JSON file (feature: theme-config). Returns a Theme you can apply.
#[cfg(feature = "theme-config")]
pub fn load_theme_from_json<P: AsRef<Path>>(path: P) -> Result<Theme, String> {
    let data = fs::read_to_string(&path).map_err(|e| format!("read failed: {e}"))?;
    let tf: ThemeFile =
        serde_json::from_str(&data).map_err(|e| format!("json parse failed: {e}"))?;

    let fg = tf.fg.as_deref().map(get).unwrap_or(WHITE);
    let bg = tf.bg.as_deref().map(get).unwrap_or(BLACK);
    let mut log = log_defaults(fg);
    if let Some(map) = tf.log_styles {
        for (k, v) in map.into_iter() {
            log.insert(Box::leak(k.into_boxed_str()), get(&v));
        }
    }
    Ok(Theme {
        name: tf.name.unwrap_or_else(|| "loaded".into()),
        fg,
        bg,
        log_styles: log,
    })
}
