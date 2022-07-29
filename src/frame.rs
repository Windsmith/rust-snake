use crate::{COLS, ROWS};

pub struct Frame {
    pub frame: Vec<Vec<char>>
}

impl Frame {
    pub fn new() -> Self {
        let frame_vec: Vec<Vec<char>> = vec![vec![' '; ROWS]; COLS];
        Frame {frame: frame_vec}
    }
}