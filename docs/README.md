<div align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <h1>
        <strong>mod-cli</strong>
        <sup>
            <br>
            <sub>DOCUMENTATION</sub>
            <br>
        </sup>
    </h1>
</div>
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <span>DOCS</span>
        <span>&nbsp;│&nbsp;</span>
        <a href="./API.md" title="API Reference"><b>API</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./GUIDELINES.md" title="Developer Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

## Overview


### Error Handling and Exit Codes

The framework surfaces structured errors internally and provides user-friendly messages by default. For programmatic flows, use `CommandRegistry::try_execute(cmd, args)` which returns a `Result<(), ModCliError>`.

Exit code policy for bundled binaries:

- 0: Success.
- 1: Runtime errors (I/O, plugin failures, unexpected conditions).
- 2: Usage/configuration errors (invalid arguments, strict-mode violations, missing shell config).
- 127: Unknown command.

Configuration parsing errors preserve the original `serde_json` error via `ModCliError::ConfigParse`, improving diagnostics.


### Optional Diagnostics with Tracing

Enable the `tracing-logs` feature to emit structured tracing events alongside themed console output through `output::hook`.

Feature flags:

```toml
[features]
tracing-logs = ["dep:tracing", "dep:tracing-subscriber"]
```

When enabled, you can configure logging with `RUST_LOG` and a tracing subscriber in your application. The provided binaries do not initialize a global subscriber, leaving control to the application.




<br>

## Documentation on docs.rs

The full API documentation is available on docs.rs:

- https://docs.rs/mod-cli

Docs are built with the following features enabled to maximize example coverage:

- `json-loader`
- `plugins`
- `internal-commands`
- `custom-commands`

When browsing examples, note that some snippets are feature-gated and only compile/run when the corresponding feature is enabled.


<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
