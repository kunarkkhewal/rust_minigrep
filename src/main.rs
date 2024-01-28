use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching {}", query);
    println!("In file {}", filepath);
}