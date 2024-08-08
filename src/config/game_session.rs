pub mod local_session;

pub struct GameConfig {
    pub secret_word_length: usize,
    pub total_lives: usize,
}

pub struct Guess {
    pub guess: char,
}

pub struct Result {
    pub was_correct: bool,
    pub hit_idxs: Vec<usize>,
}

pub trait Session {
    fn submit_config(&mut self, config: GameConfig);
    fn receive_config(self) -> GameConfig;

    fn submit_guess(&mut self, guess: Guess);
    fn receive_guess(self) -> Guess;

    fn submit_result(&mut self, result: Result);
    fn receive_result(self) -> Result;
}