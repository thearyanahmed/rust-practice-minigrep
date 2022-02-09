#!allow(non_snake_case)

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args)
            .unwrap_or_else(|err| {
                eprintln!("problem parsing arguments: {}", err);
                process::exit(1);
            });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("error running application. {}", e);

        process::exit(1);
    }
}
