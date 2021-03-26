use crate::util::Size;
use std::io::Write;
use std::{convert::TryFrom, fmt, io, ops};
pub mod event;
mod sys;

/// Defines the terminal width and height boundary. 255 cells.
pub type SIZE = u8;

impl Size {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: SIZE::try_from(width).unwrap_or_else(|_| {
                panic!("terminal width must be in range {}", Range(0..SIZE::MAX));
            }),
            height: SIZE::try_from(height).unwrap_or_else(|_| {
                panic!("terminal height must be in range {}", Range(0..SIZE::MAX));
            }),
        }
    }
}

struct Range(ops::Range<SIZE>);

impl fmt::Display for Range {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} to {}", self.0.start, self.0.end)
    }
}

pub struct Terminal {
    pub handle: io::Stdout,
    pub size: Size,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            handle: io::stdout(),
            size: Self::size(), // We get the size only once and then update it using the resize event
        }
    }

    pub fn write(&mut self, string: &str) {
        self.handle
            .write_all(string.as_bytes())
            .expect("write to the terminal failed");
    }

    pub fn flush(&mut self) {
        self.handle.flush().expect("flushing failed");
    }
}

impl Terminal {
    pub fn initialize(&mut self) {
        self.enter_alternate_dimension();
        self.enable_raw_mode();
        self.enable_mouse_capture();
        self.hide_cursor();
        self.flush();
    }

    pub fn deinitialize(&mut self) {
        self.show_cursor();
        self.disable_mouse_capture();
        self.disable_raw_mode();
        self.exit_alternate_dimension();
        self.flush();
    }
}
