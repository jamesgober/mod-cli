// Requires: --features layouts
#[cfg(feature = "layouts")]
use modcli::output::layout;
use modcli::output::print;

fn main() {
    print::line("Layout demo (enable with --features layouts):");

    #[cfg(feature = "layouts")]
    {
        let lorem_left = vec![
            "Left column fixed 24 chars".to_string(),
            "- Short bullets".to_string(),
            "- Wraps gracefully when content exceeds width".to_string(),
        ];
        let lorem_mid = vec![
            "Middle column 30% width".to_string(),
            "This paragraph is intentionally a bit longer to demonstrate line wrapping within a column whose width is a percentage of the terminal width.".to_string(),
        ];
        let lorem_right = vec![
            "Auto width column".to_string(),
            "Use for overflow or dynamic content.".to_string(),
        ];

        let layout = layout::build()
            .hgap(2)
            .vgap(1)
            .border(true)
            .row()
            .col_fixed(24)
            .content(lorem_left)
            .col_percent(30)
            .content(lorem_mid)
            .col_auto()
            .content(lorem_right)
            .end_row()
            .row()
            .col_percent(50)
            .content(vec![
                "Second row, half width left".to_string(),
                "Multiple lines are aligned across columns.".to_string(),
            ])
            .col_percent(50)
            .content(vec![
                "Right half".to_string(),
                "Try resizing your terminal to see Full/Auto behaviors.".to_string(),
            ])
            .end_row()
            .finish();

        let rendered = layout::render(&layout);
        print::line(&rendered);
    }

    #[cfg(not(feature = "layouts"))]
    {
        print::line("layouts feature not enabled. Run with --features layouts");
    }
}
