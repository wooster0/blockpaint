mod canvas;
mod debug;
mod event;
mod input;
mod terminal;
mod util;

const INSTRUCTIONS: [&str; 3] = [
    "Draw using the left and right mouse button.",
    "Pick a color from the canvas using the middle mouse button.",
    "Toggle the palette using tab and select colors with the left or right mouse button.",
];

fn main() {
    let mut terminal = terminal::Terminal::new();

    terminal.set_title("BlockPaint (Untitled)");

    terminal.initialize();
    for (index, instruction) in INSTRUCTIONS.iter().enumerate() {
        terminal.set_cursor(util::Point {
            x: 0,
            y: index as terminal::SIZE,
        });
        terminal.write(instruction);
    }
    terminal.flush();
    event::main_loop(&mut terminal);
    terminal.deinitialize();
}
