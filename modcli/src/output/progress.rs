use crate::output::hook;
use crossterm::style::{Color, Stylize};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

/// Customizable style for the progress bar
#[derive(Clone)]
pub struct ProgressStyle {
    pub fill: char,
    pub start_cap: char,
    pub end_cap: char,
    pub done_label: &'static str,
    pub show_percent: bool,
    pub color: Option<Color>,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        Self {
            fill: '#',
            start_cap: '[',
            end_cap: ']',
            done_label: "Done!",
            show_percent: true,
            color: None,
        }
    }
}

/// Struct-based progress bar
pub struct ProgressBar {
    pub total_steps: usize,
    pub current: usize,
    pub label: Option<String>,
    pub style: ProgressStyle,
}

impl ProgressBar {
    pub fn new(total_steps: usize, style: ProgressStyle) -> Self {
        Self {
            total_steps,
            current: 0,
            label: None,
            style,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn set_progress(&mut self, value: usize) {
        self.current = value.min(self.total_steps);
        self.render();
    }

    pub fn tick(&mut self) {
        self.current += 1;
        if self.current > self.total_steps {
            self.current = self.total_steps;
        }
        self.render();
    }

    pub fn start_auto(&mut self, duration_ms: u64) {
        let interval = duration_ms / self.total_steps.max(1) as u64;
        for _ in 0..self.total_steps {
            self.tick();
            thread::sleep(Duration::from_millis(interval));
        }
        println!(" {}", self.style.done_label);
    }

    fn render(&self) {
        let percent = if self.style.show_percent {
            format!(" {:>3}%", self.current * 100 / self.total_steps.max(1))
        } else {
            "".to_string()
        };

        let fill_count = self.current;
        let empty_count = self.total_steps - self.current;

        let mut bar = format!(
            "{}{}{}{}",
            self.style.start_cap,
            self.style.fill.to_string().repeat(fill_count),
            " ".repeat(empty_count),
            self.style.end_cap
        );

        if let Some(color) = self.style.color {
            bar = bar.with(color).to_string();
        }
        print!("\r");

        if let Some(ref label) = self.label {
            print!("{label} {bar}");
        } else {
            print!("{bar}");
        }

        print!("{percent}");
        if let Err(e) = stdout().flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
    }
}

// Procedural-style one-liners

pub fn show_progress_bar(label: &str, total_steps: usize, duration_ms: u64) {
    let mut bar = ProgressBar::new(total_steps, ProgressStyle::default());
    bar.set_label(label);
    bar.start_auto(duration_ms);
}

pub fn show_percent_progress(label: &str, percent: usize) {
    let clamped = percent.clamp(0, 100);
    print!("\r{label}: {clamped:>3}% complete");
    if let Err(e) = stdout().flush() {
        hook::warn(&format!("flush failed: {e}"));
    }
}

pub fn show_spinner(label: &str, cycles: usize, delay_ms: u64) {
    let spinner = ['|', '/', '-', '\\'];
    let mut stdout = stdout();
    print!("{label} ");

    for i in 0..cycles {
        let frame = spinner[i % spinner.len()];
        print!("\r{label} {frame}");
        if let Err(e) = stdout.flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        thread::sleep(Duration::from_millis(delay_ms));
    }

    println!("{label} ✓");
}
