use crate::{
    canvas::Canvas,
    terminal::SIZE,
    util::{Color, Point, Size},
};

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

    pub fn hollow_rectangle(&mut self, point: Point, size: Size, color: Color) {
        // -----
        //
        // -----
        self.blocks(point, color, size.width);
        self.blocks(
            Point {
                y: point.y + size.height - 1,
                ..point
            },
            color,
            size.width,
        );

        // +---+
        // |   |
        // +---+
        for index in 1..size.height {
            self.block(
                Point {
                    y: point.y + index,
                    ..point
                },
                color,
            );
            self.block(
                Point {
                    x: point.x + size.width - 1,
                    y: point.y + index,
                },
                color,
            );
        }
    }

    pub fn filled_rectangle(&mut self, point: Point, size: Size, color: Color) {
        for y_index in 0..size.height {
            self.blocks(
                Point {
                    y: point.y + y_index,
                    ..point
                },
                color,
                size.width,
            );
        }
    }
}
