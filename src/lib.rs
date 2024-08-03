use std::io;
use std::{error::Error};

fn get_user_input(question: &str) -> String {
    println!("{}", question);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

pub fn take_game_config() -> Config {
    let solution = get_user_input("Word to be guessed:");
    let user_facing_message = "_".repeat(solution.len());

    return Config{solution, user_facing_message};
}

fn take_guess() -> char {
    return get_user_input("Please enter some input:")
        .chars()
        .next()
        .expect("no single char in the input");
}

fn run_game_loop(config: &mut Config, input: char) -> GameState {
    if (!config.solution.contains(input)) {
        return GameState::LOST;
    }

    let indices: Vec<usize> = config.solution
        .char_indices()
        .filter_map(|(i, c)| if c == input { Some(i) } else { None })
        .collect();

    for index in indices {
        config.user_facing_message.replace_range(index..index + 1, &input.to_string());
    }

    println!("{}", config.user_facing_message);

    if (config.user_facing_message == config.solution) {
        return GameState::WON;
    }

    return GameState::PLAYING;
}

pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.user_facing_message);

    let mut state: GameState = GameState::PLAYING;
    while (state == GameState::PLAYING) {
        state = run_game_loop(config, take_guess());

        if (state == GameState::LOST) {
            println!("too bad peanut butter");
        }

        if (state == GameState::WON) {
            println!("you win!");
        }
    }

    Ok(())
}

pub struct Config {
    pub solution: String,
    pub user_facing_message: String,
}

#[derive(PartialEq, Eq)]
enum GameState {
    PLAYING,
    WON,
    LOST
}
