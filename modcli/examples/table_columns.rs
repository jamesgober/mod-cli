use modcli::output::print;
use modcli::output::table::{render_table_with_columns, Align, ColWidth, TableStyle, TruncateMode};

fn main() {
    let headers = ["Name", "Team", "Notes"];
    let rows = vec![
        vec![
            "Ada",
            "Core",
            "Long note to demo widths and truncation behavior",
        ],
        vec!["Linus", "Kernel", "Maintainer"],
        vec!["Grace", "Security", "Legend"],
    ];

    // Columns: Name fixed 12, Team percent 20%, Notes auto
    let cols = [ColWidth::Fixed(12), ColWidth::Percent(20), ColWidth::Auto];
    let aligns = [Align::Left, Align::Center, Align::Left];
    let truncs = [TruncateMode::End, TruncateMode::End, TruncateMode::Middle];

    let s = render_table_with_columns(
        &headers,
        &rows,
        TableStyle::Rounded,
        &cols,
        Some(&aligns),
        Some(&truncs),
        true,
        true,
    );

    print::line("Per-column widths (Fixed/Percent/Auto):");
    print::line(&s);
}
