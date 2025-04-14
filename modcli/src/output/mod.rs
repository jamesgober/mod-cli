pub mod gradient;
pub mod color_picker;
pub mod themes;
pub use themes::{
    RED, BLUE, GREEN, YELLOW, CYAN, MAGENTA, WHITE, BLACK, GREY, PINK, PURPLE, 
    TEAL, ORANGE, BROWN, LIGHT_BLUE, LIGHT_GREEN, LIGHT_YELLOW, LIGHT_CYAN, 
    LIGHT_MAGENTA, LIGHT_GREY, DARK_GREY, DARK_BLUE, DARK_ORANGE, DARK_PINK, 
    DARK_PURPLE, DARK_TEAL, DARK_BROWN
};
pub mod print;
pub mod hooks;
pub mod table;
pub mod progress;
pub use progress::{
    ProgressBar, ProgressStyle, show_progress_bar, show_percent_progress, show_spinner
};
pub mod input;
pub use input::{prompt_text, prompt_password, prompt_confirm};



pub use print::{
    print_multiline,
    print_status,
    print_success,
    print_warning,
    print_error,
};