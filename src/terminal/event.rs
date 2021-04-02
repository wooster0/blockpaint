//! Terminal events defined specific to usage.

use crate::util::{Point, Size};

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
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
    pub point: Point,
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
