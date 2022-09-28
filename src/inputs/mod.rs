use self::key::Key;

pub mod events;
pub mod key;

// inputs/mod.rs
pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
}
