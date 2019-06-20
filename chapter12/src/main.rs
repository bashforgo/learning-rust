use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let needle = args.get(1).unwrap_or_else(die("first argument should be a regex"));
    let haystack = args.get(2).unwrap_or_else(die("second argument should be a file"));

    println!("find {} in {}", needle, haystack);
}

fn die<T>(msg: &'static str) -> impl FnOnce() -> T {
    return move || {
        eprintln!("{}", msg);
        process::exit(1);
    };
}