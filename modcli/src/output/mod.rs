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

// Inputs
pub mod input;

// Expose public API
pub use input::{prompt_text, prompt_password, prompt_confirm};

// Deprecated
pub mod color_picker;