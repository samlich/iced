//! Select a value from a list of options.
pub use iced_native::combo_box::State;

/// A widget allowing the selection of a single value from a list of options.
pub type ComboBox<'a, T, Message> = iced_native::ComboBox<'a, T, Message>;
