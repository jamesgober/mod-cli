pub type Result<T> = std::result::Result<T, ModCliError>;

define_err::define_errors! {
    #[exhaustive]
    pub enum ModCliError {
        MissingShellConfig =>
            "shell configuration is missing. Set modcli.shell in your config or disable shell mode.",

        PluginLoad(detail: String) => "plugin load error: {detail}",

        Io(#[from] source: std::io::Error) => "io error: {source}",

        InvalidUsage(detail: String) => "invalid usage: {detail}",

        Validation(detail: String) => "validation failed: {detail}",

        UnknownCommand(name: String) => "unknown command: {name}",

        #[cfg(feature = "theme-config")]
        ConfigParse(#[from] source: serde_json::Error) => "config parse error: {source}",

        Other(detail: String) => "error: {detail}",
    }
}
