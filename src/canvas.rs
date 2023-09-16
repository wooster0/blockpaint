pub mod shapes;
pub mod tools;
use crate::{
    terminal::{Terminal, SIZE},
    util::{Color, Point, Size},
};

pub struct Canvas {
    pub cells: Vec<Cell>, //[Cell; (SIZE::MAX as usize).pow(2)],
    terminal: Terminal,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Cell {
    pub upper_block: Option<Color>,
    pub lower_block: Option<Color>,
    pub upper_point: Point,
    pub lower_point: Point,
    /// If this is `Some(...)`, this character uses up the entire cell.
    pub character: Option<char>,
}

impl Canvas {
    pub fn new() -> Self {
        // (terminal.size.width, terminal.size.height);
        Self {
            cells: vec![Default::default(); (SIZE::MAX as usize).pow(2)],
            terminal: Terminal::new(),
        }
    }

    pub fn resize_terminal(&mut self, size: Size) {
        self.terminal.size = size;
    }

    fn get_position(point: Point) -> usize {
        point.x as usize + SIZE::MAX as usize * (point.y as usize / 2)
    }

    pub fn get_cell(&self, point: Point) -> &Cell {
        let position = Self::get_position(point);

        self.cells
            .get(position)
            .unwrap_or_else(|| panic!("cell at {} is out of range", point))
    }

    pub fn write_character(&mut self, point: Point, character: char) {
        let cell = self.get_mut_cell(point);
        *cell = Cell {
            upper_block: None,
            lower_block: None,
            character: Some(character),
            ..*cell
        }
    }

    fn get_mut_cell(&mut self, point: Point) -> &mut Cell {
        let position = Self::get_position(point);

        self.cells
            .get_mut(position)
            .unwrap_or_else(|| panic!("cell at {} is out of range", point))
    }

    fn get_color(&self, point: Point) -> Color {
        let cell = self.get_cell(point);
        if point.y % 2 == 0 {
            if let Some(color) = cell.upper_block {
                return color;
            }
        } else if let Some(color) = cell.lower_block {
            return color;
        }
        Color::default()
    }

    pub fn clear(&mut self) {
        self.cells.fill_with(Cell::default)
    }

    /// Draws a half block. This method is exposed publicly in a higher level method [`Canvas::block`].
    fn half_block(&mut self, point: Point, color: Color) {
        let current_cell = self.get_cell(point);
        if point.y % 2 == 0 {
            if let Some(lower_block_color) = current_cell.lower_block {
                self.terminal.set_background_color(lower_block_color);
            }
            self.terminal.write("▀");
            let current_cell = self.get_mut_cell(point); // TODO: can a second `get` be avoided?
            *current_cell = Cell {
                upper_block: Some(color),
                upper_point: point,
                character: None,
                ..*current_cell
            };
        } else {
            if let Some(upper_block_color) = current_cell.upper_block {
                self.terminal.set_background_color(upper_block_color);
            }
            self.terminal.write("▄");
            let current_cell = self.get_mut_cell(point); // TODO: can a second `get` be avoided?
            *current_cell = Cell {
                lower_block: Some(color),
                lower_point: point,
                character: None,
                ..*current_cell
            }
        }
    }

    pub fn redraw(&mut self) {
        for cell in &self.cells.clone() {
            self.redraw_cell(cell);
        }
    }

    pub fn redraw_cell(&mut self, cell: &Cell) {
        if let Some(character) = cell.character {
            self.terminal.write(&character.to_string());
        } else {
            if let Some(upper_block_color) = cell.upper_block {
                self.block(cell.upper_point, upper_block_color);
            }
            if let Some(lower_block_color) = cell.lower_block {
                self.block(cell.lower_point, lower_block_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_point() {
        let mut canvas = Canvas::new();
        let point = Point { x: 0, y: 0 };
        let color = Color::Red;
        canvas.half_block(point, color);
        assert_eq!(canvas.get_color(point), color);
        assert_ne!(canvas.get_color(Point { x: 1, y: 0 }), color);
        assert_ne!(canvas.get_color(Point { x: 0, y: 1 }), color);

        canvas.clear();
        let point = Point { x: 0, y: 1 };
        let color = Color::Green;
        canvas.half_block(point, color);
        assert_eq!(canvas.get_color(point), color);
        assert_ne!(canvas.get_color(Point { x: 0, y: 0 }), color);
        assert_ne!(canvas.get_color(Point { x: 0, y: 2 }), color);

        canvas.clear();
        let point = Point { x: 5, y: 3 };
        let color = Color::Blue;
        canvas.half_block(point, color);
        assert_eq!(canvas.get_color(point), color);
        assert_ne!(canvas.get_color(Point { x: 5, y: 2 }), color);
        assert_ne!(canvas.get_color(Point { x: 5, y: 4 }), color);
    }
}
