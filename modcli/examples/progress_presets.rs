// Requires: --features progress-presets
use modcli::output::print;
use modcli::output::progress::{ProgressBar, ProgressStyle};

fn style_compact() -> ProgressStyle {
    ProgressStyle::default()
}
fn style_heavy() -> ProgressStyle {
    ProgressStyle::default()
}

fn main() {
    print::line("Progress presets demo (enable with --features progress-presets):");

    // Compact-like
    let mut bar1 = ProgressBar::new(20, style_compact());
    bar1.set_label("Compact (or default)");
    for _ in 0..20 {
        bar1.tick();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    println!("");

    // Heavy-like
    let mut bar2 = ProgressBar::new(25, style_heavy());
    bar2.set_label("Heavy (or default)");
    for _ in 0..25 {
        bar2.tick();
        std::thread::sleep(std::time::Duration::from_millis(24));
    }
    println!("");
}
