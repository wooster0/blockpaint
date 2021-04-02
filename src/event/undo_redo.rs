use crate::{
    canvas::{tools, Canvas},
    terminal::{
        event::{Event, KeyEvent},
        Terminal, SIZE,
    },
    util::{Color, Point},
};

pub struct Operation {
    pub tool: tools::Tool,
    pub start: Point,
    pub end: Option<Point>,
    pub color: Color,
    pub size: SIZE,
}

pub struct UndoRedoBuffer {
    buffer: Vec<Operation>,
    cursor: usize,
}

impl UndoRedoBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::<Operation>::new(),
            cursor: 0,
        }
    }

    pub fn push(&mut self, operation: Operation) {
        if self.cursor != self.buffer.len() {
            self.buffer.truncate(self.cursor);
        }
        self.buffer.push(operation);
        self.cursor += 1;
    }

    fn undo(&mut self, canvas: &mut Canvas, terminal: &mut Terminal) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.draw(canvas, terminal);
    }

    fn redo(&mut self, canvas: &mut Canvas, terminal: &mut Terminal) {
        if self.cursor == self.buffer.len() {
            return;
        }
        self.cursor += 1;
        self.draw(canvas, terminal);
    }

    fn draw(&mut self, canvas: &mut Canvas, terminal: &mut Terminal) {
        canvas.clear();
        terminal.clear();
        for operation in &mut self.buffer[..self.cursor] {
            operation.tool.draw(
                canvas,
                operation.start,
                operation.end,
                operation.color,
                operation.size,
            );
        }
        terminal.flush();
    }
}

pub fn handle(
    event: &Event,
    terminal: &mut Terminal,
    canvas: &mut Canvas,
    undo_redo_buffer: &mut UndoRedoBuffer,
) -> bool {
    if let Event::Key(key) = event {
        match key {
            KeyEvent::Char('z', _) | KeyEvent::Char('Z', _) => {
                undo_redo_buffer.undo(canvas, terminal);
            }
            KeyEvent::Char('y', _) | KeyEvent::Char('Y', _) => {
                undo_redo_buffer.redo(canvas, terminal);
            }
            _ => return false,
        }
        terminal.flush();
        true
    } else {
        false
    }
}
