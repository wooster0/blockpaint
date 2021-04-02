#![cfg(debug_assertions)]
#![allow(dead_code)]

//! Functions for easier debugging only available in debug (non-release) builds.

use crate::{terminal, util::Point};
use std::{thread, time};

/// Pauses the program until an interaction is detected.
pub fn pause() {
    terminal::Terminal::read();
}

/// Pauses the program for the given seconds.
pub fn pause_for(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}

/// Prints at a specific position.
///
/// # Examples
///
/// ```
/// debug::print(&terminal, Point { x: 0, y: 0 }, &format!("{}", point));
/// ```
pub fn print(terminal: &mut terminal::Terminal, point: Point, string: &str) {
    terminal.set_cursor(point);
    terminal.write(&string);
}
