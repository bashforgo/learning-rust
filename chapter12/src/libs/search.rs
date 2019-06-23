pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const DEFAULT_CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = DEFAULT_CONTENTS;

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "s";
        let contents = DEFAULT_CONTENTS;

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, contents)
        );
    }
}