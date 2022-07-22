use std::error::Error;
use csv::{ReaderBuilder, Trim, Reader};
use std::fs::File;
use serde::Deserialize;
use std::fmt;
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

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("running with filename '{}'", config.filename);

    let reader = read_iter(&config.filename)?;
    match stuff(reader, process_transaction) {
        Err(error) => Err(error.into()),
        _other => Ok(()),
    }
}

fn read_iter(filename: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file);

    Ok(reader)
}

fn stuff(mut reader: Reader<File>, func: fn(&Transaction)) -> Result<(), String> {
    let line_offset = 2;
    let mut res = Ok(());
    for (i, result) in reader.deserialize::<Transaction>().enumerate() {
        println!("i: {}", i);
        match result {
            Ok(transaction) => {
                let check = transaction.check();
                match check {
                    Ok(()) => func(&transaction),
                    Err(error) => {
                        res = Err(format!("Invalid transaction on line {}: ({}) {}", i + line_offset, error, transaction.format()));
                        break;
                    }
                }
                func(&transaction);
            },
            Err(error) => eprintln!("error: {}", error),
        }
    }

    res
}

fn process_transaction(transaction: &Transaction) {
    println!("transaction: {:?}", transaction.format());
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
pub struct Transaction {
    #[serde(rename = "type")]
    tx_type: TransactionType,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    tx_id: u32,
    #[serde(rename = "amount")]
    amount: Option<f64>,
}

impl<'a> Transaction {
    fn format(&self) -> String {
        format!("Transaction {{ type: {}, client: {:4?}, tx: {:8?}, amount: {:?} }}",
            self.tx_type,
            self.client_id,
            self.tx_id,
            self.amount
        )
    }

    fn check(&self) -> Result<(), &str> {
        match self.tx_type {
            TransactionType::Dispute |
            TransactionType::Resolve |
            TransactionType::Chargeback => {
                match self.amount {
                    None => Ok(()),
                    _other => Err("Transaction type cannot have an amounts field"),
                }
            },
            TransactionType::Deposit |
            TransactionType::Withdrawal => {
                match self.amount {
                    None => Err("Transaction type must have an amounts field"),
                    _other => Ok(()),
                }
            },
        }
    }
}