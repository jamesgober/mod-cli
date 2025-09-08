use modcli::output::print;
use modcli::output::table::{
    render_table_with_opts_styled, Align, TableMode, TableStyle, TruncateMode,
};

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
        vec!["4", "dave", "And another"],
    ];

    let aligns = [Align::Right, Align::Left, Align::Left];
    let truncs = [TruncateMode::End, TruncateMode::End, TruncateMode::End];

    // Styled: cyan header text, dark blue zebra background on odd rows
    let s = render_table_with_opts_styled(
        &headers,
        &rows,
        TableMode::Fixed(24),
        TableStyle::Rounded,
        Some(&aligns),
        Some(&truncs),
        true, // zebra
        true, // row separators
        Some(modcli::output::CYAN),
        Some(modcli::output::DARK_BLUE),
    );

    print::line("Styled headers + color zebra:");
    print::line(&s);
}
