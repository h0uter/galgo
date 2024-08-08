pub mod game_session;
use game_session::Session;
use std::cell::RefCell;

pub struct Config {
    // Static configuration we take at the start
    pub secret_word: String,
    pub lives: usize,
    pub session: RefCell<Box<dyn Session>>,
}

impl Config {
    pub fn build(session: Box<dyn Session>) -> Config {
        let secret_word = crate::cli::take_user_input("Provide your secret word:");
        let lives = 3; // cannot be larger than seven

        if lives > 7 {
            panic!("lives cannot be larger than 7, we dont have more hangman drawings.")
        }

        return Config { secret_word, lives, session: RefCell::new(session) };
    }
}
