#!allow(non_snake_case)

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args); 

        
    let (query, filename) = parseParams(args);

    let contents = fs::read_to_string(filename) 
                    .expect("could not read file.");

    println!("With text:\n{}using query{}\nusing filename {}\n", contents,query,filename);
}


fn parseParams(args: &[String]) (&str, &str) {
    // TODO check length
    return (&args[1], &args[2]);
}