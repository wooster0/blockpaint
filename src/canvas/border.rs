use super::Canvas;
use crate::terminal::SIZE;
use crate::util::{Color, Point};

impl Canvas {
    /// Draws a canvas-centered border.
    pub fn border(&mut self, size: SIZE, color: Color) -> Point {
        let x = self.terminal.size.width / 2 - size / 2;
        let y = self.terminal.size.height - size / 2;

        self.hollow_rectangle(x, y, size, size, color);

        Point { x, y }
    }
}
