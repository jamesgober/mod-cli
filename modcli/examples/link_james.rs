use modcli::output::print;

fn main() {
    print::line("OSC 8 hyperlink demo (James Gober):");
    print::line("Tip: set ENABLE_OSC8=true for clickable links in supported terminals");

    print::link("JamesGober.com", "https://jamesgober.com");
    print::link("GitHub: jamesgober", "https://github.com/jamesgober");
}
