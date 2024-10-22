pub struct Config {
    // Static configuration we take at the start
    pub secret_word: String,
    pub lives: usize,
}

impl Config {
    pub fn build() -> Config {
        let secret_word = crate::cli::take_user_input("Provide your secret word:");
        let lives = 3; // cannot be larger than seven

        if lives > 7 {
            panic!("lives cannot be larger than 7, we dont have more hangman drawings.")
        }

        return Config { secret_word, lives };
    }
}
