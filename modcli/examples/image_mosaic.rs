// Run with: cargo run --example image_mosaic --features images
use modcli::output::print;
#[cfg(feature = "images")]
use modcli::output::{show_image_mosaic, ImageOpts};

fn main() {
    print::line("ANSI Mosaic image demo (feature: images)");
    let _path = "docs/media/jamesgober-logo.png";

    #[cfg(feature = "images")]
    {
        let opts = ImageOpts {
            max_width: 60,
            preserve_aspect: true,
        };
        match show_image_mosaic(_path, opts) {
            Ok(s) => print::line(&s),
            Err(e) => print::line(&format!("Failed: {e}")),
        }
    }

    #[cfg(not(feature = "images"))]
    {
        print::line("images feature not enabled. Re-run with --features images");
    }
}
