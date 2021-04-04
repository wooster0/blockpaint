use crate::{
    palette,
    terminal::{Terminal, SIZE},
    util::Point,
};

#[derive(Clone, Debug)]
pub struct Field {
    point: Point,
    // Input must only be mutated through the methods provided so that we can update accordingly
    input: String,
    x_center: SIZE,
    pub cursor_x: SIZE,
}

impl Field {
    pub fn new(point: Point, string: String) -> Self {
        Field {
            x_center: Self::x_center(point.x, 0),
            point,
            cursor_x: string.len() as SIZE,
            input: string,
        }
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn set_point(&mut self, point: Point) {
        self.point = point;
        self.x_center = Self::x_center(point.x, 0);
    }

    pub fn x_center(x: SIZE, input_len: usize) -> SIZE {
        x + palette::SIZE.width / 2 - input_len as SIZE / 2
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn redraw(&self, terminal: &mut Terminal) {
        terminal.set_cursor(self.point);
        terminal.write(&" ".repeat(palette::INPUT_FIELD_WIDTH as usize));
        terminal.set_cursor(Point {
            x: self.x_center - 1,
            ..self.point
        });
        terminal.write(&self.input);
    }

    pub fn update(&mut self) {
        self.x_center = Self::x_center(self.point.x, self.input.len());
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor_x = 0;
        self.update();
    }

    pub fn remove_word_to_left_of_cursor(&mut self) {
        if let Some(space_index) = self
            .input
            .trim_end_matches(|char: char| !char.is_ascii_alphanumeric())
            .rfind(|char: char| !char.is_ascii_alphanumeric())
        {
            let cut_off_length = self.input[space_index..].len();
            self.input = self.input[..space_index].to_string();
            self.cursor_x -= cut_off_length as SIZE;
        } else {
            self.input.clear();
            self.cursor_x = 0;
        }
        self.update()
    }

    pub fn move_cursor_to_word_to_left(&mut self) {
        if let Some(space_index) = self.input[..self.cursor_x as usize]
            .trim_end_matches(|char: char| !char.is_ascii_alphanumeric())
            .rfind(|char: char| !char.is_ascii_alphanumeric())
        {
            self.cursor_x = space_index as SIZE;
        } else {
            self.cursor_x = 0;
        }
        self.update()
    }

    pub fn move_cursor_to_word_to_right(&mut self) {
        if let Some(space_index) = self.input[self.cursor_x as usize..]
            .trim_start_matches(|char: char| !char.is_ascii_alphanumeric())
            .find(|char: char| !char.is_ascii_alphanumeric())
        {
            self.cursor_x = space_index as SIZE;
        } else {
            self.cursor_x = self.input.len() as SIZE;
        }
        self.update()
    }

    pub fn remove_char(&mut self) {
        if self.cursor_x != 0 {
            if self.cursor_x == self.input.len() as SIZE {
                self.input.pop();
            } else {
                self.input.remove(self.cursor_x as usize);
            }
            self.cursor_x -= 1;
            self.update()
        }
    }

    pub fn write(&mut self, char: char) {
        self.input.insert(self.cursor_x as usize, char);
        self.cursor_x += 1;
        self.update()
    }
}
