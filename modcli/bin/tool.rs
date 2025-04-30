use modcli::ModCli;
use modcli::config::CliConfig;
use modcli::loader::sources::JsonFileSource;
use modcli::output::input::console::run_interactive_console;
use modcli::output::{
    print_multiline, print_status, print_success, print_warning, print_error,
    gradient::print_gradient_line,
    color_picker::get_color_by_name,
    progress::{
        ProgressBar, 
        ProgressStyle,
        show_progress_bar,
        show_percent_progress,
        show_spinner,
    },
    themes::{
        apply_theme,
        RED, ORANGE,
    },
};
//use modcli::output::table::render_table;
//use modcli::output::input::{prompt_text, prompt_password, prompt_confirm};
//use modcli::output::input::prompt_text_with_validation;

use crossterm::style::{Color, Stylize};
//use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Load config file
    let config = CliConfig::load("examples/config.json");

    // Call interactive console with config
    run_interactive_console(&config);


    // Apply theme if defined
    if let Some(theme) = &config.theme {
        apply_theme(theme.as_str());
    }

    // Print startup banner
    if let Some(banner) = &config.banner {
        let lines: Vec<&str> = banner.lines().collect();
        print_multiline(&lines, None);
    }



// ###############################################
/*
pub fn cli_login_form() {
    println!("🔐 Please log in below.\n");

    let username = prompt_text("Username: ");
    let password = prompt_password("Password: ");

    // Simulate login validation (replace with actual logic later)
    if username == "admin" && password == "secret123" {
        println!("\n✅ Login successful. Welcome, {}!", username);
    } else {
        println!("\n❌ Invalid credentials. Access denied.");
    }
}

cli_login_form();
 */

/*
 // loading spinner
 println!("Starting spinner...");
show_spinner("Loading", 20, 100);

println!("Progress bar test:");
show_progress_bar("Installing", 30, 1500);

for i in (0..=100).step_by(10) {
    show_percent_progress("Syncing", i);
    std::thread::sleep(Duration::from_millis(100));
}
println!();
*/

/*
// === Prompt Demo ===
println!("\n[Prompt Demo]");

let name = prompt_text("What is your name?");
let password = prompt_password("Enter your password:");
let confirmed = prompt_confirm("Are you ready to proceed?");

println!("\n--- Result ---");
println!("Name     : {}", name);
println!("Password : {}", "*".repeat(password.len()));
println!("Confirmed: {}", if confirmed { "Yes" } else { "No" });
println!("----------------\n");

 */

/*
 // Create a table
 let headers = vec!["Name", "Role", "Status"];

 let rows = vec![
     vec!["Alice", "Admin", "Active"],
     vec!["Bob", "User", "Inactive"],
     vec!["Charlie", "Guest", "Pending"],
 ];
 
 render_table(&headers, &rows);
 */

 // ###############################################


 print_gradient_line("James Gober", RED, ORANGE);

 // 🌈 Gradient Demo
 print_gradient_line(
     "Gradient Text Test!",
     Color::Rgb { r: 255, g: 0, b: 0 },
     Color::Rgb { r: 0, g: 0, b: 255 },
 );
 
 // 📦 Progress Bar Demo
 let mut bar = ProgressBar::new(30, ProgressStyle::default());
 bar.set_label("Loading");
 bar.start(2000); // 2 second animation
 
 // 🎯 Named Color Demo
 if let Some(teal) = get_color_by_name("teal") {
     println!("{}", "Color Demo: (Teal)".with(teal));
 }


// ###############################################


// ⚠ Output Hooks Demo
print_status("CLI started");
print_success("Success: Test");
print_warning("Warning: Test");
print_error("Error: Test");


// ###############################################

 

    if args.is_empty() {
        println!("No args provided.");
        return;
    }

    // Init CLI
    let mut cli = ModCli::new();

    // Load commands from external JSON source
    let source = JsonFileSource::new("examples/commands.json");
    cli.registry.load_from(Box::new(source));

    // Enforce strict argument count if enabled in config
    if let Some(strict) = config.strict_args {
        if strict && args.len() > 1 {
            eprintln!("Too many arguments. Strict mode is enabled.");
            return;
        }
    }

    cli.run(args);
}
