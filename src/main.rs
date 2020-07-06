use minigrep::Config;
use std::{env, process};

fn main() {
    // `unwrap_or_else` for non-panic error handling. Using an anonymous funtion
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // `if let` used to check if an `Err` type is actually returned.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1); // this is to exit neatly without any other info spewed out
    }
}
