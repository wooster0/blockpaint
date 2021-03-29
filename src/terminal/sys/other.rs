//! Terminal implementation for all non-Redox operating systems.

use crate::{
    terminal::{
        event::{Event, EventKind, KeyEvent, KeyModifier, MouseButton, MouseEvent},
        SIZE, {Size, Terminal},
    },
    util::{Color, Point},
};
use crossterm::{cursor, event, style, terminal, QueueableCommand};

impl Terminal {
    pub fn enter_alternate_dimension(&mut self) {
        self.handle.queue(terminal::EnterAlternateScreen).unwrap();
    }
    pub fn exit_alternate_dimension(&mut self) {
        self.handle.queue(terminal::LeaveAlternateScreen).unwrap();
    }

    pub fn set_title(&mut self, title: &str) {
        self.handle.queue(terminal::SetTitle(title)).unwrap();
    }

    pub fn enable_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap();
    }
    pub fn disable_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap();
    }

    pub fn enable_mouse_capture(&mut self) {
        self.handle.queue(event::EnableMouseCapture).unwrap();
    }
    pub fn disable_mouse_capture(&mut self) {
        self.handle.queue(event::DisableMouseCapture).unwrap();
    }

    pub fn show_cursor(&mut self) {
        self.handle.queue(cursor::Show).unwrap();
    }
    pub fn hide_cursor(&mut self) {
        self.handle.queue(cursor::Hide).unwrap();
    }

    pub fn read_event(&mut self) -> Option<Event> {
        let crossterm_event = Terminal::read();
        let event = match crossterm_event {
            event::Event::Mouse(event) => match event.kind {
                event::MouseEventKind::Moved => Event::Mouse(MouseEvent {
                    kind: EventKind::Move,
                    x: event.column as SIZE,
                    y: event.row as SIZE,
                }),
                event::MouseEventKind::Drag(button) => {
                    let button = match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Right => MouseButton::Right,
                    };
                    Event::Mouse(MouseEvent {
                        kind: EventKind::Drag(button),
                        x: event.column as SIZE,
                        y: event.row as SIZE,
                    })
                }
                event::MouseEventKind::Down(button) => {
                    let button = match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Right => MouseButton::Right,
                    };
                    Event::Mouse(MouseEvent {
                        kind: EventKind::Press(button),
                        x: event.column as SIZE,
                        y: event.row as SIZE,
                    })
                }
                event::MouseEventKind::Up(button) => {
                    let button = match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Right => MouseButton::Right,
                    };
                    Event::Mouse(MouseEvent {
                        kind: EventKind::Release(button),
                        x: event.column as SIZE,
                        y: event.row as SIZE,
                    })
                }
                event::MouseEventKind::ScrollUp => Event::Mouse(MouseEvent {
                    kind: EventKind::ScrollUp,
                    x: event.column as SIZE,
                    y: event.row as SIZE,
                }),
                event::MouseEventKind::ScrollDown => Event::Mouse(MouseEvent {
                    kind: EventKind::ScrollDown,
                    x: event.column as SIZE,
                    y: event.row as SIZE,
                }),
            },
            event::Event::Key(event::KeyEvent { code, modifiers }) => match code {
                event::KeyCode::Tab => Event::Key(KeyEvent::Tab),
                event::KeyCode::Char('w') if modifiers == event::KeyModifiers::CONTROL => {
                    Event::Key(KeyEvent::Backspace(Some(KeyModifier::Control)))
                }
                event::KeyCode::Char(key) => {
                    if modifiers == event::KeyModifiers::CONTROL {
                        Event::Key(KeyEvent::Char(key, Some(KeyModifier::Control)))
                    } else {
                        Event::Key(KeyEvent::Char(key, None))
                    }
                }
                event::KeyCode::Esc => Event::Key(KeyEvent::Esc),
                event::KeyCode::Backspace => Event::Key(KeyEvent::Backspace(None)),
                event::KeyCode::Left if modifiers == event::KeyModifiers::CONTROL => {
                    Event::Key(KeyEvent::Left(Some(KeyModifier::Control)))
                }
                event::KeyCode::Right if modifiers == event::KeyModifiers::CONTROL => {
                    Event::Key(KeyEvent::Right(Some(KeyModifier::Control)))
                }
                event::KeyCode::Up => Event::Key(KeyEvent::Up),
                event::KeyCode::Down => Event::Key(KeyEvent::Down),
                event::KeyCode::Left => Event::Key(KeyEvent::Left(None)),
                event::KeyCode::Right => Event::Key(KeyEvent::Right(None)),
                _ => return None,
            },
            event::Event::Resize(width, height) => {
                Event::Resize(Size::new(width as usize, height as usize))
            }
        };
        Some(event)
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.handle
            .queue(cursor::MoveTo(point.x as u16, point.y as u16))
            .unwrap();
    }

    pub fn move_cursor_left(&mut self, cells: SIZE) {
        self.handle.queue(cursor::MoveLeft(cells as u16)).unwrap();
    }
    pub fn move_cursor_right(&mut self, cells: SIZE) {
        self.handle.queue(cursor::MoveRight(cells as u16)).unwrap();
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.handle
            .queue(style::SetForegroundColor(Self::convert_color(color)))
            .unwrap();
    }

    pub fn enable_italic(&mut self) {
        self.write_args(format_args!("{}", style::Attribute::Italic));
    }

    pub fn disable_italic(&mut self) {
        self.write_args(format_args!("{}", style::Attribute::NoItalic));
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.handle
            .queue(style::SetBackgroundColor(Self::convert_color(color)))
            .unwrap();
    }

    fn convert_color(color: Color) -> style::Color {
        match color {
            Color::Black => style::Color::Black,
            Color::DarkGray => style::Color::DarkGrey,
            Color::Red => style::Color::Red,
            Color::DarkRed => style::Color::DarkRed,
            Color::Green => style::Color::Green,
            Color::DarkGreen => style::Color::DarkGreen,
            Color::Yellow => style::Color::Yellow,
            Color::DarkYellow => style::Color::DarkYellow,
            Color::Blue => style::Color::Blue,
            Color::DarkBlue => style::Color::DarkBlue,
            Color::Magenta => style::Color::Magenta,
            Color::DarkMagenta => style::Color::DarkMagenta,
            Color::Cyan => style::Color::Cyan,
            Color::DarkCyan => style::Color::DarkCyan,
            Color::White => style::Color::White,
            Color::Gray => style::Color::Grey,
            Color::Rgb { r, g, b } => style::Color::Rgb { r, g, b },
            Color::ByteColor(rgb) => style::Color::AnsiValue(rgb),
        }
    }

    pub fn reset_colors(&mut self) {
        self.handle.queue(style::ResetColor).unwrap();
    }

    pub fn clear(&mut self) {
        self.handle
            .queue(style::ResetColor)
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }

    pub fn size() -> Size {
        let size = terminal::size().expect("retrieving terminal size failed");
        Size::new(size.0 as usize, size.1 as usize)
    }

    pub fn read() -> event::Event {
        crossterm::event::read().expect("reading event failed")
    }
}
