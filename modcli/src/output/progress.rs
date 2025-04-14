use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;


/// Style struct for customizing progress bar (expandable)
#[derive(Clone)]
pub struct ProgressStyle {
    pub fill: char,
    pub done_label: &'static str,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        Self {
            fill: '#',
            done_label: "Done!",
        }
    }
}

/// Struct-based progress bar (more control than `show_*` functions)
pub struct ProgressBar {
    total_steps: usize,
    label: Option<String>,
    style: ProgressStyle,
}

impl ProgressBar {
    pub fn new(total_steps: usize, style: ProgressStyle) -> Self {
        Self {
            total_steps,
            label: None,
            style,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn start(&self, duration_ms: u64) {
        let interval = duration_ms / self.total_steps.max(1) as u64;
        let mut stdout = stdout();

        if let Some(ref label) = self.label {
            print!("{} [", label);
        } else {
            print!("[");
        }

        stdout.flush().unwrap();

        for _ in 0..self.total_steps {
            print!("{}", self.style.fill);
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(interval));
        }

        println!("] {}", self.style.done_label);
    }
}

/// Simple procedural variants (keep these!)
pub fn show_progress_bar(label: &str, total_steps: usize, duration_ms: u64) {
    let mut bar = ProgressBar::new(total_steps, ProgressStyle::default());
    bar.set_label(label);
    bar.start(duration_ms);
}

pub fn show_percent_progress(label: &str, percent: usize) {
    let clamped = percent.clamp(0, 100);
    print!("\r{}: {:>3}% complete", label, clamped);
    stdout().flush().unwrap();
}

pub fn show_spinner(label: &str, cycles: usize, delay_ms: u64) {
    let spinner = vec!['|', '/', '-', '\\'];
    let mut stdout = stdout();
    print!("{} ", label);

    for i in 0..cycles {
        let frame = spinner[i % spinner.len()];
        print!("\r{} {}", label, frame);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }

    println!("\r{} ✓", label);
}
