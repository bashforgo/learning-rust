use std::error::Error;
use std::fs;

use super::Config;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let query = &config.query;
    let filename = &config.filename;

    let contents = &fs::read_to_string(filename)?;

    let results = if config.case_sensitive {
        super::search(query, contents)
    } else {
        super::search_case_insensitive(query, contents)
    };

    for found in results {
        println!("{}", found);
    }

    Ok(())
}
