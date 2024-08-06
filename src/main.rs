use std::process;

fn main() {
    // Building this config will be the concern of the word master in a future increment
    let mut game_config = galgo::Config::build();

    if let Err(e) = galgo::run(&mut game_config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
