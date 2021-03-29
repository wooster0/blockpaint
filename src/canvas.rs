pub mod palette;
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

#[derive(Clone)]
pub struct Cell {
    pub upper_block: Option<Color>,
    pub lower_block: Option<Color>,
    pub point: Point,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            upper_block: Default::default(),
            lower_block: Default::default(),
            point: Default::default(),
        }
    }
}

impl Canvas {
    pub fn new() -> Self {
        // let (width, height) = (terminal.size.width, terminal.size.height);
        Self {
            cells: vec![Default::default(); (SIZE::MAX as usize).pow(2)],
            terminal: Terminal::new(),
        }
    }

    pub fn resize_terminal(&mut self, size: Size) {
        self.terminal.size = size;
    }

    fn get_point(point: Point) -> usize {
        point.x as usize + SIZE::MAX as usize * (point.y as usize / 2)
    }

    pub fn get_cell(&self, point: Point) -> &Cell {
        let position = Self::get_point(point);

        self.cells
            .get(position)
            .unwrap_or_else(|| panic!("cell at {} is out of range", point))
    }

    fn get_mut_cell(&mut self, point: Point) -> &mut Cell {
        let position = Self::get_point(point);

        self.cells
            .get_mut(position)
            .unwrap_or_else(|| panic!("cell at {} is out of range", point))
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
                lower_block: current_cell.lower_block,
                point,
            };
        } else {
            if let Some(upper_block_color) = current_cell.upper_block {
                self.terminal.set_background_color(upper_block_color);
            }
            self.terminal.write("▄");
            let current_cell = self.get_mut_cell(point); // TODO: can a second `get` be avoided?
            *current_cell = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(color),
                point,
            }
        }
    }

    pub fn redraw(&mut self) {
        for index in 0..self.cells.len() {
            let cell = self.cells[index].clone();
            if let Some(upper_block_color) = cell.upper_block {
                self.block(cell.point, upper_block_color);
            }
            if let Some(lower_block_color) = cell.lower_block {
                self.block(cell.point, lower_block_color);
            }
        }
    }
}
