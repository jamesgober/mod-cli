use modcli::input::input_builder::{prompt_text, prompt_password, confirm};

fn main() {
    println!("=== Input Builder Demo ===");

    let name = prompt_text("What's your name?", Some("Stranger"));
    println!("Hello, {}!", name);

    if confirm("Do you want to continue?", true) {
        let secret = prompt_password("Enter your password:");
        println!("Password accepted (but not shown)");
    } else {
        println!("Aborted.");
    }
}
