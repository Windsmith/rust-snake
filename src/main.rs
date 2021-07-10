extern crate termion;

use std::fmt::write;
use std::{
    char,
    io::{stdin, stdout, Read, Write},
};
use termion::{clear, cursor, raw::IntoRawMode};
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

struct Game {
    state: State,
    current_screen: &'static str,
}

impl Game {
    fn display_screen(&mut self) -> () {
        println!("{}", termion::clear::All);
        println!("{}", termion::cursor::Goto(1, 1));
        println!("{0}{1}", color::Fg(color::Red), self.current_screen);
    }

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

enum State {
    MainMenu,
    AboutToStart,
    Started,
    GameOver,
}

fn main() {
    let mut game_handle = Game {
        state: State::MainMenu,
        current_screen: MAINMENU,
    };

    let mut output = stdout().into_raw_mode().unwrap();
    let mut input = stdin();

    game_handle.display_screen();

    for key in input.keys() {
        match key.unwrap() {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Char(' ') => game_handle.change_state(),
            _ => (),
        };
    }
}
