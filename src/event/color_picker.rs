use crate::{
    canvas::Canvas,
    canvas::Cell,
    terminal::{
        event::{Event, EventKind, MouseButton, MouseEvent},
        Terminal,
    },
    util::{Color, Point},
};

pub fn handle_events(
    terminal: &mut Terminal,
    canvas: &mut Canvas,
    state: &mut crate::event::State,
    initial_point: Point,
) {
    let (cell, color) = get_cell_and_color(canvas, initial_point);
    draw_indicator(terminal, canvas, initial_point, &cell.clone(), color);

    while let Some(event) = terminal.read_event() {
        match event {
            Event::Mouse(MouseEvent { kind, point }) => {
                let (cell, color) = get_cell_and_color(canvas, point);

                match kind {
                    EventKind::Release(mouse_button) => {
                        match mouse_button {
                            MouseButton::Left => {
                                state.left_color = color;
                            }
                            MouseButton::Right => {
                                state.right_color = color;
                            }
                            _ => {}
                        }
                        terminal.flush();
                        break;
                    }
                    _ => {
                        draw_indicator(terminal, canvas, point, &cell.clone(), color);
                    }
                }
            }
            _ => {}
        }
    }
}

/// Draws the indicator that indicates where the pointer is at.
fn draw_indicator(
    terminal: &mut Terminal,
    canvas: &mut Canvas,
    point: Point,
    cell: &Cell,
    color: Color,
) {
    terminal.set_cursor(point);
    terminal.set_background_color(color.invert());
    terminal.write(" ");
    terminal.flush();

    // Clear that same spot for the next flush
    terminal.reset_colors();
    terminal.set_cursor(point);
    if cell.upper_block.is_some() || cell.lower_block.is_some() {
        canvas.redraw_cell(&cell.clone());
    } else {
        terminal.write(" ");
    }
}

fn get_cell_and_color(canvas: &mut Canvas, point: Point) -> (Cell, Color) {
    let cell = canvas.get_cell(Point {
        y: point.y * 2,
        ..point
    });
    let color = cell.upper_block.or(cell.lower_block).unwrap_or_default();
    (cell.clone(), color)
}
