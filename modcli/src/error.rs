use thiserror::Error;

pub type Result<T> = std::result::Result<T, ModCliError>;

#[derive(Debug, Error)]
pub enum ModCliError {
    #[error(
        "shell configuration is missing. Set modcli.shell in your config or disable shell mode."
    )]
    MissingShellConfig,

    #[error("plugin load error: {0}")]
    PluginLoad(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid usage: {0}")]
    InvalidUsage(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("unknown command: {0}")]
    UnknownCommand(String),

    #[cfg(feature = "theme-config")]
    #[error("config parse error: {0}")]
    ConfigParse(#[from] serde_json::Error),

    #[error("error: {0}")]
    Other(String),
}
