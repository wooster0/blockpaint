use crate::canvas::{input, palette, tools, Canvas};
use crate::terminal::{self, Terminal};
use crate::util::{Color, Point};
use terminal::event::{Event, EventKind, KeyEvent, KeyModifier, MouseButton, MouseEvent};

pub fn r#loop(terminal: Terminal) {
    let mut canvas = Canvas::new(terminal);
    let temporary_terminal = Terminal::new();
    let mut palette_canvas = Canvas::new(temporary_terminal);

    let mut palette: Option<palette::Colors> = None;
    let mut input_field: Option<input::Field> = None;
    let mut clickable_colors = Vec::<palette::ClickableColor>::new();

    let mut last_point: Option<Point> = None;

    let mut color = Color::White;
    let mut active_tool = tools::Tool::Brush;
    let mut tool_size = 1;
    loop {
        let event = canvas.terminal.read_event();
        if let Some(event) = event {
            match event {
                Event::Mouse(MouseEvent { kind, x, y }) => match kind {
                    EventKind::Drag(button) | EventKind::Press(button) if palette.is_none() => {
                        match button {
                            MouseButton::Left => {
                                active_tool.draw(
                                    &mut canvas,
                                    x,
                                    y,
                                    color,
                                    tool_size,
                                    &mut last_point,
                                );
                                canvas.terminal.flush();
                            }
                            MouseButton::Right => {}
                            _ => {}
                        }
                    }
                    EventKind::Release(_) => {
                        if let Some(new_color) = palette::get_color(&clickable_colors, x, y) {
                            {
                                // temporary
                                canvas.terminal.reset_colors();
                                canvas.terminal.set_cursor(0, 0);
                                canvas.terminal.write(&format!(
                                    "{:02},{:02},{:?}             ",
                                    x, y, new_color
                                ));
                            }
                            color = new_color;

                            if let Some(input_field) = &mut input_field {
                                input_field.clear();
                                input_field.redraw(&mut canvas.terminal);
                                canvas.terminal.hide_cursor();
                            }
                            canvas.terminal.flush();
                        }
                        last_point = None;
                    }
                    EventKind::Move if palette.is_some() => {
                        if let Some(new_color) = palette::get_color(&clickable_colors, x, y) {
                            palette::draw_border(&mut palette_canvas, new_color);
                        } else {
                            palette::draw_border(&mut palette_canvas, color);
                        }
                        canvas.terminal.hide_cursor();
                        canvas.terminal.flush();
                        last_point = None;
                    }
                    EventKind::ScrollUp if palette.is_none() => tool_size += 1,
                    EventKind::ScrollDown if palette.is_none() => {
                        if tool_size != 1 {
                            tool_size -= 1
                        }
                    }
                    _ => {
                        last_point = None;
                    }
                },
                Event::Key(key) => match key {
                    KeyEvent::Tab => {
                        palette::toggle(
                            &mut canvas,
                            &mut palette_canvas,
                            &mut clickable_colors,
                            &mut palette,
                            &mut input_field,
                            color,
                        );
                        canvas.terminal.flush();
                    }
                    KeyEvent::Char(char, _) if palette.is_some() => {
                        // We need to handle input manually and can't read directly from the standard input stream
                        // because it blocks all other input
                        if let Some(input_field) = &mut input_field {
                            if input_field.input().len() != palette::GRAYSCALE_COLOR_COUNT as usize
                            {
                                let parsed_color = input_field.write(char);
                                if let Some(new_color) = parsed_color {
                                    color = new_color;
                                    palette::draw_border(&mut palette_canvas, new_color);
                                }
                                input_field.redraw(&mut canvas.terminal);
                                canvas.terminal.show_cursor();
                                canvas.terminal.flush();
                            }
                        }
                    }
                    KeyEvent::Backspace(modifier) => {
                        if let Some(input_field) = &mut input_field {
                            let parsed_color = if let Some(KeyModifier::Control) = modifier {
                                input_field.remove_word_to_left_of_cursor()
                            } else {
                                input_field.remove_char()
                            };
                            if let Some(new_color) = parsed_color {
                                color = new_color;
                                palette::draw_border(&mut palette_canvas, new_color);
                            }
                            input_field.redraw(&mut canvas.terminal);
                            canvas.terminal.show_cursor();
                            canvas.terminal.flush();
                        }
                    }
                    KeyEvent::Char(tool @ '1'..='9', _) => {
                        use tools::Tool::*;
                        active_tool = match tool {
                            '1' => Brush,
                            '2' => Quill,
                            '3' => Rectangle,
                            _ => todo!(),
                        };
                    }
                    KeyEvent::Char('w', _) => {}
                    KeyEvent::Char('s', modifier) => {
                        if let Some(KeyModifier::Control) = modifier {
                            //window::open(&mut canvas.terminal, 20);
                        } else {
                        }
                    }
                    KeyEvent::Char('a', _) => {}
                    KeyEvent::Char('d', _) => {}
                    KeyEvent::Char('c', Some(KeyModifier::Control)) => break,
                    KeyEvent::Left(Some(KeyModifier::Control)) => {
                        if let Some(input_field) = &mut input_field {
                            input_field.move_cursor_to_word_to_left();
                            input_field.redraw(&mut canvas.terminal);
                            canvas.terminal.show_cursor();
                            canvas.terminal.flush();
                        }
                    }
                    KeyEvent::Right(Some(KeyModifier::Control)) => {
                        if let Some(input_field) = &mut input_field {
                            input_field.move_cursor_to_word_to_right();
                            input_field.redraw(&mut canvas.terminal);
                            canvas.terminal.show_cursor();
                            canvas.terminal.flush();
                        }
                    }
                    KeyEvent::Left(_) => {
                        if let Some(input_field) = &mut input_field {
                            if input_field.cursor_x != 0 {
                                canvas.terminal.move_cursor_left(1);
                                canvas.terminal.flush();
                                input_field.cursor_x -= 1;
                            }
                        }
                    }
                    KeyEvent::Right(_) => {
                        if let Some(input_field) = &mut input_field {
                            if input_field.cursor_x != input_field.input().len() as terminal::SIZE {
                                canvas.terminal.move_cursor_right(1);
                                canvas.terminal.flush();
                                input_field.cursor_x += 1;
                            }
                        }
                    }
                    KeyEvent::Up => {}
                    KeyEvent::Down => {}
                    KeyEvent::Esc => break,
                    _ => {}
                },
                Event::Resize(size) => {
                    canvas.terminal.size = size.clone();
                    palette_canvas.terminal.size = size;
                    // if let Some(palette) = &palette {
                    //     canvas.terminal.clear();
                    //     canvas.redraw();
                    //     clickable_colors.clear();
                    //     palette.open(palette.x, palette.y, &mut canvas, &mut clickable_colors);
                    // }
                }
            }
        }
    }
}
