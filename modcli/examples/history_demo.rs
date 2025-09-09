use modcli::shell::history;
use std::path::PathBuf;

fn main() {
    // Optional custom path via --history=<path>
    let args: Vec<String> = std::env::args().skip(1).collect();
    let custom_path = args
        .iter()
        .find_map(|a| a.strip_prefix("--history=").map(|s| PathBuf::from(s)));

    // Load history (empty if none)
    let mut entries = history::load(custom_path.as_deref());
    println!("Loaded {} history entries", entries.len());

    // Add a new entry passed after --add
    if let Some(idx) = args.iter().position(|a| a == "--add") {
        if let Some(cmd) = args.get(idx + 1) {
            history::add(custom_path.as_deref(), cmd).expect("append history");
            entries.push(cmd.clone());
            println!("Added entry: {cmd}");
        }
    }

    // Search with --search=<query>
    if let Some(q) = args.iter().find_map(|a| a.strip_prefix("--search=")) {
        let hits = history::search(&entries, q, 10);
        println!("Search '{q}' -> {} hits:", hits.len());
        for h in hits {
            println!("  {h}");
        }
    }

    // Save back
    history::save(custom_path.as_deref(), &entries).expect("save history");
}
