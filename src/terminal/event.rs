//! Terminal events defined specific to usage.

use crate::{terminal::SIZE, util::Size};

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum EventKind {
    ScrollUp,
    ScrollDown,
    Move,
    Drag(MouseButton),
    Press(MouseButton),
    Release(MouseButton),
}

pub struct MouseEvent {
    pub kind: EventKind,
    pub x: SIZE,
    pub y: SIZE,
}

pub enum KeyModifier {
    Control,
}

pub enum KeyEvent {
    Up,
    Down,
    Left(Option<KeyModifier>),
    Right(Option<KeyModifier>),
    Char(char, Option<KeyModifier>),
    Tab,
    Esc,
    Backspace(Option<KeyModifier>),
}

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(Size),
}
