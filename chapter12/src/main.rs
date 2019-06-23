use std::env;

use chapter12 as grep;

use grep::Config;

fn main() {
    let args = env::args().collect();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let config = Config::new(&args, case_sensitive).unwrap_or_else(|err| grep::die(err));

    grep::run(&config).unwrap_or_else(|err| grep::die(&err.to_string()));
}
