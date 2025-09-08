<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br><b>mod-cli</b><br>
    <sub><sup>API REFERENCE</sup></sub>
</h1>
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./README.md" title="Documentation"><b>DOCS</b></a>
        <span>&nbsp;│&nbsp;</span>
        <span>API</span>
        <span>&nbsp;│&nbsp;</span>
        <a href="./GUIDELINES.md" title="Developer Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

## Table of Contents
- **[Installation](#installation)**
- **[Feature Flags](#feature-flags)**
- **[Getting Started](#getting-started)**
  - **[Basic Usage](#basic-usage)**
  - **[Set a custom prefix](#set-custom-prefix)**

<hr>
<br>
<h2 id="installation">Installation</h2>

<br>


### Install Manually
Add this to your `Cargo.toml`:
```toml
[dependencies]
mod-cli = "0.5.3"
```

<br>

### Install via Terminal
```bash
# Basic installation
cargo add mod-cli
```

<br>

<hr>
<a href="#top">&uarr; <b>TOP</b></a>
<br>
<br>

<h2 id="feature-flags">Feature Flags</h2>

| Feature               | Default | Description |
|----------------------|:-------:|-------------|
| `internal-commands`  |  on     | Enables built-in helper commands like `help`, `ping`, etc. |
| `custom-commands`    |  on     | Enables ergonomic re-exports for user-defined commands. |
| `json-loader`        |  off    | Enables loading commands from JSON sources. |
| `plugins`            |  off    | Enables plugin support for dynamic runtime command injection (via libloading). |
| `tracing-logs`       |  off    | Emits `tracing` events via `output::hook` alongside themed console output. |
| `dispatch-cache`     |  off    | Optional single-entry dispatch cache to speed repeated command invocations. |


<br>
<hr>
<a href="#top">&uarr; <b>TOP</b></a>
<br>
<br>


<h2 id="getting-started">Getting Started</h2>

<br>

<h2 id="basic-usage">Basic Usage</h2>

```rust
use modcli::ModCli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cli = ModCli::new();
    cli.run(args);
}
```

<br>

<h2 id="set-custom-prefix">Set a custom prefix</h2>

```rust
use modcli::ModCli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cli = ModCli::new();

    // Set a custom prefix
    cli.set_prefix("myCLI");

    cli.run(args);
}
```


<!-- =============================================================== -->
<br><hr><a href="#top">&uarr; <b>TOP</b></a><br><br>
<!-- =============================================================== -->

<!-- // COLORS -->
<h2 id="colors-styles">Output Styles &a Colors</h2>
<br>









<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>


<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
