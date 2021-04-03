use crate::{
    canvas::{tools, Canvas},
    palette::{self, colors::ClickableColor},
    terminal::{self, Terminal, SIZE},
    util::{Color, Point, Size},
};
use terminal::event::{Event, EventKind, KeyEvent, KeyModifier, MouseButton, MouseEvent};
mod color_picker;
pub mod input;
mod key_movement;
mod undo_redo;

#[derive(Clone, Default)]
pub struct State {
    pub last_point: Option<Point>,
    pub left_color: Color,
    pub right_color: Color,
    pub input_field_color: Option<Color>,
    pub tool: tools::Tool,
    pub tool_size: SIZE,
}

pub fn main_loop(terminal: &mut Terminal) {
    // The main canvas for the image
    let mut primary_canvas = Canvas::new();
    // The secondary canvas for things like the palette
    let mut secondary_canvas = Canvas::new();

    let mut save_input_field: Option<crate::input::Field> = None;
    let mut clickable_colors = Vec::<ClickableColor>::new();
    let mut undo_redo_buffer = undo_redo::UndoRedoBuffer::new();
    // The `Point` doesn't matter here because it's re-set every time the palette is opened
    let mut palette_input_field = crate::input::Field::new(Point { x: 0, y: 0 }, String::new());

    let mut state = State {
        left_color: Color::White,
        tool_size: 1,
        ..Default::default()
    };

    while let Some(event) = terminal.read_event() {
        print_diagnostics(terminal);

        if undo_redo::handle(&event, terminal, &mut primary_canvas, &mut undo_redo_buffer) {
            continue;
        }

        if key_movement::handle(&event) {
            continue;
        }

        match event {
            Event::Mouse(MouseEvent { kind, point }) => match kind {
                EventKind::Drag(button) | EventKind::Press(button) => {
                    let color = match button {
                        MouseButton::Left => state.left_color,
                        MouseButton::Right => state.right_color,
                        _ => continue,
                    };
                    let point = Point {
                        y: point.y * 2,
                        ..point
                    };
                    undo_redo_buffer.push(undo_redo::Operation {
                        tool: state.tool.clone(),
                        start: point,
                        end: state.last_point,
                        color,
                        size: state.tool_size,
                    });
                    state.tool.draw(
                        &mut primary_canvas,
                        point,
                        state.last_point,
                        color,
                        state.tool_size,
                    );
                    state.last_point = Some(point);
                    terminal.flush();
                }
                EventKind::Release(MouseButton::Middle) => {
                    color_picker::handle_events(terminal, &mut primary_canvas, &mut state);
                }
                EventKind::ScrollUp => {
                    state.tool_size += 1;
                }
                EventKind::ScrollDown => {
                    if state.tool_size != 1 {
                        state.tool_size -= 1
                    }
                }
                _ => {
                    state.last_point = None;
                }
            },
            Event::Key(key) => match key {
                KeyEvent::Tab => {
                    let palette_point =
                        palette::colors::draw(terminal, &mut clickable_colors, &state);

                    palette_input_field.set_point(palette_point);
                    palette::events::handle(
                        terminal,
                        &mut clickable_colors,
                        &mut state,
                        &mut palette_input_field,
                    );
                    terminal.clear();
                    primary_canvas.redraw();
                    clickable_colors.clear();
                    terminal.hide_cursor();
                    terminal.flush();
                }
                KeyEvent::Char(tool @ '1'..='9', _) => {
                    use tools::Tool::*;
                    state.tool = match tool {
                        '1' => Brush,
                        '2' => Quill,
                        '3' => Rectangle,
                        '4' => Bucket,
                        _ => todo!(),
                    };
                }
                KeyEvent::Char('s', modifier) => {
                    if let Some(KeyModifier::Control) = modifier {
                        if save_input_field.is_none() {
                            let rows = 3;
                            let size = Size {
                                width: 20,
                                height: rows * 2,
                            };
                            let border_point = terminal.get_centered_border_point(&size);
                            secondary_canvas.hollow_rectangle(border_point, size, Color::White);
                            terminal.flush();
                            let input_field = crate::input::Field::new(
                                Point {
                                    x: border_point.x + 1,
                                    y: border_point.y + 1,
                                },
                                String::new(),
                            );
                            save_input_field = Some(input_field);
                            if let Some(mut input_field) = save_input_field {
                                loop {
                                    if let Some(event) = terminal.read_event() {
                                        input::handle(
                                            &event,
                                            terminal,
                                            &mut input_field,
                                            &mut state,
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                    }
                }
                KeyEvent::Char('c', Some(KeyModifier::Control)) => break,
                KeyEvent::Esc => break,
                _ => {}
            },
            Event::Resize(size) => {
                terminal.size = size.clone();
                primary_canvas.resize_terminal(size.clone());
                secondary_canvas.resize_terminal(size);

                terminal.clear();
                primary_canvas.redraw();
                // if palette.is_some() {
                //     secondary_canvas.redraw();
                //     palette::draw(terminal, &mut clickable_colors, &state);
                // }
                terminal.flush();
            }
        }
    }
}

#[cfg(debug_assertions)]
fn print_diagnostics(terminal: &mut Terminal) {
    terminal.set_cursor(Point { x: 0, y: 0 });
    terminal.write(&format!("Flush count: {}", terminal.flush_count));
}

#[cfg(not(debug_assertions))]
fn print_diagnostics(terminal: &mut Terminal) {}
