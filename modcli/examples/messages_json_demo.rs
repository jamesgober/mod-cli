use modcli::output::messages;

fn main() {
    // Load i18n bundle (requires feature: theme-config)
    match messages::load_messages_from_json("modcli/examples/messages/en.json") {
        Ok(()) => {}
        Err(e) => eprintln!("Failed to load messages: {e}"),
    }

    // Show a header value (would be used by help)
    let header = messages::message_or_default("help.header", "Help:");
    println!("{header}");
}
