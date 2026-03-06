use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }
    
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
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
