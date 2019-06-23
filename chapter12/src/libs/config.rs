#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>, case_sensitive: bool) -> Result<Config, &str> {
        let query = match args.get(1) {
            Some(query) => query.clone(),
            None => {
                return Err("please pass a query");
            }
        };
        let filename = match args.get(2) {
            Some(filename) => filename.clone(),
            None => {
                return Err("please pass a filename");
            }
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_usage_insensitive() {
        assert_eq!(
            Ok(Config {
                query: String::from("the query"),
                filename: String::from("the filename"),
                case_sensitive: false,
            }),
            Config::new(
                &vec![
                    String::from("the program"),
                    String::from("the query"),
                    String::from("the filename")
                ],
                false
            )
        )
    }

    #[test]
    fn proper_usage_sensitive() {
        assert_eq!(
            Ok(Config {
                query: String::from("the query"),
                filename: String::from("the filename"),
                case_sensitive: true,
            }),
            Config::new(
                &vec![
                    String::from("the program"),
                    String::from("the query"),
                    String::from("the filename")
                ],
                true
            )
        )
    }

    #[test]
    fn no_filename() {
        assert_eq!(
            "some error",
            Config::new(
                &vec![String::from("the program"), String::from("the query")],
                true
            )
            .map(|_| "success")
            .unwrap_or("some error")
        )
    }

    #[test]
    fn no_query() {
        assert_eq!(
            "some error",
            Config::new(&vec![String::from("the program")], false)
                .map(|_| "success")
                .unwrap_or("some error")
        )
    }
}
