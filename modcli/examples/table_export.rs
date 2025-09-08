use modcli::output::print;
use modcli::output::table::{
    render_table_csv, render_table_markdown, write_table_csv, write_table_markdown,
};

fn main() {
    let headers = ["Name", "Role", "Team"];
    let rows = vec![
        vec!["Ada", "Engineer", "Core"],
        vec!["Linus", "Maintainer", "Kernel"],
        vec!["Grace", "Security", "Ops"],
    ];

    // Preview in terminal
    let md = render_table_markdown(&headers, &rows);
    let csv = render_table_csv(&headers, &rows);
    print::line("Markdown preview:\n");
    println!("{md}");
    print::line("CSV preview:\n");
    println!("{csv}");

    // Write to files under target/
    let _ = std::fs::create_dir_all("target/out");
    let md_path = "target/out/table.md";
    let csv_path = "target/out/table.csv";

    match write_table_markdown(md_path, &headers, &rows) {
        Ok(_) => print::line(&format!("Wrote {md_path}")),
        Err(e) => print::line(&format!("Failed to write {md_path}: {e}")),
    }
    match write_table_csv(csv_path, &headers, &rows) {
        Ok(_) => print::line(&format!("Wrote {csv_path}")),
        Err(e) => print::line(&format!("Failed to write {csv_path}: {e}")),
    }
}
