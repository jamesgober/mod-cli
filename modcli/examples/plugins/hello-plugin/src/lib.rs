use mod_cli::command::Command;

struct HelloPlugin;

impl Command for HelloPlugin {
    fn name(&self) -> &str { "hello-plugin" }
    fn help(&self) -> Option<&str> { Some("Example plugin command") }
    fn validate(&self, _args: &[String]) -> Result<(), String> { Ok(()) }
    fn execute(&self, _args: &[String]) { println!("Hello from example plugin!"); }
}

#[no_mangle]
pub fn register_command() -> Box<dyn Command> {
    Box::new(HelloPlugin)
}
