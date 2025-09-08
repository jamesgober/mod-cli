use modcli::ModCli;
use modcli::output::{hook, print};

fn main() {
    // Grab CLI args
    let args: Vec<String> = std::env::args().skip(1).collect();

    // No args? status message and exit success
    if args.is_empty() {
        print::status("⚠️ No command given. Try `help`.");
        return;
    }

    // Create and configure CLI
    let mut cli = ModCli::new();
    cli.set_prefix("mod");

    // Enforce strict argument mode (1 command only)
    if args.len() > 1 {
        hook::warn("Too many arguments; expected a single command.");
    }

    // Now safely run the CLI with args
    cli.run(args);
}
