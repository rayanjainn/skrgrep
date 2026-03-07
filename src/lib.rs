use std::{fs, path::Path};

const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn search(query: &str, path: &Path) {
    if path.is_file() {
        search_file(path, query);
    } else if path.is_dir() {
        search_directory(path, query);
    }
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

fn search_file(path: &Path, query: &str) -> Vec<String> {
    let mut res = Vec::new();
    if let Ok(contents) = fs::read_to_string(path) {
        for (i, line) in contents.lines().enumerate() {
            if let Some(idx) = line.find(&query) {
                let highlighted = highlight(line, query, true);
                println!("{}[{}:{}:{}]{} {}", GREEN, path.display(), i + 1, idx + 1, RESET, highlighted);
                res.push(String::from(line));
            }
        }
    }
    res
}


fn search_file_insensitive(path: &Path, query: &str) -> Vec<String> {
    let mut res = Vec::new();
    let query_lower = query.to_lowercase();
    if let Ok(contents) = fs::read_to_string(path) {
        for (i, line) in contents.lines().enumerate() {
            if let Some(idx) = line.to_lowercase().find(&query_lower) {
                let highlighted = highlight(line, query, false);
                println!("{}[{}:{}:{}]{} {}", GREEN, path.display(), i + 1, idx + 1, RESET, highlighted);
                res.push(String::from(line));
            }
        }
    }
    res
}

fn search_directory(path: &Path, query: &str) {
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            search_file(&path, query);
        } else if path.is_dir() {
            search_directory(&path, query);
        }
    }
}

pub fn search_case_insensitive (query: &str, path: &Path) {    
    if path.is_file() {
        search_file_insensitive(path, query);
    } else if path.is_dir() {
        search_directory(path, query);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "How";
        let path = Path::new("poem.txt");
        assert_eq!(vec![String::from("How dreary to be somebody!"), 
                        String::from("How public, like a frog")], search_file(path, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "to";
        let path = Path::new("poem.txt");
        assert_eq!(vec![String::from("Are you nobody, too?"),
                        String::from("How dreary to be somebody!"),
                        String::from("To tell your name the livelong day"),
                        String::from("To an admiring bog!")], search_file_insensitive(path, query));
    }
}
