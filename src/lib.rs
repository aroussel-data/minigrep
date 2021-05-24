use std::error::Error;
use std::fs;

pub struct Config {
    _query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
                return Err("not enough args");
            }

        let _query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { _query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("reading contents of {}...\n", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    println!("contents are:\n\n{}", contents);

    Ok(())
}
