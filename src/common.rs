use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq)]
pub enum ClientRole {
    WordMaster,
    GuessingPlayer,
}

pub struct WordMasterConfig {
    pub solution: String,
}

pub struct GuessingPlayerConfig {
    pub lives: i8,
    pub partial_solution: String,
}

pub enum RoleSpecificConfig {
    WordMaster(WordMasterConfig),
    GuessingPlayer(GuessingPlayerConfig),
}

pub struct Config {
    pub connection: TcpStream,
    pub role_config: RoleSpecificConfig,
}

#[derive(Serialize, Deserialize)]
pub struct Guess {
    pub partial_solution: String,
    pub guessed_char: char,
    pub lives: i8,
}

#[derive(Serialize, Deserialize)]
pub struct GuessResult {
    pub was_right: bool,
    pub new_partial_solution: String,
}