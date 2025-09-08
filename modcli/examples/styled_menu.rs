use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Stylize},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();
    let title = "Styled, paginated menu (Up/Down, PgUp/PgDn, Home/End, Enter, Esc)";

    // Build a long list to demo paging
    let items: Vec<String> = (1..=50).map(|i| format!("Item {i:02}")).collect();
    let page_size: usize = 10;
    let mut cursor_idx: usize = 0; // absolute index

    terminal::enable_raw_mode().expect("raw mode");
    execute!(stdout, terminal::Clear(ClearType::All)).ok();

    loop {
        let total_pages = items.len().div_ceil(page_size);
        let page = cursor_idx / page_size;
        let start = page * page_size;
        let end = (start + page_size).min(items.len());

        // Draw
        execute!(stdout, cursor::MoveTo(0, 0)).ok();
        println!("{}\n", title.bold());
        println!("Page {}/{}\n", page + 1, total_pages);
        for (i, label) in items.iter().enumerate().take(end).skip(start) {
            if i == cursor_idx {
                // Selected row: inverted with accent color
                let line = format!("  > {label}")
                    .with(Color::Black)
                    .on(Color::Cyan)
                    .bold();
                println!("{line}");
            } else {
                // Zebra striping for readability
                if (i - start) % 2 == 0 {
                    println!("    {label}");
                } else {
                    println!("{}", format!("    {label}").with(Color::DarkGrey));
                }
            }
        }
        println!("\nUse keys: ↑/↓, PgUp/PgDn, Home/End, Enter to select, Esc to cancel.");
        stdout.flush().ok();

        // Events
        if let Ok(Event::Key(k)) = event::read() { match k.code {
                KeyCode::Up => {
                    cursor_idx = cursor_idx.saturating_sub(1);
                }
                KeyCode::Down => {
                    if cursor_idx + 1 < items.len() {
                        cursor_idx += 1;
                    }
                }
                KeyCode::PageUp => {
                    if cursor_idx >= page_size {
                        cursor_idx -= page_size;
                    } else {
                        cursor_idx = 0;
                    }
                }
                KeyCode::PageDown => {
                    if cursor_idx + page_size < items.len() {
                        cursor_idx += page_size;
                    } else {
                        cursor_idx = items.len().saturating_sub(1);
                    }
                }
                KeyCode::Home => {
                    cursor_idx = 0;
                }
                KeyCode::End => {
                    cursor_idx = items.len().saturating_sub(1);
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().ok();
                    println!(
                        "\nSelected: {} (#{})",
                        items[cursor_idx].as_str().bold(),
                        cursor_idx
                    );
                    break;
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode().ok();
                    println!("\nCanceled");
                    break;
                }
                _ => {}
            } }
    }
}
