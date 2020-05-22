//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
//!
//! [`Button`]: type.Button.html
//! [`State`]: struct.State.html
pub use iced_native::combo_box::State;

pub type ComboBox<'a, T, Message> = iced_native::ComboBox<'a, T, Message>;
