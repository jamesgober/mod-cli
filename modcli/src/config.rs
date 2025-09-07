use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

use crate::error::ModCliError;
use crate::output::hook;

static CONFIG: OnceLock<CliConfig> = OnceLock::new();
static CONFIG_PATH: OnceLock<String> = OnceLock::new();
static RAW_CONFIG: &str = include_str!("../examples/config.json");

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CliConfig {
    pub modcli: ModCliSection,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModCliSection {
    pub name: Option<String>,
    pub prefix: Option<String>,
    pub banner: Option<String>,
    pub theme: Option<String>,
    pub delay: Option<u64>,
    pub strict: Option<bool>,
    pub force_shell: Option<bool>,
    pub shell: Option<ShellConfig>,
    pub messages: Option<MessageConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ShellConfig {
    pub prompt: Option<String>,
    pub welcome: Option<Vec<String>>,
    pub goodbye: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageConfig {
    pub no_command: Option<String>,
    pub not_found: Option<String>,
}

impl Default for MessageConfig {
    fn default() -> Self {
        MessageConfig {
            no_command: Some("⚠️ No command given. Try `help`.".to_string()),
            not_found: Some("⚠️ Command not found.".to_string()),
        }
    }
}

impl Default for ModCliSection {
    fn default() -> Self {
        ModCliSection {
            name: Some("mod-cli".into()),
            prefix: None,
            banner: None,
            theme: None,
            delay: None,
            strict: None,
            force_shell: None,
            shell: None,
            messages: Some(MessageConfig::default()),
        }
    }
}

impl CliConfig {
    /// Loads config from: custom path > project root > examples/ > embedded
    pub fn load(_unused: Option<&str>) -> &'static CliConfig {
        CONFIG.get_or_init(|| {
            // 👇 Custom override path if set
            if let Some(p) = CONFIG_PATH.get() {
                if let Ok(data) = fs::read_to_string(p) {
                    if let Ok(cfg) = parse(&data) {
                        return cfg;
                    } else {
                        hook::error(
                            "Invalid config format at custom path; falling back to defaults.",
                        );
                    }
                }
            }

            // Fallbacks...
            if let Ok(data) = fs::read_to_string("config.json") {
                if let Ok(cfg) = parse(&data) {
                    return cfg;
                } else {
                    hook::error("Invalid config.json format; trying examples/config.json.");
                }
            }

            if let Ok(data) = fs::read_to_string("examples/config.json") {
                if let Ok(cfg) = parse(&data) {
                    return cfg;
                } else {
                    hook::error("Invalid examples/config.json format; using embedded defaults.");
                }
            }

            match parse(RAW_CONFIG) {
                Ok(cfg) => cfg,
                Err(_) => {
                    hook::error(
                        "Embedded example config failed to parse; using built-in defaults.",
                    );
                    CliConfig::default()
                }
            }
        })
    }

    /// Owned config loader (non-global). Prefer this in library code for better testability.
    pub fn load_owned(path: Option<&str>) -> CliConfig {
        if let Some(p) = path {
            if let Ok(data) = fs::read_to_string(p) {
                if let Ok(cfg) = parse(&data) {
                    return cfg;
                }
            }
        }

        if let Ok(data) = fs::read_to_string("config.json") {
            if let Ok(cfg) = parse(&data) {
                return cfg;
            }
        }

        if let Ok(data) = fs::read_to_string("examples/config.json") {
            if let Ok(cfg) = parse(&data) {
                return cfg;
            }
        }

        parse(RAW_CONFIG).unwrap_or_default()
    }
}

fn parse(data: &str) -> Result<CliConfig, ModCliError> {
    serde_json::from_str::<CliConfig>(data).map_err(ModCliError::from)
}

pub fn set_path(path: &str) {
    let _ = CONFIG_PATH.set(path.to_string()); // only sets once
}
