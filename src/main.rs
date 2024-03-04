use std::{process, env};
extern crate mini_grep;

use mini_grep::Config;

fn main() {
    // collect returns iterator to vector
    // you don't need to often annotate types in Rust but collection must be!
    let args: Vec<String> = env::args().collect();
    // debug formater {:?}
    // println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

