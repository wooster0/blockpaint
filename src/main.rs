mod canvas;
mod debug;
mod event;
mod input;
mod terminal;
mod util;

fn main() {
    let mut terminal = terminal::Terminal::new();

    terminal.set_title("BlockPaint (Untitled)");

    terminal.initialize();
    event::main_loop(&mut terminal);
    terminal.deinitialize();
}
