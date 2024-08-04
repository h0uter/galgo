use std::process;

fn main() {
    let mut game_config = galgo::Config::build();

    if let Err(e) = galgo::run(&mut game_config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
