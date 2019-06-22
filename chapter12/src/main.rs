use std::env;
use std::fs;
use std::process;

use chapter12::Config;

fn main() {
    let config = Config::new(&env::args().collect()).unwrap_or_else(|err| die(err));

    let query = &config.query;
    let filename = &config.filename;

    eprintln!("find {} in {}", query, filename);

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| die(format!("'{}' file not found", filename).as_ref()));

    println!("{}", contents);
}

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}