use modcli::output::print;

fn main() {
    print::line("OSC 8 hyperlink demo:");
    print::line("(Set ENABLE_OSC8=true to enable clickable links in supported terminals)");

    print::link("mod-cli on docs.rs", "https://docs.rs/mod-cli");
    print::link(
        "GitHub: jamesgober/mod-cli",
        "https://github.com/jamesgober/mod-cli",
    );
}
