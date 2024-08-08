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

    fn receive_config(&mut self) -> GameConfig {
        self.config.take().unwrap()
    }

    fn submit_guess(&mut self, guess: Guess) {
        self.guess = Option::from(guess);
    }

    fn receive_guess(&mut self) -> Guess {
        self.guess.take().unwrap()
    }

    fn submit_result(&mut self, result: Result) {
        self.result = Option::from(result);
    }

    fn receive_result(&mut self) -> Result {
        self.result.take().unwrap()
    }
}