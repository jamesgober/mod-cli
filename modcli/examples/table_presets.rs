use modcli::output::print;
#[cfg(feature = "table-presets")]
use modcli::output::table::{render_table, TableMode, TableStyle};

fn main() {
    print::line("Preset styles (requires feature table-presets):");

    #[cfg(feature = "table-presets")]
    {
        let headers = ["Lang", "Perf", "Use-Case"];
        let rows = vec![
            vec!["Rust", "Top-tier", "Systems, CLIs"],
            vec!["Go", "Great", "Servers, Tools"],
            vec!["Python", "Good", "Data, Glue"],
        ];

        let ascii = render_table(&headers, &rows, TableMode::Flex, TableStyle::ascii_preset());
        print::line("ASCII preset:");
        print::line(&ascii);

        let rounded = render_table(
            &headers,
            &rows,
            TableMode::Flex,
            TableStyle::rounded_preset(),
        );
        print::line("Rounded preset:");
        print::line(&rounded);

        let heavy = render_table(&headers, &rows, TableMode::Flex, TableStyle::heavy_preset());
        print::line("Heavy preset:");
        print::line(&heavy);
    }

    #[cfg(not(feature = "table-presets"))]
    {
        print::line("table-presets feature not enabled. Run with --features table-presets");
    }
}
