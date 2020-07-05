use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // `unwrap_or_else` for non-panic error handling. Using an anonymous funtion
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // clone creates a new copy for our struct to own.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
