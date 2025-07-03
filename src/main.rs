use std::{env, process};

use minigrep::config::{Config, print_help};
use minigrep::runner;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--help")) {
        print_help();
        return;
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        println!("Usage: minigrep [OPTIONS] <PATTERN> <FILE>");
        println!("Try 'minigrep --help' for more information.");
        process::exit(1);
    });

    if let Err(e) = runner::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

