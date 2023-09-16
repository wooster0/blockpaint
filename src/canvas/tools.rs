use super::Canvas;
use crate::{
    terminal::SIZE,
    util::{Color, Point, Size},
};

mod bucket;

impl Canvas {
    /// Sets the terminal cursor accordingly and then draws a block.
    pub fn block_at(&mut self, point: Point, color: Color) {
        self.terminal.set_cursor(Point {
            y: point.y / 2,
            ..point
        });
        self.half_block(point, color);
    }

    /// Draws a block.
    pub fn block(&mut self, point: Point, color: Color) {
        self.terminal.set_foreground_color(color);
        self.block_at(point, color);
        self.terminal.reset_colors();
    }

    /// Sets the terminal cursor accordingly and then efficiently draws multiple blocks in a row.
    pub fn blocks_at(&mut self, point: Point, color: Color, count: SIZE) {
        self.terminal.set_cursor(Point {
            y: point.y / 2,
            ..point
        });
        for index in 0..count {
            self.half_block(
                Point {
                    x: point.x + index,
                    ..point
                },
                color,
            );
        }
    }

    /// Efficiently draws multiple blocks in a row.
    pub fn blocks(&mut self, point: Point, color: Color, count: SIZE) {
        self.terminal.set_foreground_color(color);
        self.blocks_at(point, color, count);
        self.terminal.reset_colors();
    }

    pub fn line(
        &self,
        x1: SIZE,
        y1: SIZE,
        x2: SIZE,
        y2: SIZE,
    ) -> bracket_geometry::prelude::Bresenham {
        use bracket_geometry::prelude::{Bresenham, Point};
        Bresenham::new(Point::new(x1, y1), Point::new(x2, y2))
    }

    pub fn brush(&mut self, point: Point, color: Color, size: SIZE) {
        match size {
            1 => {
                self.terminal.set_foreground_color(color);
                self.block_at(point, color); // Middle dot
            }
            2 => {
                self.terminal.set_foreground_color(color);
                self.block_at(
                    // Left dot
                    Point {
                        y: point.y - 1,
                        ..point
                    },
                    color,
                );
                self.block_at(
                    // Upper dot
                    Point {
                        x: point.x - 1,
                        ..point
                    },
                    color,
                );
                self.block_at(point, color); // Middle dot
                self.block_at(
                    // Lower dot
                    Point {
                        x: point.x + 1,
                        ..point
                    },
                    color,
                );
                self.block_at(
                    // Right dot
                    Point {
                        y: point.y + 1,
                        ..point
                    },
                    color,
                );
            }
            _ => {
                self.circle(point, color, size - 1);
                return; // `circle` has already reset the color
            }
        }
        self.terminal.reset_colors();
    }

    pub fn quill(&mut self, point: Point, color: Color, size: SIZE) {
        self.terminal.set_foreground_color(color);
        for size in 0..=size {
            if size % 2 == 0 {
                self.block_at(
                    Point {
                        y: point.y + size / 2,
                        ..point
                    },
                    color,
                );
            } else {
                self.block_at(
                    Point {
                        y: point.y - size / 2,
                        ..point
                    },
                    color,
                );
            }
        }
        self.terminal.reset_colors();
    }
}

#[derive(Clone, PartialEq)]
pub enum Tool {
    Brush,
    Quill,
    Rectangle,
    Bucket,
    Text,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Brush
    }
}

impl Tool {
    pub fn draw(
        &self,
        canvas: &mut Canvas,
        point: Point,
        last_point: Option<Point>,
        color: Color,
        tool_size: SIZE,
    ) {
        if let Some(last_point) = last_point {
            if last_point == point {
                self.r#use(canvas, point, color, tool_size);
            } else {
                for draw_point in canvas.line(last_point.x, last_point.y, point.x, point.y) {
                    let draw_point = Point {
                        x: draw_point.x as SIZE,
                        y: draw_point.y as SIZE,
                    };
                    self.r#use(canvas, draw_point, color, tool_size);
                }
            }
        } else {
            self.r#use(canvas, point, color, tool_size);
        }
    }

    fn r#use(&self, canvas: &mut Canvas, point: Point, color: Color, size: SIZE) {
        match self {
            Tool::Brush => {
                canvas.brush(point, color, size);
            }
            Tool::Quill => {
                canvas.quill(point, color, size);
            }
            Tool::Rectangle => {
                canvas.hollow_rectangle(
                    point,
                    Size {
                        width: size,
                        height: size,
                    },
                    color,
                );
            }
            Tool::Bucket => {
                canvas.bucket(point, color);
            }
            Tool::Text => {
                // This is handled in src/event.rs.
            }
        }
    }
}
