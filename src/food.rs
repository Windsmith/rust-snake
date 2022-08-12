use rand::Rng;

use crate::{snake::Snake, COLS, ROWS, TerminalDraw, frame::Frame};

pub struct Food {
    x: usize,
    y: usize
}

impl Food {
    pub fn new() -> Self {
        Food {x:0, y:0}
    }

    pub fn change_location(&mut self, snake: &Snake) {
        let invalid_xs = snake.get_xs();
        let invalid_ys = snake.get_ys();

        let mut rng = rand::thread_rng();

        let mut x;
        let mut y;

        loop {
            x = rng.gen_range(0..COLS);
            if !invalid_xs.contains(&x) { break; }
        }

        loop {
            y = rng.gen_range(0..ROWS);
            if !invalid_xs.contains(&y) { break; }
        }

        self.x = x;
        self.y = y;
    }
}

impl TerminalDraw for Food {
    fn draw(&self, frame: &mut Frame) {
        frame.frame[self.x][self.y] = '*';
    }
}