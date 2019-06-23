use std::process;

pub fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}
