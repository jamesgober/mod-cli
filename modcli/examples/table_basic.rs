use modcli::output::print;
use modcli::output::table::{render_table, TableMode, TableStyle};

fn main() {
    let headers = ["Name", "Role", "Notes"];
    let rows = vec![
        vec!["Alice", "Engineer", "Works on back-end services"],
        vec!["Bob", "Designer", "Focus on UX and visuals"],
        vec!["Carol", "PM", "Keeps the trains running on time"],
    ];

    // Flex mode (fits to content up to terminal width)
    let t1 = render_table(&headers, &rows, TableMode::Flex, TableStyle::Rounded);
    print::line("Flex / Rounded:");
    print::line(&t1);

    // Full mode (fills terminal width evenly)
    let t2 = render_table(&headers, &rows, TableMode::Full, TableStyle::Heavy);
    print::line("Full / Heavy:");
    print::line(&t2);

    // Fixed width columns
    let t3 = render_table(&headers, &rows, TableMode::Fixed(18), TableStyle::Ascii);
    print::line("Fixed(18) / Ascii:");
    print::line(&t3);
}
