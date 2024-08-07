use std::error::Error;
// config
pub mod config;
pub use crate::config::Config;

// CLI input output
mod cli;

// game core

fn run_guess_loop(config: &Config, guess: char, player_state: &mut PlayerState) -> GameState {
    // runs the logic for a single letter guess

    if !config.secret_word.contains(guess) {
        player_state.wrong_guesses += 1;

        crate::cli::print_hangman_stage(player_state.wrong_guesses + (6 - config.lives));

        if player_state.wrong_guesses >= config.lives {
            return GameState::LOST;
        } else {
            crate::cli::print_wrong_guess(&(config.lives - player_state.wrong_guesses));
            return GameState::PLAYING;
        }
    }

    update_correctly_guessed_letters(config, player_state, guess);

    crate::cli::print_correct_guess(&player_state.correctly_guessed_letters);

    if player_state.correctly_guessed_letters == config.secret_word {
        return GameState::WON;
    }

    return GameState::PLAYING;
}

fn update_correctly_guessed_letters(config: &Config, player_state: &mut PlayerState, guess: char) {
    // check for hits of guess in solution
    let hit_idxs: Vec<usize> = config
        .secret_word
        .char_indices()
        .filter_map(|(i, c)| if c == guess { Some(i) } else { None })
        .collect();

    // Fill in the correctly guessed letters in the user facing message
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
        state = run_guess_loop(config, crate::cli::take_guess(), &mut player_state);

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
