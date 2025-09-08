use modcli::input::{buttons, raw_buttons};
use modcli::output::print;
use modcli::output::themes::{apply_theme, Theme};

fn main() {
    print::line("Buttons demo:");

    // Simple stdin hotkeys row
    let idx = buttons("Proceed?", [("Yes", 'y'), ("No", 'n'), ("Cancel", 'c')])
        .default(0)
        .get();
    print::line(&format!("stdin buttons => index {idx}"));

    // Raw-mode, themed
    apply_theme("blue");
    if let Some(i) = raw_buttons(
        "Raw buttons (Left/Right, Enter, hotkeys, Esc)",
        [
            ("Build", 'b'),
            ("Test", 't'),
            ("Deploy", 'd'),
            ("Cancel", 'c'),
        ],
    )
    .cursor(0)
    .get()
    {
        print::line(&format!("raw buttons => index {i}"));
    } else {
        print::line("raw buttons => canceled");
    }
    Theme::reset();
}
