use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching {}", query);
    println!("In file {}", filepath);

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read file");

    println!("With Text:\n{contents}")
}