pub use crate::{
    canvas::{input, Canvas},
    terminal::{Terminal, SIZE},
    util::{Color, Point},
    window,
};

#[derive(Clone, Copy, Debug)]
pub struct ClickableColor {
    pub x: SIZE,
    pub y: SIZE,
    pub width: SIZE,
    pub color: Color,
}

pub fn get_color(clickable_colors: &Vec<ClickableColor>, x: SIZE, y: SIZE) -> Option<Color> {
    for clickable_field in clickable_colors {
        for index in 0..clickable_field.width {
            if x == clickable_field.x + index && y == clickable_field.y {
                return Some(clickable_field.color);
            }
        }
    }
    None
}

pub const WIDTH: SIZE = 26;

pub struct Colors;

impl Colors {
    pub fn draw(
        &mut self,
        mut x: SIZE,
        mut y: SIZE,
        canvas: &mut Canvas,
        clickable_colors: &mut Vec<ClickableColor>,
    ) -> Point {
        use Color::*;

        // Move inside the window's corners
        x += 1;
        y += 1;

        //
        // 4-bit colors
        //

        // The first 8 colors.
        let bright_colors = [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];

        let four_bit_color_center = WIDTH / 2 - bright_colors.len() as SIZE;
        x += four_bit_color_center;

        canvas.terminal.set_cursor(x, y);

        for (index, color) in bright_colors.iter().enumerate() {
            canvas.terminal.set_background_color(*color);
            canvas.terminal.write("  ");

            clickable_colors.push(ClickableColor {
                x: x + index as SIZE * 2,
                y,
                width: 2,
                color: *color,
            });
        }

        y += 1;

        // The next 8 colors.
        let dark_colors = [
            DarkGray,
            DarkRed,
            DarkGreen,
            DarkYellow,
            DarkBlue,
            DarkMagenta,
            DarkCyan,
            Gray,
        ];

        canvas.terminal.set_cursor(x, y);

        for (index, color) in dark_colors.iter().enumerate() {
            canvas.terminal.set_background_color(*color);
            canvas.terminal.write("  ");

            clickable_colors.push(ClickableColor {
                x: x + index as SIZE * 2,
                y,
                width: 2,
                color: *color,
            });
        }

        //
        // 8-bit colors
        //

        // We want to keep this as small as possible so we remove the first 17 colors
        // that are identical to the 4-bit colors and also remove all colors inside of the 8-bit colors that are identical.

        x -= four_bit_color_center;
        y += 1;

        canvas.terminal.set_cursor(x, y);
        canvas.terminal.write("X");
        canvas.terminal.flush();

        // Filter duplicates
        let high_intensity_colors = [244, 196, 46, 226, 21, 201, 51, 231];
        let four_bit_colors = high_intensity_colors.len() as SIZE * 2;
        let grayscale_color_count = 24;
        let colors = (four_bit_colors + 1..u8::MAX - grayscale_color_count)
            .filter(|color| !high_intensity_colors.contains(color))
            .enumerate();

        let previous_x = x;
        for (index, color) in colors {
            if index as SIZE % WIDTH == 0 {
                x = previous_x;
                canvas.terminal.set_cursor(x, y);
                y += 1;
            }
            let byte_color = Color::ByteColor(color);
            canvas.terminal.set_background_color(byte_color);
            canvas.terminal.write(" ");

            clickable_colors.push(ClickableColor {
                x: x,
                y: y - 1,
                width: 1,
                color: byte_color,
            });
            x += 1;
        }
        x = previous_x;

        let grayscale_colors = u8::MAX - grayscale_color_count + 1..=u8::MAX;

        x += 1;

        canvas.terminal.set_cursor(x, y);
        for (index, color) in grayscale_colors.enumerate() {
            let byte_color = Color::ByteColor(color);
            canvas.terminal.set_background_color(byte_color);
            canvas.terminal.write(" ");

            clickable_colors.push(ClickableColor {
                x: x + index as SIZE,
                y,
                width: 1,
                color: byte_color,
            });
        }

        x -= 1;
        y += 1;

        canvas.terminal.reset_colors();

        Point { x, y }
    }
}

pub fn toggle(
    canvas: &mut Canvas,
    clickable_colors: &mut Vec<ClickableColor>,
    palette: &mut Option<Colors>,
    input_field: &mut Option<input::Field>,
    color: Color,
) {
    if palette.is_some() {
        canvas.terminal.clear();
        canvas.redraw();
        clickable_colors.clear();
        *palette = None;
        *input_field = None;
        canvas.terminal.hide_cursor();
    } else {
        let window_point = crate::window::draw(&mut canvas.terminal, WIDTH, color);

        *palette = Some(Colors);
        if let Some(palette) = palette {
            let palette_point =
                palette.draw(window_point.x, window_point.y, canvas, clickable_colors);

            *input_field = Some(input::Field::new(palette_point.x, palette_point.y));
            if let Some(input_field) = &input_field {
                canvas
                    .terminal
                    .set_cursor(input_field.x_center, input_field.y);
                canvas.terminal.show_cursor();
            }
        }
    }
}
