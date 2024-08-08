use super::{Session, GameConfig, Guess, Result};

pub struct LocalSession {
    config: Option<GameConfig>,
    guess: Option<Guess>,
    result: Option<Result>,
}

impl LocalSession {
    pub fn build() -> Self {
        Self{config: None, guess: None, result: None}
    }
}

impl Session for LocalSession {
    fn submit_config(&mut self, config: GameConfig) {
        self.config = Option::from(config);
    }

    fn receive_config(self) -> GameConfig {
        self.config.unwrap()
    }

    fn submit_guess(&mut self, guess: Guess) {
        self.guess = Option::from(guess);
    }

    fn receive_guess(self) -> Guess {
        self.guess.unwrap()
    }

    fn submit_result(&mut self, result: Result) {
        self.result = Option::from(result);
    }

    fn receive_result(self) -> Result {
        self.result.unwrap()
    }
}