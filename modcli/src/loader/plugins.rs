use std::fs;
use std::path::PathBuf;
use libloading::{Library, Symbol};
use crate::command::Command;

pub struct PluginLoader {
    path: PathBuf,
}

impl PluginLoader {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    pub fn load_plugins(&self) -> Vec<Box<dyn Command>> {
        let mut plugins = vec![];

        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("so") {
                    unsafe {
                        let lib = Library::new(&path).expect("Failed to load plugin library");

                        let func: Symbol<fn() -> Box<dyn Command>> =
                            lib.get(b"register_command").expect("Plugin missing register_command");

                        let command = func();
                        plugins.push(command);
                        std::mem::forget(lib); // keep plugin alive
                    }
                }
            }
        }

        plugins
    }
}