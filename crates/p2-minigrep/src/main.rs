use p2_minigrep::Config;
use std::{env, fs, process};

fn main() {
    let config = Config::parse(env::args().skip(1)).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(2);
    });
    // TODO: read the file, run search, print matches; exit 1 on I/O error.
    let _ = (config, fs::read_to_string("/dev/null"));
}
