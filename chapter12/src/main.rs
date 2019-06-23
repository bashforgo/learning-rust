use std::env;

use chapter12 as grep;

use grep::Config;

fn main() {
    let args = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| grep::die(err));

    grep::run(&config).unwrap_or_else(|err| grep::die(&err.to_string()));
}

