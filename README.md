<h1 align="center">
    <img width="120px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br><strong>MOD-CLI</strong><br>
    <sub><sup><sup>CLI FRAMEWORK</sup></sup></sub>
</h1>
<div align="center">
    <div>
        <a href="https://crates.io/crates/mod-cli" alt="ModCLI on Crates.io"><img alt="Crates.io" src="https://img.shields.io/crates/v/mod-cli"></a>
        <span>&nbsp;</span>
        <a href="https://crates.io/crates/mod-cli" alt="Download ModCLI"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/mod-cli?color=%230099ff"></a>
        <span>&nbsp;</span>
        <a href="https://docs.rs/mod-cli" title="ModCLI Documentation"><img alt="docs.rs" src="https://img.shields.io/docsrs/mod-cli"></a>
        <span>&nbsp;</span>
        <a href="https://github.com/jamesgober/mod-cli/actions/workflows/ci.yml" title="CI Status"><img alt="CI" src="https://github.com/jamesgober/mod-cli/actions/workflows/ci.yml/badge.svg"></a>
    </div>
</div>
<br>
<p>
  <strong>MOD-CLI</strong> is a lightweight, modular CLI framework for Rust.
  Register commands, style output, and build interactive flows with a clean, zero-bloat core.
  Focus on your app, not boilerplate.
  
</p>

<br>
<br>

<h2>Documentation</h2>
<p>
  Full API docs are on <a href="https://docs.rs/mod-cli" title="Docs.rs: mod-cli">docs.rs</a>.
  Docs are built with a minimal, stable feature set (<code>internal-commands</code>, <code>custom-commands</code>) to ensure MSRV compatibility.
</p>

<br>
<h2>Key Features</h2>
<li>
    <strong>Custom Commands</strong> - Define your own commands with execution logic.
</li>
<br>
<li>
    <strong>Colors and Gradients</strong> – Full-spectrum foreground and background colors with multi-color gradient support.
</li>
<br>
<li>
    <strong>Styled Output</strong> – Compose bold, italic, underlined, and colorized output using a chainable builder. Includes table rendering and ANSI-safe formatting.
</li>
<br>
<li>
    <strong>Animated Loaders</strong> – Built-in progress bars, spinners, and percent indicators with customizable characters, labels, and themes.
</li>
<br>
<li>
    <strong>Interactive Console</strong> – Launch an interactive shell with command routing, custom input handlers, prompt styling, and exit messages.
</li>
<br>
<li>
    <strong>Modular Architecture</strong> – Drop-in components for printing, styling, theming, command sources, and shell extensions. Easily replace or extend any layer.
</li>
<br>
<li>
    <strong>Zero-Bloat Core</strong> – Minimal dependencies, clean structure, and fully optional features through Cargo flags.
</li>

<hr><br>

<h2>Installation</h2>

Add the library to your `Cargo.toml`:
```toml
[dependencies]
mod-cli = "0.6.3"
```
<br>

Add the library with features:
```toml
[dependencies]
mod-cli = { version = "0.6.3", features = ["gradients", "table-presets"] }
```
<br>

<br>
<h3>Feature Flags</h3>

| Feature               | Description |
|----------------------|-------------|
| `internal-commands`  | Built-in helper commands like `help`, `ping`, etc. |
| `custom-commands`    | Ergonomic helpers for user-defined commands. |
| `tracing-logs`       | Emit `tracing` events via `output::hook` alongside console output. |
| `dispatch-cache`     | Single-entry dispatch cache to speed repeated invocations. |
| `gradients`          | Named gradient helpers (24‑bit RGB) with zero extra deps. |
| `layouts`            | Lightweight layout engine for terminal rows/columns. |
| `table-presets`      | Convenience presets for `TableStyle` (ASCII, Rounded, Heavy). |
| `progress-presets`   | Convenience constructors for `ProgressStyle` (compact, heavy). |
| `theme-config`       | Enable theme config serialization (serde/serde_json). |
| `images`             | Optional image support (png/jpeg) via the `image` crate. |


<!-- Removed experimental sections for plugins/json-loader to align with current feature set. -->


<hr><br>

<h2>Usage</h2>


### Basic Usage
```rust
use modcli::ModCli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cli = ModCli::new();
    cli.run(args);
}
```

### Set a custom prefix
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


#### Using named colors
```rust
use modcli::output::{build, colors, print};

let teal = colors::get("teal"); // returns a Color (or fallback)
let demo = build()
    .part("Color Demo:").space()
    .part("Teal").color(teal).bold().get();
print::line(&demo);

```


---

### Using Gradients

#### Two-color gradient:
```rust
use modcli::output::{
    gradient,
    print,
    RED, ORANGE
};

let gradient_text = gradient::two_color("Two color gradient", RED, ORANGE);
print::line(&gradient_text);

```

#### Three-color gradient:
```rust
use modcli::output::{
    gradient,
    print,
    BLUE, GREEN, YELLOW
};

let gradient_text = gradient::three_color("Three color gradient", BLUE, GREEN, YELLOW);
print::line(&gradient_text);

```

#### Multi-color gradient:
```rust
use modcli::output::{
    gradient,
    print,
    RED, ORANGE, YELLOW, GREEN, BLUE
};

let gradient_text = gradient::multi_color("Multi-color gradient", vec![RED, ORANGE, YELLOW, GREEN, BLUE]);
print::line(&gradient_text);

```

#### Using RGB with gradients:
```rust
use modcli::output::{
    gradient,
    print
};

let gradient_text = gradient::two_color(
    "Gradient Output", 
    Color::Rgb { r: 255, g: 0, b: 0 },
    Color::Rgb { r: 0, g: 0, b: 255 },
);
print::line(&gradient_text);

```

### Themes

<!-- Optional screenshot/GIF. Replace the src with your asset path. -->
<!-- <img src="docs/media/themes-demo.gif" alt="Themes demo" width="600" /> -->

```rust
use modcli::output::{print, themes};

// Apply a theme (changes terminal fg/bg until reset)
themes::apply_theme("blue");
print::line("Applied theme: blue");

// Read colors for log categories from current theme
let t = themes::current_theme();
let categories = ["error","warn","success","info","debug","trace","notice","status"]; 
for key in categories {
    let color = t.get_log_color(key);
    println!("{key}: {:?}", color);
}

// Reset at the end
themes::Theme::reset();
```

---

### Output Styles
```rust
use modcli::output::{
    print,
    build,
    BLUE
};

 // 📦 Progress Bar Demo
 let testing = build()
        .part("Testing")
        .color(BLUE)
        .bold()
        .get();

print::line(&testing);

// Outputs "Testing" in bold/blue.
```

#### Multiple Styles:
```rust
use modcli::output::{
    gradient,
    print,
    build,
    BLUE, LIGHT_BLUE
};

 // 📦 Progress Bar Demo
 let testing = build()
        .part("Label:").color(BLUE).bold().space()
        .part("This content has").space()
        .part("multiple").color(LIGHT_BLUE).bold().space()
        .part("styles").underline().space()
        .part("and").italic().space()
        .part("colors").underline().space()
        .part("!")
        .get();

print::line(&testing);
```

#### Style + Gradients:
```rust
use modcli::output::{
    print,
    build,
    BLUE, GREEN
};

let gradient_text = gradient::two_color("Gradient Output", BLUE, GREEN);
let testing = build()
        .part(&gradient_text).bold().space()
        .part("+ Styled!")
        .get();

print::line(&testing);

```

---

### Progress Bar &amp; Animated Loaders

#### Auto Progress:
```rust
use modcli::output::{
    progress::{
        show_progress_bar, 
    }
};

show_progress_bar("Testing", 45, 1500);
```

> Displays
```bash
Label [#############################################] 100% Done!
```

#### Manual control:
```rust
use modcli::output::{
    build,
    progress::{
        ProgressBar, 
        ProgressStyle,
    },
    LIGHT_BLUE
};

 // Progress Bar Demo
 let label = build()
    .part("Loading")
    .color(LIGHT_BLUE)
    .bold()
    .get();

let mut bar = ProgressBar::new(30, ProgressStyle {
    fill: '■',
    done_label: "Complete!",
    color: Some(LIGHT_BLUE),
    ..Default::default()
});

bar.set_label(&label);
bar.start_auto(2000); // auto-fill in 2 seconds

```

#### Manual .tick() control (like during a loop):
```rust
use std::time::Duration;
use modcli::output::{
    progress::{
        ProgressBar, 
        ProgressStyle
    },
    ORANGE
};
use modcli::console::run_shell;


 let mut bar = ProgressBar::new(10, ProgressStyle {
    fill: '■',
    done_label: "Done!",
    color: Some(ORANGE),
    ..Default::default()
});
 bar.set_label("Syncing");
 
 for _ in 0..10 {
     bar.tick();
     std::thread::sleep(Duration::from_millis(200));
 }
 println!(" {}", bar.style.done_label);

```

#### Animated Spinner (Loading/Waiting):
```rust
use modcli::output::{
    progress::{
        show_spinner
    }
};
show_spinner("Loading", 20, 100);
```

#### Animated Percentage Loader:
```rust
use std::thread::sleep;
use std::time::Duration;
use modcli::output::{
    progress::{
        show_percent_progress
    }
};

for i in (0..=100).step_by(10) {
    show_percent_progress("Loading", i);
    sleep(Duration::from_millis(100));
}
println!();
```
---

### Tables

#### Table Example: Flex Width, Heavy Borders
```rust
use modcli::output::table::{render_table, TableMode, TableStyle};

let headers = ["Name", "Age", "Role"];
let rows = vec![
    vec!["Alice", "29", "Engineer"],
    vec!["Bob", "35", "Manager"],
    vec!["Charlie", "41", "CTO"],
];

render_table(&headers, &rows, TableMode::Flex, TableStyle::Heavy);
```

> Outputs
```bash
┏━━━━━━━━┳━━━━━━━━┳━━━━━━━━┓
┃Name    ┃Age     ┃Role    ┃
┣━━━━━━━━╋━━━━━━━━╋━━━━━━━━┫
┃Alice   ┃29      ┃Engineer┃
┃Bob     ┃35      ┃Manager ┃
┃Charlie ┃41      ┃CTO     ┃
┗━━━━━━━━┻━━━━━━━━┻━━━━━━━━┛

```

#### Table Example: Fixed Width, Rounded Borders
```rust
use crate::output::table::{render_table, TableMode, TableStyle};

let headers = ["Name", "Age", "Role"];
let rows = vec![
    vec!["Alice", "29", "Engineer"],
    vec!["Bob", "35", "Manager"],
    vec!["Charlie", "41", "CTO"],
];

render_table(&headers, &rows, TableMode::Fixed(15), TableStyle::Rounded);
```
> Outputs
```bash
╭───────────────┬───────────────┬───────────────╮
│Name           │Age            │Role           │
├───────────────┼───────────────┼───────────────┤
│Alice          │29             │Engineer       │
│Bob            │35             │Manager        │
│Charlie        │41             │CTO            │
╰───────────────┴───────────────┴───────────────╯
```

#### Table Example: Fixed Width, Ascii Borders
```rust
use crate::output::table::{render_table, TableMode, TableStyle};

let headers = ["Name", "Age", "Role"];
let rows = vec![
    vec!["Alice", "29", "Engineer"],
    vec!["Bob", "35", "Manager"],
    vec!["Charlie", "41", "CTO"],
];

render_table(&headers, &rows, TableMode::Fixed(15), TableStyle::Ascii);
```
> Outputs
```bash
+---------------+---------------+---------------+
|Name           |Age            |Role           |
+---------------+---------------+---------------+
|Alice          |29             |Engineer       |
|Bob            |35             |Manager        |
|Charlie        |41             |CTO            |
+---------------+---------------+---------------+
```

<br><br>


<h2>Creating Custom Commands</h2>

### File Structure
```
my_project/
├── src/
│   ├── commands/
│   │   └── greet.rs      ← define `GreetCommand` here
```
> Create a commands folder in src/, then put the command in its own file:


### Custom Command File
```rust
use modcli::command::Command;

pub struct GreetCommand;

impl Command for GreetCommand {
    fn name(&self) -> &str {
        "greet"
    }

    fn aliases(&self) -> &[&str] {
        &["hi"]
    }

    fn help(&self) -> Option<&str> {
        Some("Greets the user.")
    }

    fn validate(&self, _args: &[String]) -> Result<(), String> {
        Ok(())
    }

    fn execute(&self, _args: &[String]) {
        println!("Greetings!");
    }
}
```
> greet.rs


### Register your command in `main.rs`, `tool.rs`, etc.
```rust
mod commands;
use modcli::ModCli;
use commands::greet::GreetCommand;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cli = ModCli::new();

    // Register function
    cli.registry.register(Box::new(GreetCommand));


    cli.run(args);
}
```


#### Test Command
```sh
$ myCLI greet
Greetings!

$ myCLI help
List of available commands...
```

<br><br>

<h2>Interactive Shell</h3>

### ModCLI supports an interactive console mode (like a REPL):
```rust
use modcli::config::CliConfig;
use modcli::console::run_shell;

fn main() {
    let config = CliConfig::load(None);
    run_shell(&config);
}
```


### Adding a custom console command for `shell`: 
```rust
use modcli::ModCli;
use modcli::shell_commands::{register, ShellCommand};

fn greet_handler(_input: &str) -> bool {
    println!("👋 Hello from shell command!");
    true
}

fn main() {
    register(ShellCommand {
        name: "greet",
        aliases: &["hi", "wave"],
        help: "Greets the user with a friendly hello",
        handler: greet_handler,
    });
}
```

#### Config File Example (config.json)
```json
{
  "modcli": {
        "name"  : "mod-cli",
        "prefix": "mod",
        "banner": "Welcome to ModCLI",
        "delay" : 0,
        "theme" : "default",
        "strict": false,
        "force_shell": false,
        "shell": {
            "prompt": "Tool >",
            "welcome": ["Welcome to the console."],
            "goodbye": ["Bye!"]
        },
        "messages": {
            "no_command": "No command provided.",
            "not_found": "Command not found."
        }
  }
}
```
> Default location: `project_root/config.json` 

#### Manually set the config path (if not project root)
```rust
use modcli::config;

fn main() {
    config::set_path(("my/custom/config.json");

```


<br><hr><br>

> [!WARNING]
> **Pre-release**: This project is in active development.
> The core is stable but features are evolving. Production use is possible, 
> but interfaces may still evolve until 1.0.


<br>
<!--
:: LICENSE
============================================================================ -->
<div id="license">
    <hr>
    <h2>📌 License</h2>
    <p>Licensed under the <b>Apache License</b>, version 2.0 (the <b>"License"</b>); you may not use this software, including, but not limited to the source code, media files, ideas, techniques, or any other associated property or concept belonging to, associated with, or otherwise packaged with this software except in compliance with the <b>License</b>.</p>
    <p>You may obtain a copy of the <b>License</b> at: <a href="http://www.apache.org/licenses/LICENSE-2.0" title="Apache-2.0 License" target="_blank">http://www.apache.org/licenses/LICENSE-2.0</a>.</p>
    <p>Unless required by applicable law or agreed to in writing, software distributed under the <b>License</b> is distributed on an "<b>AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND</b>, either express or implied.</p>
    <p>See the <a href="./LICENSE" title="Software License file">LICENSE</a> file included with this project for the specific language governing permissions and limitations under the <b>License</b>.</p>
    <br>
</div>





<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
