use crate::output::{hook, print};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub fn interactive_menu() -> Option<usize> {
    let mut stdout = stdout();
    let options = ["🍕 Pizza", "🍔 Burger", "🌮 Taco", "❌ Exit"];
    let mut selected = 0;

    if let Err(e) = terminal::enable_raw_mode() {
        hook::error(&format!("failed to enable raw mode: {e}"));
        return None;
    }
    if let Err(e) = execute!(stdout, terminal::Clear(ClearType::All)) {
        hook::warn(&format!("failed to clear terminal: {e}"));
    }

    loop {
        if let Err(e) = execute!(stdout, cursor::MoveTo(0, 0)) {
            hook::warn(&format!("failed to move cursor: {e}"));
        }

        println!("\nPick your poison:\n");
        for (i, option) in options.iter().enumerate() {
            if i == selected {
                println!("  > {option}"); // Highlighted
            } else {
                println!("    {option}"); // Normal
            }
        }

        if let Err(e) = stdout.flush() {
            hook::warn(&format!("flush failed: {e}"));
        }
        match event::read() {
            Ok(Event::Key(key_event)) => match key_event.code {
                KeyCode::Up => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Err(e) = terminal::disable_raw_mode() {
                        hook::warn(&format!("disable raw mode failed: {e}"));
                    }
                    return Some(selected);
                }
                KeyCode::Esc => {
                    if let Err(e) = terminal::disable_raw_mode() {
                        hook::warn(&format!("disable raw mode failed: {e}"));
                    }
                    return None;
                }
                _ => {}
            },
            Ok(_) => {}
            Err(e) => {
                print::warn(&format!("event read failed: {e}"));
            }
        }
    }
}
