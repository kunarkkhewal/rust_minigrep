use std::{env, process};

use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = rust_minigrep::run(config) {
        println!("Application error, {e}");
        process::exit(1);
    }
}
