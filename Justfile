# Justfile: quick commands to run examples with proper features

set shell := ["bash", "-cu"]

# Run all non-feature-gated examples
all-basic:
	cargo run --example style_builder
	cargo run --example table_basic
	cargo run --example table_align
	cargo run --example table_separators
	cargo run --example table_colors
	cargo run --example progress_basic
	cargo run --example progress_custom
	cargo run --example gradient_multi
	cargo run --example themes_demo

# Run feature-gated examples for gradients, layouts, and presets
all-features:
	cargo run --example gradient_two --features gradients
	cargo run --example gradient_multi --features gradients
	cargo run --example layout_demo --features layouts
	cargo run --example table_presets --features table-presets
	cargo run --example progress_presets --features progress-presets

# Convenience single-target recipes
gradient-two:
	cargo run --example gradient_two --features gradients

gradient-multi:
	cargo run --example gradient_multi --features gradients

layouts:
	cargo run --example layout_demo --features layouts

table-presets:
	cargo run --example table_presets --features table-presets

table-align:
	cargo run --example table_align

table-separators:
	cargo run --example table_separators

progress-presets:
	cargo run --example progress_presets --features progress-presets

table-colors:
	cargo run --example table_colors

themes-load:
	cargo run --example themes_load --features theme-config

# Generate completion scripts into target/completions
completions:
	mkdir -p target/completions
	cargo run --example gen_completions -- bash > target/completions/modcli.bash
	cargo run --example gen_completions -- zsh > target/completions/_modcli
	cargo run --example gen_completions -- fish > target/completions/modcli.fish

# Generate man page into target/man
manpages:
	mkdir -p target/man
	cargo run --example gen_man > target/man/modcli.1

# Install completions (may need sudo depending on your setup)
install-completions:
	@echo "Installing bash completion to /usr/local/etc/bash_completion.d/modcli.bash (may require sudo)"
	mkdir -p /usr/local/etc/bash_completion.d || true
	cp target/completions/modcli.bash /usr/local/etc/bash_completion.d/
	@echo "Installing zsh completion to /usr/local/share/zsh/site-functions/_modcli"
	mkdir -p /usr/local/share/zsh/site-functions || true
	cp target/completions/_modcli /usr/local/share/zsh/site-functions/
	@echo "Installing fish completion to ~/.config/fish/completions/modcli.fish"
	mkdir -p ~/.config/fish/completions
	cp target/completions/modcli.fish ~/.config/fish/completions/

# Install man page (may require sudo)
install-manpages:
	@echo "Installing man page to /usr/local/share/man/man1/modcli.1"
	mkdir -p /usr/local/share/man/man1 || true
	cp target/man/modcli.1 /usr/local/share/man/man1/

# Open API docs (macOS)
docs-open:
	open docs/API.md

# Open API docs to Forms section (attempts file URL anchor)
docs-open-forms:
	open "file://$(pwd)/docs/API.md#forms"

# Examples: Help (Markdown) and Messages
help-markdown:
	cargo run --example help_markdown_demo

messages-demo:
	cargo run --example messages_demo

messages-json:
	cargo run --example messages_json_demo --features theme-config

# Example: Keymap demo
menu-keymap:
	cargo run --example menu_keymap_demo
