use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub fn interactive_menu() -> Option<usize> {
    let mut stdout = stdout();
    let options = vec!["🍕 Pizza", "🍔 Burger", "🌮 Taco", "❌ Exit"];
    let mut selected = 0;

    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All)).unwrap();

    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        println!("\nPick your poison:\n");
        for (i, option) in options.iter().enumerate() {
            if i == selected {
                println!("  > {}", option); // Highlighted
            } else {
                println!("    {}", option); // Normal
            }
        }

        stdout.flush().unwrap();

        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().unwrap();
                    return Some(selected);
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    return None;
                }
                _ => {}
            }
        }
    }
}
