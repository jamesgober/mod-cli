use crossterm::style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::sync::OnceLock;
use crate::output::colors::*;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub fg: Color,
    pub bg: Color,
    pub log_styles: HashMap<&'static str, Color>,
}

impl Theme {
    pub fn apply(&self) {
        let _ = write!(stdout(), "{}{}", SetForegroundColor(self.fg), SetBackgroundColor(self.bg));
        let _ = stdout().flush();
    }

    pub fn reset() {
        let _ = write!(stdout(), "{}", ResetColor);
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
    map.insert("default", base);
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
    THEME
        .get()
        .cloned()
        .unwrap_or_else(|| Theme {
            name: "default".into(),
            fg: WHITE,
            bg: BLACK,
            log_styles: log_defaults(WHITE),
        })
}