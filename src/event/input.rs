use crate::{
    input, palette,
    terminal::{
        self,
        event::{Event, EventKind, KeyEvent, KeyModifier, MouseEvent},
        Terminal,
    },
};

// We need to handle text input manually and can't read directly from the standard input stream
// because it blocks all other input
pub fn handle(event: &Event, terminal: &mut Terminal, input_field: &mut input::Field) -> bool {
    match event {
        Event::Key(key) => match key {
            KeyEvent::Char(char, _) => {
                if input_field.input().len() != palette::INPUT_FIELD_WIDTH as usize {
                    input_field.write(*char);
                    input_field.redraw(terminal);
                    terminal.show_cursor();
                    terminal.flush();
                }
            }
            KeyEvent::Backspace(modifier) => {
                if let Some(KeyModifier::Control) = modifier {
                    input_field.remove_word_to_left_of_cursor()
                } else {
                    input_field.remove_char()
                };
                input_field.redraw(terminal);
                terminal.show_cursor();
                terminal.flush();
            }
            KeyEvent::Left(modifier) => {
                if let Some(KeyModifier::Control) = modifier {
                    input_field.move_cursor_to_word_to_left();
                    input_field.redraw(terminal);
                } else if input_field.cursor_x != 0 {
                    terminal.move_cursor_left(1);
                    input_field.cursor_x -= 1;
                }
                terminal.show_cursor();
                terminal.flush();
            }
            KeyEvent::Right(modifier) => {
                if let Some(KeyModifier::Control) = modifier {
                    input_field.move_cursor_to_word_to_right();
                    input_field.redraw(terminal);
                } else if input_field.cursor_x != input_field.input().len() as terminal::SIZE {
                    terminal.move_cursor_right(1);
                    input_field.cursor_x += 1;
                }
                terminal.show_cursor();
                terminal.flush();
            }
            _ => return false,
        },
        Event::Mouse(MouseEvent { kind, point: _ }) => {
            match kind {
                EventKind::Release(_) => {
                    terminal.hide_cursor();

                    // We need to have this event available for the palette event handler as well
                    return false;
                }
                _ => return false,
            }
        }
        _ => {
            return false;
        }
    }
    true
}
