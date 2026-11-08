use p2_minigrep::Config;
use std::{env, fs, process};

fn main() {
    let config = Config::parse(env::args().skip(1)).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(2);
    });

    let contents = match fs::read_to_string(&config.path) {
        Ok(c) => c,
        Err(_) => process::exit(1),
    };

    let results = if config.ignore_case {
        p2_minigrep::search_case_insensitive(&config.query, &contents)
    } else {
        p2_minigrep::search(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }
}
