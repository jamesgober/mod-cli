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

### Changed
- Core simplified for performance and security. Removed runtime configuration and plugin systems from the core crate; direct code configuration is now the default and only model. See `modcli/src/lib.rs` and `modcli/src/loader.rs`.
- Table renderer now returns a `String` for composability and testability instead of printing directly. See `modcli/src/output/table.rs`.
- Parser improvements for correctness and micro-optimizations: correct handling of escaped quotes and backslashes in and out of quoted segments; empty quoted segments now emit empty args. See `modcli/src/parser.rs`.
- Unify user messaging via `output::hook` (status/error/unknown) in binaries and registry execution paths.
- Binaries simplified to direct dispatch without config or shell. See `modcli/bin/tool.rs` and `modcli/bin/modcli.rs`.
- Integration tests relocated from repository `tests/` to `modcli/tests/` so they run with the crate. Problematic tests that depended on removed features were disabled under `modcli/tests_disabled/`.
- Dependencies trimmed and versions aligned for lean builds: `terminal_size = 0.3`, `once_cell = 1.19`, `rpassword = 7.3.x`. Removed heavy/unused deps.

### Removed
- Removed runtime native plugin loading from core; deleted plugin loader module and feature flags. Files archived under `modcli/attic/loader/plugins.rs`.
- Removed JSON loader and config suite from core; deleted modules and feature flags. Files archived under `modcli/attic/`.
- Removed internal shell command and shell runtime from core to eliminate dependency on config. Files archived under `modcli/attic/`.
- Removed examples that referenced JSON/plugins (`examples/full_app.rs`, `examples/commands.json`) from the build; archived under `modcli/attic/examples/`.

### Fixed
- Parser edge cases that previously split tokens incorrectly across nested quotes and escaped spaces now pass comprehensive tests in `modcli/tests/parser_edge_tests.rs` and `modcli/tests/parser_tests.rs`.
- Binaries now emit consistent, themed output for no-args and error paths and exit with appropriate codes.

### Added
- Introduced structured error variants in `modcli/src/error.rs`:
  - `ConfigParse(serde_json::Error)` for JSON configuration parsing failures.
  - `InvalidUsage(String)` to represent argument/validation issues from commands.
  - `UnknownCommand(String)` for unresolved commands after prefix/alias routing.
- Added `CommandRegistry::try_execute()` returning `Result<(), ModCliError>` for programmatic error handling while keeping existing `execute()` user-facing behavior.
- Minor performance hinting: added `#[inline(always)]` to hot-path helpers in `modcli/src/output/gradient.rs`.
- Optional diagnostics: added `tracing-logs` feature with `tracing` + `tracing-subscriber` integration via `output::hook` (emits tracing events alongside themed console output when enabled).
- Added regression tests in `tests/error_regressions.rs` covering unknown command, invalid usage, and JSON loader failure modes.
- Added parser edge-case tests in `tests/parser_edge_tests.rs` for nested quotes and escaped backslashes/spaces.
- Added shell extensions tests in `tests/shell_extensions_tests.rs` and print tests in `tests/print_tests.rs`.
- Added rustdoc examples for `parser::parse_line()` and loader APIs (`execute`, `try_execute`).
- Docs: Added "Using Features", "Contributing Performance", and tracing initialization example in `docs/README.md`.
- Docs: Added "Error Code Mapping" section for `bin/modcli.rs` exit codes.
- Benchmarks: Added `parser_bench.rs` for `parser::parse_line()` (simple/quoted/escaped cases).
- Benchmarks: Added `registry_try_execute_bench.rs` for dispatch (`try_execute`) across name/alias/prefix cases.

### Changed
- `modcli/src/config.rs::parse()` now preserves original `serde_json` errors via `ConfigParse` instead of stringifying, improving diagnostics.
- `modcli/src/loader/plugins.rs` now uses themed logging (`hook::warn`) instead of `eprintln!` for consistent, centralized messaging.
- `modcli/src/loader.rs` `execute()` now delegates to `try_execute()` and logs structured errors uniformly via `output::hook`.
- Module declarations normalized in `modcli/src/lib.rs` (moved `pub mod error;` to group with other modules).
- `modcli/bin/modcli.rs` now uses `CommandRegistry::try_execute()` and maps failures to non-zero exit codes (usage → `2`, unknown command → `127`, other errors → `1`).
- `modcli/src/loader/plugins.rs` only attempts to load regular files (skips directories/symlinks) to avoid erroneous plugin loads.
- `modcli/src/console.rs` shell input parsing now uses robust `parser::parse_line()` (supports quotes/escaping).
- `modcli/src/output/input/menu.rs` hardened to avoid unwraps and handle terminal errors gracefully.
- `modcli/src/output/progress.rs` made resilient to stdout flush errors (no unwraps), logs via hooks.
- `modcli/src/shell_commands.rs` now handles poisoned mutex locks gracefully with warnings instead of panicking.

### Fixed
- Removed panicking `expect()` calls in `modcli/src/loader/sources.rs`; I/O/JSON errors are now logged via `hook::error` and fail gracefully (no crash), returning an empty command list.
- Hardened CLI exit codes in `modcli/bin/tool.rs`:
  - Strict mode violations now exit with code `2`.
  - Missing shell config maps to exit code `2`; other shell errors map to `1`.
- Resolved duplicated `#[cfg(feature = "custom-commands")]` attribute in `modcli/src/loader.rs` flagged by clippy.
- Removed remaining unwraps in interactive UI paths (menu, progress), ensuring no runtime panics in production paths.



<br>


## [0.5.3] - 2025-09-07 
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
[Unreleased]: https://github.com/jamesgober/mod-cli/compare/v0.5.3...HEAD
[0.5.3]: https://github.com/jamesgober/mod-cli/compare/v0.5.0...v0.5.3
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