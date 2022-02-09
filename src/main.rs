#!allow(non_snake_case)

use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args); 

    let conf = parseParams(&args);

    let contents = fs::read_to_string(conf.filename) 
                    .expect("could not read file.");

    println!("With text:\n{}", contents);
}


fn parseParams(args: &[String]) -> Config {
    // TODO check length
    return Config{
        query: args[1].clone(),
        filename: args[2].clone()
    };
}