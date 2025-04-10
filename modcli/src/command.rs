/// Defines the trait for commands to implement.
pub trait Command {
    fn name(&self) -> &str;
    fn aliases(&self) -> &[&str] {
        &[]
    }
    fn help(&self) -> Option<&str> {
        None
    }
    fn hidden(&self) -> bool {
        false
    }
    fn execute(&self, args: &[String]);
}

/// Trait for defining custom CLI commands.
pub trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[String]);
    fn description(&self) -> &'static str;
}