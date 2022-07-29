use frame::Frame;

pub mod frame;
pub mod renderer;
pub mod snake;

pub const COLS: usize = 80;
pub const ROWS: usize = 20;

pub trait TerminalDraw {
    fn draw(&self, frame: &mut Frame);
}