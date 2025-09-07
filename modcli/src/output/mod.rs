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
pub mod gradient;
pub mod colors;
pub mod themes;
pub mod style;
pub mod print;
pub mod hook;
pub mod progress;
pub mod table;

// Expose public API
pub use colors::{
    RED, BLUE, GREEN, YELLOW, CYAN, MAGENTA, WHITE, BLACK, GREY, PINK, PURPLE, 
    TEAL, ORANGE, BROWN, LIGHT_BLUE, LIGHT_GREEN, LIGHT_YELLOW, LIGHT_CYAN, 
    LIGHT_MAGENTA, LIGHT_GREY, DARK_GREY, DARK_BLUE, DARK_ORANGE, DARK_PINK, 
    DARK_PURPLE, DARK_TEAL, DARK_BROWN
};
pub use style::build;
pub use progress::{
    ProgressBar, ProgressStyle, show_progress_bar, show_percent_progress, show_spinner
};

// Compile the input submodule crate-visibly; public access is via `modcli::input::*` re-exports
pub(crate) mod input;