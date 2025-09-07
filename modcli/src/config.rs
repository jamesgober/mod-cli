use serde::Deserialize;
use std::sync::OnceLock;
use std::fs;

static CONFIG: OnceLock<CliConfig> = OnceLock::new();
static CONFIG_PATH: OnceLock<String> = OnceLock::new();
static RAW_CONFIG: &str = include_str!("../examples/config.json");

#[derive(Debug, Deserialize)]
pub struct CliConfig {
    pub modcli: ModCliSection,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ShellConfig {
    pub prompt: Option<String>,
    pub welcome: Option<Vec<String>>,
    pub goodbye: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
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

impl CliConfig {
    /// Loads config from: custom path > project root > examples/ > embedded
    pub fn load(_unused: Option<&str>) -> &'static CliConfig {
        CONFIG.get_or_init(|| {
            // 👇 Custom override path if set
            if let Some(p) = CONFIG_PATH.get() {
                if let Ok(data) = fs::read_to_string(p) {
                    return parse(&data);
                }
            }
    
            // Fallbacks...
            if let Ok(data) = fs::read_to_string("config.json") {
                return parse(&data);
            }
    
            if let Ok(data) = fs::read_to_string("examples/config.json") {
                return parse(&data);
            }
    
            parse(RAW_CONFIG)
        })
    }

    /// Owned config loader (non-global). Prefer this in library code for better testability.
    pub fn load_owned(path: Option<&str>) -> CliConfig {
        if let Some(p) = path {
            if let Ok(data) = fs::read_to_string(p) {
                return parse(&data);
            }
        }

        if let Ok(data) = fs::read_to_string("config.json") {
            return parse(&data);
        }

        if let Ok(data) = fs::read_to_string("examples/config.json") {
            return parse(&data);
        }

        parse(RAW_CONFIG)
    }
}

fn parse(data: &str) -> CliConfig {
    serde_json::from_str(data).expect("Failed to parse CLI config JSON")
}

pub fn set_path(path: &str) {
    let _ = CONFIG_PATH.set(path.to_string()); // only sets once
}