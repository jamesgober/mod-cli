use modcli::output::{gradient, print};

#[cfg(feature = "gradients")]
use modcli::output::gradient_extras;

fn main() {
    // Without names (raw colors via existing API)
    let s = gradient::two_color(
        "Two-color (raw)",
        modcli::output::BLUE,
        modcli::output::GREEN,
    );
    print::line(&s);

    // With names (feature = "gradients")
    #[cfg(feature = "gradients")]
    {
        let s2 = gradient_extras::two_named("Two-color (named)", "teal", "violet");
        print::line(&s2);
    }
}
