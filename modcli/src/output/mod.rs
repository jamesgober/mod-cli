//! Output utilities for styled text, gradients, progress, and tables.
//!
//! # Examples
//!
//! ## Styled text builder
//! ```no_run
//! use modcli::output::{build, print, BLUE};
//!
//! let msg = build()
//!     .part("Hello").color(BLUE).bold().space()
//!     .part("world!")
//!     .get();
//! print::line(&msg);
//! ```
//!
//! ## Gradients
//! ```no_run
//! use modcli::output::{gradient, print, RED, ORANGE};
//! let text = gradient::two_color("Sunrise", RED, ORANGE);
//! print::line(&text);
//! ```
//!
//! ## Progress bar
//! ```no_run
//! use modcli::output::progress::{show_progress_bar};
//! show_progress_bar("Downloading", 20, 1000);
//! ```
//!
//! ## Tables
//! ```no_run
//! use modcli::output::table::{render_table, TableMode, TableStyle};
//! let headers = ["Name", "Age"];
//! let rows = vec![ vec!["Alice", "29"], vec!["Bob", "35"] ];
//! render_table(&headers, &rows, TableMode::Flex, TableStyle::Rounded);
//! ```
pub mod colors;
pub mod gradient;
pub mod hook;
pub mod print;
pub mod progress;
pub mod style;
pub mod table;
pub mod themes;

// Expose public API
pub use colors::{
    BLACK, BLUE, BROWN, CYAN, DARK_BLUE, DARK_BROWN, DARK_GREY, DARK_ORANGE, DARK_PINK,
    DARK_PURPLE, DARK_TEAL, GREEN, GREY, LIGHT_BLUE, LIGHT_CYAN, LIGHT_GREEN, LIGHT_GREY,
    LIGHT_MAGENTA, LIGHT_YELLOW, MAGENTA, ORANGE, PINK, PURPLE, RED, TEAL, WHITE, YELLOW,
};
pub use progress::{
    show_percent_progress, show_progress_bar, show_spinner, ProgressBar, ProgressStyle,
};
pub use style::build;

// Compile the input submodule crate-visibly; public access is via `modcli::input::*` re-exports
pub(crate) mod input;
