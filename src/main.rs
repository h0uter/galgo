use std::{process};

mod networking;
mod io_utils;
mod common;
mod game;

fn main() {
    let mut game_config = game::init_game_config();

    if let Err(e) = game::run(&mut game_config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
