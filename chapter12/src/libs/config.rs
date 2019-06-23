pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        let query = match args.get(1) {
            Some(query) => query.clone(),
            None => {
                return Err("please pass a regex");
            }
        };
        let filename = match args.get(1) {
            Some(filename) => filename.clone(),
            None => {
                return Err("please pass a filename");
            }
        };

        Ok(Config { query, filename })
    }
}
