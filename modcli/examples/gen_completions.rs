use std::env;

// Minimal, generic completion scripts that at least wire the command name.
// Users can extend these, or we can later auto-generate from a registry.
fn bash() -> &'static str {
    r#"# bash completion for modcli (generic)
_modcli_completions() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Basic fallback: complete files when appropriate
    case "$prev" in
        -h|--help|-V|--version)
            COMPREPLY=()
            return 0
            ;;
    esac

    # No subcommand tree yet; default to file completion
    COMPREPLY=( $(compgen -f -- "$cur") )
}
complete -F _modcli_completions modcli
"#
}

fn zsh() -> &'static str {
    r#"#compdef modcli
# zsh completion for modcli (generic)
local -a _modcli_commands

# No command tree yet; fallback to files
_arguments '*:file:_files'
"#
}

fn fish() -> &'static str {
    r#"# fish completion for modcli (generic)
complete -c modcli -f
"#
}

fn main() {
    let out = env::args().nth(1).unwrap_or_default();
    match out.as_str() {
        "bash" => print!("{}", bash()),
        "zsh" => print!("{}", zsh()),
        "fish" => print!("{}", fish()),
        _ => {
            eprintln!("usage: gen_completions <bash|zsh|fish>");
            std::process::exit(2);
        }
    }
}
