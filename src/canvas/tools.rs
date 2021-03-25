use super::Canvas;
use crate::{terminal::SIZE, util::Color};

impl Canvas {
    pub fn dot(&mut self, x: SIZE, y: SIZE, color: Color) {
        self.terminal.set_cursor(x, y / 2);
        self.half_block(x, y, color);
    }

    pub fn brush(&mut self, x: SIZE, y: SIZE, color: Color, size: SIZE) {
        match size {
            1 => self.dot(x, y, color), // Middle dot
            2 => {
                self.dot(x, y - 1, color); // Left dot
                self.dot(x - 1, y, color); // Upper dot
                self.dot(x, y, color); // Middle dot
                self.dot(x + 1, y, color); // Lower dot
                self.dot(x, y + 1, color); // Right dot
            }
            _ => {
                self.circle(x, y, color, size - 1);
            }
        }
    }

    pub fn circle(&mut self, x: SIZE, y: SIZE, color: Color, radius: SIZE) {
        let radius = radius as i32;
        let center_x = x as i32;
        let center_y = y as i32;

        // Original: https://stackoverflow.com/a/59211338/15415674
        // Changes were made.
        let radius_sqr = radius.pow(2);
        let mut x = -radius;
        while x < radius {
            let hh = ((radius_sqr - x * x) as f64).sqrt() as i32;
            let rx = center_x + x;
            let ph = center_y + hh;
            let mut y = center_y - hh;
            while y < ph {
                self.dot(rx as SIZE, y as SIZE, color);
                y += 1;
            }
            x += 1;
        }
    }

    pub fn quill(&mut self, x: SIZE, y: SIZE, color: Color, size: SIZE) {
        for size in 0..=size {
            if size % 2 == 0 {
                self.dot(x, y + size / 2, color);
            } else {
                self.dot(x, y - size / 2, color);
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
        }
    }
}

fn normalize(value: SIZE) -> SIZE {
    value * 2
}
