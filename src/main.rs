extern crate termion;

use std::fmt::write;
use std::{
    char,
    io::{stdin, stdout, Read, Write},
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
     ||                    Press Space to Start the Game                    || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
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
     ||                                 @~~~~~                              || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                                                                     || \n\r\
     ||                    Press arrow keys to start the game               || \n\r\
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
                self.state = State::AboutToStart;
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
    AboutToStart,
    Started,
    GameOver,
}

fn main() {
    // game handler: manages game state and screen
    let mut game_handle = Game {
        state: State::MainMenu,
        current_screen: MAINMENU,
    };

    // raw terminal output: user does not need to hit enter when giving input.
    // just the presence of the output variable puts the terminal in raw mode.
    let mut output = stdout().into_raw_mode().unwrap();

    // stdin handle
    // Not used anywhere right now, feel free to remove.
    let mut input = stdin();

    // display the main screen when the program starts.
    game_handle.display_screen();

    // check for keyboard input.
    for key in input.keys() {
        match key.unwrap() {
            //quit the game if "q" is entered.
            termion::event::Key::Char('q') => break,

            //space key progresses the game.
            termion::event::Key::Char(' ') => game_handle.change_state(),
            _ => (),
        };
    }
}
