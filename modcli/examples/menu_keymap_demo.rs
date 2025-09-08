use crossterm::event::KeyCode;
use modcli::input::builders::KeyMap;
use modcli::input::{raw_multi_select_paged, raw_select};

fn main() {
    // Define a vim-like keymap for navigation
    let km = KeyMap {
        up: KeyCode::Char('k'),
        down: KeyCode::Char('j'),
        left: KeyCode::Char('h'),
        right: KeyCode::Char('l'),
        page_up: KeyCode::PageUp,
        page_down: KeyCode::PageDown,
        home: KeyCode::Home,
        end: KeyCode::End,
        confirm: KeyCode::Enter,
        cancel: KeyCode::Esc,
        backspace: KeyCode::Backspace,
        toggle_char: ' ',
    };

    // Raw select with custom keymap
    if let Some(idx) = raw_select(
        "Pick one (use j/k to move, Enter to select):",
        ["Alpha", "Beta", "Gamma", "Delta"],
    )
    .keymap(km.clone())
    .get()
    {
        println!("You picked index {idx}");
    } else {
        println!("Canceled");
    }

    // Raw paged multi-select with custom keymap, using j/k and space to toggle
    let items = (1..=50).map(|i| format!("Item {i}"));
    if let Some(picks) =
        raw_multi_select_paged("Pick many (j/k nav, space toggle, Enter done):", items)
            .page_size(8)
            .keymap(km)
            .get()
    {
        println!("Picked: {picks:?}");
    } else {
        println!("Canceled");
    }
}
