//! Image rendering helpers (feature: images)
//! - Auto-detect best renderer (future): Kitty, iTerm2, Sixel
//! - Universal fallback: ANSI truecolor mosaic

#[cfg(feature = "images")]
use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};
use std::path::Path;

/// Options for rendering images.
#[derive(Clone, Copy, Debug)]
pub struct ImageOpts {
    pub max_width: u32,
    pub preserve_aspect: bool,
}

impl Default for ImageOpts {
    fn default() -> Self {
        Self {
            max_width: 80,
            preserve_aspect: true,
        }
    }
}

/// Auto-detect best rendering. For now, falls back to ANSI mosaic.
pub fn show(path: impl AsRef<Path>, opts: ImageOpts) -> Result<String, String> {
    show_mosaic(path, opts)
}

/// Universal ANSI truecolor mosaic fallback (works on most terminals).
/// Returns a String with ANSI escapes that can be printed directly.
pub fn show_mosaic(path: impl AsRef<Path>, opts: ImageOpts) -> Result<String, String> {
    #[cfg(not(feature = "images"))]
    {
        return Err("images feature not enabled".into());
    }

    #[cfg(feature = "images")]
    {
        let img = image::open(path.as_ref()).map_err(|e| format!("image open failed: {e}"))?;
        Ok(render_mosaic(&img, opts))
    }
}

/// Detect Kitty support (basic env heuristics).
pub fn supports_kitty() -> bool {
    std::env::var_os("WEZTERM_EXECUTABLE").is_some()
        || std::env::var("TERM")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .contains("kitty")
        || std::env::var("TERM_PROGRAM")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .contains("wezterm")
}

/// Detect iTerm2 support.
pub fn supports_iterm2() -> bool {
    std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_ascii_lowercase()
        .contains("iterm")
}

/// Detect Sixel support.
pub fn supports_sixel() -> bool {
    std::env::var("TERM")
        .unwrap_or_default()
        .to_ascii_lowercase()
        .contains("sixel")
        || std::env::var_os("VTE_VERSION").is_some()
}

#[cfg(feature = "images")]
fn render_mosaic(img: &DynamicImage, opts: ImageOpts) -> String {
    let (w, h) = img.dimensions();
    let target_w = opts.max_width.min(w.max(1));
    // Use half-height blocks: two vertical pixels per row using upper half block '▀'
    let scale = target_w as f32 / w.max(1) as f32;
    let target_h = if opts.preserve_aspect {
        ((h as f32 * scale) as u32).max(1)
    } else {
        h
    };

    // Ensure even height for pairing two pixels (top/bottom) per cell row
    let target_h_even = if target_h % 2 == 0 {
        target_h
    } else {
        target_h.saturating_sub(1).max(1)
    };

    let resized = img.resize_exact(target_w, target_h_even, FilterType::Triangle);
    let mut out = String::with_capacity((target_w * target_h_even * 12) as usize);

    // Build rows using '▀' with FG=top pixel, BG=bottom pixel
    for y in (0..target_h_even).step_by(2) {
        for x in 0..target_w {
            let p_top = resized.get_pixel(x, y).to_rgba();
            let p_bot = resized.get_pixel(x, y + 1).to_rgba();
            let (r1, g1, b1, a1) = (p_top[0], p_top[1], p_top[2], p_top[3]);
            let (r2, g2, b2, a2) = (p_bot[0], p_bot[1], p_bot[2], p_bot[3]);
            // Simple alpha blend over black
            let (r1, g1, b1) = alpha_over_black(r1, g1, b1, a1);
            let (r2, g2, b2) = alpha_over_black(r2, g2, b2, a2);
            // ANSI 24-bit FG/BG + '▀'
            use std::fmt::Write as _;
            let _ = write!(
                out,
                "\x1b[38;2;{r1};{g1};{b1}m\x1b[48;2;{r2};{g2};{b2}m▀"
            );
        }
        out.push('\n');
        // Reset at end of line
        out.push_str("\x1b[0m");
    }

    out
}

#[cfg(feature = "images")]
#[inline(always)]
fn alpha_over_black(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8) {
    // Premultiplied alpha over black: c' = c * (a/255)
    let af = (a as f32) / 255.0;
    (
        (r as f32 * af) as u8,
        (g as f32 * af) as u8,
        (b as f32 * af) as u8,
    )
}
