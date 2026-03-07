use std::process;
use clap::Parser;
use skrgrep::{run_config, Config};

fn main() {
    let config = Config::parse();

    if let Err(e) = run_config(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
