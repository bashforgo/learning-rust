use std::error::Error;
use std::fs;

use super::Config;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let query = &config.query;
    let filename = &config.filename;

    eprintln!("find {} in {}", query, filename);

    let contents = fs::read_to_string(filename)?;

    println!("{}", contents);

    Ok(())
}
