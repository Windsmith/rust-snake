use crate::{ROWS, COLS, TerminalDraw, frame::Frame};

pub struct Snake {
    x: usize,
    y: usize,
    body: Vec<BodyPart>,
    direction: Direction 
}

struct BodyPart {
    x: usize,
    y: usize
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new()-> Self {
        let pos_x = COLS/2;
        let pos_y = ROWS/2;
        Snake {
            x: pos_x,
            y: pos_y,
            body: vec![
                BodyPart {x: pos_x + 1, y: pos_y},
                BodyPart {x: pos_x + 2, y: pos_y},
                BodyPart {x: pos_x + 3, y: pos_y}
                ],
            direction: Direction::Left
        }
    }
}

impl TerminalDraw for Snake {
    fn draw(&self, frame: &mut Frame) {
        frame.frame[self.x][self.y] = '@';
        for part in &self.body {
            frame.frame[part.x][part.y] = 'o';
        }
    }
}
