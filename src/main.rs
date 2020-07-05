use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("Reading file {}", filename);
    //read a file into a string
    let contents: String = fs::read_to_string(filename).expect("Error reading the file specified");
    println!("Read:\n{}", contents);
}
