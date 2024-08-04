use std::error::Error;
use std::io;

fn get_user_input(question: &str) -> String {
    println!("{}", question);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn take_guess() -> char {
    return get_user_input("Provide the character you want to guess: ")
        .chars()
        .next()
        .expect("More than one character was guessed.");
}

fn run_game_loop(config: &mut Config, input: char) -> GameState {
    if !config.secret_word.contains(input) {
        return GameState::LOST;
    }

    // check for hits of guess in solution
    let hit_idxs: Vec<usize> = config
        .secret_word
        .char_indices()
        .filter_map(|(i, c)| if c == input { Some(i) } else { None })
        .collect();

    // Fill in the correctly guessed letters in the user facing message
    for hit_idx in hit_idxs {
        config
            .user_facing_message
            .replace_range(hit_idx..hit_idx + 1, &input.to_string());
    }

    println!("status: {}", config.user_facing_message);

    if config.user_facing_message == config.secret_word {
        return GameState::WON;
    }

    return GameState::PLAYING;
}

pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.user_facing_message);

    let mut state: GameState = GameState::PLAYING;
    while state == GameState::PLAYING {
        state = run_game_loop(config, take_guess());

        if state == GameState::LOST {
            println!("too bad peanut butter");
        }

        if state == GameState::WON {
            println!("you win!");
        }
    }

    Ok(())
}

pub struct Config {
    pub secret_word: String,
    pub user_facing_message: String,
}

impl Config {
    pub fn build() -> Config {
        let secret_word = get_user_input("Provide your secret word:");
        let user_facing_message = "_".repeat(secret_word.len());

        return Config {
            secret_word,
            user_facing_message,
        };
    }
}

#[derive(PartialEq, Eq)]
enum GameState {
    PLAYING,
    WON,
    LOST,
}
