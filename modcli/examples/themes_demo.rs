use crossterm::style::{Color, Stylize};
use modcli::output::{print, themes};

fn main() {
    print::line("Themes demo:");

    // Apply a theme (changes terminal fg/bg until reset)
    themes::apply_theme("blue");
    print::line("Applied theme: blue (fg: WHITE, bg: BLUE)");

    // Show log-style colors resolved by the current theme
    let t = themes::current_theme();
    let keys = [
        "error", "warn", "success", "info", "debug", "trace", "notice", "status", "default",
    ];
    for key in keys.iter() {
        let color: Color = t.get_log_color(key);
        let line = format!("{key:>7}: colored by theme")
            .with(color)
            .to_string();
        println!("{line}");
    }

    // Switch theme mid-run
    themes::apply_theme("inverted");
    print::line("Switched theme: inverted (fg: BLACK, bg: WHITE)");

    // Reset terminal colors at the end
    themes::Theme::reset();
    println!("Reset to default colors.");
}
