use std::{env, error::Error, path::{PathBuf}, process};
use minigrep::{search, search_case_insensitive};

struct Config {
    query: String,
    path: PathBuf,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let path = PathBuf::from(&args[2]);

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, path, ignore_case })
    }
}

fn run (config: Config) -> Result<(), Box<dyn Error>>{
    if config.ignore_case {
        search_case_insensitive(&config.query, &config.path)
    } else {
        search(&config.query, &config.path)
    };
    Ok(())
}

fn main() {
    let args : Vec<String> = env::args().collect();
    
    let config = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            println!("Problem parsing arguments: {}", e);
            process::exit(1);
        }
    };
    
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
