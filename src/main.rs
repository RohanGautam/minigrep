use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("Reading file {}", config.filename);
    //read a file into a string
    let contents: String =
        fs::read_to_string(config.filename).expect("Error reading the file specified");
    println!("Read:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}
// args can be `&[String]`  too
fn parse_config(args: &Vec<String>) -> Config {
    // clone creates a new copy for our struct to own.
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
