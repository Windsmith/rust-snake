use std::{io::{Stdout, Write, Error}, convert::TryInto};

use crossterm::{execute, cursor::MoveTo};

use crate::frame::Frame;

pub struct Renderer {
    current_frame: Frame,
}

impl Renderer {
    pub fn new() -> Renderer {
        let frame = Frame::new();
        Renderer { current_frame: frame }
    }

    pub fn render(&mut self, stdout: &mut Stdout) -> Result<(), Error> {
        for (col_index, col) in self.current_frame.frame.iter().enumerate() {
            for (row_index, row_charactor) in col.iter().enumerate() {
                execute!(stdout, MoveTo(col_index.try_into().unwrap(), row_index.try_into().unwrap()))?;
                print!("{}", row_charactor);
            }
        }

        return Result::Ok(());
    }
}