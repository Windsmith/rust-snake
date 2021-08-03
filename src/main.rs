extern crate termion;

use std::{
    char,
    io::{stdin, stdout, Read, Write},
    fmt::write,
    process,
};
use termion::{
    clear,
    cursor::{self, HideCursor},
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
     ||                               @~~~~~                                || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     =========================================================================";

// struct that handles state of the game and the screen that has to be displayed
// for the corresponding state.
struct Game {
    state: State,
    current_screen: &'static str,
    score: u8,
}

impl Game {
    // display the current screen.
    // only needs to be called at the game start. change_state() changes the screen on its own when the state
    // of the game is changed.
    fn display_screen(&mut self) -> () {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        println!("{0}{1}", color::Fg(color::Red), self.current_screen);
    }

    // change the state of the game depending on the state its currently at.
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
}

// represents state of the game.
enum State {
    MainMenu,
    Started,
    GameOver,
}

// Struct representing position coordinates of the terminal
struct Coords {
    x_pos: u16,
    y_pos: u16,
}

// Struct representing the Snake's body parts
struct BodyPart {
    position: Coords
}

// Struct representing the Snake
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

    // Change direction of the snake and its body parts
    fn change_direction(&mut self, direction: &str) {
        // change direction
    }

    // Move continously in a single direction
    fn perpetually_move(&mut self, direction: &str) {
        // move perpetually
    }

    fn print_snake(&self) {
        print!("{}", termion::cursor::Goto(self.position.x_pos, self.position.y_pos));
        print!("@");
        for part in &self.body {
            print!("{}", termion::cursor::Goto(part.position.x_pos, part.position.y_pos));
            print!("~");
        }
    }
}

fn main() {
    // game handler: manages game state and screen
    let mut game_handle = Game {
        state: State::MainMenu,
        current_screen: MAINMENU,
        score: 0,
    };

    let mut snake = Snake::new();

    // raw terminal output: user does not need to hit enter when giving input.
    // just the presence of the output variable puts the terminal in raw mode.
    let mut output = stdout().into_raw_mode().unwrap();

    // stdin handle
    let mut input = stdin();

    // display the main screen when the program starts.
    game_handle.display_screen();

    // check for general keyboard input.
    // this set of inputs is for states outside the main game
    for key in input.keys() {
        match key.unwrap() {
            //quit the game if "q" is entered.
            termion::event::Key::Char('q') => process::exit(0x0100),

            //space key progresses the game.
            termion::event::Key::Char(' ') => {
                game_handle.change_state();
                snake.print_snake();
                break
            },
            _ => (),
        };
    }

    // stdin handle
    // makes another one because the previous one is lost after moving into input.keys()
    // TODO: find solution to avoid duplication
    let mut input = stdin();

    // check for game input.
    // this set of inputs are for when the game has started
    for key in input.keys() {
        match key.unwrap() {
            //quit the game if "q" is entered.
            termion::event::Key::Char('q') => process::exit(0x0100),

            //change direction of movement using arrow keys
            termion::event::Key::Up => snake.change_direction("up"),
            termion::event::Key::Down => snake.change_direction("down"),
            termion::event::Key::Left => snake.change_direction("left"),
            termion::event::Key::Right => snake.change_direction("right"),

            _ => (),
        }
    }
}
