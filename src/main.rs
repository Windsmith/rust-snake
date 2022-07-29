use std::{io::{stdout, Write}, convert::TryInto};
use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, size, SetSize, Clear}, event::{read, Event, KeyCode, KeyEvent}, style::{SetBackgroundColor, Color}, cursor::{Hide, Show}};
use rust_snake::{COLS, ROWS, renderer::Renderer};

fn main() -> Result<()> {
    let mut stdout = stdout();

    //setup
    enable_raw_mode()?;
    execute!(stdout, 
        EnterAlternateScreen,
        SetBackgroundColor(Color::DarkYellow),
        SetSize(COLS.try_into().unwrap(), ROWS.try_into().unwrap()),
        SetBackgroundColor(Color::Black),
        Hide
    )?;

    let mut renderer = Renderer::new();
    renderer.render(&mut stdout)?;

    //main game loop
    'gameloop: loop {
        while let Event::Key(KeyEvent {code, .. }) = read()? {
            match code {
                //q to exit
                KeyCode::Char('q') => break 'gameloop,
                _ => continue,
            }
        }
    }

    //cleanup
    execute!(stdout,
        LeaveAlternateScreen,
        Show,
    )?;
    disable_raw_mode()?;

    return Result::Ok(());
    
}