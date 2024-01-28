use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filepath) = parse_config(&args);

    

    println!("Searching {}", query);
    println!("In file {}", filepath);

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read file");

    println!("With Text:\n{contents}")
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filepath = &args[2];

    (query, filepath)
}