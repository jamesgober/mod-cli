use crate::command::Command;
use crate::modcli_version;
use crate::output::{build, print, BLUE, GREY, LIGHT_BLUE};
pub struct FrameworkCommand;

impl Command for FrameworkCommand {
    /// Command name
    fn name(&self) -> &'static str {
        "framework"
    }

    // Command help
    fn help(&self) -> Option<&str> {
        Some("Framework Information")
    }

    // Command hidden
    fn hidden(&self) -> bool {
        true
    }

    // Command validate
    fn validate(&self, args: &[String]) -> Result<(), String> {
        if !args.is_empty() {
            Err("framework does not accept any arguments.".into())
        } else {
            Ok(())
        }
    }

    // Command execute
    fn execute(&self, _args: &[String]) {
        // Construct framework information
        let framework = build()
            .part("Mod")
            .color(LIGHT_BLUE)
            .bold()
            .part("cli")
            .color(BLUE)
            .part(":")
            .color(GREY)
            .space()
            .part("version:")
            .space()
            .part(modcli_version())
            .bold()
            .get();

        // Construct framework description
        let description = build()
            .part("⬢")
            .color(BLUE)
            .space()
            .part("cli framework for")
            .italic()
            .space()
            .part("Rust")
            .italic()
            .bold()
            .color(GREY)
            .space()
            .get();

        print::newline();
        print::line(&framework);
        print::line(&description);
        print::newline();
    }
}
