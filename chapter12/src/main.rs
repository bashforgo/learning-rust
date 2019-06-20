use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let needle = args
        .get(1)
        .unwrap_or_else(|| die("first argument should be a regex"));
    let haystack = args
        .get(2)
        .unwrap_or_else(|| die("second argument should be a file"));

    eprintln!("find {} in {}", needle, haystack);

    let contents = fs::read_to_string(haystack)
        .unwrap_or_else(|_| die(format!("'{}' file not found", haystack).as_ref()));

    println!("{}", contents);
}

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}