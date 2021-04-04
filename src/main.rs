use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("reading contents of {}...\n", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("An error occurred reading the file.");

    println!("contents are:\n\n{}", contents);
}

struct Config {
    _query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
                return Err("not enough args");
            }

        let _query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { _query, filename })
    }
}
