use std::io::prelude::*;
use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use serde::Deserialize;

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

    // let mut reader = csv::Reader::from_reader(io::std)
    read_iter(&config.filename, process_line)

    // Ok(())
}

fn read_iter(filename: &str, func: fn(&str)) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        func(&line.unwrap());
    }

    Ok(())
}

fn process_line(line: &str) {
    println!("line: {}", line);
}

#[derive(Debug, Deserialize)]
enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    tx_type: TransactionType,
    client_id: u16,
    tx_id: u32,
    amount: Option<f64>,
}

// impl Transaction {

// }