use std::error::Error;
use std::io;

// CLI input output

fn take_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn take_guess() -> char {
    return take_user_input("Provide the character you want to guess: ")
        .chars()
        .next()
        .expect("More than one character was guessed.");
}

const HANGMAN_STAGES: [&str; 7] = [
    "
       +---+
       |   |
           |
           |
           |
           |
     =========",
    "
       +---+
       |   |
       O   |
           |
           |
           |
     =========",
    "
       +---+
       |   |
       O   |
       |   |
           |
           |
     =========",
    "
       +---+
       |   |
       O   |
      /|   |
           |
           |
     =========",
    "
       +---+
       |   |
       O   |
      /|\\  |
           |
           |
     =========",
    "
       +---+
       |   |
       O   |
      /|\\  |
      /    |
           |
     =========",
    "
       +---+
       |   |
       O   |
      /|\\  |
      / \\  |
           |
     =========",
];

fn print_hangman_stage(incorrect_guesses: usize) {
    if incorrect_guesses < HANGMAN_STAGES.len() {
        println!("{}", HANGMAN_STAGES[incorrect_guesses]);
    } else {
        println!("Invalid number of incorrect guesses.");
    }
}

// game core

struct PlayerState {
    wrong_guesses: usize,
}

fn run_game_loop(config: &mut Config, guess: char, player_state: &mut PlayerState) -> GameState {
    if !config.secret_word.contains(guess) {
        player_state.wrong_guesses += 1;

        print_hangman_stage(player_state.wrong_guesses + (6 - config.lives));
        
        if player_state.wrong_guesses == config.lives {
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

    // return GameState::PLAYING;
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
        state = run_game_loop(config, take_guess(), &mut player_state);

        if state == GameState::LOST {
            println!("");
            println!("too bad peanut butter... 🤡🤡🤡 YOU LOST! 🤡🤡🤡");
        }

        if state == GameState::WON {
            println!("");
            println!("🎉🎉🎉 YOU WIN! 🎉🎉🎉");
        }
    }

    Ok(())
}

// config

pub struct Config {
    pub secret_word: String,
    pub correctly_guessed_letters: String,
    pub lives: usize,
}

impl Config {
    pub fn build() -> Config {
        let secret_word = take_user_input("Provide your secret word:");
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

#[derive(PartialEq, Eq)]
enum GameState {
    PLAYING,
    WON,
    LOST,
}
