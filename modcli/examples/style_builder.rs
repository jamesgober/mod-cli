use modcli::output::{build, print};

fn main() {
    let msg = build()
        .base()
        .color(modcli::output::WHITE)
        .background(modcli::output::DARK_BLUE)
        .done()
        .part("Hello")
        .bold()
        .space()
        .part("from")
        .italic()
        .space()
        .part("mod-cli!")
        .color(modcli::output::CYAN)
        .underline()
        .space()
        .part("Styled")
        .color(modcli::output::YELLOW)
        .space()
        .part("Output")
        .color(modcli::output::GREEN)
        .strike()
        .space()
        .part("Builder")
        .color(modcli::output::MAGENTA)
        .blink()
        .get();

    print::line(&msg);
}
