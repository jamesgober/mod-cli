use modcli::input::{form, FormValue};
use modcli::output::print;

fn main() {
    print::line("Form demo:");

    let values = form()
        .text("Username", |t| t.required().min_len(3).max_len(16))
        .text("Password", |t| t.required().min_len(8).mask('•'))
        .number("Thread count", |n| n.default(8.0).min(1.0).max(64.0))
        .confirm("Enable cache?", true)
        .run()
        .expect("form failed");

    print::line("\nForm result:");
    for (label, value) in values {
        match value {
            FormValue::Text(s) => {
                if label.to_lowercase().contains("password") {
                    print::line(&format!("{label} = {}", "*".repeat(s.len().min(12))));
                } else {
                    print::line(&format!("{label} = {s}"));
                }
            }
            FormValue::Number(n) => print::line(&format!("{label} = {n}")),
            FormValue::Confirm(b) => print::line(&format!("{label} = {b}")),
        }
    }
}
