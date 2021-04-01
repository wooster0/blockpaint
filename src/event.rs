use crate::canvas::{palette, tools, Canvas};
use crate::terminal::{self, Terminal, SIZE};
use crate::util::{Color, Point, Size};
use terminal::event::{Event, EventKind, KeyEvent, KeyModifier, MouseButton, MouseEvent};
mod input;
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

    let mut palette: Option<palette::Colors> = None;
    let mut palette_input_field: Option<crate::input::Field> = None;
    let mut save_input_field: Option<crate::input::Field> = None;
    let mut clickable_colors = Vec::<palette::ClickableColor>::new();
    let mut undo_redo_buffer = undo_redo::UndoRedoBuffer::new();

    let mut state = State {
        left_color: Color::White,
        tool_size: 1,
        ..Default::default()
    };

    while let Some(event) = terminal.read_event() {
        if let (Some(input_field), _) | (_, Some(input_field)) =
            (&mut palette_input_field, &mut save_input_field)
        {
            if input::handle_input(terminal, &event, input_field, &mut state) {
                continue;
            }
        }

        if undo_redo::handle_undo_redo(terminal, &mut primary_canvas, &event, &mut undo_redo_buffer)
        {
            continue;
        }

        match event {
            Event::Mouse(MouseEvent { kind, x, y }) => {
                let point = Point { x, y };

                match kind {
                    EventKind::Drag(button) | EventKind::Press(button) if palette.is_none() => {
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
                    EventKind::Drag(button) if palette.is_some() => {
                        if let Some(new_color) = palette::get_color(&clickable_colors, point) {
                            {
                                // temporary
                                terminal.reset_colors();
                                terminal.set_cursor(Point { x: 0, y: 0 });
                                terminal.write(&format!(
                                    "{:02},{:02},{:?}             ",
                                    x, y, new_color
                                ));
                            }
                            match button {
                                MouseButton::Left => {
                                    palette::draw_left_color(terminal, new_color);
                                }
                                MouseButton::Right => {
                                    palette::draw_right_color(terminal, new_color);
                                }
                                _ => {}
                            }
                            terminal.flush();
                        }
                    }
                    EventKind::Release(MouseButton::Middle) if palette.is_none() => {
                        tools::color_picker(terminal, &mut primary_canvas, &mut state);
                    }
                    EventKind::Release(button) if palette.is_some() => {
                        if let Some(new_color) = palette::get_color(&clickable_colors, point) {
                            match button {
                                MouseButton::Left => {
                                    state.left_color = new_color;
                                    palette::draw_left_color(terminal, new_color);
                                }
                                MouseButton::Right => {
                                    state.right_color = new_color;
                                    palette::draw_right_color(terminal, new_color);
                                }
                                _ => {}
                            }
                        } else {
                            match button {
                                MouseButton::Left => {
                                    palette::draw_left_color(terminal, state.left_color);
                                }
                                MouseButton::Right => {
                                    palette::draw_right_color(terminal, state.right_color);
                                }
                                _ => {}
                            }
                        }
                        if let (Some(input_field), _) | (_, Some(input_field)) =
                            (&mut palette_input_field, &mut save_input_field)
                        {
                            terminal.reset_colors();
                            input_field.clear();
                            input_field.redraw(terminal);
                            terminal.hide_cursor();
                        }
                        terminal.flush();
                        state.last_point = None;
                    }
                    EventKind::Move if palette.is_some() => {
                        if let (Some(_), _) | (_, Some(_)) =
                            (&palette_input_field, &save_input_field)
                        {
                            terminal.hide_cursor();
                            terminal.flush();
                        }
                    }
                    EventKind::ScrollUp if palette.is_none() => state.tool_size += 1,
                    EventKind::ScrollDown if palette.is_none() => {
                        if state.tool_size != 1 {
                            state.tool_size -= 1
                        }
                    }
                    _ => {
                        state.last_point = None;
                    }
                }
            }
            Event::Key(key) => match key {
                KeyEvent::Tab => {
                    // loop {
                    //     let event = canvas.terminal.read_event();
                    //     match event {

                    //     }
                    // }
                    palette::toggle(
                        terminal,
                        &mut primary_canvas,
                        &mut clickable_colors,
                        &mut palette,
                        &mut palette_input_field,
                        &state,
                    );
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
                KeyEvent::Char('w', _) => {}
                KeyEvent::Char('s', modifier) => {
                    if let Some(KeyModifier::Control) = modifier {
                        if save_input_field.is_none() {
                            let rows = 3;
                            let size = Size {
                                width: 20,
                                height: rows * 2,
                            };
                            let border_point = terminal.get_centered_border_point(&size);
                            secondary_canvas.hollow_rectangle(
                                border_point,
                                size.width,
                                size.height,
                                Color::White,
                            );
                            terminal.flush();
                            let input_field = crate::input::Field::new(Point {
                                x: border_point.x + 1,
                                y: border_point.y + 1,
                            });
                            save_input_field = Some(input_field);
                            if let Some(mut input_field) = save_input_field {
                                loop {
                                    if let Some(event) = terminal.read_event() {
                                        input::handle_input(
                                            terminal,
                                            &event,
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
                KeyEvent::Char('a', _) => {}
                KeyEvent::Char('d', _) => {}
                KeyEvent::Char('c', Some(KeyModifier::Control)) => break,
                KeyEvent::Up => {}
                KeyEvent::Down => {}
                KeyEvent::Esc => break,
                _ => {}
            },
            Event::Resize(size) => {
                terminal.size = size.clone();
                primary_canvas.resize_terminal(size.clone());
                secondary_canvas.resize_terminal(size);

                terminal.clear();
                primary_canvas.redraw();
                if palette.is_some() {
                    secondary_canvas.redraw();
                    palette::draw(terminal, &mut clickable_colors, &state);
                }
                terminal.flush();
            }
        }
    }
}
