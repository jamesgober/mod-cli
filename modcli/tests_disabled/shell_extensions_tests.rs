use modcli::config::{CliConfig, ModCliSection, MessageConfig};
use modcli::shell_extensions::dispatch_shell_command;

fn minimal_cfg(name: Option<&str>) -> CliConfig {
    CliConfig {
        modcli: ModCliSection {
            name: name.map(|s| s.to_string()),
            prefix: None,
            banner: None,
            theme: None,
            delay: None,
            strict: None,
            force_shell: None,
            shell: None,
            messages: Some(MessageConfig::default()),
        },
    }
}

#[test]
fn shell_extensions_clear_returns_true() {
    let cfg = minimal_cfg(Some("proj"));
    assert!(dispatch_shell_command("clear", &cfg));
}

#[test]
fn shell_extensions_project_returns_true() {
    let cfg = minimal_cfg(Some("proj"));
    assert!(dispatch_shell_command("project", &cfg));
}

#[test]
fn shell_extensions_help_returns_true() {
    let cfg = minimal_cfg(Some("proj"));
    assert!(dispatch_shell_command("?", &cfg));
    assert!(dispatch_shell_command("shell help", &cfg));
}

#[test]
fn shell_extensions_unknown_returns_false() {
    let cfg = minimal_cfg(Some("proj"));
    assert!(!dispatch_shell_command("nonexistent", &cfg));
}
