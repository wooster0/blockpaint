use super::Canvas;
use crate::{
    terminal::SIZE,
    util::{Color, Point, Size},
};

mod bucket;

impl Canvas {
    /// Draws a block.
    pub fn block(&mut self, point: Point, color: Color) {
        self.terminal.set_cursor(Point {
            y: point.y / 2,
            ..point
        });
        self.terminal.set_foreground_color(color);
        self.half_block(point, color);
        self.terminal.reset_colors();
    }

    /// Efficiently draws multiple blocks in a row.
    pub fn blocks(&mut self, point: Point, color: Color, count: SIZE) {
        self.terminal.set_cursor(Point {
            y: point.y / 2,
            ..point
        });
        self.terminal.set_foreground_color(color);
        for index in 0..count {
            self.half_block(
                Point {
                    x: point.x + index,
                    ..point
                },
                color,
            );
        }
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
            1 => self.block(point, color), // Middle dot
            2 => {
                self.block(
                    // Left dot
                    Point {
                        y: point.y - 1,
                        ..point
                    },
                    color,
                );
                self.block(
                    // Upper dot
                    Point {
                        x: point.x - 1,
                        ..point
                    },
                    color,
                );
                self.block(point, color); // Middle dot
                self.block(
                    // Lower dot
                    Point {
                        x: point.x + 1,
                        ..point
                    },
                    color,
                );
                self.block(
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
            }
        }
    }

    pub fn quill(&mut self, point: Point, color: Color, size: SIZE) {
        for size in 0..=size {
            if size % 2 == 0 {
                self.block(
                    Point {
                        y: point.y + size / 2,
                        ..point
                    },
                    color,
                );
            } else {
                self.block(
                    Point {
                        y: point.y - size / 2,
                        ..point
                    },
                    color,
                );
            }
        }
    }
}

#[derive(Clone)]
pub enum Tool {
    Brush,
    Quill,
    Rectangle,
    Bucket,
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
        }
    }
}
