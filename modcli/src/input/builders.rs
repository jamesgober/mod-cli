use crate::output::hook;
use crate::output::themes::current_theme;
use crossterm::style::{Color, Stylize};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};

pub struct TextInput<'a> {
    label: &'a str,
    default: Option<String>,
    required: bool,
    min_len: Option<usize>,
    max_len: Option<usize>,
    validator: Option<Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>>,
    mask: Option<char>,
}

impl<'a> TextInput<'a> {
    pub fn default(mut self, v: impl Into<String>) -> Self {
        self.default = Some(v.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn min_len(mut self, n: usize) -> Self {
        self.min_len = Some(n);
        self
    }
    pub fn max_len(mut self, n: usize) -> Self {
        self.max_len = Some(n);
        self
    }
    pub fn validate<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + Send + Sync + 'static,
    {
        self.validator = Some(Box::new(f));
        self
    }
    pub fn mask(mut self, ch: char) -> Self {
        self.mask = Some(ch);
        self
    }

    pub fn get(self) -> Result<String, String> {
        loop {
            let hint = self
                .default
                .as_deref()
                .map(|d| format!(" [{d}]"))
                .unwrap_or_default();
            print!("{}{} ", self.label, hint);
            if let Err(e) = stdout().flush() {
                hook::warn(&format!("flush failed: {e}"));
            }

            let input = if self.mask.is_some() {
                // Basic masked input without backspace handling for now
                let mut s = String::new();
                if let Err(e) = stdin().read_line(&mut s) {
                    hook::error(&format!("failed to read: {e}"));
                    return Err("read error".into());
                }
                s
            } else {
                let mut s = String::new();
                if let Err(e) = stdin().read_line(&mut s) {
                    hook::error(&format!("failed to read: {e}"));
                    return Err("read error".into());
                }
                s
            };
            let trimmed = input.trim();
            let value = if trimmed.is_empty() {
                self.default.as_deref().unwrap_or("")
            } else {
                trimmed
            };

            if self.required && value.is_empty() {
                hook::warn("Value required");
                continue;
            }
            if let Some(n) = self.min_len {
                if value.chars().count() < n {
                    hook::warn(&format!("Min length {n}"));
                    continue;
                }
            }
            if let Some(n) = self.max_len {
                if value.chars().count() > n {
                    hook::warn(&format!("Max length {n}"));
                    continue;
                }
            }
            if let Some(v) = &self.validator {
                if let Err(msg) = v(value) {
                    hook::warn(&msg);
                    continue;
                }
            }

            return Ok(value.to_string());
        }
    }
}

pub fn text(label: &str) -> TextInput {
    TextInput {
        label,
        default: None,
        required: false,
        min_len: None,
        max_len: None,
        validator: None,
        mask: None,
    }
}

pub struct NumberInput<'a> {
    label: &'a str,
    default: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    validator: Option<Box<dyn Fn(f64) -> Result<(), String> + Send + Sync>>,
}

impl<'a> NumberInput<'a> {
    pub fn default(mut self, v: f64) -> Self {
        self.default = Some(v);
        self
    }
    pub fn min(mut self, v: f64) -> Self {
        self.min = Some(v);
        self
    }
    pub fn max(mut self, v: f64) -> Self {
        self.max = Some(v);
        self
    }
    pub fn step(mut self, v: f64) -> Self {
        self.step = v;
        self
    }
    pub fn validate<F>(mut self, f: F) -> Self
    where
        F: Fn(f64) -> Result<(), String> + Send + Sync + 'static,
    {
        self.validator = Some(Box::new(f));
        self
    }

    pub fn get(self) -> Result<f64, String> {
        loop {
            let hint = self
                .default
                .map(|d| format!(" [{}]", d))
                .unwrap_or_default();
            print!("{}{} ", self.label, hint);
            if let Err(e) = stdout().flush() {
                hook::warn(&format!("flush failed: {e}"));
            }

            let mut s = String::new();
            if let Err(e) = stdin().read_line(&mut s) {
                hook::error(&format!("failed to read: {e}"));
                return Err("read error".into());
            }
            let trimmed = s.trim();
            let value = if trimmed.is_empty() {
                self.default.unwrap_or(0.0)
            } else {
                match trimmed.parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => {
                        hook::warn("Enter a valid number");
                        continue;
                    }
                }
            };

            if let Some(min) = self.min {
                if value < min {
                    hook::warn(&format!("Min {min}"));
                    continue;
                }
            }
            if let Some(max) = self.max {
                if value > max {
                    hook::warn(&format!("Max {max}"));
                    continue;
                }
            }
            if let Some(v) = &self.validator {
                if let Err(msg) = v(value) {
                    hook::warn(&msg);
                    continue;
                }
            }
            return Ok(value);
        }
    }
}

pub fn number(label: &str) -> NumberInput {
    NumberInput {
        label,
        default: None,
        min: None,
        max: None,
        step: 1.0,
        validator: None,
    }
}

pub struct ConfirmInput<'a> {
    label: &'a str,
    default_yes: bool,
}
impl<'a> ConfirmInput<'a> {
    pub fn default_yes(mut self) -> Self {
        self.default_yes = true;
        self
    }
    pub fn default_no(mut self) -> Self {
        self.default_yes = false;
        self
    }
    pub fn get(self) -> bool {
        let hint = if self.default_yes { "[Y/n]" } else { "[y/N]" };
        print!("{} {} ", self.label, hint);
        if let Err(e) = stdout().flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        let mut s = String::new();
        if let Err(e) = stdin().read_line(&mut s) {
            hook::warn(&format!("failed to read: {e}"));
            return self.default_yes;
        }
        match s.trim().to_lowercase().as_str() {
            "y" | "yes" => true,
            "n" | "no" => false,
            "" => self.default_yes,
            _ => self.default_yes,
        }
    }
}

pub fn confirm(label: &str) -> ConfirmInput {
    ConfirmInput {
        label,
        default_yes: true,
    }
}

// --- Menus & Buttons (simple stdin-based) ---

pub struct SelectInput<'a> {
    label: &'a str,
    items: Vec<String>,
    initial: Option<usize>,
}

impl<'a> SelectInput<'a> {
    pub fn initial(mut self, idx: usize) -> Self {
        self.initial = Some(idx);
        self
    }
    pub fn get(self) -> Result<usize, String> {
        println!("{}", self.label);
        for (i, it) in self.items.iter().enumerate() {
            println!("  {}. {}", i + 1, it);
        }
        print!(
            "Enter choice [1-{}]{}: ",
            self.items.len(),
            self.initial
                .map(|i| format!(" (default {})", i + 1))
                .unwrap_or_default()
        );
        if let Err(e) = stdout().flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        let mut s = String::new();
        if let Err(e) = stdin().read_line(&mut s) {
            return Err(format!("read error: {e}"));
        }
        let trimmed = s.trim();
        if trimmed.is_empty() {
            if let Some(i) = self.initial {
                return Ok(i);
            }
        }
        match trimmed.parse::<usize>() {
            Ok(n) if n >= 1 && n <= self.items.len() => Ok(n - 1),
            _ => Err("invalid selection".into()),
        }
    }
}

pub fn select(label: &str, items: impl IntoIterator<Item = impl Into<String>>) -> SelectInput {
    SelectInput {
        label,
        items: items.into_iter().map(Into::into).collect(),
        initial: None,
    }
}

pub struct MultiSelectInput<'a> {
    label: &'a str,
    items: Vec<String>,
}

impl<'a> MultiSelectInput<'a> {
    pub fn get(self) -> Result<Vec<usize>, String> {
        println!("{}", self.label);
        for (i, it) in self.items.iter().enumerate() {
            println!("  {}. {}", i + 1, it);
        }
        println!("Enter comma-separated indexes (e.g., 1,3,4) or empty for none:");
        print!("> ");
        if let Err(e) = stdout().flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        let mut s = String::new();
        if let Err(e) = stdin().read_line(&mut s) {
            return Err(format!("read error: {e}"));
        }
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for part in trimmed.split(',') {
            let t = part.trim();
            if let Ok(n) = t.parse::<usize>() {
                if n >= 1 && n <= self.items.len() {
                    out.push(n - 1);
                }
            }
        }
        Ok(out)
    }
}

pub fn multi_select(
    label: &str,
    items: impl IntoIterator<Item = impl Into<String>>,
) -> MultiSelectInput {
    MultiSelectInput {
        label,
        items: items.into_iter().map(Into::into).collect(),
    }
}

pub struct ButtonsInput<'a> {
    buttons: Vec<(String, char)>,
    default: Option<usize>,
    label: &'a str,
}

impl<'a> ButtonsInput<'a> {
    pub fn default(mut self, idx: usize) -> Self {
        self.default = Some(idx);
        self
    }
    pub fn get(self) -> usize {
        println!("{}", self.label);
        println!(
            "{}",
            self.buttons
                .iter()
                .map(|(t, k)| format!("[{}] {}", k, t))
                .collect::<Vec<_>>()
                .join("  ")
        );
        let default_hint = self
            .default
            .and_then(|i| {
                self.buttons
                    .get(i)
                    .map(|(_, k)| format!(" (default {})", k))
            })
            .unwrap_or_default();
        print!("Choose by hotkey{}: ", default_hint);
        if let Err(e) = stdout().flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        let mut s = String::new();
        if let Err(e) = stdin().read_line(&mut s) {
            hook::warn(&format!("read error: {e}"));
            return self.default.unwrap_or(0);
        }
        let ch = s.trim().chars().next();
        if let Some(c) = ch {
            if let Some((idx, _)) = self.buttons.iter().enumerate().find(|(_, (_, k))| *k == c) {
                return idx;
            }
        }
        self.default.unwrap_or(0)
    }
}

pub fn buttons(
    label: &str,
    buttons: impl IntoIterator<Item = (impl Into<String>, char)>,
) -> ButtonsInput {
    ButtonsInput {
        label,
        buttons: buttons.into_iter().map(|(t, k)| (t.into(), k)).collect(),
        default: None,
    }
}

// --- Raw-mode interactive select ---

/// Customizable key bindings for raw-mode inputs.
#[derive(Clone)]
pub struct KeyMap {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub page_up: KeyCode,
    pub page_down: KeyCode,
    pub home: KeyCode,
    pub end: KeyCode,
    pub confirm: KeyCode,
    pub cancel: KeyCode,
    pub backspace: KeyCode,
    pub toggle_char: char, // for multi-select
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
            page_up: KeyCode::PageUp,
            page_down: KeyCode::PageDown,
            home: KeyCode::Home,
            end: KeyCode::End,
            confirm: KeyCode::Enter,
            cancel: KeyCode::Esc,
            backspace: KeyCode::Backspace,
            toggle_char: ' ',
        }
    }
}

pub struct RawSelectInput<'a> {
    label: &'a str,
    items: Vec<String>,
    initial: usize,
    keymap: KeyMap,
}

impl<'a> RawSelectInput<'a> {
    pub fn initial(mut self, idx: usize) -> Self {
        self.initial = idx.min(self.items.len().saturating_sub(1));
        self
    }
    pub fn keymap(mut self, km: KeyMap) -> Self {
        self.keymap = km;
        self
    }
    pub fn get(self) -> Option<usize> {
        let mut stdout = stdout();
        let mut selected = self.initial;
        if terminal::enable_raw_mode().is_err() {
            return None;
        }
        let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
        loop {
            let _ = execute!(stdout, cursor::MoveTo(0, 0));
            println!("{}\n", self.label);
            for (i, it) in self.items.iter().enumerate() {
                if i == selected {
                    println!("  > {}", it);
                } else {
                    println!("    {}", it);
                }
            }
            let _ = stdout.flush();
            if let Ok(Event::Key(k)) = event::read() {
                match k.code {
                    c if c == self.keymap.up => {
                        selected = selected.saturating_sub(1);
                    }
                    c if c == self.keymap.down => {
                        if selected + 1 < self.items.len() {
                            selected += 1;
                        }
                    }
                    c if c == self.keymap.confirm => {
                        let _ = terminal::disable_raw_mode();
                        return Some(selected);
                    }
                    c if c == self.keymap.cancel => {
                        let _ = terminal::disable_raw_mode();
                        return None;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn raw_select(
    label: &str,
    items: impl IntoIterator<Item = impl Into<String>>,
) -> RawSelectInput {
    RawSelectInput {
        label,
        items: items.into_iter().map(Into::into).collect(),
        initial: 0,
        keymap: KeyMap::default(),
    }
}

pub struct RawMultiSelectInput<'a> {
    label: &'a str,
    items: Vec<String>,
    initial: usize,
    keymap: KeyMap,
}

impl<'a> RawMultiSelectInput<'a> {
    pub fn initial(mut self, idx: usize) -> Self {
        self.initial = idx.min(self.items.len().saturating_sub(1));
        self
    }
    pub fn keymap(mut self, km: KeyMap) -> Self {
        self.keymap = km;
        self
    }
    pub fn get(self) -> Option<Vec<usize>> {
        let mut stdout = stdout();
        let mut cursor_idx = self.initial;
        let mut picked: Vec<bool> = vec![false; self.items.len()];
        if terminal::enable_raw_mode().is_err() {
            return None;
        }
        let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
        loop {
            let _ = execute!(stdout, cursor::MoveTo(0, 0));
            println!(
                "{}\n(space=toggle, enter=confirm, esc=cancel)\n",
                self.label
            );
            for (i, it) in self.items.iter().enumerate() {
                let mark = if picked[i] { "[x]" } else { "[ ]" };
                if i == cursor_idx {
                    println!("  > {} {}", mark, it);
                } else {
                    println!("    {} {}", mark, it);
                }
            }
            let _ = stdout.flush();
            if let Ok(Event::Key(k)) = event::read() {
                match k.code {
                    c if c == self.keymap.up => {
                        cursor_idx = cursor_idx.saturating_sub(1);
                    }
                    c if c == self.keymap.down => {
                        if cursor_idx + 1 < self.items.len() {
                            cursor_idx += 1;
                        }
                    }
                    KeyCode::Char(c) if c == self.keymap.toggle_char => {
                        if let Some(p) = picked.get_mut(cursor_idx) {
                            *p = !*p;
                        }
                    }
                    c if c == self.keymap.confirm => {
                        let _ = terminal::disable_raw_mode();
                        let res: Vec<usize> = picked
                            .iter()
                            .enumerate()
                            .filter_map(|(i, b)| if *b { Some(i) } else { None })
                            .collect();
                        return Some(res);
                    }
                    c if c == self.keymap.cancel => {
                        let _ = terminal::disable_raw_mode();
                        return None;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn raw_multi_select(
    label: &str,
    items: impl IntoIterator<Item = impl Into<String>>,
) -> RawMultiSelectInput {
    RawMultiSelectInput {
        label,
        items: items.into_iter().map(Into::into).collect(),
        initial: 0,
        keymap: KeyMap::default(),
    }
}

// --- Paged raw-mode builders ---

pub struct RawPagedSelectInput<'a> {
    label: &'a str,
    items: Vec<String>,
    cursor: usize,
    page_size: usize,
    keymap: KeyMap,
}

impl<'a> RawPagedSelectInput<'a> {
    pub fn initial(mut self, idx: usize) -> Self {
        self.cursor = idx.min(self.items.len().saturating_sub(1));
        self
    }
    pub fn page_size(mut self, n: usize) -> Self {
        self.page_size = n.max(1);
        self
    }
    pub fn keymap(mut self, km: KeyMap) -> Self {
        self.keymap = km;
        self
    }
    pub fn get(mut self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        let mut stdout = stdout();
        if terminal::enable_raw_mode().is_err() {
            return None;
        }
        let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
        let mut query = String::new();
        loop {
            // Filter
            let ql = query.to_lowercase();
            let filtered: Vec<usize> = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, s)| s.to_lowercase().contains(&ql))
                .map(|(i, _)| i)
                .collect();
            if filtered.is_empty() {
                self.cursor = 0;
            } else if self.cursor >= filtered.len() {
                self.cursor = filtered.len() - 1;
            }
            let total_pages = (filtered.len() + self.page_size - 1) / self.page_size;
            let page = if filtered.is_empty() {
                0
            } else {
                self.cursor / self.page_size
            };
            let start = page * self.page_size;
            let end = (start + self.page_size).min(filtered.len());

            let _ = execute!(stdout, cursor::MoveTo(0, 0));
            println!("{}\n", self.label);
            println!("Search: {}\n", query.as_str().with(Color::DarkGrey));
            println!(
                "Page {}/{}\n",
                if total_pages == 0 { 0 } else { page + 1 },
                total_pages
            );
            let theme = current_theme();
            let sel_bg = theme.get_log_color("menu_selected_bg");
            let sel_fg = theme.get_log_color("menu_selected_fg");
            let stripe_fg = theme.get_log_color("menu_stripe_fg");
            for (row, fi) in (start..end).enumerate().map(|(row, i)| (row, filtered[i])) {
                let label = &self.items[fi];
                if (start + row) == self.cursor {
                    let line = format!("  > {}", label).with(sel_fg).on(sel_bg).bold();
                    println!("{}", line);
                } else if row % 2 == 1 {
                    println!("{}", format!("    {}", label).with(stripe_fg));
                } else {
                    println!("    {}", label);
                }
            }
            println!(
                "\nKeys: ↑/↓ PgUp/PgDn Home/End Enter Esc  (type to search, Backspace clears)"
            );
            let _ = stdout.flush();

            if let Ok(Event::Key(k)) = event::read() {
                match k.code {
                    c if c == self.keymap.up => {
                        self.cursor = self.cursor.saturating_sub(1);
                    }
                    c if c == self.keymap.down => {
                        if !filtered.is_empty() && self.cursor + 1 < filtered.len() {
                            self.cursor += 1;
                        }
                    }
                    c if c == self.keymap.page_up => {
                        if self.cursor >= self.page_size {
                            self.cursor -= self.page_size;
                        } else {
                            self.cursor = 0;
                        }
                    }
                    c if c == self.keymap.page_down => {
                        if !filtered.is_empty() && self.cursor + self.page_size < filtered.len() {
                            self.cursor += self.page_size;
                        } else {
                            self.cursor = filtered.len().saturating_sub(1);
                        }
                    }
                    c if c == self.keymap.home => {
                        self.cursor = 0;
                    }
                    c if c == self.keymap.end => {
                        self.cursor = filtered.len().saturating_sub(1);
                    }
                    c if c == self.keymap.confirm => {
                        let _ = terminal::disable_raw_mode();
                        return filtered.get(self.cursor).copied();
                    }
                    c if c == self.keymap.cancel => {
                        let _ = terminal::disable_raw_mode();
                        return None;
                    }
                    c if c == self.keymap.backspace => {
                        query.pop();
                    }
                    KeyCode::Char(c) => {
                        query.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn raw_select_paged(
    label: &str,
    items: impl IntoIterator<Item = impl Into<String>>,
) -> RawPagedSelectInput {
    RawPagedSelectInput {
        label,
        items: items.into_iter().map(Into::into).collect(),
        cursor: 0,
        page_size: 10,
        keymap: KeyMap::default(),
    }
}

pub struct RawPagedMultiSelectInput<'a> {
    label: &'a str,
    items: Vec<String>,
    cursor: usize,
    page_size: usize,
    picked: Vec<bool>,
    keymap: KeyMap,
}

impl<'a> RawPagedMultiSelectInput<'a> {
    pub fn initial(mut self, idx: usize) -> Self {
        self.cursor = idx.min(self.items.len().saturating_sub(1));
        self
    }
    pub fn page_size(mut self, n: usize) -> Self {
        self.page_size = n.max(1);
        self
    }
    pub fn keymap(mut self, km: KeyMap) -> Self {
        self.keymap = km;
        self
    }
    pub fn get(mut self) -> Option<Vec<usize>> {
        if self.items.is_empty() {
            return Some(Vec::new());
        }
        let mut stdout = stdout();
        if terminal::enable_raw_mode().is_err() {
            return None;
        }
        let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
        let mut query = String::new();
        loop {
            // Filter
            let ql = query.to_lowercase();
            let filtered: Vec<usize> = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, s)| s.to_lowercase().contains(&ql))
                .map(|(i, _)| i)
                .collect();
            if filtered.is_empty() {
                self.cursor = 0;
            } else if self.cursor >= filtered.len() {
                self.cursor = filtered.len() - 1;
            }
            let total_pages = (filtered.len() + self.page_size - 1) / self.page_size;
            let page = if filtered.is_empty() {
                0
            } else {
                self.cursor / self.page_size
            };
            let start = page * self.page_size;
            let end = (start + self.page_size).min(filtered.len());

            let _ = execute!(stdout, cursor::MoveTo(0, 0));
            println!("{}\n(space=toggle)\n", self.label);
            println!("Search: {}\n", query.as_str().with(Color::DarkGrey));
            println!(
                "Page {}/{}\n",
                if total_pages == 0 { 0 } else { page + 1 },
                total_pages
            );
            let theme = current_theme();
            let sel_bg = theme.get_log_color("menu_selected_bg");
            let sel_fg = theme.get_log_color("menu_selected_fg");
            let stripe_fg = theme.get_log_color("menu_stripe_fg");
            for (row, fi) in (start..end).enumerate().map(|(row, i)| (row, filtered[i])) {
                let mark = if *self.picked.get(fi).unwrap_or(&false) {
                    "[x]"
                } else {
                    "[ ]"
                };
                if (start + row) == self.cursor {
                    let line = format!("  > {} {}", mark, self.items[fi])
                        .with(sel_fg)
                        .on(sel_bg)
                        .bold();
                    println!("{}", line);
                } else if row % 2 == 1 {
                    println!(
                        "{}",
                        format!("    {} {}", mark, self.items[fi]).with(stripe_fg)
                    );
                } else {
                    println!("    {} {}", mark, self.items[fi]);
                }
            }
            println!("\nKeys: ↑/↓ PgUp/PgDn Home/End Space Enter Esc  (type to search, Backspace clears)");
            let _ = stdout.flush();

            if let Ok(Event::Key(k)) = event::read() {
                match k.code {
                    c if c == self.keymap.up => {
                        self.cursor = self.cursor.saturating_sub(1);
                    }
                    c if c == self.keymap.down => {
                        if !filtered.is_empty() && self.cursor + 1 < filtered.len() {
                            self.cursor += 1;
                        }
                    }
                    c if c == self.keymap.page_up => {
                        if self.cursor >= self.page_size {
                            self.cursor -= self.page_size;
                        } else {
                            self.cursor = 0;
                        }
                    }
                    c if c == self.keymap.page_down => {
                        if !filtered.is_empty() && self.cursor + self.page_size < filtered.len() {
                            self.cursor += 1;
                        } else {
                            self.cursor = filtered.len().saturating_sub(1);
                        }
                    }
                    c if c == self.keymap.home => {
                        self.cursor = 0;
                    }
                    c if c == self.keymap.end => {
                        self.cursor = filtered.len().saturating_sub(1);
                    }
                    KeyCode::Char(c1) if c1 == self.keymap.toggle_char => {
                        if let Some(fi) = filtered.get(self.cursor) {
                            if let Some(p) = self.picked.get_mut(*fi) {
                                *p = !*p;
                            }
                        }
                    }
                    c if c == self.keymap.confirm => {
                        let _ = terminal::disable_raw_mode();
                        let res: Vec<usize> = self
                            .picked
                            .iter()
                            .enumerate()
                            .filter_map(|(i, b)| if *b { Some(i) } else { None })
                            .collect();
                        return Some(res);
                    }
                    c if c == self.keymap.cancel => {
                        let _ = terminal::disable_raw_mode();
                        return None;
                    }
                    c if c == self.keymap.backspace => {
                        query.pop();
                    }
                    KeyCode::Char(c) => {
                        query.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn raw_multi_select_paged(
    label: &str,
    items: impl IntoIterator<Item = impl Into<String>>,
) -> RawPagedMultiSelectInput {
    let v: Vec<String> = items.into_iter().map(Into::into).collect();
    let picked = vec![false; v.len()];
    RawPagedMultiSelectInput {
        label,
        items: v,
        cursor: 0,
        page_size: 10,
        picked,
        keymap: KeyMap::default(),
    }
}

// --- Form builder (sequential prompts using existing inputs) ---

pub enum FormValue {
    Text(String),
    Number(f64),
    Confirm(bool),
}

enum FormItem {
    Text {
        label: String,
        cfg: Box<dyn Fn(TextInput) -> TextInput>,
    },
    Number {
        label: String,
        cfg: Box<dyn Fn(NumberInput) -> NumberInput>,
    },
    Confirm {
        label: String,
        default_yes: bool,
    },
}

pub struct FormBuilder {
    items: Vec<FormItem>,
}

impl FormBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn text(mut self, label: &str, cfg: impl Fn(TextInput) -> TextInput + 'static) -> Self {
        self.items.push(FormItem::Text {
            label: label.to_string(),
            cfg: Box::new(cfg),
        });
        self
    }

    pub fn number(
        mut self,
        label: &str,
        cfg: impl Fn(NumberInput) -> NumberInput + 'static,
    ) -> Self {
        self.items.push(FormItem::Number {
            label: label.to_string(),
            cfg: Box::new(cfg),
        });
        self
    }

    pub fn confirm(mut self, label: &str, default_yes: bool) -> Self {
        self.items.push(FormItem::Confirm {
            label: label.to_string(),
            default_yes,
        });
        self
    }

    /// Run the form sequentially; returns the collected values paired with their labels.
    pub fn run(self) -> Result<Vec<(String, FormValue)>, String> {
        let mut out = Vec::with_capacity(self.items.len());
        for item in self.items {
            match item {
                FormItem::Text { label, cfg } => {
                    let v = cfg(text(&label)).get()?;
                    out.push((label, FormValue::Text(v)));
                }
                FormItem::Number { label, cfg } => {
                    let v = cfg(number(&label)).get()?;
                    out.push((label, FormValue::Number(v)));
                }
                FormItem::Confirm { label, default_yes } => {
                    let v = if default_yes {
                        confirm(&label).default_yes().get()
                    } else {
                        confirm(&label).default_no().get()
                    };
                    out.push((label, FormValue::Confirm(v)));
                }
            }
        }
        Ok(out)
    }
}

pub fn form() -> FormBuilder {
    FormBuilder::new()
}

// --- Raw-mode buttons row (left/right + hotkeys) ---

pub struct RawButtonsInput<'a> {
    label: &'a str,
    buttons: Vec<(String, char)>,
    cursor: usize,
    disabled: HashSet<usize>,
    helps: Vec<Option<String>>, // one per button (optional)
    danger: HashSet<usize>,     // destructive actions
    confirm_on_danger: bool,
    keymap: KeyMap,
}

impl<'a> RawButtonsInput<'a> {
    pub fn cursor(mut self, idx: usize) -> Self {
        self.cursor = idx.min(self.buttons.len().saturating_sub(1));
        self
    }
    pub fn disabled(mut self, idxs: &[usize]) -> Self {
        for &i in idxs {
            self.disabled.insert(i);
        }
        self
    }
    pub fn helps<I, S>(mut self, helps: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Option<String>>,
    {
        self.helps = helps.into_iter().map(|s| s.into()).collect();
        self
    }
    pub fn danger(mut self, idxs: &[usize]) -> Self {
        for &i in idxs {
            self.danger.insert(i);
        }
        self
    }
    pub fn confirm_on_danger(mut self, yes: bool) -> Self {
        self.confirm_on_danger = yes;
        self
    }
    pub fn keymap(mut self, km: KeyMap) -> Self {
        self.keymap = km;
        self
    }
    pub fn get(mut self) -> Option<usize> {
        if self.buttons.is_empty() {
            return None;
        }
        let mut stdout = stdout();
        if terminal::enable_raw_mode().is_err() {
            return None;
        }
        let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
        loop {
            let theme = current_theme();
            let sel_bg = theme.get_log_color("menu_selected_bg");
            let sel_fg = theme.get_log_color("menu_selected_fg");
            let _ = execute!(stdout, cursor::MoveTo(0, 0));
            println!("{}\n", self.label);
            // Render buttons as a row
            let mut line = String::new();
            for (i, (title, key)) in self.buttons.iter().enumerate() {
                let token = format!("[{}] {}", key, title);
                let is_disabled = self.disabled.contains(&i);
                let is_danger = self.danger.contains(&i);
                let styled = if i == self.cursor {
                    // Selected state
                    if is_disabled {
                        token.as_str().with(Color::DarkGrey).on(sel_bg).to_string()
                    } else if is_danger {
                        token
                            .as_str()
                            .with(sel_fg)
                            .on(Color::Red)
                            .bold()
                            .to_string()
                    } else {
                        token.as_str().with(sel_fg).on(sel_bg).bold().to_string()
                    }
                } else if is_disabled {
                    token.as_str().with(Color::DarkGrey).to_string()
                } else if is_danger {
                    token.as_str().with(Color::Red).to_string()
                } else {
                    token.clone()
                };
                line.push_str(&format!(" {}", styled));
            }
            println!("{}\n", line);
            // Tooltip/help under the row if provided
            if let Some(Some(help)) = self.helps.get(self.cursor) {
                println!("{}\n", help);
            }
            println!(
                "Keys: Left/Right to move, Enter select, hotkeys {}, Esc cancel",
                self.buttons.iter().map(|(_, k)| k).collect::<String>()
            );
            let _ = stdout.flush();

            if let Ok(Event::Key(k)) = event::read() {
                match k.code {
                    c if c == self.keymap.left => {
                        // Move left skipping disabled if possible
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                        while self.disabled.contains(&self.cursor) && self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    c if c == self.keymap.right => {
                        if self.cursor + 1 < self.buttons.len() {
                            self.cursor += 1;
                        }
                        while self.cursor + 1 < self.buttons.len()
                            && self.disabled.contains(&self.cursor)
                        {
                            self.cursor += 1;
                        }
                    }
                    c if c == self.keymap.confirm => {
                        if self.disabled.contains(&self.cursor) {
                            continue;
                        }
                        // Danger confirmation if enabled
                        if self.confirm_on_danger && self.danger.contains(&self.cursor) {
                            let _ = terminal::disable_raw_mode();
                            println!("Confirm '{}'? [y/N] ", self.buttons[self.cursor].0);
                            use std::io::stdin;
                            let mut buf = String::new();
                            let _ = stdin().read_line(&mut buf);
                            if !matches!(buf.trim().to_lowercase().as_str(), "y" | "yes") {
                                let _ = terminal::enable_raw_mode();
                                continue;
                            }
                            return Some(self.cursor);
                        } else {
                            let _ = terminal::disable_raw_mode();
                            return Some(self.cursor);
                        }
                    }
                    c if c == self.keymap.cancel => {
                        let _ = terminal::disable_raw_mode();
                        return None;
                    }
                    KeyCode::Char(c) => {
                        if let Some((idx, _)) = self
                            .buttons
                            .iter()
                            .enumerate()
                            .find(|(i, (_, k))| *k == c && !self.disabled.contains(i))
                        {
                            if self.confirm_on_danger && self.danger.contains(&idx) {
                                let _ = terminal::disable_raw_mode();
                                println!("Confirm '{}'? [y/N] ", self.buttons[idx].0);
                                use std::io::stdin;
                                let mut buf = String::new();
                                let _ = stdin().read_line(&mut buf);
                                if !matches!(buf.trim().to_lowercase().as_str(), "y" | "yes") {
                                    let _ = terminal::enable_raw_mode();
                                    continue;
                                }
                                return Some(idx);
                            }
                            let _ = terminal::disable_raw_mode();
                            return Some(idx);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn raw_buttons(
    label: &str,
    buttons: impl IntoIterator<Item = (impl Into<String>, char)>,
) -> RawButtonsInput {
    RawButtonsInput {
        label,
        buttons: buttons.into_iter().map(|(t, k)| (t.into(), k)).collect(),
        cursor: 0,
        disabled: HashSet::new(),
        helps: Vec::new(),
        danger: HashSet::new(),
        confirm_on_danger: true,
        keymap: KeyMap::default(),
    }
}
