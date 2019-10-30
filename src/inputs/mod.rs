use super::coord::{Dir, RotateDir};

pub use self::event_reader::EventReader;
pub use self::inputs::Inputs;
pub use self::keys::KeyConverter;

mod event_reader;
#[allow(clippy::module_inception)]
mod inputs;
mod keys;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Order {
    Move(Dir),
    Rotate(RotateDir),
    Select,
    Help,
    Quit,
}
