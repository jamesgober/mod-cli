use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Stylize},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();
    let mut username = String::new();
    let mut threads: i32 = 8;
    let mut enable_cache = true;
    let mut focus: usize = 0; // 0=username,1=threads,2=toggle,3=submit

    terminal::enable_raw_mode().expect("raw");
    execute!(stdout, terminal::Clear(ClearType::All)).ok();

    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).ok();
        println!("Raw Form UI (Tab to move, Enter to submit field, Esc to quit)\n");

        // Username
        if focus == 0 {
            println!("{} {}", "▶ Username:".bold(), username.clone());
        } else {
            println!("  Username: {username}");
        }

        // Threads
        if focus == 1 {
            println!("{} {}", "▶ Threads:".bold(), threads);
        } else {
            println!("  Threads: {threads}");
        }

        // Toggle
        let toggle_label = if enable_cache {
            "ON".with(Color::Green)
        } else {
            "OFF".with(Color::Red)
        };
        if focus == 2 {
            println!("{} {}", "▶ Enable cache:".bold(), toggle_label);
        } else {
            println!("  Enable cache: {toggle_label}");
        }

        // Submit button
        let submit = "[ Submit ]";
        if focus == 3 {
            println!("\n{}", submit.with(Color::Black).on(Color::Cyan).bold());
        } else {
            println!("\n{submit}");
        }

        println!("\nKeys: Tab/Shift+Tab move • Enter confirm • Backspace edit • +/- for threads • Esc quit");
        stdout.flush().ok();

        if let Ok(Event::Key(k)) = event::read() {
            match k.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Tab => {
                    focus = (focus + 1) % 4;
                }
                KeyCode::BackTab => {
                    focus = if focus == 0 { 3 } else { focus - 1 };
                }
                KeyCode::Enter => {
                    if focus == 3 {
                        break;
                    }
                    // noop per-field (we edit live)
                }
                KeyCode::Char(c) => {
                    match focus {
                        0 => {
                            if k.modifiers.contains(KeyModifiers::CONTROL) { /* ignore */
                            } else if c != '+' && c != '-' {
                                username.push(c);
                            }
                        }
                        1 => {
                            if c == '+' {
                                threads = (threads + 1).min(512);
                            } else if c == '-' {
                                threads = (threads - 1).max(1);
                            }
                        }
                        2 => {
                            if c == ' ' {
                                enable_cache = !enable_cache;
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Backspace => {
                    if focus == 0 {
                        username.pop();
                    }
                }
                KeyCode::Left => {
                    if focus == 1 {
                        threads = (threads - 1).max(1);
                    }
                }
                KeyCode::Right => {
                    if focus == 1 {
                        threads = (threads + 1).min(512);
                    }
                }
                // '+' and '-' handled in KeyCode::Char above
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode().ok();
    println!(
        "\nResult:\n  Username = {username}\n  Threads = {threads}\n  Enable cache = {enable_cache}"
    );
}
