use modcli::output::print;
use modcli::output::progress::{show_percent_progress, show_progress_bar, show_spinner};

fn main() {
    print::line("Spinner (quick demo):");
    show_spinner("Loading", 12, 50);

    println!();
    print::line("Percent updates:");
    for p in (0..=100).step_by(20) {
        show_percent_progress("Downloading", p);
        std::thread::sleep(std::time::Duration::from_millis(120));
    }
    println!();

    println!();
    print::line("Auto progress bar:");
    show_progress_bar("Processing", 30, 900);
}
