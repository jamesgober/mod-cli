use modcli::input::{buttons, multi_select, select};
use modcli::output::input::interactive_menu;
use modcli::output::print; // built-in raw-mode demo

fn main() {
    print::line("Menu demo:");

    // 1) Single-select (stdin numeric)
    let idx = loop {
        match select("Choose one:", ["Alpha", "Beta", "Gamma"])
            .initial(2)
            .get()
        {
            Ok(i) => break i,
            Err(e) => {
                print::line(&format!("Invalid selection: {e}. Try again."));
                continue;
            }
        }
    };
    print::line(&format!("Single-select => index {idx}"));

    // 2) Multi-select (stdin comma list)
    let picks = multi_select("Choose many:", ["Red", "Green", "Blue", "Yellow"])
        .get()
        .unwrap_or_default();
    print::line(&format!("Multi-select => {picks:?}"));

    // 3) Buttons (hotkeys)
    let which = buttons("Proceed?", [("Yes", 'y'), ("No", 'n'), ("Cancel", 'c')])
        .default(0)
        .get();
    print::line(&format!("Buttons => {which}"));

    // 4) Raw-mode interactive arrow menu (built-in demo list)
    print::line("\nRaw-mode interactive menu (arrow keys, Enter, Esc):");
    if let Some(i) = interactive_menu() {
        let labels = ["🍕 Pizza", "🍔 Burger", "🌮 Taco", "❌ Exit"];
        print::line(&format!("Picked: {} (#{i})", labels[i]));
    } else {
        print::line("Menu canceled");
    }
}
