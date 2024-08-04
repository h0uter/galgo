use std::error::Error;

// CLI input output
mod cli;

// game core

struct PlayerState {
    wrong_guesses: usize,
}

fn run_game_loop(config: &mut Config, guess: char, player_state: &mut PlayerState) -> GameState {
    if !config.secret_word.contains(guess) {
        player_state.wrong_guesses += 1;

        crate::cli::print_hangman_stage(player_state.wrong_guesses + (6 - config.lives));

        if player_state.wrong_guesses >= config.lives {
            return GameState::LOST;
        } else {
            println!(
                "WRONG... lives remaining {}",
                config.lives - player_state.wrong_guesses
            );
            return GameState::PLAYING;
        }
    }

    update_correctly_guessed_letters(config, guess);

    println!("CORRECT... status: {}", config.correctly_guessed_letters);

    if config.correctly_guessed_letters == config.secret_word {
        return GameState::WON;
    }

    return GameState::PLAYING;
}

fn update_correctly_guessed_letters(config: &mut Config, guess: char) {
    // check for hits of guess in solution
    let hit_idxs: Vec<usize> = config
        .secret_word
        .char_indices()
        .filter_map(|(i, c)| if c == guess { Some(i) } else { None })
        .collect();

    // Fill in the correctly guessed letters in the user facing message
    for hit_idx in hit_idxs {
        config
            .correctly_guessed_letters
            .replace_range(hit_idx..hit_idx + 1, &guess.to_string());
    }
}

pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    let mut state: GameState = GameState::PLAYING;
    let mut player_state = PlayerState { wrong_guesses: 0 };

    while state == GameState::PLAYING {
        state = run_game_loop(config, crate::cli::take_guess(), &mut player_state);

        if state == GameState::LOST {
            crate::cli::print_loss()
        }

        if state == GameState::WON {
            crate::cli::print_win()
        }
    }

    Ok(())
}

#[derive(PartialEq, Eq)]
enum GameState {
    PLAYING,
    WON,
    LOST,
}

// config

pub struct Config {
    pub secret_word: String,
    pub correctly_guessed_letters: String,
    pub lives: usize,
}

impl Config {
    pub fn build() -> Config {
        let secret_word = crate::cli::take_user_input("Provide your secret word:");
        let correctly_guessed_letters = "_".repeat(secret_word.len());
        let lives = 3; // cannot be larger than seven

        if lives > 7 {
            panic!("lives cannot be larger than 7, we dont have more hangman drawings.")
        }

        return Config {
            secret_word,
            correctly_guessed_letters,
            lives,
        };
    }
}
