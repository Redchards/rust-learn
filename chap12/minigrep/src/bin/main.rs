use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("Searching for {} in file '{}'", config.query, config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error while processing : {}", e);
        process::exit(1);
    }
}

