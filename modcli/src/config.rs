use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct CliConfig {
    pub theme: Option<String>,
    pub strict_args: Option<bool>,
    pub prompt_prefix: Option<String>,
    pub banner: Option<String>,
    pub welcome: Option<Vec<String>>,
    pub goodbye: Option<Vec<String>>,
    pub line_delay: Option<u64>,
    pub no_command_message: Option<String>,
}

impl CliConfig {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Failed to read config JSON");
        serde_json::from_str(&data).expect("Invalid config JSON format")
    }
}