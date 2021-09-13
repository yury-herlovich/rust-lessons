use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        if args[1] == "-i" && args.len() >= 4 {
            return Ok(Config {
                query: args[2].clone(),
                filename: args[3].clone(),
                case_sensitive: false,
            });
        } else if args[1] != "-i" {
            return Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: true,
            });
        }

        return Err("failed to parse arguments");
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results;
    if config.case_sensitive {
        results = search(config.query.as_str(), contents.as_str());
    } else {
        results = search_case_insensitive(config.query.as_str(), contents.as_str());
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod search {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "pick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(query, contents)
        );
    }
}

#[cfg(test)]
mod config {
    use super::Config;

    #[test]
    fn parse_with_2_args() {
        let args = vec![
            String::from(""),
            String::from("query"),
            String::from("filename"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("filename", config.filename);
        assert_eq!(true, config.case_sensitive);
    }

    #[test]
    fn parse_args_with_case_insensitive() {
        let args = vec![
            String::from(""),
            String::from("-i"),
            String::from("query"),
            String::from("filename"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("filename", config.filename);
        assert_eq!(false, config.case_sensitive);
    }
}
