use std::{io::{stdout, self}, convert::TryInto, thread, time::Duration, sync::{Arc, Mutex}};
use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, size, SetSize, Clear}, event::{read, Event, KeyCode, KeyEvent}, style::{SetBackgroundColor, Color}, cursor::{Hide, Show}};
use crossbeam::{channel, Sender};
use rust_snake::{COLS, ROWS, renderer::Renderer, snake::{Snake, Direction}};


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

    let mut renderer = Arc::new(Mutex::new(Renderer::new())); 
    let mut snake = Arc::new(Mutex::new(Snake::new())); 

    {
        let mut renderer_unlocked = renderer.lock().unwrap();
        let mut snake_unlocked = snake.lock().unwrap();
        renderer_unlocked.render(&mut stdout)?;
        renderer_unlocked.draw_object(&mut stdout, &*snake_unlocked);
        renderer_unlocked.render(&mut stdout)?;
    }

    let renderer_cloned_ref = Arc::clone(&renderer);
    let snake_cloned_ref = Arc::clone(&snake);

    //main game loop
    let (render_tx, render_rx) = channel::unbounded();
    let (direction_tx, direction_rx) = channel::unbounded(); 

    let mut render_handle = thread::spawn(move || {

        let mut move_condition = render_rx.recv().unwrap();
        
        let mut renderer_unlocked = renderer_cloned_ref.lock().unwrap();
        let mut snake_unlocked = snake_cloned_ref.lock().unwrap();

        while move_condition {
            let dir = direction_rx.recv_timeout(Duration::from_secs_f32(0.01)).unwrap_or(&snake_unlocked.direction);
            snake_unlocked.direction = *dir;
            snake_unlocked.move_snake();
            renderer_unlocked.draw_object(&mut stdout, &*snake_unlocked);
            renderer_unlocked.render(&mut stdout);
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
                },
                KeyCode::Down => {
                    if game_start {
                        direction_tx.send(&Direction::Down).unwrap();
                    }
                },
                KeyCode::Left => {
                    if game_start {
                        direction_tx.send(&Direction::Left).unwrap();
                    }
                },
                KeyCode::Up => {
                    if game_start {
                        direction_tx.send(&Direction::Up).unwrap();
                    }
                },
                KeyCode::Right => {
                    if game_start {
                        direction_tx.send(&Direction::Right).unwrap();
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