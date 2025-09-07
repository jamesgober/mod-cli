use modcli::loader::CommandRegistry;
use modcli::command::Command;
use modcli::error::ModCliError;

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
    fn name(&self) -> &str { "bad" }
    fn help(&self) -> Option<&str> { Some("Always invalid") }
    fn validate(&self, _args: &[String]) -> Result<(), String> { Err("invalid on purpose".into()) }
    fn execute(&self, _args: &[String]) { panic!("should not execute on invalid") }
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

#[cfg(feature = "json-loader")]
mod json_loader {
    use super::*;
    use modcli::loader::sources::JsonFileSource;
    use std::fs;

    #[test]
    fn json_loader_missing_file_no_panic() {
        let mut reg = CommandRegistry::new();
        let source = JsonFileSource::new("/this/path/should/not/exist/____.json");
        reg.load_from(Box::new(source));
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn json_loader_invalid_json_no_panic() {
        // Create a temp invalid json file
        let dir = std::env::temp_dir();
        let path = dir.join("modcli_invalid.json");
        let _ = fs::write(&path, "{ invalid json ");

        let mut reg = CommandRegistry::new();
        let source = JsonFileSource::new(path.to_string_lossy().to_string());
        reg.load_from(Box::new(source));
        assert_eq!(reg.len(), 0);

        let _ = fs::remove_file(path);
    }
}
