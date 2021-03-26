use crate::canvas::Canvas;
use crate::terminal::SIZE;
use crate::util::Color;

impl Canvas {
    pub fn circle(&mut self, x: SIZE, y: SIZE, color: Color, radius: SIZE) {
        let radius = radius as i32;
        let center_x = x as i32;
        let center_y = y as i32;

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
                self.block(rx as SIZE, y as SIZE, color);
                y += 1;
            }
            x += 1;
        }
    }

    pub fn hollow_rectangle(&mut self, x: SIZE, y: SIZE, width: SIZE, height: SIZE, color: Color) {
        // -----
        //
        // -----
        self.blocks(x, y, color, width);
        self.blocks(x, y + height - 1, color, width);

        // +---+
        // |   |
        // +---+
        for index in 1..height {
            self.block(x, y + index, color);
            self.block(x + width - 1, y + index, color);
        }
    }

    pub fn filled_rectangle(&mut self, x: SIZE, y: SIZE, width: SIZE, height: SIZE, color: Color) {
        for y_index in 0..height {
            self.blocks(x, y + y_index, color, width);
        }
    }
}
