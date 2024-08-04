use std::io;

pub fn take_user_input(prompt: &str) -> String {
    // read whatever from stdin as string
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

pub fn take_guess() -> char {
    // take a single character as guess input
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

pub fn print_hangman_stage(incorrect_guesses: usize) {
    if incorrect_guesses < HANGMAN_STAGES.len() {
        println!("{}", HANGMAN_STAGES[incorrect_guesses]);
    } else {
        panic!("Invalid number of incorrect guesses.");
    }
}
