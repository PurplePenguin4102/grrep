use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let cfg = parse_config(&args);
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", cfg.query);
    println!("in {}", cfg.filename);

    if let Err(e) = run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }

    /*
    --- old main code ---
    let contents = fs::read_to_string(cfg.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    */

}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn parse_config(args: &[String]) -> Config { 
    Config {query: String::from(&args[1]), filename: String::from(&args[2])}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
// alternate exception handling::    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    Ok(())
}

/*
---
example using tuples
---

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("searching for {}", query);
    println!("in {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> (&str, &str)) { 
    (&args[1]), &args[2])
}
*/