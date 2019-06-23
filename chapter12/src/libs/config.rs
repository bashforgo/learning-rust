#[derive(PartialEq, Debug)]
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
        let filename = match args.get(2) {
            Some(filename) => filename.clone(),
            None => {
                return Err("please pass a filename");
            }
        };

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_usage() {
        assert_eq!(
            Ok(Config {
                query: String::from("the query"),
                filename: String::from("the filename")
            }),
            Config::new(&vec![
                String::from("the program"),
                String::from("the query"),
                String::from("the filename")
            ])
        )
    }

    #[test]
    fn no_filename() {
        assert_eq!(
            "some error",
            Config::new(&vec![
                String::from("the program"),
                String::from("the query"),
            ])
            .map(|_| "success")
            .unwrap_or("some error")
        )
    }

    #[test]
    fn no_query() {
        assert_eq!(
            "some error",
            Config::new(&vec![String::from("the program")])
                .map(|_| "success")
                .unwrap_or("some error")
        )
    }
}
