use crossbeam::{Sender};

use crate::{ROWS, COLS, TerminalDraw, frame::Frame, food::Food};

pub struct Snake {
    x: usize,
    y: usize,
    body: Vec<BodyPart>,
    pub direction: Direction 
}

struct BodyPart {
    x: usize,
    y: usize
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite_of(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
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

    /* Obsolete
    pub fn change_dir(&mut self, dir: &Direction)
    {   
        if !(*dir == Direction::opposite_of(self.direction)) {
            self.direction = *dir;
        }
        
    }
    */

    pub fn handle_potential_collisions(&mut self, food: &mut Food, tx: &Sender<bool>) -> bool {
        let mut inc_speed = false;
        
        if food.x == self.x && food.y == self.y {
            food.change_location(self);
            self.body.push(BodyPart {x: self.body[self.body.len()-1].x, y: self.body[self.body.len()-1].y});
            inc_speed = true;
        }

        for index in 0..self.body.len() {
            if self.x == self.body[index].x && self.y == self.body[index].y {
                tx.send(true);
            }
        }

        inc_speed
    }

    pub fn get_xs(&self) -> Vec<usize> {
        let mut x_vals = Vec::new();
        x_vals.push(self.x);

        for part in self.body.iter(){
            x_vals.push(part.x);
        }

        x_vals
    } 

    pub fn get_ys(&self) -> Vec<usize> {
        let mut y_vals = Vec::new();
        y_vals.push(self.y);

        for part in self.body.iter(){
            y_vals.push(part.y);
        }

        y_vals
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
