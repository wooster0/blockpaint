use crate::event::State;
use crate::terminal::{
    self,
    event::{Event, KeyEvent, KeyModifier},
    Terminal,
};
use crate::{canvas::palette, input};

// We need to handle text input manually and can't read directly from the standard input stream
// because it blocks all other input
pub fn handle_input(
    terminal: &mut Terminal,
    event: &Event,
    input_field: &mut input::Field,
    state: &mut State,
) -> bool {
    if let Event::Key(key) = event {
        match key {
            KeyEvent::Char(char, _) => {
                if input_field.input().len() != palette::GRAYSCALE_COLOR_COUNT as usize {
                    let parsed_color = input_field.write(*char);
                    if let Some(new_color) = parsed_color {
                        state.left_color = new_color;
                    }
                    input_field.redraw(terminal);
                    terminal.show_cursor();
                    terminal.flush();
                }
            }
            KeyEvent::Backspace(modifier) => {
                let parsed_color = if let Some(KeyModifier::Control) = modifier {
                    input_field.remove_word_to_left_of_cursor()
                } else {
                    input_field.remove_char()
                };
                if let Some(new_color) = parsed_color {
                    state.left_color = new_color;
                }
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
                terminal.flush();
            }
            _ => return false,
        }
        true
    } else {
        false
    }
}
