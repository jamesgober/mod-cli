// Requires: --features theme-config
use modcli::output::print;
#[cfg(feature = "theme-config")]
use modcli::output::themes;

fn main() {
    print::line("Load theme from JSON (feature: theme-config):");

    #[cfg(feature = "theme-config")]
    {
        let path = "modcli/examples/themes/sample_theme.json";
        match themes::load_theme_from_json(path) {
            Ok(t) => {
                // Apply loaded theme (temporary via ThemeGuard)
                let _guard = themes::ThemeGuard::apply(&t.name);
                print::line(&format!("Applied loaded theme: {}", t.name));
                // Show log color categories
                for key in [
                    "error", "warn", "success", "info", "debug", "trace", "notice", "status",
                ]
                .iter()
                {
                    let c = t.get_log_color(key);
                    println!("{key:>7}: {c:?}");
                }
            }
            Err(e) => {
                print::line(&format!("Failed to load theme: {e}"));
            }
        }
    }

    #[cfg(not(feature = "theme-config"))]
    {
        print::line("theme-config feature not enabled. Run with --features theme-config");
    }
}
