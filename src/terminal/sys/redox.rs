//! (Unimplemented) terminal implementation for the Redox operating system.

use std::io;
use termion::screen;

pub fn enter_alternate_dimension(stdout: &mut io::Stdout) {
    write!(stdout, screen::ToAlternateScreen);
}

pub fn exit_alternate_dimension(stdout: &mut io::Stdout) {
    write!(stdout, screen::ToMainScreen);
}
