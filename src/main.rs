use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // `unwrap_or_else` for non-panic error handling. Using an anonymous funtion
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("Reading file {}", config.filename);
    // `if let` used to check if an `Err` type is actually returned.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1); // this is to exit neatly without any other info spewed out
    }
}
