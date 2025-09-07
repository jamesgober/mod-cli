pub mod input_builder;
pub mod secure;

// Unified input API re-exports
pub use crate::output::input::{
    prompt_text,
    prompt_password,
    prompt_confirm,
    interactive_menu,
};