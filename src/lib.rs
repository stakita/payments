use std::error::Error;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config {
            filename,
        })
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("running with filename '{}'", config.filename);
    Ok(())
}