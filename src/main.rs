extern crate termion;

use std::{
    io::{stdin, stdout},
    process,
};
use termion::{
    raw::IntoRawMode,
};
use termion::{color, input::TermRead};

const MAINMENU: &str =
    "========================================================================= \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||     ▄▄▄▄▄    ▄   ██   █  █▀ ▄███▄        ▄▀  ██   █▀▄▀█ ▄███▄       || \n\r\
     ||     █     ▀▄   █  █ █  █▄█   █▀   ▀     ▄▀    █ █  █ █ █ █▀   ▀     || \n\r\
     ||   ▄  ▀▀▀▀▄ ██   █ █▄▄█ █▀▄   ██▄▄       █ ▀▄  █▄▄█ █ ▄ █ ██▄▄       || \n\r\
     ||    ▀▄▄▄▄▀  █ █  █ █  █ █  █  █▄   ▄▀    █   █ █  █ █   █ █▄   ▄▀    || \n\r\
     ||            █  █ █    █   █   ▀███▀       ███     █    █  ▀███▀      || \n\r\
     ||            █   ██   █   ▀                       █    ▀              || \n\r\
     ||                    ▀                           ▀                    || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                               @~~~~~                                || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                    Press Space to Start the Game                    || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     =========================================================================";

const GAME_SCREEN: &str =
    "========================================================================= \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     =========================================================================";

struct Game {
    state: State,
    current_screen: &'static str,
    score: u8,
}

impl Game {
    // only needs to be called at the game start. change_state() changes the screen on its own when the state
    // of the game is changed.
    fn display_screen(&mut self) -> () {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        println!("{0}{1}", color::Fg(color::Red), self.current_screen);
    }

    // function will be called only when state needs to move forward, so it doesn't need any arguments.
    // function also updates the screen on its own, so only change_state() has to be called.
    fn change_state(&mut self) -> () {
        match self.state {
            State::MainMenu => {
                self.state = State::Started;
                self.current_screen = GAME_SCREEN;
                self.display_screen()
            }
            _ => (),
        }
    }

    fn exit_game(&mut self){
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Restore);
        print!("{}", termion::cursor::Goto(1,1))
    }

}

enum State {
    MainMenu,
    Started,
    GameOver,
}

struct Coords {
    x_pos: u16,
    y_pos: u16,
}

struct BodyPart {
    position: Coords
}

struct Snake {
    position: Coords,
    body: Vec<BodyPart>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            position: Coords {x_pos: 33, y_pos: 16},
            body: vec![
                BodyPart {position: Coords {x_pos: 34, y_pos: 16}},
                BodyPart {position: Coords {x_pos: 35, y_pos: 16}},
                BodyPart {position: Coords {x_pos: 36, y_pos: 16}},
                BodyPart {position: Coords {x_pos: 37, y_pos: 16}},
                BodyPart {position: Coords {x_pos: 38, y_pos: 16}},
            ]
        }
    }

    fn change_direction(&mut self, direction: &str) {
        // change direction
    }

    fn perpetually_move(&mut self, direction: &str) {
        match direction {
            "up" => {
                
            }
            _ => print!("bro")
        }
    }

    fn print_snake(&self) {
        //Not sure why, but print! doesn't work here. So println! is used although the newline is unnecessary
        println!("{}@", termion::cursor::Goto(self.position.x_pos, self.position.y_pos));
        for part in &self.body {
            println!("{}~", termion::cursor::Goto(part.position.x_pos, part.position.y_pos));
        }
    }
}

fn main() {
    let mut game_handle = Game {
        state: State::MainMenu,
        current_screen: MAINMENU,
        score: 0,
    };

    let mut snake = Snake::new();

    // raw terminal output: user does not need to hit enter when giving input.
    // just the presence of the output variable puts the terminal in raw mode.
    let _output = stdout().into_raw_mode().unwrap();

    let input = stdin();

    game_handle.display_screen();

    // check for general keyboard input.
    // this set of inputs is for states outside the main game
    for key in input.keys() {
        match key.unwrap() {
            termion::event::Key::Char('q') => {
                game_handle.exit_game();
                return
            }

            //space key progresses the game.
            termion::event::Key::Char(' ') => {
                game_handle.change_state();
                break
            },
            _ => (),
        };
    }

    // makes another one because the previous one is lost after moving into input.keys()
    // TODO: find solution to avoid duplication
    let input = stdin();

    snake.print_snake();

    // check for game input.
    // this set of inputs are for when the game has started
    for key in input.keys() {
        match key.unwrap() {
            termion::event::Key::Char('q') => {
                game_handle.exit_game();
                return
            }

            //change direction of movement using arrow keys
            termion::event::Key::Up => snake.change_direction("up"),
            termion::event::Key::Down => snake.change_direction("down"),
            termion::event::Key::Left => snake.change_direction("left"),
            termion::event::Key::Right => snake.change_direction("right"),

            _ => print!("bro"),
        }
    }
}
