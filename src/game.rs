use std::{error::Error};
use std::net::TcpStream;
use crate::io_utils::get_user_input;
use crate::common::{ClientRole, Config, WordMasterConfig, GuessingPlayerConfig, RoleSpecificConfig, Guess, GuessResult};
use crate::networking::{establish_connection, receive_guess, receive_solution_size, send_guess_result, send_solution_size, try_guess};

fn take_guess() -> char {
    return get_user_input("Please enter some input:")
        .chars()
        .next()
        .expect("no single char in the input");
}

fn run_word_master_game_loop(connection: &mut TcpStream, config: &mut WordMasterConfig) -> GameState {
    let guess = receive_guess(connection);

    eprintln!("Opponent guessed {}.", guess.guessed_char);

    if !config.solution.contains(guess.guessed_char) {
        let guess_result = GuessResult{was_right: false, new_partial_solution: guess.partial_solution };
        send_guess_result(connection, guess_result);

        if guess.lives <= 1 {
            return GameState::WON;
        }

        return GameState::PLAYING;
    }

    let indices: Vec<usize> = config.solution
        .char_indices()
        .filter_map(|(i, c)| if c == guess.guessed_char { Some(i) } else { None })
        .collect();

    let mut new_partial_solution = guess.partial_solution.to_string();
    for index in indices {
        new_partial_solution.replace_range(index..index + 1, &guess.guessed_char.to_string());
    }

    let guess_result = GuessResult{was_right: true, new_partial_solution: new_partial_solution.to_string()};
    send_guess_result(connection, guess_result);

    if new_partial_solution == config.solution {
        return GameState::LOST;
    }

    return GameState::PLAYING;
}

fn run_guessing_player_game_loop(connection: &mut TcpStream, config: &mut GuessingPlayerConfig) -> GameState {
    println!("\n| guessing... lives: {}\n|", config.lives);
    println!("| {}", config.partial_solution);
    let guessed_char = take_guess();

    let guess: Guess = Guess{partial_solution: config.partial_solution.to_string(), guessed_char: guessed_char, lives: config.lives};
    let guess_result = try_guess(connection, guess);
    config.partial_solution = guess_result.new_partial_solution;

    if !config.partial_solution.contains("_") {
        return GameState::WON;
    }

    if !guess_result.was_right {
        config.lives -= 1;
    }

    if config.lives == 0 {
        return GameState::LOST;
    }

    return GameState::PLAYING;
}

fn run_game_loop(config: &mut Config) -> GameState {
    match config.role_config {
        RoleSpecificConfig::WORD_MASTER(ref mut role_config) => return run_word_master_game_loop(& mut config.connection, role_config),
        RoleSpecificConfig::GUESSING_PLAYER(ref mut role_config)=> return run_guessing_player_game_loop(& mut config.connection, role_config),
    }
}

pub fn init_game_config() -> Config {
    let (role, mut connection) = establish_connection();

    let role_config = || -> RoleSpecificConfig {
        if role == ClientRole::WORD_MASTER {
            let solution = get_user_input("Word to be guessed:");

            send_solution_size(&mut connection, solution.len());

            return RoleSpecificConfig::WORD_MASTER(WordMasterConfig{solution});
        }

        let lives: i8 = 3;
        let partial_solution = "_".repeat(receive_solution_size(&mut connection));

        return RoleSpecificConfig::GUESSING_PLAYER(GuessingPlayerConfig{lives, partial_solution});
    }();

    return Config{connection, role_config};
}

pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    let mut state: GameState = GameState::PLAYING;
    while state == GameState::PLAYING {
        state = run_game_loop(config);

        if state == GameState::LOST {
            println!("too bad peanut butter");
        }

        if state == GameState::WON {
            println!("you win!");
        }
    }

    Ok(())
}

#[derive(PartialEq, Eq)]
enum GameState {
    PLAYING,
    WON,
    LOST
}