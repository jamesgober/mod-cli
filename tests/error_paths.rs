use modcli::console::run_shell;
use modcli::error::ModCliError;
use modcli::loader::CommandRegistry;
use modcli::config::{CliConfig, ModCliSection, MessageConfig};

#[test]
fn console_missing_shell_config_returns_error() {
    // Build a minimal config without shell section
    let cfg = CliConfig {
        modcli: ModCliSection {
            name: Some("mod-cli".into()),
            prefix: None,
            banner: None,
            theme: None,
            delay: None,
            strict: None,
            force_shell: None,
            shell: None,
            messages: Some(MessageConfig::default()),
        },
    };

    let result = run_shell(&cfg);
    match result {
        Err(ModCliError::MissingShellConfig) => {}
        other => panic!("expected MissingShellConfig, got {:?}", other),
    }
}

#[test]
fn registry_unknown_command_does_not_panic() {
    let reg = CommandRegistry::new();
    // Should not panic; will print unknown hook
    reg.execute("this-command-does-not-exist", &[]);
}

#[cfg(feature = "plugins")]
#[test]
fn plugin_loader_on_empty_dir_does_not_panic() {
    use std::fs;
    use std::path::PathBuf;

    let mut reg = CommandRegistry::new();

    // create a temp directory
    let dir = std::env::temp_dir().join("modcli_empty_plugins_test");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");

    reg.load_plugins(dir.to_str().unwrap());

    // No plugins registered; len unchanged
    assert_eq!(reg.len(), 0);

    let _ = fs::remove_dir_all(dir);
}
