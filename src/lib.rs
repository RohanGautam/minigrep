use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `?` will RETURN the error(ending fn execution) if it encounters an `Err` in the `Result` it follows.
    let contents: String = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // use next to discard the first argument
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Didnt recieve query param"),
        };
        let filename = match args.next() {
            Some(val) => val,
            None => return Err("Didnt recieve filename param"),
        };

        // `is_err` will return true if env var not found(and theres an error)
        // this means we do case sensitive search by default.
        // to do case_insensitive search, linux : `CASE_INSENSITIVE=1 cargo run to poem.txt`
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // also, var names are same as in the struct, so e dont need explicit key:val initialization style.
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// the lifetime params say that: the search results will be valid as long as the contents are valid. Can last even after query goes out of scope.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut lines: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."])
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Rust:", "Trust me."]
        )
    }
}
