use crate::terminal::SIZE;
use crate::util::Size;

pub mod colors;
pub mod events;

pub const SIZE: Size = Size {
    width: 26,
    height: 12,
};
pub const GRAYSCALE_COLOR_COUNT: SIZE = 24;
pub const INPUT_FIELD_WIDTH: SIZE = GRAYSCALE_COLOR_COUNT;
pub const FOUR_BIT_COLOR_COUNT: SIZE = 8 * 2;
