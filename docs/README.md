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
 
## Using Features

The crate exposes feature flags to tailor functionality and surface area:

- `internal-commands`: Built-in helper commands (e.g., `help`, `ping`).
- `custom-commands`: Ergonomic helpers for user-defined commands.
- `tracing-logs`: Emit `tracing` events from `output::hook` while printing themed console output.
- `dispatch-cache`: Optional single-entry dispatch cache to accelerate repeated command invocations.
- `gradients`: Named gradient helpers (24-bit RGB) with zero additional dependencies.
- `layouts`: Lightweight layout engine for terminal rows/columns.
- `table-presets`: Convenience presets for `TableStyle` (ASCII, Rounded, Heavy).
- `progress-presets`: Convenience constructors for `ProgressStyle`.
- `theme-config`: Enable theme config serialization (serde/serde_json).
- `images`: Optional integration with the `image` crate (png/jpeg).

Enable features in your application’s Cargo.toml:

```toml
[dependencies]
mod-cli = { version = "0.6.4", features = ["gradients", "table-presets"] }
```

## Feature Matrix

| Feature             | Default | Description                                                                 |
|---------------------|:-------:|-----------------------------------------------------------------------------|
| `internal-commands` |   on    | Includes built-in helper commands.                                          |
| `custom-commands`   |   on    | Ergonomic re-exports for user-defined commands.                             |
| `tracing-logs`      |   off   | Emits `tracing` events from hooks alongside themed console output.          |
| `dispatch-cache`    |   off   | Single-entry dispatch cache to speed repeated command invocations.          |
| `gradients`         |   off   | Named gradient helpers (24-bit RGB).                                        |
| `layouts`           |   off   | Lightweight layout engine for terminal rows/columns.                        |
| `table-presets`     |   off   | Convenience presets for `TableStyle` (ASCII, Rounded, Heavy).               |
| `progress-presets`  |   off   | Convenience constructors for `ProgressStyle`.                                |
| `theme-config`      |   off   | Theme config serialization (serde/serde_json).                               |
| `images`            |   off   | Optional `image` crate integration (png/jpeg).                               |

## Examples

The repository includes runnable examples under `modcli/examples/` covering styled output, gradients, tables, progress, layouts, and themes.

Basic (no extra features):

```console
$ cargo run --example style_builder
$ cargo run --example table_basic
$ cargo run --example progress_basic
$ cargo run --example progress_custom
$ cargo run --example gradient_multi
$ cargo run --example themes_demo
```

Feature-gated examples (enable flags as shown):

```console
$ cargo run --example gradient_two --features gradients
$ cargo run --example gradient_multi --features gradients
$ cargo run --example table_presets --features table-presets
$ cargo run --example progress_presets --features progress-presets
$ cargo run --example layout_demo --features layouts
```

Justfile shortcuts (optional):

```console
$ just all-basic
$ just all-features
```

## Contributing Performance

When micro-benchmarks are available (Criterion recommended):

```console
$ cargo bench
```

Tips:
- Run with `--features json-loader,plugins,internal-commands,custom-commands` to stress more code paths.
- Use `perf`/`dtrace`/`Instruments` or `cargo-flamegraph` to profile hotspots.
- Focus on command dispatch, output building, and print paths.

## Profiling Quickstart

- Linux (perf):
  - `sudo perf record -- cargo bench`
  - `sudo perf report`

- macOS (Instruments):
  - `cargo bench` and attach Instruments (Time Profiler) to the benchmark process.

- macOS (dtrace):
  - `sudo dtrace -n 'profile-997 /pid==$PID/ { @[ustack()] = count(); }' -c "cargo bench"`

- Flamegraph (all platforms with support):
  - `cargo install flamegraph`
  - `cargo flamegraph --bench <bench-name>`


## Error Code Mapping

The `modcli` binary maps errors to exit codes for shell/automation friendliness:

- 0: Success.
- 1: Runtime errors (I/O, plugin errors, unexpected conditions).
- 2: Usage/configuration errors (invalid args, strict-mode violations, missing shell config).
- 127: Unknown command.

Example:

```console
$ modcli --version
v0.6.3

$ modcli unknown
Unknown command: unknown
$ echo $?
127

$ modcli  # no command
No command provided.
$ echo $?
2
```


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


#### Example: initialize tracing (feature-gated)

```rust
// Cargo.toml (enable the feature in your binary crate)
// [dependencies]
// mod-cli = { version = "*", features = ["tracing-logs"] }

#[cfg(feature = "tracing-logs")]
fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .init();
}

fn main() {
    #[cfg(feature = "tracing-logs")]
    init_tracing();

    // ... rest of your app
}
```

Notes:
- Use `RUST_LOG=modcli=debug` to see framework events.
- Hooks print themed console output regardless; tracing emits structured events when the feature is enabled.


## Security Considerations

The framework is designed with conservative defaults and explicit feature gates. Use the following guidance when building secure CLI applications:

1) Plugins (feature: `plugins`)

- Only load plugins from trusted locations. The loader restricts to regular files with platform extensions (`.so`, `.dylib`, `.dll`).
- Consider a plugin discovery allowlist (directory or hash/signature) in your app.
- Prefer versioned/ABI-stable registration symbols if you control both sides; today the loader expects `register_command`.
- Run with least privilege (drop permissions before loading, if possible) and consider containerization/sandboxing for untrusted plugins.

2) Input validation and secrets

- All interactive input paths avoid panics and log with themed hooks; errors do not expose sensitive data.
- Password prompts never echo and return empty strings on I/O error — validate inputs and re-prompt as needed.
- Sanitize/validate arguments before execution. Use `Command::validate()` to block invalid inputs early.

3) Unsafe code policy

- Avoid `unsafe` in application code; the framework limits `unsafe` to the plugin loader where it is required for dynamic loading.
- Use feature gates (`plugins`, `json-loader`, etc.) to minimize exposed surface when not needed.

4) Operational guidance

- Set strict argument handling in config when appropriate (usage errors exit with code `2`).
- Capture logs with `tracing-logs` for auditability in development and staging.
- Keep dependencies updated, and use `cargo deny`/`cargo audit` in CI (example workflows included).




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
