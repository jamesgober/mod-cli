use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "modcli".into());
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".into());
    let about = env::var("CARGO_PKG_DESCRIPTION").unwrap_or_else(|_| "Modern CLI framework".into());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Minimal groff man page (section 1)
    println!(".TH {} 1 {} \\\"{}\\\"", name.to_uppercase(), now, version);
    println!(".SH NAME");
    println!("{name} \\− {about}");
    println!(".SH SYNOPSIS");
    println!(".B {name}");
    println!("[\\-\\-version] [\\-\\-modcli] <command> [args...]\\n");
    println!(".SH DESCRIPTION");
    println!("{name} is a high\\-performance CLI framework.");
    println!("Commands are provided by your application via a registry.");
    println!(".SH OPTIONS");
    println!(".TP");
    println!("\\fB\\-\\-version\\fR  Print framework version");
    println!(".TP");
    println!("\\fB\\-\\-modcli\\fR  Show framework info");
    println!(".SH SEE ALSO");
    println!("Project: https://github.com/jamesgober/mod-cli");
}
