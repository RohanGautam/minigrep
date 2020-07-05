use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `?` will RETURN the error(ending fn execution) if it encounters an `Err` in the `Result` it follows.
    let contents: String = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("> {}", line);
    }

    // `()` is a unit type. It means that we mostly do not care about return type if it goes well.
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

// the lifetime params say that: the search results will be valid as long as the contents are valid. Can last even after query goes out of scope.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            // the `line` added to `lines` is a reference to the line in the contents variable.
            // this is why the result and contents MUST have the same lifetime. Because the result contains
            // references to the content.
            lines.push(line);
        }
    }
    lines
}

// Tests! In TDD(test driven development) you write tests before the code, and write code after that such that the tests pass.
// gives you clarity on as to what you're working towards.
#[cfg(test)]
mod tests {
    // use every fn from above
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."])
    }
}
