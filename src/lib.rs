use std::fmt::Write;
use std::io;
use std::{env, error::Error, fs};

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

fn run_game_loop(config: &mut Config, input: char) -> bool {
    if (!config.solution.contains(input)) {
        println!("too bad peanut butter");
        return false;
    }

    // let match_idx = solution.find(input).expect("didnt find input");

    let indices: Vec<usize> = config.solution
        .char_indices()
        .filter_map(|(i, c)| if c == input { Some(i) } else { None })
        .collect();

    for index in indices {
        config.user_facing_message.replace_range(index..index + 1, &input.to_string());
    }

    println!("{}", config.user_facing_message);

    if (config.user_facing_message == config.solution) {
        println!("you win!");
        return false;
    }

    return true;
}

pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.user_facing_message);

    while (run_game_loop(config, take_guess())) {}

    Ok(())
}

pub struct Config {
    pub solution: String,
    pub user_facing_message: String,
}
