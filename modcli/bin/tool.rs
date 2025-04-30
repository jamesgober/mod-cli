use crossterm::style::Color;
use modcli::ModCli;
use modcli::config::CliConfig;
use modcli::loader::sources::JsonFileSource;
use modcli::output::{
    themes::apply_theme,
    gradient,
    colors,
    print,
    build,
    table::{
        render_table, 
        TableMode, 
        TableStyle
    },
    progress::{
        ProgressBar, 
        ProgressStyle,
    },
    RED, BLUE, ORANGE, YELLOW, GREEN, LIGHT_BLUE
};
use modcli::output::input::console::run_interactive_console;


//use modcli::output::table::render_table;
//use modcli::output::input::{prompt_text, prompt_password, prompt_confirm};
//use modcli::output::input::prompt_text_with_validation;




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

    if let Some(banner) = &config.banner {
        let delay = config.line_delay.unwrap_or(0);
        print::scroll(&banner.lines().collect::<Vec<&str>>(), delay);
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

 let gradient_test = gradient::two_color("Gradient Output", BLUE, GREEN);
 let testing = build()
         .part(&gradient_test).bold().space()
         .part("+ Styled!")
         .get();
 
 print::line(&testing);


let gradient_text = gradient::two_color("Command Line Interface", RED, ORANGE);
print::line(&gradient_text);

let gradient_multi = gradient::multi_color("Glorious Text", vec![RED, ORANGE, YELLOW, GREEN, BLUE]);
let style_built = build()
    .part(&gradient_multi).space()
    .part("Working!").bold().get();

print::line(&style_built);

let gradient_rgb = gradient::two_color(
    "Gradient using RGB", 
    Color::Rgb { r: 255, g: 0, b: 0 },
    Color::Rgb { r: 0, g: 0, b: 255 },
);
print::line(&gradient_rgb);

 
 // 📦 Progress Bar Demo
 let label = build()
    .part("Loading")
    .color(LIGHT_BLUE)
    .bold()
    .get();

let mut bar = ProgressBar::new(30, ProgressStyle {
    fill: '■',
    done_label: "Complete!",
    color: Some(LIGHT_BLUE),
    ..Default::default()
});

bar.set_label(&label);
bar.start_auto(2000); // auto-fill in 2 seconds
 

 // 🎯 Named Color Demo
 let teal = colors::get("teal"); // always returns a Color (or fallback)
 let demo = build()
     .part("Color Demo:").space()
     .part("Teal").color(teal).bold().get();

 print::line(&demo);



// ###############################################


let headers_raw = [
    build().part("Name").bold().get(),
    build().part("Age").bold().get(),
    build().part("Role").bold().get(),
];
let headers: Vec<&str> = headers_raw.iter().map(|s| s.as_str()).collect();

let name1 = build().part("Alice").color(RED).bold().get();
let name2 = build().part("Bob").color(BLUE).italic().get();
let name3 = build().part("Charlie").color(GREEN).underline().get();

let rows = vec![
    vec![&name1, "29", "Engineer"],
    vec![&name2, "35", "Manager"],
    vec![&name3, "41", "CTO"],
];

render_table(&headers, &rows, TableMode::Flex, TableStyle::Heavy);

// ⚠ Output Hooks Demo
print::status("CLI started");
print::debug("Debug message");
print::info("Information ");
print::warn("Warn message");
print::error("Error message");
print::success("Success message");


// ###############################################


    if args.is_empty() || args.len() == 1 {
        let msg = &config.no_command_message.unwrap_or_else(|| {
            "⚠️ No command given. Try `help`.".to_string()
        });
        print::status(&msg);
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
