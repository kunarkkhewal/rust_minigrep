use std::{env, process};

use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });


    if let Err(e) = rust_minigrep::run(config) {
        process::exit(1);
    }
}
