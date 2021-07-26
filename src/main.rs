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
        println!("{}", termion::clear::All);
        println!("{}", termion::cursor::Goto(1, 1));
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
    x_pos: u8,
    y_pos: u8
}

// Struct representing the Snake
struct Snake {
    position: Coords
    body: vec![BodyParts]
}

// Struct representing the Snake's body parts
struct BodyParts {
    position: Coords
}

impl Snake {
    // Change direction of the snake and its body parts
    fn change_direction(direction: str) {
        // change direction
    }
}

fn main() {
    // game handler: manages game state and screen
    let mut game_handle = Game {
        state: State::MainMenu,
        current_screen: MAINMENU,
        score: 0
    };

    // raw terminal output: user does not need to hit enter when giving input.
    // just the presence of the output variable puts the terminal in raw mode.
    let mut output = stdout().into_raw_mode().unwrap();

    // stdin handle
    // Not used anywhere right now, feel free to remove.
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
                game_handle.change_state()
                break
            },
            _ => (),
        };
    }

    // check for game input.
    // this set of inputs are for when the game has started
    for key in input.keys() {
        match key.unwrap() {
            //quit the game if "q" is entered.
            termion::event::Key::Char('q') => process::exit(0x0100),

            //change direction of movement using arrow keys
            termion::event::Key::Up => change_direction("up"),
            termion::event::Key::Down => change_direction("down"),
            termion::event::Key::Left => change_direction("left"),
            termion::event::Key::Right => change_direction("right"),
        }
    }
}
