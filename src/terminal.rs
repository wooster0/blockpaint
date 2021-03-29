use crate::util::{Point, Size};
use std::io::Write;
use std::{fmt, io};
pub mod event;
mod sys;

/// Defines the terminal width and height boundary. 255 cells.
pub type SIZE = u8;

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

    pub fn write_args(&mut self, string: fmt::Arguments) {
        self.handle
            .write_fmt(string)
            .expect("formatted write to the terminal failed");
    }

    pub fn flush(&mut self) {
        self.handle.flush().expect("flushing failed");
    }

    pub fn get_centered_border_point(&self, size: &Size) -> Point {
        let mut point = Point {
            x: self.size.width / 2 - size.width / 2,
            y: self.size.height - size.height / 2,
        };
        point.y /= 2;
        point.y += 1;
        point
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
