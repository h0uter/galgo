use std::{process};

fn main() {
    let mut game_config = galgo::take_game_config();

    if let Err(e) = galgo::run(&mut game_config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
