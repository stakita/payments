// use std::error::Error;
// use csv::{ReaderBuilder, Trim, Reader};
// use std::fs::File;
// use serde::Deserialize;
// use std::fmt;
use std::iter::Iterator;


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
/*
pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("running with filename '{}'", config.filename);

    let transaction_reader = transaction_line_iter(&config.filename)?;
    match process_lines(transaction_reader, process_transaction) {
        Err(error) => Err(error.into()),
        _other => Ok(()),
    }
}

fn transaction_line_iter(filename: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file);

    Ok(reader)
}

fn process_lines(mut reader: Reader<File>, func: fn(&TransactionLine)) -> Result<(), String> {
    let line_offset = 2;
    let mut res = Ok(());
    for (i, result) in reader.deserialize::<TransactionLine>().enumerate() {
        println!("i: {}", i);
        match transaction.validate() {
            Ok(()) => match func(&transaction) {
                Err(error) => {
                    eprintln!("recoverable error: {}", error);
                },
                _other => (),
                Err(error) => eprintln!("error: {}", error),
            },
            Err(error) => eprintln!("Error: {}", error),
        };
    }

    res
}

fn process_transaction(transaction: &TransactionLine) {
    // println!("transaction: {:?}", transaction.format());

    Ok(()) => println!("doing stuff with transaction: {}", transaction.format()),
    Err(error) => Err(format!("Invalid transaction on line {}: ({}) {}", i + line_offset, error, transaction.format()));
}

#[derive(Debug, Deserialize)]
enum TransactionType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Deposit    => write!(f, "Deposit   "),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
            TransactionType::Dispute    => write!(f, "Dispute   "),
            TransactionType::Resolve    => write!(f, "Resolve   "),
            TransactionType::Chargeback => write!(f, "Chargeback"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionLine {
    #[serde(rename = "type")]
    tx_type: TransactionType,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    tx_id: u32,
    #[serde(rename = "amount")]
    amount: Option<f64>,
}

impl<'a> TransactionLine {
    fn format(&self) -> String {
        format!("TransactionLine {{ type: {}, client: {:4?}, tx: {:8?}, amount: {:?} }}",
            self.tx_type,
            self.client_id,
            self.tx_id,
            self.amount
        )
    }

    fn validate(&self) -> Result<(), &str> {
        match self.tx_type {
            TransactionType::Dispute |
            TransactionType::Resolve |
            TransactionType::Chargeback => {
                match self.amount {
                    None => Ok(()),
                    _other => Err("TransactionLine type cannot have an amounts field"),
                }
            },
            TransactionType::Deposit |
            TransactionType::Withdrawal => {
                match self.amount {
                    None => Err("TransactionLine type must have an amounts field"),
                    _other => Ok(()),
                }
            },
        }
    }
}
*/
