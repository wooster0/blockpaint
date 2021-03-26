use super::Canvas;
use crate::{terminal::SIZE, util::Color};

impl Canvas {
    /// Draws a block.
    pub fn block(&mut self, x: SIZE, y: SIZE, color: Color) {
        self.terminal.set_cursor(x, y / 2);
        self.terminal.set_foreground_color(color);
        self.half_block(x, y, color);
        self.terminal.reset_colors();
    }

    /// Efficiently draws multiple blocks in a row.
    pub fn blocks(&mut self, x: SIZE, y: SIZE, color: Color, count: SIZE) {
        self.terminal.set_cursor(x, y / 2);
        self.terminal.set_foreground_color(color);
        for index in 0..count {
            self.half_block(x + index, y, color);
        }
        self.terminal.reset_colors();
    }

    pub fn brush(&mut self, x: SIZE, y: SIZE, color: Color, size: SIZE) {
        match size {
            1 => self.block(x, y, color), // Middle dot
            2 => {
                self.block(x, y - 1, color); // Left dot
                self.block(x - 1, y, color); // Upper dot
                self.block(x, y, color); // Middle dot
                self.block(x + 1, y, color); // Lower dot
                self.block(x, y + 1, color); // Right dot
            }
            _ => {
                self.circle(x, y, color, size - 1);
            }
        }
    }

    pub fn quill(&mut self, x: SIZE, y: SIZE, color: Color, size: SIZE) {
        for size in 0..=size {
            if size % 2 == 0 {
                self.block(x, y + size / 2, color);
            } else {
                self.block(x, y - size / 2, color);
            }
        }
    }

    pub fn line(
        &self,
        x1: SIZE,
        y1: SIZE,
        x2: SIZE,
        y2: SIZE,
    ) -> bracket_geometry::prelude::Bresenham {
        use bracket_geometry::prelude::{Bresenham, Point};
        Bresenham::new(Point::new(x1, y1), Point::new(x2, y2))
    }
}

pub enum Tool {
    Brush,
    Quill,
    Rectangle,
}

pub use crate::util::Point;

impl Tool {
    pub fn draw(
        &self,
        canvas: &mut Canvas,
        x: SIZE,
        y: SIZE,
        color: Color,
        size: SIZE,
        last_point: &mut Option<Point>,
    ) {
        let y = normalize(y);
        if let Some(point) = last_point {
            for point in canvas.line(point.x, point.y, x, y) {
                self.r#use(canvas, point.x as SIZE, point.y as SIZE, color, size);
            }
        } else {
            self.r#use(canvas, x, y, color, size);
        }
        *last_point = Some(Point { x, y });
    }

    fn r#use(&self, canvas: &mut Canvas, x: SIZE, y: SIZE, color: Color, size: SIZE) {
        match self {
            Tool::Brush => {
                canvas.brush(x, y, color, size);
            }
            Tool::Quill => {
                canvas.quill(x, y, color, size);
            }
            Tool::Rectangle => {
                canvas.hollow_rectangle(x, y, size, size, color);
            }
        }
    }
}

fn normalize(value: SIZE) -> SIZE {
    value * 2
}
