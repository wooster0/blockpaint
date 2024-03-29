use crate::palette::{self, FOUR_BIT_COLOR_COUNT, GRAYSCALE_COLOR_COUNT}; // TODO: import SIZE here once terminal::SIZE is gone
use crate::{
    terminal::{Terminal, SIZE},
    util::{Color, Point},
};
#[derive(Clone, Debug)]
pub struct ClickableColor {
    pub point: Point,
    pub width: SIZE,
    pub color: Color,
}

pub fn get_color(clickable_colors: &[ClickableColor], point: Point) -> Option<Color> {
    let (x, y) = (point.x, point.y);
    for clickable_color in clickable_colors {
        for index in 0..clickable_color.width {
            if x == clickable_color.point.x + index && y == clickable_color.point.y {
                return Some(clickable_color.color);
            }
        }
    }
    None
}

enum Direction {
    Left,
    Right,
}

pub fn draw_left_color(terminal: &mut Terminal, color: Color) {
    draw_direction_color(terminal, color, Direction::Left);
}

pub fn draw_right_color(terminal: &mut Terminal, color: Color) {
    draw_direction_color(terminal, color, Direction::Right);
}

fn draw_direction_color(terminal: &mut Terminal, color: Color, direction: Direction) {
    let mut point = terminal.get_centered_border_point(&palette::SIZE);
    if let Direction::Right = direction {
        point.x += palette::SIZE.width;
        point.x -= 5;
    }
    terminal.set_cursor(point);
    terminal.set_foreground_color(color.invert());
    terminal.set_background_color(color);
    terminal.write(match direction {
        Direction::Left => "  L  ",
        Direction::Right => "  R  ",
    });
    point.y += 1;
    terminal.set_cursor(point);
    terminal.write("     ");
}

/// Draws the palette's colors using background-colored spaces.
pub fn draw(
    terminal: &mut Terminal,
    clickable_colors: &mut Vec<ClickableColor>,
    state: &crate::event::State,
) -> Point {
    use Color::*;

    let mut point = terminal.get_centered_border_point(&palette::SIZE);

    draw_left_color(terminal, state.left_color);
    draw_right_color(terminal, state.right_color);

    //
    // 4-bit colors
    //

    // The first 8 colors
    let bright_colors = [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];

    let four_bit_color_center = palette::SIZE.width / 2 - bright_colors.len() as SIZE;
    point.x += four_bit_color_center;

    terminal.set_cursor(point);

    for (index, color) in bright_colors.iter().enumerate() {
        terminal.set_background_color(*color);
        terminal.write("  ");

        clickable_colors.push(ClickableColor {
            point: Point {
                x: point.x + index as SIZE * 2,
                ..point
            },
            width: 2,
            color: *color,
        });
    }

    point.y += 1;

    terminal.set_cursor(point);

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

    for (index, color) in dark_colors.iter().enumerate() {
        terminal.set_background_color(*color);
        terminal.write("  ");

        clickable_colors.push(ClickableColor {
            point: Point {
                x: point.x + index as SIZE * 2,
                ..point
            },

            width: 2,
            color: *color,
        });
    }

    //
    // 8-bit colors
    //

    // We want to keep this as small as possible so we remove the first 17 colors
    // that are identical to the 4-bit colors and also remove all duplicates inside of the 8-bit colors

    point.x -= four_bit_color_center;

    // Filter duplicates
    let high_intensity_colors = [244, 196, 46, 226, 21, 201, 51, 231];
    let colors = (FOUR_BIT_COLOR_COUNT + 1..u8::MAX - GRAYSCALE_COLOR_COUNT)
        .filter(|color| !high_intensity_colors.contains(color))
        .enumerate();

    for (index, color) in colors {
        if index as SIZE % palette::SIZE.width == 0 {
            if index > 0 {
                point.x -= palette::SIZE.width;
            }
            point.y += 1;
            terminal.set_cursor(point);
        }
        let byte_color = Color::ByteColor(color);
        terminal.set_background_color(byte_color);
        terminal.write(" ");
        clickable_colors.push(ClickableColor {
            point: Point {
                x: point.x,
                ..point
            },
            width: 1,
            color: byte_color,
        });
        point.x += 1;
    }
    point.x -= palette::SIZE.width;
    point.y += 1;

    point.x += 1;
    terminal.set_cursor(point);

    let grayscale_colors = u8::MAX - GRAYSCALE_COLOR_COUNT + 1..=u8::MAX;

    for (index, color) in grayscale_colors.enumerate() {
        let byte_color = Color::ByteColor(color);
        terminal.set_background_color(byte_color);
        terminal.write(" ");

        clickable_colors.push(ClickableColor {
            point: Point {
                x: point.x + index as SIZE,
                ..point
            },
            width: 1,
            color: byte_color,
        });
    }
    point.x -= 1;
    point.y += 1;

    point
}
