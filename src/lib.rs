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

fn take_guess() -> char {
    return get_user_input("Please enter some input:")
        .chars()
        .next()
        .expect("no single char in the input");
}

fn run_game_loop(solution: &String, user_facing_message: &mut String, input: char) -> bool {
    if (!solution.contains(input)) {
        println!("too bad peanut butter");
        return false;
    }

    // let match_idx = solution.find(input).expect("didnt find input");

    let indices: Vec<usize> = solution
        .char_indices()
        .filter_map(|(i, c)| if c == input { Some(i) } else { None })
        .collect();

    for index in indices {
        user_facing_message.replace_range(index..index + 1, &input.to_string());
    }

    println!("{}", user_facing_message);

    if (user_facing_message == solution) {
        println!("you win!");
        return false;
    }

    return true;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let solution = fs::read_to_string(config.file_path)?;
    let mut user_facing_message: String = "_".repeat(solution.len());

    println!("{}", user_facing_message);

    while (run_game_loop(&solution, &mut user_facing_message, take_guess())) {}

    Ok(())
}

pub struct Config {
    pub guess: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // pop the caller name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a file path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            guess: query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_result() {
        let query = "monomorphic";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let empty_vec: Vec<&str> = Vec::new();

        assert_eq!(empty_vec, search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
