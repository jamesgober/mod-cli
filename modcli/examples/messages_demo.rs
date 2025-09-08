use crossterm::style::{Color, Stylize};
use modcli::output::{hook, messages, themes};

fn main() {
    // Optional: apply a theme so styles match
    let _guard = themes::ThemeGuard::apply("blue");

    // 1) Override a catalog key (help header)
    let header = "Commands (custom header)"
        .with(Color::Cyan)
        .bold()
        .to_string();
    messages::set_message("help.header", header);

    // 2) Intercept all 'status' messages and colorize them dim green
    messages::set_output_interceptor(|category, text| {
        if category == "status" {
            std::borrow::Cow::Owned(text.with(Color::Green).dim().to_string())
        } else {
            std::borrow::Cow::Owned(text.to_string())
        }
    });

    // 3) Demonstrate hooks flowing through interceptor
    hook::status("System warming up...");
    hook::success("Ready!");

    // 4) Show how a help header might be consumed
    let header = messages::message_or_default("help.header", "Commands");
    println!("\n{header}");
    println!("  foo  Do foo things");
    println!("  bar  Do bar things");
}
