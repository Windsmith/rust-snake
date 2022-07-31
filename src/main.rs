use std::{io::{stdout, self}, convert::TryInto, thread, time::Duration};
use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, size, SetSize, Clear}, event::{read, Event, KeyCode, KeyEvent}, style::{SetBackgroundColor, Color}, cursor::{Hide, Show}};
use crossbeam::channel;
use rust_snake::{COLS, ROWS, renderer::Renderer, snake::Snake};


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
    let mut snake = Snake::new();

    renderer.render(&mut stdout)?;
    renderer.draw_object(&mut stdout, &snake);
    renderer.render(&mut stdout)?;

    //main game loop
    let (render_tx, render_rx) = channel::unbounded();
    let mut render_handle = thread::spawn(move || {

        let move_condition = render_rx.recv().unwrap();

        while move_condition {
            snake.move_snake();
            renderer.draw_object(&mut stdout, &snake);
            renderer.render(&mut stdout);
            thread::sleep(Duration::from_secs_f32(0.1))
        }
    });


    let mut game_start: bool = false;

    'gameloop: loop {
        while let Event::Key(KeyEvent {code, .. }) = read()? {
            match code {
                //q to exit
                KeyCode::Char('q') => break 'gameloop,
                //space to start
                KeyCode::Char(' ') => {
                    if !game_start {
                        game_start = true;
                        render_tx.send(true).unwrap();
                    }
                }
                _ => continue,
            }
        }
    }

    //cleanup
    
    execute!(io::stdout(),
        Show,
        LeaveAlternateScreen)?;
    disable_raw_mode()?;

    return Result::Ok(());
    
}