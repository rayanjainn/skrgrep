use std::process;
use clap::Parser;
use skrgrep::{run_config, Config};

fn main() {
    // Parser::parse() automatically handles -h, --help, and invalid args
    let config = Config::parse();

    // Pass the config to the library's run function
    if let Err(e) = run_config(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
