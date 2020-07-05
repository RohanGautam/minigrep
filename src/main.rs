use std::{env, error::Error, fs, process};

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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1); // this is to exit neatly without any other info spewed out
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `?` will RETURN the error(ending fn execution) if it encounters an `Err` in the `Result` it follows.
    let contents: String = fs::read_to_string(config.filename)?;
    println!("Read:\n{}", contents);
    // `()` is a unit type. It means that we mostly do not care abput return type if it goes well.
    // We do, however, care about the errors that might occour, and thats why the result type exists with a
    // dynamic error return type
    return Ok(());
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
