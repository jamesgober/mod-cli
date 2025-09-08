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
- **[Output Styles & Colors](#colors-styles)**
- **[Themes](#themes)**
- **[Hyperlinks (OSC 8)](#hyperlinks-osc-8)**
- **[Gradients](#gradients-feature-gradients)**
  - **[Gradient Palettes & Easing](#gradient-palettes--easing)**
- **[Layouts](#layouts-feature-layouts)**
- **[Tables](#tables-presets-feature-table-presets)**
  - **[Alignment and Truncation](#tables-alignment-and-truncation)**
  - **[Styled header + colorized zebra](#tables-styled-header--colorized-zebra)**
  - **[Per-column widths](#tables-per-column-widths-fixed--percent--auto)**
  - **[Exporters (Markdown/CSV/JSON)](#tables-exporters-markdown--csv--json)**
  - **[Exporters to files](#tables-exporters-to-files)**
- **[Progress](#progress-presets-feature-progress-presets)**
  - **[Bytes / Rate / ETA](#progress-bytes-rate-eta)**
  - **[MultiProgress](#multiprogress-stacked-bars)**
 - **[Inputs & Menus](#inputs--menus)**
  - **[Forms](#forms)**
 - **[Completions & Man Pages](#completions--man-pages)**
 - **[Examples](#examples)**
 - **[Startup Banner](#startup-banner)**
 - **[Gated & Nested Commands](#gated--nested-commands)**
  - **[Help Templates & Markdown](#help-templates--markdown)**
  - **[i18n Bundles (JSON)](#i18n-bundles-json)**
  - **[Keybinding Customization](#keybinding-customization)**
  - **[Hooks & Error Formatting](#hooks--error-formatting)**

<hr>
<br>
<h2 id="installation">Installation</h2>

<br>


### Install Manually
Add this to your `Cargo.toml`:
```toml
[dependencies]
mod-cli = "0.6.0"
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
| `tracing-logs`       |  off    | Emits `tracing` events via `output::hook` alongside themed console output. |
| `dispatch-cache`     |  off    | Optional single-entry dispatch cache to speed repeated command invocations. |
| `gradients`          |  off    | Adds named gradient helpers that wrap 24-bit RGB gradients (no new deps). |
| `layouts`            |  off    | Adds a lightweight layout engine for composing rows/columns in the terminal. |
| `table-presets`      |  off    | Adds convenience presets for `TableStyle` (ASCII, Rounded, Heavy). |
| `progress-presets`   |  off    | Adds convenience constructors for `ProgressStyle` (compact, heavy). |


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
<h2 id="colors-styles">Output Styles & Colors</h2>
<br>


### Colors (named and RGB)

```rust
use crossterm::style::Color;
use modcli::output::{build, colors, print};

let teal = colors::get("teal");            // named color helper
let neon = Color::Rgb { r: 57, g: 255, b: 20 }; // raw 24-bit RGB

let s = build()
    .part("Named:").space().part("teal").color(teal).space()
    .part("RGB:").space().part("neon").color(neon)
    .get();
print::line(&s);
```

### Themes

```rust
use modcli::output::{print, themes};

// Apply a theme (changes terminal fg/bg until reset)
themes::apply_theme("blue");
print::line("Applied theme: blue");

// Read colors for log categories from current theme
let t = themes::current_theme();
let error_color = t.get_log_color("error");
print::line(&format!("error color: {:?}", error_color));

// Reset at the end
themes::Theme::reset();
```

### Hyperlinks (OSC 8)

Clickable hyperlinks via OSC 8 sequences with automatic fallback.

- By default, prints `text (url)` for compatibility.
- Set `ENABLE_OSC8=true` to emit OSC 8 sequences in supported terminals.

```rust
use modcli::output::print;

// Fallback: prints "mod-cli docs (https://docs.rs/mod-cli)" unless ENABLE_OSC8=true
print::link("mod-cli docs", "https://docs.rs/mod-cli");
print::link("GitHub: jamesgober/mod-cli", "https://github.com/jamesgober/mod-cli");
```

### Gradients (feature: `gradients`)

```rust
use crossterm::style::Color;
use modcli::output::{gradient, print};

// Two-color (raw RGB)
let g1 = gradient::two_color(
    "Deploying…",
    Color::Rgb { r: 0, g: 200, b: 200 },
    Color::Rgb { r: 190, g: 0, b: 255 },
);
print::line(&g1);

// Named helpers (requires: features = ["gradients"]) 
#[cfg(feature = "gradients")]
{
    use modcli::output::gradient_extras;
    let g2 = gradient_extras::two_named("Deploying…", "teal", "violet");
    let g3 = gradient_extras::multi_named("Rainbow", &["red","orange","yellow","green","blue","violet"]);
    print::line(&g2);
    print::line(&g3);
}
```

### Layouts (feature: `layouts`)

```rust
#[cfg(feature = "layouts")]
{
    use modcli::output::{layout, print};

    let left = vec!["Status".into(), "OK".into(), "".into(), "Uptime: 12m".into()];
    let right = vec![
        "Logs:".into(),
        "[INFO] init ok".into(),
        "[WARN] retry net".into(),
        "[OK]   ready".into(),
    ];

    let lay = layout::build()
        .row()
            .col_percent(35).content(left)
            .col_auto().content(right)
        .end_row()
        .hgap(2).vgap(1).border(true)
        .finish();

    let s = layout::render(&lay);
    print::line(&s);
}
```

### Tables (presets; feature: `table-presets`)

```rust
use modcli::output::table::{render_table, TableMode, TableStyle};

let headers = ["Name", "Role", "Team"];
let rows = vec![
  vec!["Ada", "Engineer", "Core"],
  vec!["Linus", "Maintainer", "Kernel"],
];

let t_heavy = render_table(&headers, &rows, TableMode::Flex, TableStyle::Heavy);

#[cfg(feature = "table-presets")]
{
  let t_ascii = render_table(&headers, &rows, TableMode::Fixed(14), TableStyle::ascii_preset());
  let t_round = render_table(&headers, &rows, TableMode::Fixed(14), TableStyle::rounded_preset());
}

### Tables: Alignment and Truncation

```rust
use modcli::output::table::{render_table_with, Align, TableMode, TableStyle, TruncateMode};

let headers = ["Name", "Score", "Notes"];
let rows = vec![
    vec!["Alice", "98", "Top performer — consistent results"],
    vec!["Bob", "7", "Needs improvement on long-form tasks"],
    vec!["Carol", "1234", "Extremely long numeric score to show truncation"],
];

let aligns = [Align::Left, Align::Right, Align::Center];
let truncs = [TruncateMode::End, TruncateMode::Start, TruncateMode::Middle];

let s = render_table_with(&headers, &rows, TableMode::Fixed(14), TableStyle::Rounded, Some(&aligns), Some(&truncs));
println!("{}", s);
```

### Tables: Styled header + colorized zebra

```rust
use modcli::output::table::{render_table_with_opts_styled, Align, TableMode, TableStyle, TruncateMode};

let headers = ["ID", "User", "Comment"];
let rows = vec![
  vec!["1", "alice", "Short"],
  vec!["2", "bob", "This is a long comment that will wrap or truncate"],
  vec!["3", "carol", "Another one"],
];
let aligns = [Align::Right, Align::Left, Align::Left];
let truncs = [TruncateMode::End, TruncateMode::End, TruncateMode::End];

let s = render_table_with_opts_styled(
    &headers, &rows, TableMode::Fixed(24), TableStyle::Rounded,
    Some(&aligns), Some(&truncs),
    true,  /* zebra */
    true,  /* row separators */
    Some(modcli::output::CYAN),           /* header color */
    Some(modcli::output::DARK_BLUE),      /* zebra bg */
);
println!("{}", s);
```

### Tables: Per-column widths (Fixed / Percent / Auto)

```rust
use modcli::output::table::{render_table_with_columns, Align, ColWidth, TableStyle, TruncateMode};

let headers = ["Name", "Team", "Notes"];
let rows = vec![
    vec!["Ada", "Core", "Long note to demo widths and truncation"],
    vec!["Linus", "Kernel", "Maintainer"],
];

let cols = [ColWidth::Fixed(12), ColWidth::Percent(20), ColWidth::Auto];
let aligns = [Align::Left, Align::Center, Align::Left];
let truncs = [TruncateMode::End, TruncateMode::End, TruncateMode::Middle];

let s = render_table_with_columns(&headers, &rows, TableStyle::Rounded, &cols, Some(&aligns), Some(&truncs), true, true);
println!("{}", s);
```

### Tables: Exporters (Markdown / CSV / JSON)

```rust
use modcli::output::table::{render_table_markdown, render_table_csv, render_table_json};

let headers = ["Name", "Role"];
let rows = vec![ vec!["Alice","Engineer"], vec!["Bob","Designer"] ];

let md = render_table_markdown(&headers, &rows);
let csv = render_table_csv(&headers, &rows);
let json = render_table_json(&headers, &rows);

println!("Markdown:\n{}", md);
println!("CSV:\n{}", csv);
println!("JSON:\n{}", json);
```

### Progress presets (feature: `progress-presets`)

```rust
use modcli::output::progress::{ProgressBar, ProgressStyle};

let mut bar = ProgressBar::new(20, ProgressStyle::default());
#[cfg(feature = "progress-presets")]
{
    bar.style = ProgressStyle::compact();
}
bar.set_label("Syncing");
bar.start_auto(1000);
```









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
