pub mod local_session;

pub struct GameConfig {
    secret_word_length: usize,
    total_lives: usize,
}

pub struct Guess {
    guess: char,
}

pub struct Result {
    was_correct: bool,
    hit_idxs: Vec<usize>,
}

pub trait Session {
    fn submit_config(&mut self, config: GameConfig);
    fn receive_config(self) -> GameConfig;

    fn submit_guess(&mut self, guess: Guess);
    fn receive_guess(self) -> Guess;

    fn submit_result(&mut self, result: Result);
    fn receive_result(self) -> Result;
}