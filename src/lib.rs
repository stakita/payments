use csv::{ReaderBuilder, Trim, Reader};
use std::fs::File;
use serde::Deserialize;
use std::fmt;
use std::iter::Iterator;
use anyhow::Result;
#[macro_use]
extern crate anyhow;

mod services;

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

pub fn run(config:Config) -> Result<()> {
    // TODO: remove
    eprintln!("running with filename '{}'", config.filename);

    let transaction_reader = transaction_line_iter(&config.filename)?;
    process_lines(transaction_reader)
}

fn transaction_line_iter(filename: &str) -> Result<Reader<File>> {
    let file = File::open(filename)?;
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file);

    Ok(reader)
}

fn process_lines(mut reader: Reader<File>) -> Result<()> {
    let line_offset = 2;
    for (i, line_result) in reader.deserialize::<TransactionLine>().enumerate() {
        // TODO: remove
        eprintln!("i: {}", i);
        let transaction = line_result.unwrap();

        transaction.validate().map_err(|error| {
            anyhow!("Error processing input line {}: {}", i + line_offset, error)
        })?;

        process_transaction(&transaction).map_err(|error| {
            anyhow!("Error processing input line {}: {}", i + line_offset, error)
        })?;
    }
    Ok(())
}

fn process_transaction(transaction: &TransactionLine) -> Result<()> {
    println!("transaction: {:?}", transaction.format());

    match &transaction.tx_type {
        TransactionType::Deposit => services::transaction::deposit(
            transaction.client_id,
            transaction.tx_id,
            transaction.amount.unwrap()
        ),
        TransactionType::Withdrawal => services::transaction::withdrawal(
            transaction.client_id,
            transaction.tx_id,
            transaction.amount.unwrap()
        ),
        TransactionType::Dispute => services::transaction::dispute(
            transaction.client_id,
            transaction.tx_id
        ),
        TransactionType::Resolve => services::transaction::resolve(
            transaction.client_id,
            transaction.tx_id
        ),
        TransactionType::Chargeback => services::transaction::chargeback(
            transaction.client_id,
            transaction.tx_id
        ),
        _other => Err(anyhow!("Unhandled type: {}", &transaction.tx_type)),
    }

    // Ok(()) => println!("doing stuff with transaction: {}", transaction.format()),
    // Err(error) => Err(format!("Invalid transaction on line {}: ({}) {}", i + line_offset, error, transaction.format()));
    // Ok(())
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

    fn validate(&self) -> Result<()> {
        match self.tx_type {
            TransactionType::Dispute |
            TransactionType::Resolve |
            TransactionType::Chargeback => {
                match self.amount {
                    None => Ok(()),
                    _other => Err(anyhow!("TransactionLine type '{}' cannot have an amounts field", self.tx_type)),
                }
            },
            TransactionType::Deposit |
            TransactionType::Withdrawal => {
                match self.amount {
                    None => Err(anyhow!("TransactionLine type '{}' must have an amounts field", self.tx_type)),
                    _other => Ok(()),
                }
            },
        }
    }
}
