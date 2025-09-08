#[cfg(feature = "gradients")]
use modcli::output::gradient_extras;
use modcli::output::{gradient, print};

fn main() {
    // Multi-color raw
    let s = gradient::multi_color(
        "Multi-color (raw)",
        vec![
            modcli::output::RED,
            modcli::output::ORANGE,
            modcli::output::YELLOW,
            modcli::output::GREEN,
            modcli::output::BLUE,
        ],
    );
    print::line(&s);

    // Multi-color with names (feature = gradients)
    #[cfg(feature = "gradients")]
    {
        let s2 = gradient_extras::multi_named(
            "Multi-color (named)",
            &["red", "orange", "yellow", "green", "blue"],
        );
        print::line(&s2);
    }
}
