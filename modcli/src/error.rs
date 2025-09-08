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

    #[error("unknown command: {0}")]
    UnknownCommand(String),

    #[error("error: {0}")]
    Other(String),
}
