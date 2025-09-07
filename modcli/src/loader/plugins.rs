use crate::command::Command;
use libloading::{Library, Symbol};
use std::fs;
use std::path::PathBuf;

pub struct PluginLoader {
    path: PathBuf,
}

impl PluginLoader {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    pub fn load_plugins(&self) -> Vec<Box<dyn Command>> {
        let mut plugins = vec![];

        // Determine the platform-specific extension we should load
        #[cfg(target_os = "linux")]
        let allowed_ext = "so";
        #[cfg(target_os = "macos")]
        let allowed_ext = "dylib";
        #[cfg(target_os = "windows")]
        let allowed_ext = "dll";

        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                // Only try to load the native extension for this platform
                if ext == allowed_ext {
                    unsafe {
                        match Library::new(&path) {
                            Ok(lib) => {
                                match lib
                                    .get::<Symbol<fn() -> Box<dyn Command>>>(b"register_command")
                                {
                                    Ok(func) => {
                                        let command = func();
                                        plugins.push(command);
                                        std::mem::forget(lib); // keep plugin alive
                                    }
                                    Err(err) => {
                                        eprintln!(
                                            "[plugin] missing register_command in {}: {err}",
                                            path.display()
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("[plugin] skipping {}: {}", path.display(), err);
                            }
                        }
                    }
                }
            }
        }

        plugins
    }
}
