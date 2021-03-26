pub mod border;
pub mod input;
pub mod palette;
pub mod shapes;
pub mod tools;
use crate::{
    terminal::{Terminal, SIZE},
    util::Color,
};

pub struct Canvas {
    pub terminal: Terminal,
    pub cells: Vec<Cell>,
}

#[derive(Clone)]
pub struct Cell {
    pub upper_block: Option<Color>,
    pub lower_block: Option<Color>,
    pub x: SIZE,
    pub y: SIZE,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            upper_block: None,
            lower_block: None,
            x: 0,
            y: 0,
        }
    }
}

impl Canvas {
    pub fn new(terminal: Terminal) -> Self {
        Self {
            terminal,
            cells: vec![Cell::default(); (SIZE::MAX as usize).pow(2)],
        }
    }

    fn get_mut_cell(&mut self, x: SIZE, y: SIZE) -> &mut Cell {
        let position = x as usize + SIZE::MAX as usize * (y as usize / 2);

        self.cells
            .get_mut(position)
            .unwrap_or_else(|| panic!("cell at ({}, {}) is out of range", x, y))
    }

    /// Draws a half block. This method is exposed publicly in a higher level method [`Canvas::block`].
    fn half_block(&mut self, x: SIZE, y: SIZE, color: Color) {
        let current_cell = self.get_mut_cell(x, y);
        if y % 2 == 0 {
            if let Some(lower_block_color) = current_cell.lower_block {
                self.terminal.set_background_color(lower_block_color);
            }
            self.terminal.write("▀");
            let current_cell = self.get_mut_cell(x, y); // TODO: can this second `get` be avoided?
            *current_cell = Cell {
                upper_block: Some(color),
                lower_block: current_cell.lower_block,
                x,
                y,
            };
        } else {
            if let Some(upper_block_color) = current_cell.upper_block {
                self.terminal.set_background_color(upper_block_color);
            }
            self.terminal.write("▄");
            let current_cell = self.get_mut_cell(x, y); // TODO: can this second `get` be avoided?
            *current_cell = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(color),
                x,
                y,
            }
        }
    }

    fn redraw(&mut self) {
        for index in 0..self.cells.len() {
            let cell = self.cells[index].clone();
            if let Some(upper_block_color) = cell.upper_block {
                self.block(cell.x, cell.y, upper_block_color);
            }
            if let Some(lower_block_color) = cell.lower_block {
                self.block(cell.x, cell.y, lower_block_color);
            }
        }
        self.terminal.flush();
    }
}
