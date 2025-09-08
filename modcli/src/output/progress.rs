use crate::output::hook;
use crossterm::style::{Color, Stylize};
use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

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

impl Default for MultiProgress {
    fn default() -> Self {
        Self::new()
    }
}

// --- MultiProgress ---

/// Minimal multi-progress manager that stacks multiple ProgressBar lines.
/// Rendering uses ANSI cursor movement; works in most modern terminals.
pub struct MultiProgress {
    bars: Vec<ProgressBar>,
}

impl MultiProgress {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }

    /// Add a bar and return its index for later updates.
    pub fn add_bar(&mut self, label: &str, total_steps: usize, style: ProgressStyle) -> usize {
        let mut bar = ProgressBar::new(total_steps, style);
        bar.set_label(label);
        if bar.start_time.is_none() {
            bar.start_time = Some(Instant::now());
        }
        self.bars.push(bar);
        self.bars.len() - 1
    }

    pub fn get_bar_mut(&mut self, idx: usize) -> Option<&mut ProgressBar> {
        self.bars.get_mut(idx)
    }

    pub fn tick(&mut self, idx: usize) {
        if let Some(b) = self.bars.get_mut(idx) {
            b.tick();
        }
    }
    pub fn set_progress(&mut self, idx: usize, value: usize) {
        if let Some(b) = self.bars.get_mut(idx) {
            b.set_progress(value);
        }
    }
    pub fn set_bytes_processed(&mut self, idx: usize, bytes: u64) {
        if let Some(b) = self.bars.get_mut(idx) {
            b.set_bytes_processed(bytes);
        }
    }

    /// Redraw all bars stacked. This moves the cursor up N lines then re-renders.
    pub fn refresh(&mut self) {
        let n = self.bars.len();
        if n == 0 {
            return;
        }
        // Move cursor up n lines (except for first draw)
        print!("\x1B[{n}A"); // ANSI: CUU n
        for b in &self.bars {
            b.render();
            println!();
        }
        let _ = stdout().flush();
    }

    /// Finish all bars and print their done labels on separate lines.
    pub fn finish(&mut self) {
        for b in &self.bars {
            // Ensure bar is fully rendered at 100%
            b.render();
            println!(" {}", b.style.done_label);
        }
        let _ = stdout().flush();
    }
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

// --- Helpers ---

fn human_bytes_per_sec(bps: f64) -> String {
    let abs = bps.abs();
    const K: f64 = 1024.0;
    let (value, unit) = if abs >= K * K * K {
        (bps / (K * K * K), "GiB/s")
    } else if abs >= K * K {
        (bps / (K * K), "MiB/s")
    } else if abs >= K {
        (bps / K, "KiB/s")
    } else {
        (bps, "B/s")
    };
    if value.abs() >= 100.0 {
        format!("{value:>4.0} {unit}")
    } else {
        format!("{value:>4.1} {unit}")
    }
}

fn human_duration(d: Duration) -> String {
    let mut secs = d.as_secs();
    let h = secs / 3600;
    secs %= 3600;
    let m = secs / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        format!("{m:02}:{s:02}")
    }
}

/// Struct-based progress bar
pub struct ProgressBar {
    pub total_steps: usize,
    pub current: usize,
    pub label: Option<String>,
    pub style: ProgressStyle,
    // Byte-based tracking (optional)
    pub total_bytes: Option<u64>,
    pub bytes_processed: u64,
    // Timing
    start_time: Option<Instant>,
    last_tick: Option<Instant>,
    // Control
    paused: bool,
}

impl ProgressBar {
    pub fn new(total_steps: usize, style: ProgressStyle) -> Self {
        Self {
            total_steps,
            current: 0,
            label: None,
            style,
            total_bytes: None,
            bytes_processed: 0,
            start_time: None,
            last_tick: None,
            paused: false,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn set_progress(&mut self, value: usize) {
        self.current = value.min(self.total_steps);
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        self.last_tick = Some(Instant::now());
        self.render();
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }
        self.current += 1;
        if self.current > self.total_steps {
            self.current = self.total_steps;
        }
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        self.last_tick = Some(Instant::now());
        self.render();
    }

    /// Set total bytes (enables byte-based progress). You may update processed bytes independently.
    pub fn set_bytes_total(&mut self, total: u64) {
        self.total_bytes = Some(total);
    }

    /// Set processed bytes; will render percent, rate, and ETA when total is known.
    pub fn set_bytes_processed(&mut self, processed: u64) {
        self.bytes_processed = processed;
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        self.last_tick = Some(Instant::now());
        self.render();
    }

    /// Convenience to set both total and processed bytes in one call.
    pub fn set_bytes(&mut self, total: u64, processed: u64) {
        self.total_bytes = Some(total);
        self.set_bytes_processed(processed);
    }

    /// Pause updates by tick(); direct set_ calls will still render.
    pub fn pause(&mut self) {
        self.paused = true;
    }
    /// Resume updates.
    pub fn resume(&mut self) {
        self.paused = false;
        self.last_tick = Some(Instant::now());
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
        let mut percent_val: usize = self.current * 100 / self.total_steps.max(1);

        // In bytes mode, compute percent from bytes
        if let Some(total) = self.total_bytes {
            if total > 0 {
                percent_val = ((self.bytes_processed.saturating_mul(100)) / total.max(1)) as usize;
            }
        }

        let percent = if self.style.show_percent {
            format!(" {:>3}%", percent_val.min(100))
        } else {
            String::new()
        };

        // Determine visual fill based on either steps or bytes percent
        let fill_from_percent =
            |pct: usize, width: usize| -> usize { ((pct.min(100) * width) / 100).min(width) };

        let (fill_count, empty_count) = if self.total_bytes.is_some() {
            let fill = fill_from_percent(percent_val, self.total_steps);
            (fill, self.total_steps - fill)
        } else {
            (self.current, self.total_steps - self.current)
        };

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

        // Bytes-specific tail: rate and ETA
        if let Some(total) = self.total_bytes {
            let elapsed = self.start_time.map(|t| t.elapsed()).unwrap_or_default();
            let rate_bps = if elapsed.as_secs_f64() > 0.0 {
                self.bytes_processed as f64 / elapsed.as_secs_f64()
            } else {
                0.0
            };
            let remaining = total.saturating_sub(self.bytes_processed);
            let eta_secs = if rate_bps > 0.0 {
                (remaining as f64 / rate_bps).round() as u64
            } else {
                0
            };

            let rate_str = human_bytes_per_sec(rate_bps);
            let eta_str = human_duration(Duration::from_secs(eta_secs));
            print!("{percent}  {rate_str}  ETA {eta_str}");
        } else {
            print!("{percent}");
        }

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

/// Emoji spinner demo using moon phases. Compatible with most modern terminals.
pub fn show_emoji_spinner(label: &str, cycles: usize, delay_ms: u64) {
    const FRAMES: [&str; 8] = ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];
    let mut stdout = stdout();
    print!("{label} ");

    for i in 0..cycles {
        let frame = FRAMES[i % FRAMES.len()];
        print!("\r{label} {frame}");
        if let Err(e) = stdout.flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        thread::sleep(Duration::from_millis(delay_ms));
    }

    println!("{label} ✅");
}
