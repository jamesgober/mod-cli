use std::sync::{Mutex, OnceLock};

use modcli::command::Command;
use modcli::loader::CommandRegistry;

static EXEC_LOG: OnceLock<Mutex<Vec<&'static str>>> = OnceLock::new();

fn log_exec(label: &'static str) {
    EXEC_LOG
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .push(label);
}

#[derive(Default)]
struct Hello;

impl Command for Hello {
    fn name(&self) -> &str {
        "hello"
    }
    fn help(&self) -> Option<&str> {
        Some("hello test")
    }
    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Ok(())
    }
    fn execute(&self, _args: &[String]) {
        log_exec("hello_executed")
    }
}

#[test]
fn executes_command_with_prefix_routing() {
    let mut reg = CommandRegistry::new();
    reg.set_prefix("tool");
    reg.register(Box::new(Hello));

    // Execute using prefixed syntax
    reg.execute("tool:hello", &[]);

    let log = EXEC_LOG.get().unwrap().lock().unwrap();
    assert!(
        log.contains(&"hello_executed"),
        "prefixed command did not execute"
    );
}
