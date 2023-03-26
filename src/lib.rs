use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(cmd_args: &Vec<String>) -> Result<Self, &'static str> {
        if cmd_args.len() < 3 {
            return Err("Not enough arguments provided...");
        }
        let (query, filepath) = (cmd_args[1].to_string(), cmd_args[2].to_string());
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {query, filepath, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    // dbg!(contents);

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results.iter() {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);        
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "prod";
        let contents = "Rust: 
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust: 
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust: ", "Trust me."], search_case_insensitive(query, contents));
    }
}
