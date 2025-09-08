use modcli::input::{raw_multi_select_paged, raw_select_paged};
use modcli::output::print;
use modcli::output::themes::{apply_theme, Theme};

fn main() {
    // Optionally apply a theme so selection colors pop
    apply_theme("blue");

    print::line("Paged raw-mode menu demo (with search):");

    // Single-select paged
    let single_items = (1..=50).map(|n| format!("Item {n:02}")).collect::<Vec<_>>();
    match raw_select_paged("Pick one (type to search):", single_items)
        .initial(5)
        .page_size(10)
        .get()
    {
        Some(i) => print::line(&format!("Selected index: {i}")),
        None => print::line("Canceled"),
    }

    // Multi-select paged
    let colors = [
        "Red", "Green", "Blue", "Yellow", "Cyan", "Magenta", "Black", "White", "Gold", "Silver",
        "Orange", "Teal", "Purple", "Pink", "Brown", "Grey", "Navy", "Olive",
    ];
    match raw_multi_select_paged("Pick many (space toggles):", colors)
        .page_size(8)
        .get()
    {
        Some(picks) => print::line(&format!("Picked: {picks:?}")),
        None => print::line("Canceled"),
    }

    // Reset terminal colors
    Theme::reset();
}
