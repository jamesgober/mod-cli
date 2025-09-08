use std::sync::{Mutex, OnceLock};

use modcli::command::Command;
use modcli::loader::CommandRegistry;

// Shared test state to verify side effects
static EXEC_LOG: OnceLock<Mutex<Vec<&'static str>>> = OnceLock::new();

fn log_exec(label: &'static str) {
    EXEC_LOG
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .push(label);
}

#[derive(Default)]
struct AliasCmd;

impl Command for AliasCmd {
    fn name(&self) -> &str {
        "primary"
    }

    fn aliases(&self) -> &[&str] {
        &["alt", "p"]
    }

    fn help(&self) -> Option<&str> {
        Some("Alias test command")
    }

    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Ok(())
    }

    fn execute(&self, _args: &[String]) {
        log_exec("alias_executed")
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
        // If this is called, validation guard failed; we panic to fail the test.
        panic!("execute() should NOT be called when validate() fails");
    }
}

#[test]
fn alias_resolution_executes_primary_command() {
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(AliasCmd::default()));

    // Execute using alias
    reg.execute("alt", &[]);

    let log = EXEC_LOG.get().unwrap().lock().unwrap();
    assert!(
        log.contains(&"alias_executed"),
        "alias did not route to primary command"
    );
}

#[test]
fn validate_error_prevents_execute() {
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(InvalidCmd::default()));

    // Should not panic; execute must not be called due to validation error
    reg.execute("bad", &[]);
}
