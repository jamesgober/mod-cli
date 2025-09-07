<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>CHANGELOG</b>
</h1>
<div align="center">
    This document contains a curated, chronologically ordered list of all notable changes for each version and/or release of this project. 
    <br>
    The format of this changelog is based on <a href="https://keepachangelog.com/en/1.1.0/">Keep a Changelog</a>.
    <br><br><br>
</div>

## [Unreleased]


### Added
- Added `docs/README.md` section.
- Added `docs/API.md` section.
- Added `docs/GUIDELINES.md` section.
- Added alias matching in command registry (registers aliases and resolves them at runtime).
- Added `json-loader` feature flag to gate JSON command source loading.
- Added tests for alias resolution and validation guard in `CommandRegistry`.
- Added example plugin crate: `modcli/examples/plugins/hello-plugin`.
- Added GitHub Actions CI workflow (Linux/macOS) with matrix build/test, plugin smoke test, and `json-loader` feature tests.
- Added MSRV (1.74.0) to CI matrix and enabled cargo caches for faster builds.
- Added `cargo-audit` scheduled workflow.
- Added `cargo-deny` workflow for license/advisory checks.
- Added `examples/full_app.rs` demonstrating custom command, prefix routing, gradients, JSON loader, and plugin loading.
- Added Criterion benchmarks for output builder, gradient generation, and table rendering.


### Changed
- Reformatted CHANGELOG.md.
- Refactored `CommandRegistry` to include an alias map for efficient alias resolution.
- Plugin loader now supports platform-specific dynamic libraries: `.so` (Linux/Unix), `.dylib` (macOS), `.dll` (Windows).
- README feature table updated to include `json-loader`.
- Removed unused `clap` dependency to reduce bloat.
- Removed unused `interactive` feature flag from `modcli/Cargo.toml`.
- Unified input re-exports via `modcli::input::*` for a single ergonomic API surface.
- Table renderer now truncates long cell content with an ellipsis and pads to column width.
- Added crate-level and API rustdoc for `modcli::ModCli` and related items.
- Removed `help` special-casing from `CommandRegistry` dispatcher; commands now support context-aware execution via `execute_with(&self, args, &CommandRegistry)` and `HelpCommand` owns its behavior.
- Introduced owned configuration loading: `CliConfig::load_owned(path)` and `ModCli::with_owned_config(cfg)` to avoid global singletons in library/test usage; existing global loader retained for backward compatibility in the binary.

### Fixed
- Validation no longer executes the command when `validate()` returns an error (prevents side-effects on invalid input).







<br>


<!-- 0.5.0 | CLI Framework
============================================
Enhanced design, interactive shell, and functionality.
============================================ -->
## [0.5.0] - 2025-04-30 

### Added
- Added interactive console via `shell`.
- Added `shell` start command.
- Added Internal `shell` commands.
- Added Custom `shell` Command Support.
- Added newline method to Output/Print.
- Added `set_path` support for custom config.
- Added config designated paths (default).
- Added concrete *embedded config* settings at compile-time.
- Added `force_shell` setting.

### Changed
- Refactored config module.
- Refactored example config file.
- Updated config format.
- Refactored ModCli Class.

### Removed
- Command: `echo`.
- Command: `benchmark`.
- Removed  `Bin/Shell`.
- Removed  `Output/Input/Console`.
- Removed empty &amp; unused files.


<hr><br><br>

<!-- 0.4.0 | CLI Framework
============================================
Enhanced Output, Visual Improvements, Test Ready.
============================================ -->
## [0.4.0] - 2025-04-30 

### Added
- Added `three color` and `multi-color` functionality to output/gradient.
- Added `output/colors` module.
- Added `output/style` module.
- Added custom "no command" message.

### Changed
- Refactored `output/gradient` module.
- Refactored `output/themes` module.
- Refactored `output/print` module.
- Refactored `output/hooks` module.
- Changed `output/hooks` to `output/hook`.
- Refactored `output/input/console` module.
- Refactored `output/progress` module.
- Refactored `output/table` module.

### Deprecated
- Module: `color_picker.rs` replaced by `colors.rs`.
  - `list_named_colors()`
  - `print_color_swatch()`
  - `get_color_by_name()`


<hr><br><br>

<!-- 0.3.8 | Functional CLI With Styles
============================================
Styles, Colors, Gradients, Themes, Animations, Custom Commands
============================================ -->
## [0.3.8] - 2025-04-29 

### Added
- Added support for terminal colors and text styling options.
- Standardized logging color constants (`COLOR_SUCCESS`, etc.)
- CLI theme management via `apply_theme()` for terminal foreground/background styling.
- Output hook system (`print_info`, `print_warn`, `print_error`, etc.)
- CLI commands can now display standardized status messages.
- Gradient printing system with RGB interpolation.
- Animated CLI progress utilities (progress bar, spinner, percent).
- CLI Input builder (forms, secure prompt, interactive menus).
- Added support for custom commands registration and handling.
- Added `Console Mode` support via `Shell`, enabling persistent interactive CLI sessions.

### Changed
- Refactored output into modular format.
- Improved theme system prep with internal constants.

<hr><br><br>


<!-- 0.3.0 | Structural Foundation
============================================
Config, plugin system, tables, etc
============================================ -->
## [0.3.0] - 2025-04-13 

### Added
- External plugin system via `PluginLoader` (behind `plugins` feature flag).
- CLI output theme engine with support for dark/light/custom styles.
- Basic table rendering system (auto-width, adaptive layout).
- `print_multiline()` utility for styled multi-line output with optional delay.
- Reading CLI behavior config via `config.json`.
- CLI startup banners (defined in config).
- Performance benchmarking support

### Changed
- CLI registry to no longer require mut (technically internal).


<hr><br><br>


<!-- 0.2.0 | Command Structure
============================================
Custom commanda, Version, Help, etc.
============================================-->
## [0.2.0] - 2025-04-10 

- `--version` flag for app versioning.
- Hidden `--modcli` flag for framework internals.
- Built-in help command with usage guidance.
- Command argument validation system.
- JSON-based CommandSource loader support.
- Name, alias, and description loading via JSON input
- CLI configuration loader with `CliConfig` struct
- Supports loading CLI settings from a JSON file (*examples/config.json*)
Fields include:
  - `theme`: Optional string to represent CLI theme (e.g., "monochrome", "default").
  - `strict_args`: Optional boolean to enable strict argument checking.
  - `banner`: Optional string to display a CLI welcome message.
  - > Note: Settings are parsed but not yet applied in runtime logic. Behavioral integration will be introduced in a future release.
- Support for `hidden` field on commands to hide them from help output.

### Changed
- CLI runner to no longer require mut (technically internal).


<br>

<!-- 0.1.0
============================================
Initial Commit
============================================-->
## [0.1.0] - 2025-04-10 

- Initial commit &amp; first build.



<!--
 PRE-RELEASE =========================================================== -->
[Unreleased]: https://github.com/jamesgober/mod-cli/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/jamesgober/mod-cli/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/jamesgober/mod-cli/compare/v0.3.8...v0.4.0
[0.3.8]: https://github.com/jamesgober/mod-cli/compare/v0.3.0...v0.3.8
[0.3.0]: https://github.com/jamesgober/mod-cli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jamesgober/mod-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jamesgober/mod-cli/compare/v0.1.0...HEAD


<!--#######################################################################################################
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>