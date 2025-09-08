#[cfg(feature = "layouts")]
use modcli::output::layout::{build, render};
use modcli::output::print;

fn main() {
    #[cfg(feature = "layouts")]
    {
        let left = vec![
            "Status".to_string(),
            "OK".to_string(),
            "".to_string(),
            "Uptime: 12m".to_string(),
        ];
        let right = vec![
            "Logs:".to_string(),
            "[INFO] init ok".to_string(),
            "[WARN] retry net".to_string(),
            "[OK]   ready".to_string(),
        ];

        let lay = build()
            .row()
            .col_percent(35)
            .content(left)
            .col_auto()
            .content(right)
            .end_row()
            .hgap(2)
            .vgap(1)
            .border(true)
            .finish();

        let s = render(&lay);
        print::line(&s);
    }

    #[cfg(not(feature = "layouts"))]
    {
        print::line("Enable feature 'layouts' to run this example");
    }
}
