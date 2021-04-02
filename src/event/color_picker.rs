use crate::{
    canvas::Canvas,
    terminal::{
        event::{Event, EventKind, MouseButton, MouseEvent},
        Terminal,
    },
    util::Point,
};

pub fn handle_events(
    terminal: &mut Terminal,
    canvas: &mut Canvas,
    state: &mut crate::event::State,
) {
    terminal.show_cursor();
    while let Some(event) = terminal.read_event() {
        match event {
            Event::Mouse(MouseEvent { kind, point }) => match kind {
                EventKind::Release(mouse_button) => {
                    let cell = canvas.get_cell(Point {
                        y: point.y * 2,
                        ..point
                    });
                    let color = cell.upper_block.or(cell.lower_block).unwrap_or_default();
                    match mouse_button {
                        MouseButton::Left => {
                            state.left_color = color;
                        }
                        MouseButton::Right => {
                            state.right_color = color;
                        }
                        _ => {}
                    }
                    terminal.hide_cursor();
                    terminal.flush();
                    break;
                }
                _ => {
                    terminal.set_cursor(point);
                    terminal.flush();
                }
            },
            _ => {}
        }
    }
}
