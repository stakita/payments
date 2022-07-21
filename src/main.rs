use std::env;
use std::process;
use payments::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(2);
    });

    if let Err(e) = payments::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
