use std::error::Error;
use csv::{ReaderBuilder, Trim};
use std::fs::File;
use serde::Deserialize;
use std::fmt;

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

    read_iter(&config.filename, process_transaction)
}

fn read_iter(filename: &str, func: fn(&Transaction)) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file);

    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        func(&transaction);
    }

    Ok(())
}

fn process_transaction(transaction: &Transaction) {
    // println!("transaction: {:?}", transaction);
    transaction.print();
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

impl Transaction {
    fn print(&self) {
        println!("Transaction - type: {}, client: {:4?}, tx: {:8?}, amount: {:?}",
            self.tx_type,
            self.client_id,
            self.tx_id,
            self.amount
        );
    }
}