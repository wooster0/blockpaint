use super::colors;
use crate::{
    input,
    terminal::{
        event::{Event, EventKind, KeyEvent, MouseButton, MouseEvent},
        Terminal,
    },
    util::Point,
};

pub fn handle(
    terminal: &mut Terminal,
    clickable_colors: &mut Vec<colors::ClickableColor>,
    state: &mut crate::event::State,
    input_field: &mut input::Field,
) {
    while let Some(event) = terminal.read_event() {
        if crate::event::input::handle(&event, terminal, input_field, state) {
            continue;
        }

        match event {
            Event::Mouse(MouseEvent { kind, point }) => {
                match kind {
                    EventKind::Release(button) => {
                        if let Some(selected_color) = colors::get_color(&clickable_colors, point) {
                            match button {
                                MouseButton::Left => {
                                    state.left_color = selected_color;
                                    colors::draw_left_color(terminal, selected_color);
                                }
                                MouseButton::Right => {
                                    state.right_color = selected_color;
                                    colors::draw_right_color(terminal, selected_color);
                                }
                                _ => {}
                            }
                        } else {
                            match button {
                                MouseButton::Left => {
                                    colors::draw_left_color(terminal, state.left_color);
                                }
                                MouseButton::Right => {
                                    colors::draw_right_color(terminal, state.right_color);
                                }
                                _ => {}
                            }
                        }
                        terminal.flush();
                        state.last_point = None;
                    }
                    EventKind::Drag(button) => {
                        if let Some(new_color) = colors::get_color(&clickable_colors, point) {
                            {
                                // temporary
                                terminal.reset_colors();
                                terminal.set_cursor(Point { x: 0, y: 0 });
                                terminal.write(&format!("{},{:?}             ", point, new_color));
                            }
                            match button {
                                MouseButton::Left => {
                                    colors::draw_left_color(terminal, new_color);
                                }
                                MouseButton::Right => {
                                    colors::draw_right_color(terminal, new_color);
                                }
                                _ => {}
                            }
                            terminal.flush();
                        }
                    }
                    _ => {}
                }
            }
            Event::Key(key) => match key {
                KeyEvent::Tab => {
                    return;
                }
                _ => {}
            },
            Event::Resize(_) => {}
        }
    }
}
