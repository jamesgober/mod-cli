use crate::command::Command;
use crate::output::hook;
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub trait CommandSource {
    fn load_commands(&self) -> Vec<Box<dyn Command>>;
}

/// JSON-defined command format
#[derive(Debug, Deserialize)]
struct JsonCommand {
    name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    help: Option<String>,
    #[serde(default)]
    hidden: bool,
}

/// A command loaded from JSON metadata
struct DynamicJsonCommand {
    name: String,
    #[allow(dead_code)]
    aliases: Vec<String>,
    help: Option<String>,
    hidden: bool,
}

impl Command for DynamicJsonCommand {
    fn name(&self) -> &str {
        &self.name
    }

    fn aliases(&self) -> &[&str] {
        &[]
    }

    fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Ok(())
    }

    fn execute(&self, args: &[String]) {
        println!("Executed dynamic command '{}': {:?}", self.name, args);
    }
}

/// JSON loader from file path
pub struct JsonFileSource {
    path: String,
}

impl JsonFileSource {
    pub fn new<P: Into<String>>(path: P) -> Self {
        Self { path: path.into() }
    }
}

impl CommandSource for JsonFileSource {
    fn load_commands(&self) -> Vec<Box<dyn Command>> {
        let path = Path::new(&self.path);
        let data = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                hook::error(&format!(
                    "Failed to read JSON file '{}': {e}",
                    path.display()
                ));
                return vec![];
            }
        };

        let defs: Vec<JsonCommand> = match serde_json::from_str(&data) {
            Ok(d) => d,
            Err(e) => {
                hook::error(&format!("Invalid JSON in '{}': {e}", path.display()));
                return vec![];
            }
        };

        defs.into_iter()
            .map(|cmd| {
                Box::new(DynamicJsonCommand {
                    name: cmd.name,
                    aliases: cmd.aliases,
                    help: cmd.help,
                    hidden: cmd.hidden,
                }) as Box<dyn Command>
            })
            .collect()
    }
}
