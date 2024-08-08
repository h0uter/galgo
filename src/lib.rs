use std::error::Error;
// config
pub mod config;
pub use crate::config::Config;
use crate::config::game_session;

// CLI input output
mod cli;

// game core

// This function will be the concern of the "word master" in a later increment
fn check_guess(config: &Config) {
    let guess = config.session.borrow_mut().receive_guess().guess;

    let mut was_correct = false;
    let mut hit_idxs: Vec<usize> = Vec::new();

    if config.secret_word.contains(guess) {
        was_correct = true;
        hit_idxs = determine_hit_idxs(config, guess);
    }

    config.session.borrow_mut().submit_result(game_session::Result{was_correct, hit_idxs});
}

// This function and the print_and_determine function below will be the concern of
// the "guessing player" in a later increment.
fn take_guessing_player_guess(config: &Config) -> char {
    let guess = cli::take_guess();
    config.session.borrow_mut().submit_guess(game_session::Guess{guess});
    return guess;
}

fn print_and_determine_game_state(config: &Config, player_state: &mut PlayerState, guess: char) -> GameState {
    let result= config.session.borrow_mut().receive_result();
    write_correct_characters(player_state, result.hit_idxs, guess);

    if !result.was_correct {
        player_state.wrong_guesses += 1;
        cli::print_hangman_stage(player_state.wrong_guesses + (6 - config.lives));

        if player_state.wrong_guesses >= config.lives {
            return GameState::LOST;
        } else {
            cli::print_wrong_guess(&(config.lives - player_state.wrong_guesses));
            return GameState::PLAYING;
        }
    }

    cli::print_correct_guess(&player_state.correctly_guessed_letters);

    if player_state.correctly_guessed_letters == config.secret_word {
        return GameState::WON;
    }

    return GameState::PLAYING;
}

fn run_game_loop(config: &Config, player_state: &mut PlayerState) -> GameState {
    // Guessing player
    let guess = take_guessing_player_guess(config);

    // Word master
    check_guess(config);

    // Guessing player
    return print_and_determine_game_state(config, player_state, guess);
}

fn determine_hit_idxs(config: &Config, guess: char) -> Vec<usize> {
    return config
        .secret_word
        .char_indices()
        .filter_map(|(i, c)| if c == guess { Some(i) } else { None })
        .collect();
}

fn write_correct_characters(player_state: &mut PlayerState, hit_idxs: Vec<usize>, guess: char) {
    for hit_idx in hit_idxs {
        player_state
            .correctly_guessed_letters
            .replace_range(hit_idx..hit_idx + 1, &guess.to_string());
    }
}

fn run_game(config: &Config) {
    // run a single galgo game
    let mut state: GameState = GameState::PLAYING;
    let mut player_state = PlayerState::build(config);

    while state == GameState::PLAYING {
        state = run_game_loop(config, &mut player_state);

        if state == GameState::LOST {
            crate::cli::print_loss()
        }

        if state == GameState::WON {
            crate::cli::print_win()
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // run galgo with potential rematches

    while crate::cli::take_user_input("Start a new game? (yes/no)")
        .eq_ignore_ascii_case("yes") {
            run_game(config);
    }

    Ok(())
}

#[derive(PartialEq, Eq)]
enum GameState {
    PLAYING,
    WON,
    LOST,
}

struct PlayerState {
    // Dynamic state of the game
    wrong_guesses: usize,
    correctly_guessed_letters: String,
}

impl PlayerState {
    pub fn build(config: &Config) -> PlayerState {
        let wrong_guesses = 0;
        let correctly_guessed_letters = "_".repeat(config.secret_word.len());

        return PlayerState {
            wrong_guesses,
            correctly_guessed_letters,
        };
    }
}
