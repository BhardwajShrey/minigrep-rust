use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String
}

impl Config {
    pub fn build(cmd_args: &Vec<String>) -> Result<Self, &'static str> {
        if cmd_args.len() < 3 {
            return Err("Not enough arguments provided...");
        }
        let (query, filepath) = (cmd_args[1].to_string(), cmd_args[2].to_string());

        Ok(Self {query, filepath})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    // dbg!(contents);

    Ok(())
}
