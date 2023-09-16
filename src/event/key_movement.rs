//! Movement of the drawing point using the WASD and arrow keys.

use crate::terminal::event::{Event, KeyEvent};

pub fn handle(event: &Event) -> bool {
    false
    //match event {
    //    Event::Key(key) => match key {
    //        KeyEvent::Char('w', _) => {}
    //        KeyEvent::Char('s', _) => {
    //            // We need to have this event available for the main event handler as well
    //            return false;
    //        }
    //        KeyEvent::Char('a', _) => {}
    //        KeyEvent::Char('d', _) => {}
    //        KeyEvent::Up => {}
    //        KeyEvent::Down => {}
    //        _ => return false,
    //    },
    //    _ => return false,
    //}
    //true
}
