use modcli::output::print;
use modcli::output::table::{render_table_with, Align, TableMode, TableStyle, TruncateMode};

fn main() {
    let headers = ["Name", "Score", "Notes"];
    let rows = vec![
        vec!["Alice", "98", "Top performer — consistent results"],
        vec!["Bob", "7", "Needs improvement on long-form tasks"],
        vec![
            "Carol",
            "1234",
            "Extremely long numeric score to show truncation",
        ],
    ];

    // Per-column alignment: Name=Left, Score=Right, Notes=Center
    let aligns = [Align::Left, Align::Right, Align::Center];
    // Truncation: Name=End, Score=Start, Notes=Middle
    let truncs = [TruncateMode::End, TruncateMode::Start, TruncateMode::Middle];

    // Use Fixed width small to visibly demonstrate truncation and alignment
    let s = render_table_with(
        &headers,
        &rows,
        TableMode::Fixed(14),
        TableStyle::Rounded,
        Some(&aligns),
        Some(&truncs),
    );

    print::line("Alignment + Truncation Demo:");
    print::line(&s);
}
