use crate::{
    canvas::{input, Canvas},
    terminal::{Terminal, SIZE},
    util::{Color, Point},
};

#[derive(Clone, Copy, Debug)]
pub struct ClickableColor {
    pub x: SIZE,
    pub y: SIZE,
    pub width: SIZE,
    pub color: Color,
}

pub fn get_color(clickable_colors: &[ClickableColor], x: SIZE, y: SIZE) -> Option<Color> {
    for clickable_color in clickable_colors {
        for index in 0..clickable_color.width {
            if x == clickable_color.x + index && y == clickable_color.y {
                return Some(clickable_color.color);
            }
        }
    }
    None
}

pub const WIDTH: SIZE = 26;
pub const GRAYSCALE_COLOR_COUNT: SIZE = 24;

pub struct Colors;

impl Colors {
    /// Draws the palette's colors using background-colored spaces.
    pub fn draw(
        mut x: SIZE,
        mut y: SIZE,
        terminal: &mut Terminal,
        clickable_colors: &mut Vec<ClickableColor>,
    ) -> Point {
        use Color::*;

        //
        // 4-bit colors
        //

        // The first 8 colors
        let bright_colors = [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];

        let four_bit_color_center = WIDTH / 2 - bright_colors.len() as SIZE;
        x += four_bit_color_center;

        terminal.set_cursor(x, y);

        for (index, color) in bright_colors.iter().enumerate() {
            terminal.set_background_color(*color);
            terminal.write("  ");

            clickable_colors.push(ClickableColor {
                x: x + index as SIZE * 2,
                y,
                width: 2,
                color: *color,
            });
        }

        y += 1;

        // The next 8 colors
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

        terminal.set_cursor(x, y);

        for (index, color) in dark_colors.iter().enumerate() {
            terminal.set_background_color(*color);
            terminal.write("  ");

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
        // that are identical to the 4-bit colors and also remove all colors inside of the 8-bit colors that are identical

        x -= four_bit_color_center;
        y += 1;

        // Filter duplicates
        let high_intensity_colors = [244, 196, 46, 226, 21, 201, 51, 231];
        let four_bit_colors = high_intensity_colors.len() as SIZE * 2;
        let colors = (four_bit_colors + 1..u8::MAX - GRAYSCALE_COLOR_COUNT)
            .filter(|color| !high_intensity_colors.contains(color))
            .enumerate();

        let previous_x = x;
        for (index, color) in colors {
            if index as SIZE % WIDTH == 0 {
                x = previous_x;
                terminal.set_cursor(x, y);
                y += 1;
            }
            let byte_color = Color::ByteColor(color);
            terminal.set_background_color(byte_color);
            terminal.write(" ");

            clickable_colors.push(ClickableColor {
                x,
                y: y - 1,
                width: 1,
                color: byte_color,
            });
            x += 1;
        }
        x = previous_x;

        let grayscale_colors = u8::MAX - GRAYSCALE_COLOR_COUNT + 1..=u8::MAX;

        x += 1;

        terminal.set_cursor(x, y);
        for (index, color) in grayscale_colors.enumerate() {
            let byte_color = Color::ByteColor(color);
            terminal.set_background_color(byte_color);
            terminal.write(" ");

            clickable_colors.push(ClickableColor {
                x: x + index as SIZE,
                y,
                width: 1,
                color: byte_color,
            });
        }

        x -= 1;
        y += 1;

        terminal.reset_colors();

        Point { x, y }
    }
}

pub fn draw_border(palette_canvas: &mut Canvas, color: Color) -> Point {
    palette_canvas.border(WIDTH + 2, color)
}

pub fn toggle(
    main_canvas: &mut Canvas,
    palette_canvas: &mut Canvas,
    clickable_colors: &mut Vec<ClickableColor>,
    palette: &mut Option<Colors>,
    input_field: &mut Option<input::Field>,
    color: Color,
) {
    if palette.is_some() {
        main_canvas.terminal.clear();
        main_canvas.redraw();
        clickable_colors.clear();
        *palette = None;
        *input_field = None;
        main_canvas.terminal.hide_cursor();
    } else {
        let mut border_point = draw_border(palette_canvas, color);

        // Go inside the border
        border_point.x += 1;
        border_point.y += 1;

        palette_canvas.filled_rectangle(border_point.x, border_point.y, WIDTH, WIDTH, Color::Black);

        border_point.y /= 2;
        border_point.y += 1;

        let palette_point = Colors::draw(
            border_point.x,
            border_point.y,
            &mut palette_canvas.terminal,
            clickable_colors,
        );
        let new_input_field = input::Field::new(palette_point.x, palette_point.y);
        new_input_field.redraw(&mut main_canvas.terminal);
        *palette = Some(Colors);
        *input_field = Some(new_input_field);
    }
}
