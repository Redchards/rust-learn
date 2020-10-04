use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {} in file '{}'", config.query, config.filename);

    let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("Content of the file: {}", content);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
