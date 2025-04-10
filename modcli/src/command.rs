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