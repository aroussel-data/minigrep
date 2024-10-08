use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {_err}");
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    }
}
