use crossterm::style::{Color, Stylize};
use modcli::output::{print, themes};
use modcli::{set_startup_banner, ModCli};

fn main() {
    // Register a one-time banner callback (runs at the start of ModCli::run)
    set_startup_banner(|| {
        let _guard = themes::ThemeGuard::apply("blue");
        let art = r#"
   __  ___        __     ____ _     ___ 
  /  |/  /____ _ / /_   / __ ( )   /   |
 / /|_/ // __ `// __ \ / / / /| | / /| |
/ /  / // /_/ // /_/ // /_/ / | |/ ___ |
/_/  /_/ \__,_//_.___/ \____/  |__/_/  |_|
"#;
        print::line(&art.with(Color::Cyan).bold().to_string());
        print::line("Welcome to mod-cli! Type 'help' to see available commands.");
        themes::Theme::reset();
        println!("");
    });

    // Minimal CLI to demonstrate banner showing before dispatch
    let mut cli = ModCli::new();
    // Use a dummy command name so this exits gracefully
    cli.run(vec!["help".into()]);
}
