use std::io::{stdout, Write};
use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}, event::{read, Event, KeyCode, KeyEvent}};

fn main() -> Result<()> {
    let mut stdout = stdout();

    //setup
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    //main game loop
    'gameloop: loop {
        while let Event::Key(KeyEvent {code, .. }) = read()? {
            match code {
                //q to exit
                KeyCode::Char(q) => break 'gameloop,
                _ => continue,
            }
        }
    }

    //cleanup
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    return Result::Ok(());
    
}