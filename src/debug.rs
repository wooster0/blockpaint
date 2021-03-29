#![cfg(debug_assertions)]
#![allow(dead_code)]

//! Functions for easier debugging only available in debug (non-release) builds.

use crate::terminal;
use crate::util::Point;
use std::{thread, time};

/// Pauses the program until an interaction is detected.
pub fn pause() {
    terminal::Terminal::read();
}

/// Pauses the program for the given seconds.
pub fn pause_for(milliseconds: u64) {
    thread::sleep(time::Duration::from_secs(milliseconds));
}

/// Prints at a specific position.
///
/// # Examples
///
/// ```
/// debug::print(&terminal, 0, 0, format!("({}, {})", x, y));
/// ```
// This is made to be used with `format` instead of `format_args` simply because it's faster to type.
pub fn print(terminal: &mut terminal::Terminal, point: Point, string: String) {
    terminal.set_cursor(point);
    terminal.write(&string);
}
