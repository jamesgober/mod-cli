use modcli::command::Command;
use modcli::error::ModCliError;
use modcli::loader::CommandRegistry;

#[test]
fn registry_try_execute_unknown_returns_error() {
    let reg = CommandRegistry::new();
    let err = reg.try_execute("does-not-exist", &[]).unwrap_err();
    match err {
        ModCliError::UnknownCommand(name) => assert_eq!(name, "does-not-exist"),
        other => panic!("expected UnknownCommand, got {other:?}"),
    }
}

#[derive(Default)]
struct InvalidCmd;

impl Command for InvalidCmd {
    fn name(&self) -> &str {
        "bad"
    }
    fn help(&self) -> Option<&str> {
        Some("Always invalid")
    }
    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Err("invalid on purpose".into())
    }
    fn execute(&self, _args: &[String]) {
        panic!("should not execute on invalid")
    }
}

#[test]
fn registry_try_execute_invalid_usage_returns_error() {
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(InvalidCmd::default()));
    let err = reg.try_execute("bad", &[]).unwrap_err();
    match err {
        ModCliError::InvalidUsage(msg) => assert!(msg.contains("invalid")),
        other => panic!("expected InvalidUsage, got {other:?}"),
    }
}
