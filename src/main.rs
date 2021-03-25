mod canvas;
mod debug;
mod event;
mod terminal;
mod util;
pub mod window;

fn main() {
    let mut terminal = terminal::Terminal::new();

    terminal.set_title("BlockPaint (Untitled)");

    terminal.initialize();
    let temporary_terminal = terminal::Terminal::new();
    event::r#loop(temporary_terminal);
    terminal.deinitialize();
}
