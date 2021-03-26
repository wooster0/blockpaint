use crate::terminal::SIZE;
use std::convert::TryFrom;

pub struct Point {
    pub x: SIZE,
    pub y: SIZE,
}

#[derive(Clone)]
pub struct Size {
    pub width: SIZE,
    pub height: SIZE,
}

/// Tries to parse the input into an RGB color.
/// It can parse the following RGB notations:
///
/// 8-bit,       e.g. (255, 0, 0),
/// Hexadecimal, e.g. #FF0000
///
/// See https://en.wikipedia.org/wiki/RGB_color_model for more information.
// TODO:
// Float,       e.g. (1.0, 0.0, 0.0),
// Percentage,  e.g. (100%, 0%, 0%),
pub fn parse_rgb_color(string: &str) -> Option<Color> {
    let mut r: Option<u8> = None;
    let mut g: Option<u8> = None;
    let mut b: Option<u8> = None;

    let mut component = &mut r;

    let mut hexdigits_in_a_row = 0;
    let mut index = 0;
    for char in string.chars() {
        match char {
            '0'..='9' => {
                if let Some(byte) = char.to_digit(10) {
                    *component = if let Some(component) = *component {
                        Some(
                            u8::try_from(component as usize * 10 + byte as usize)
                                .unwrap_or(u8::MAX),
                        )
                    } else {
                        Some(byte as u8)
                    };
                };
                hexdigits_in_a_row += 1;
            }
            _ if char.is_ascii_hexdigit() => {
                if let Some(color) = parse_hex(string, index) {
                    return Some(color);
                }
                hexdigits_in_a_row += 1;
            }
            _ => {
                component = match (r, g, b) {
                    (None, None, None) => &mut r,
                    (Some(_), None, None) => &mut g,
                    (Some(_), Some(_), None) => &mut b,
                    (Some(r), Some(g), Some(b)) => return Some(Color::Rgb { r, g, b }),
                    _ => return None,
                };
                hexdigits_in_a_row = 0;
            }
        }

        index += 1;

        if hexdigits_in_a_row == 6 && index >= hexdigits_in_a_row {
            if let Some(color) = parse_hex(string, index - hexdigits_in_a_row) {
                return Some(color);
            }
        }
    }

    match (r, g, b) {
        (Some(r), None, None) => Some(Color::Rgb { r, g: 0, b: 0 }),
        (Some(r), Some(g), None) => Some(Color::Rgb { r, g, b: 0 }),
        (Some(r), Some(g), Some(b)) => Some(Color::Rgb { r, g, b }),
        _ => None,
    }
}

fn parse_hex(string: &str, index: usize) -> Option<Color> {
    if let (Some(r), Some(g), Some(b)) = (
        &string.get(index..index + 2),
        &string.get(index + 2..index + 4),
        &string.get(index + 4..index + 6),
    ) {
        let r = u8::from_str_radix(r, 16);
        let g = u8::from_str_radix(g, 16);
        let b = u8::from_str_radix(b, 16);
        if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
            Some(Color::Rgb { r, g, b })
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    // 4-bit colors
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    Black,
    Gray,
    DarkGray,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // 8-bit colors
    ByteColor(u8),
    // 24-bit colors
    Rgb { r: u8, g: u8, b: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(string: &str) -> Option<Color> {
        parse_rgb_color(string)
    }

    fn rgb(r: u8, g: u8, b: u8) -> Option<Color> {
        Some(Color::Rgb { r, g, b })
    }

    #[test]
    fn test_parse_rgb_color() {
        assert_eq!(parse("255, 255, 255"), rgb(255, 255, 255));
        assert_eq!(parse("200,255,255"), rgb(200, 255, 255));
        assert_eq!(parse("-200,-255,-255"), rgb(200, 255, 255));
        assert_eq!(parse("(255,200,255)"), rgb(255, 200, 255));
        assert_eq!(parse("www255,255,200www"), rgb(255, 255, 200));
        assert_eq!(parse("    www100www,www0www,www100www"), rgb(100, 0, 100));
        assert_eq!(parse("www100www,www20www,,,"), rgb(100, 20, 0));
        assert_eq!(parse("   123"), rgb(123, 0, 0));
        assert_eq!(parse("99999,99999,99999"), rgb(255, 255, 255));
        assert_eq!(parse("FF0000,00FF00,0000FF"), rgb(255, 0, 0));
        assert_eq!(parse("00FF00"), rgb(0, 255, 0));
        assert_eq!(parse("    00FF00"), rgb(0, 255, 0));
        assert_eq!(parse("-50,-50,-50-00FF00"), rgb(50, 50, 50));
        assert_eq!(parse("256"), rgb(255, 0, 0));
        assert_eq!(parse("99999"), rgb(255, 0, 0));
        assert_eq!(parse("rgb(123,255,100)"), rgb(123, 255, 100));
        assert_eq!(parse("123,255,100"), rgb(123, 255, 100));
        // assert_eq!(parse("255,255,255555555"), rgb(255, 255, 255));
        // assert_eq!(parse("255,255,255efefef"), rgb(255, 255, 255));
    }
}
