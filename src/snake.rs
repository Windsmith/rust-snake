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

    pub fn move_snake(&mut self) {

        let body_positions: Vec<(usize, usize)> = self.body.iter()
                                                    .map(|part| (part.x, part.y))
                                                    .collect();

        for (index, body_part) in self.body.iter_mut().enumerate() {
            if index == 0 {
                body_part.x = self.x;
                body_part.y = self.y;
            }
            else {
                body_part.x = body_positions[index-1].0;
                body_part.y = body_positions[index-1].1;
            }
        }

        match self.direction {
            Direction::Up => {
                if self.y == 0 { self.y = 19 }
                else { self.y -= 1 }
            },
            Direction::Down => {
                if self.y == 19 { self.y = 0 }
                else { self.y += 1 }
            },
            Direction::Left => {
                if self.x == 0 { self.x = 79 }
                else { self.x -= 1 }
            }
            Direction::Right => {
                if self.x == 79 { self.x = 0 }
                else { self.x += 1 }
            }
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
