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
- **[JSON Command Source](#json-command)**
- **[Examples](#examples)**
- **[Getting Started](#getting-started)**
  - **[Basic Usage](#basic-usage)**
  - **[Set a custom prefix](#set-custom-prefix)**
  - **[Using named colors](#using-named-colors)**
- **[Using Gradients](#using-gradients)**
  - **[Two-color gradient](#two-color-gradient)**
  - **[Three-color gradient](#three-color-gradient)**
  - **[Multi-color gradient](#multi-color-gradient)**
  - **[Using RGB with gradients](#using-rgb-with-gradients)**




<br><br>
<h2 id="installation">Installation</h2>

### Default Installation

#### Install Manually

Add this to your `Cargo.toml`:
```toml
[dependencies]
mod-cli = "0.5.0"
```

<br>

#### Install via Terminal
```bash
# Basic installation
cargo add mod-cli
```


<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="feature-flags">Feature Flags</h2>

| Feature               | Description                                           |
|------------------------|-------------------------------------------------------|
| `internal-commands`    | Enables built-in test/dev commands like `ping`, `hello` |
| `custom-commands`      | Enables CLI custom command creation.                  |
| `json-loader`          | Enables external command loading from JSON config     |
| `plugins`              | Enables plugin support for dynamic runtime command injection |



<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="examples">Examples</h2>



<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="json-command">JSON Command Source</h2>






<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="examples">Examples</h2>






<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="getting-started">Getting Started</h2>


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

<br>

<h2 id="using-named-colors">Using named colors</h2>

```rust
 let teal = colors::get("teal"); // always returns a Color (or fallback)
 let demo = build()
     .part("Color Demo:").space()
     .part("Teal").color(teal).bold().get();

 print::line(&demo, 0);

```


<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="using-gradients">Using Gradients</h2>

<br>
<h2 id="two-color-gradient">Two-color gradient</h2>

```rust
use modcli::output::{
    gradient,
    print,
    RED, ORANGE
};

let gradient_text = gradient::two_color("Two color gradient", RED, ORANGE);
print::line(&gradient_text);

```

<br>
<h2 id="three-color-gradient">Three-color gradient</h2>

```rust
use modcli::output::{
    gradient,
    print,
    BLUE, GREEN, YELLOW
};

let gradient_text = gradient::three_color("Three color gradient", BLUE, GREEN, YELLOW);
print::line(&gradient_text);

```

<br>
<h2 id="multi-color-gradient">Multi-color gradient</h2>

```rust
use modcli::output::{
    gradient,
    print,
    RED, ORANGE, YELLOW, GREEN, BLUE
};

let gradient_text = gradient::multi_color("Multi-color gradient", vec![RED, ORANGE, YELLOW, GREEN, BLUE]);
print::line(&gradient_text);

```

<br>
<h2 id="using-rgb-with-gradients">Using RGB with gradients</h2>

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





<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
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
