use std::env;
use std::process;
use std::error::Error;

use grrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let cfg = parse_config(&args);
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", cfg.query);
    println!("in {}", cfg.filename);

    if let Err(e) = grrep::run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
