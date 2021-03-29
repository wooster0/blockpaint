use crate::canvas::Canvas;
use crate::terminal::SIZE;
use crate::util::{Color, Point};

impl Canvas {
    pub fn circle(&mut self, point: Point, color: Color, radius: SIZE) {
        let radius = radius as i32;
        let center_x = point.x as i32;
        let center_y = point.y as i32;

        // Original: https://stackoverflow.com/a/59211338/15415674
        // Changes were made
        let radius_sqr = radius.pow(2);
        let mut x = -radius;
        while x < radius {
            let hh = ((radius_sqr - x * x) as f64).sqrt() as i32;
            let rx = center_x + x;
            let ph = center_y + hh;
            let mut y = center_y - hh;
            while y < ph {
                self.block(
                    Point {
                        x: rx as SIZE,
                        y: y as SIZE,
                    },
                    color,
                );
                y += 1;
            }
            x += 1;
        }
    }

    pub fn hollow_rectangle(&mut self, point: Point, width: SIZE, height: SIZE, color: Color) {
        // -----
        //
        // -----
        self.blocks(point, color, width);
        self.blocks(
            Point {
                y: point.y + height - 1,
                ..point
            },
            color,
            width,
        );

        // +---+
        // |   |
        // +---+
        for index in 1..height {
            self.block(
                Point {
                    y: point.y + index,
                    ..point
                },
                color,
            );
            self.block(
                Point {
                    x: point.x + width - 1,
                    y: point.y + index,
                },
                color,
            );
        }
    }

    pub fn filled_rectangle(&mut self, point: Point, width: SIZE, height: SIZE, color: Color) {
        for y_index in 0..height {
            self.blocks(
                Point {
                    y: point.y + y_index,
                    ..point
                },
                color,
                width,
            );
        }
    }
}
