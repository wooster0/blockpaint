pub mod input;
pub mod palette;
pub mod tools;
use crate::{
    terminal::{self},
    util::Color,
};

pub struct Canvas {
    pub terminal: terminal::Terminal,
    pub cells: Vec<Cell>,
}

#[derive(Clone)]
pub struct Cell {
    pub upper_block: Option<Color>,
    pub lower_block: Option<Color>,
    pub x: terminal::SIZE,
    pub y: terminal::SIZE,
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
    pub fn new(terminal: terminal::Terminal) -> Self {
        Self {
            terminal,
            cells: vec![Cell::default(); (terminal::SIZE::MAX as usize).pow(2)],
        }
    }

    pub fn half_block(&mut self, x: terminal::SIZE, y: terminal::SIZE, color: Color) {
        let position = x as usize + terminal::SIZE::MAX as usize * (y as usize / 2);

        let current_cell = &self
            .cells
            .get(position)
            .unwrap_or_else(|| panic!("coloring block at ({}, {}) (out of range)", x, y));

        self.terminal.set_foreground_color(color);
        if y % 2 == 0 {
            if let Some(lower_block_color) = current_cell.lower_block {
                self.terminal.set_background_color(lower_block_color);
            }
            self.terminal.write("▀");
            self.cells[position] = Cell {
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
            self.cells[position] = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(color),
                x,
                y,
            }
        }
        self.terminal.reset_colors();
    }

    pub fn redraw(&mut self) {
        for index in 0..self.cells.len() {
            let cell = &self.cells[index].clone();
            if let Some(upper_block_color) = cell.upper_block {
                self.dot(cell.x, cell.y, upper_block_color);
            }
            if let Some(lower_block_color) = cell.lower_block {
                self.dot(cell.x, cell.y, lower_block_color);
            }
        }
        self.terminal.flush();
    }
}
