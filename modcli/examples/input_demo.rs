use modcli::input::{confirm, number, text};
use modcli::input::{interactive_menu, prompt_confirm, prompt_password, prompt_text};
use modcli::output::print;

fn main() {
    print::line("Input demo:");

    // Text input
    let name = prompt_text("Your name");
    print::line(&format!("Hello, {name}!"));

    // Password (masked)
    let secret = prompt_password("Enter a secret (masked)");
    print::line(&format!("Secret length: {}", secret.len()));

    // Confirm
    let ok = prompt_confirm("Do you want to continue?");
    print::line(&format!("Continue: {ok}"));

    // Menu (single-select) - uses built-in demo options
    if let Some(idx) = interactive_menu() {
        let labels = ["🍕 Pizza", "🍔 Burger", "🌮 Taco", "❌ Exit"];
        print::line(&format!("Selected: {} (#{idx})", labels[idx]));
    } else {
        print::line("Menu canceled");
    }

    // Builder-style API
    print::line("\nBuilder-style inputs:");
    let username = text("Username").min_len(3).max_len(16).get().unwrap();
    print::line(&format!("User: {username}"));

    let age = number("Age")
        .min(1.0)
        .max(130.0)
        .default(30.0)
        .get()
        .unwrap();
    print::line(&format!("Age: {}", age as i32));

    let agree = confirm("Agree to Terms?").default_yes().get();
    print::line(&format!("Agree: {agree}"));
}
