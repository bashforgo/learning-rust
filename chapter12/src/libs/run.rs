use std::error::Error;
use std::fs;

use super::Config;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let query = &config.query;
    let filename = &config.filename;

    let contents = fs::read_to_string(filename)?;

    for found in super::search(query, &contents) {
        println!("{}", found);
    }

    Ok(())
}
