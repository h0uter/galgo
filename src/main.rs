use std::process;

use galgo::config::game_session::local_session::LocalSession;

fn main() {
    // Building this config will be the concern of the word master in a future increment
    let local_session = Box::new(LocalSession::build());
    let mut game_config = galgo::Config::build(local_session);

    if let Err(e) = galgo::run(&mut game_config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
