use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    // dbg!(&cmd_args);
    // first arg in vector will always be the name of the binary
    let config = Config::build(&cmd_args).unwrap_or_else(
        |err| {
            eprintln!("Error parsing arguments: {}", err);
            process::exit(1);
        }
    );

    println!("Searching for '{}' in file {}...", config.query, config.filepath);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error encountered: {}", e);
        process::exit(1);
    }
}
