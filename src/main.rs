use std::{io::{stdout, self}, convert::TryInto, thread, time::Duration, sync::{Arc, Mutex}};
use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, SetSize}, event::{read, Event, KeyCode, KeyEvent}, style::{SetBackgroundColor, Color}, cursor::{Hide, Show}};
use crossbeam::{channel};
use rust_snake::{COLS, ROWS, renderer::Renderer, snake::{Snake, Direction}, food::Food};


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

    let renderer = Arc::new(Mutex::new(Renderer::new())); 
    let snake = Arc::new(Mutex::new(Snake::new())); 
    let food = Arc::new(Mutex::new(Food::new()));

    {
        let mut renderer_unlocked = renderer.lock().unwrap();
        let snake_unlocked = snake.lock().unwrap();
        renderer_unlocked.render(&mut stdout)?;
        renderer_unlocked.draw_object(&*snake_unlocked);
        renderer_unlocked.render(&mut stdout)?;
    }

    let renderer_cloned_ref = Arc::clone(&renderer);
    let snake_cloned_ref = Arc::clone(&snake);
    let food_cloned_ref = Arc::clone(&food);

    //main game loop
    
    let (render_tx, render_rx) = channel::unbounded();
    let (direction_tx, direction_rx) = channel::unbounded(); 

    let (game_over_tx, game_over_rx) = channel::unbounded();

    thread::spawn(move || {

        let move_condition = render_rx.recv().unwrap();
        let mut snake_speed = 0.15;

        let mut renderer_unlocked = renderer_cloned_ref.lock().unwrap();
        let mut snake_unlocked = snake_cloned_ref.lock().unwrap();
        let mut food_unlocked = food_cloned_ref.lock().unwrap();

        food_unlocked.change_location(&snake_unlocked);

        while move_condition {
            let dir = direction_rx.recv_timeout(Duration::from_secs_f32(0.01)).unwrap_or(&snake_unlocked.direction);

            if !(*dir == Direction::opposite_of(snake_unlocked.direction)) {
                snake_unlocked.direction = *dir;
            }
            
            snake_unlocked.move_snake();
            renderer_unlocked.draw_two_objects(&*food_unlocked, &*snake_unlocked);
            renderer_unlocked.render(&mut stdout);
            if snake_unlocked.handle_potential_collisions(&mut *food_unlocked, &game_over_tx)
            {   
                if snake_speed < 0.05 { snake_speed -= 0.002; }
                else { snake_speed -= 0.01; }
            };
            thread::sleep(Duration::from_secs_f32(snake_speed))
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

            let game_over_check = game_over_rx.recv_timeout(Duration::from_secs_f32(0.01)).unwrap_or(false);
            if game_over_check {break 'gameloop}
        }
    }
    
    //cleanup
    execute!(io::stdout(),
        Show,
        LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Result::Ok(());
    
    
}