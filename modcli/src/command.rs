/// Defines the trait for commands to implement.
///
/// # Example
/// ```no_run
/// use modcli::command::Command;
/// use modcli::ModCli;
///
/// struct Hello;
///
/// impl Command for Hello {
///     fn name(&self) -> &str { "hello" }
///     fn help(&self) -> Option<&str> { Some("Greets the user") }
///     fn validate(&self, _args: &[String]) -> Result<(), String> { Ok(()) }
///     fn execute(&self, _args: &[String]) { println!("Hello!"); }
/// }
///
/// let mut cli = ModCli::new();
/// cli.registry.register(Box::new(Hello));
/// let args = vec!["hello".to_string()];
/// cli.run(args);
/// ```
use crate::loader::CommandRegistry;

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

    /// Capability requirements for visibility/authorization.
    /// The parent application grants capabilities at runtime on the registry.
    /// Default: no requirements.
    fn required_caps(&self) -> &[&str] {
        &[]
    }

    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Ok(())
    }

    fn execute(&self, args: &[String]);

    /// Execute with access to the registry context. Default delegates to `execute`.
    /// Commands that need registry access (e.g., `help`) can override this.
    fn execute_with(&self, args: &[String], _registry: &CommandRegistry) {
        self.execute(args)
    }
}
