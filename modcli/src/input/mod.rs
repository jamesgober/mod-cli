pub mod input_builder;
pub mod secure;

// Unified input API re-exports
pub use crate::output::input::{interactive_menu, prompt_confirm, prompt_password, prompt_text};
