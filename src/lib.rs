use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `?` will RETURN the error(ending fn execution) if it encounters an `Err` in the `Result` it follows.
    let contents: String = fs::read_to_string(config.filename)?;
    println!("Read:\n{}", contents);
    // `()` is a unit type. It means that we mostly do not care abput return type if it goes well.
    // We do, however, care about the errors that might occour, and thats why the result type exists with a
    // dynamic error return type
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // clone creates a new copy for our struct to own.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
