use std::{io::{Stdout, Write, Error}, convert::TryInto};

use crossterm::{execute, cursor::MoveTo};

use crate::{frame::Frame, TerminalDraw};

pub struct Renderer {
    current_frame: Frame,
    unrendered_frame: Option<Frame>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let frame = Frame::new();
        Renderer { current_frame: frame, unrendered_frame: None }
    }

    pub fn render(&mut self, stdout: &mut Stdout) -> Result<(), Error> {
        if let Some(frame) = &self.unrendered_frame {
            let new_frame_characters = &frame.frame;
            for (col_index, col) in self.current_frame.frame.iter().enumerate() {
                for (row_index, row_charactor) in col.iter().enumerate() {
                    if *row_charactor != new_frame_characters[col_index][row_index] {
                        execute!(stdout, MoveTo(col_index.try_into().unwrap(), row_index.try_into().unwrap()))?;
                        print!("{}", new_frame_characters[col_index][row_index]);
                        stdout.flush()?;
                    }
                    
                }
            }
            self.current_frame = Frame {frame: frame.frame.clone()};
        } else {
            for (col_index, col) in self.current_frame.frame.iter().enumerate() {
                for (row_index, row_charactor) in col.iter().enumerate() {
                    execute!(stdout, MoveTo(col_index.try_into().unwrap(), row_index.try_into().unwrap()))?;
                    print!("{}", row_charactor);
                }
            }
        } 

        return Result::Ok(());
    }

    pub fn draw_object<T: TerminalDraw>(&mut self, obj: &T) {
        let mut new_frame = Frame::new();
        obj.draw(&mut new_frame);
        self.unrendered_frame = Some(new_frame)
    }

    pub fn draw_two_objects<T: TerminalDraw, U: TerminalDraw>(&mut self, obj1: &T, obj2: &U) {
        let mut new_frame = Frame::new();
        obj1.draw(&mut new_frame);
        obj2.draw(&mut new_frame);
        self.unrendered_frame = Some(new_frame)
    }
}