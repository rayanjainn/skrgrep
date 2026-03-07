use std::{error::Error, fs, path::Path};
use clap::Parser; // Parser trait for CLI derivation
use rayon::prelude::*; // Rayon traits for parallel iterators
use walkdir::WalkDir; // WalkDir for recursive directory traversal

// CLI color codes
const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

/// skrgrep: A fast, multi-threaded grep implementation in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    pub query: String,

    #[arg(default_value = ".")]
    pub path: std::path::PathBuf,

    #[arg(short, long)]
    pub ignore_case: bool,
}

pub fn run_config(config: Config) -> Result<(), Box<dyn Error>> {
    let files: Vec<_> = WalkDir::new(&config.path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != ".git" && name != "target" && name != "node_modules"
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    files.par_iter().for_each(|entry| {
        let path = entry.path();
        
        if config.ignore_case {
            search_file_insensitive(path, &config.query);
        } else {
            search_file(path, &config.query);
        }
    });

    Ok(())
}

fn highlight(line: &str, query: &str, case_sensitive: bool) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    
    let line_for_search = if case_sensitive { line.to_string() } else { line.to_lowercase() };
    let query_for_search = if case_sensitive { query.to_string() } else { query.to_lowercase() };

    for (start, _) in line_for_search.match_indices(&query_for_search) {
        result.push_str(&line[last_end..start]);
        result.push_str(RED);
        result.push_str(&line[start..start + query.len()]);
        result.push_str(RESET);
        last_end = start + query.len();
    }
    result.push_str(&line[last_end..]);
    result
}

fn search_file(path: &Path, query: &str) {
    if let Ok(contents) = fs::read_to_string(path) {
        for (i, line) in contents.lines().enumerate() {
            if let Some(idx) = line.find(&query) {
                let highlighted = highlight(line, query, true);
                println!("{}[{}:{}:{}]{} {}", GREEN, path.display(), i + 1, idx + 1, RESET, highlighted);
            }
        }
    }
}

fn search_file_insensitive(path: &Path, query: &str) {
    let query_lower = query.to_lowercase();
    if let Ok(contents) = fs::read_to_string(path) {
        for (i, line) in contents.lines().enumerate() {
            if let Some(idx) = line.to_lowercase().find(&query_lower) {
                let highlighted = highlight(line, query, false);
                println!("{}[{}:{}:{}]{} {}", GREEN, path.display(), i + 1, idx + 1, RESET, highlighted);
            }
        }
    }
}
