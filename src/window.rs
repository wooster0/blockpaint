use crate::canvas::Canvas;
use crate::terminal::{Terminal, SIZE};
use crate::util::{Color, Point};

pub fn draw(terminal: &mut Terminal, mut size: SIZE, color: Color) -> Point {
    size += 2; // Ignore the border

    let width = size;
    let height = size / 2;
    let x = terminal.size.width / 2 - width / 2;
    let mut y = terminal.size.height / 2 - height / 2;
    let previous_y = y;

    terminal.set_foreground_color(color);

    terminal.set_cursor(x, y);

    // let temporary_terminal = Terminal::new();
    // let mut canvas = Canvas::new(temporary_terminal);

    terminal.write("╔");
    for _ in 2..width {
        terminal.write("═");
    }
    terminal.write("╗");

    for _ in 2..height {
        y += 1;
        terminal.set_cursor(x, y);
        terminal.write("║");
        terminal.set_cursor(x + width - 1, y);
        terminal.write("║");
    }

    terminal.set_cursor(x, y + 1);
    terminal.write("╚");
    for _ in 2..width {
        terminal.write("═");
    }
    terminal.write("╝");

    y = previous_y;

    Point { x, y }
}

// struct Window {
//     x: SIZE,
//     y: SIZE,
//     width: SIZE,
//     height: SIZE,
// }

// impl Window {
//     pub fn new(terminal: &Terminal, width: SIZE, height: SIZE) -> Self {
//         Self {
//             x: terminal.size.width / 2 - width / 2,
//             y: terminal.size.height / 2 - height / 2,
//             width,
//             height,
//         }
//     }

//     pub fn draw(&self) {
//         terminal.set_cursor(self.x, self.y);
//         terminal.write("┌");
//         for _ in 0..self.width {
//             terminal.write("─");
//         }
//         terminal.write("┐");
//         for index in 1..self.height {
//             terminal.set_cursor(self.x, self.y + index);
//             terminal.write("│");
//             terminal.set_cursor(self.x + self.width + 1, self.y + index);
//             terminal.write("│");
//         }
//         terminal.set_cursor(self.x, self.y + self.height);
//         terminal.write("└");
//         for _ in 0..self.width {
//             terminal.write("─");
//         }
//         terminal.write("┘");
//         terminal.flush();
//     }
// }

// trait Window {}
