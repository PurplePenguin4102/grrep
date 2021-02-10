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
}
