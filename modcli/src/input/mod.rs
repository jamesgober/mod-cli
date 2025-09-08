pub mod builders;
pub mod input_builder;
pub mod secure;

// Unified input API re-exports
pub use crate::output::input::{interactive_menu, prompt_confirm, prompt_password, prompt_text};
// New builder-style API
pub use builders::{
    buttons, confirm, form, multi_select, number, raw_buttons, raw_multi_select,
    raw_multi_select_paged, raw_select, raw_select_paged, select, text, ConfirmInput, FormBuilder,
    FormValue, NumberInput, TextInput,
};
