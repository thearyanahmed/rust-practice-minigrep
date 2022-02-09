#!allow(non_snake_case)

use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let conf: Config = Config{
            query: args[1].clone(),
            filename: args[2].clone()
        };

        return Ok(conf);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args)
            .unwrap_or_else(|err| {
                println!("problem parsing arguments: {}", err);
                process::exit(1);
            });

    if let Err(e) = run(conf) {
        println!("error running application. {}", e);

        process::exit(1);
    }
}

fn run(conf: Config) -> Result<(), Box<dyn Error>>{
    let _contents = fs::read_to_string(conf.filename) 
                    .expect("could not read file.")?;

    return Ok(());
}

