use modcli::output::print;
use modcli::output::table::{render_table_with_opts, Align, TableMode, TableStyle, TruncateMode};

fn main() {
    let headers = ["ID", "User", "Comment"];
    let rows = vec![
        vec!["1", "alice", "Short"],
        vec![
            "2",
            "bob",
            "This is a long comment that will wrap or truncate",
        ],
        vec!["3", "carol", "Another one"],
    ];

    let aligns = [Align::Right, Align::Left, Align::Left];
    let truncs = [TruncateMode::End, TruncateMode::End, TruncateMode::End];

    let s = render_table_with_opts(
        &headers,
        &rows,
        TableMode::Fixed(18),
        TableStyle::Heavy,
        Some(&aligns),
        Some(&truncs),
        true, // zebra
        true, // row separators
    );

    print::line("Zebra + Row Separators:");
    print::line(&s);
}
