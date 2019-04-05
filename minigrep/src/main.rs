extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 'collect' turns the iterator into a vector! Since 'collect'
    // can create many different collections, we need to annotate
    // the type.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!(
        "Searching for \"{}\" in file \"{}\"", 
        config.query, config.filename
    );
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

