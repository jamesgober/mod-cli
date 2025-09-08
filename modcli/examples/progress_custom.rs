use crossterm::style::Color;
use modcli::output::print;
use modcli::output::progress::{ProgressBar, ProgressStyle};

fn main() {
    print::line("Custom ProgressStyle with label and color:");

    let mut style = ProgressStyle::default();
    style.fill = '█';
    style.start_cap = '⟦';
    style.end_cap = '⟧';
    style.done_label = "All set!";
    style.show_percent = true;
    style.color = Some(Color::Green);

    let mut bar = ProgressBar::new(40, style);
    bar.set_label("Uploading");

    // Manually tick to demonstrate API
    for _ in 0..40 {
        bar.tick();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    println!(" \nDone.");
}
