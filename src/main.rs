use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

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
impl Config {
    fn new(args: &Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // clone creates a new copy for our struct to own.
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
