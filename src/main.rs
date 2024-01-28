use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args); //parse_config(&args);

    println!("Searching {}", config.query);
    println!("In file {}", config.filepath);

    let contents = fs::read_to_string(config.filepath)
        .expect("Should have been able to read file");

    println!("With Text:\n{contents}")
}

#[derive(Debug)]
struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filepath = args[2].clone();
        Config {query, filepath}
    }
}