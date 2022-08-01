use csv::{ReaderBuilder, Trim, Reader};
use repositories::account::in_memory::build_account_repository_in_memory;
use std::fs::File;
use serde::Deserialize;
use std::fmt;
use std::iter::Iterator;
use anyhow::Result;
#[macro_use]
extern crate anyhow;

pub mod repositories;
pub mod core;

pub mod services;
use crate::services::payment::{PaymentService, PaymentServiceTrait};
use crate::repositories::transaction::in_memory::build_transaction_repository_in_memory;

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
    // Instantiate here to inject the service into the application functions, specifically process_transaction()
    let transaction_repository = Box::new(build_transaction_repository_in_memory());
    let account_repository = Box::new(build_account_repository_in_memory());

    let mut payment_service: Box<dyn PaymentServiceTrait> = Box::new(PaymentService::new(transaction_repository, account_repository));

    let line_offset = 2;
    for (i, line_result) in reader.deserialize::<TransactionLine>().enumerate() {
        // TODO: remove
        eprintln!("i: {}", i);
        let transaction = line_result.unwrap();

        transaction.validate().map_err(|error| {
            anyhow!("Error processing input line {}: {}", i + line_offset, error)
        })?;

        process_transaction(&transaction, &mut payment_service).map_err(|error| {
            anyhow!("Error processing input line {}: {}", i + line_offset, error)
        })?;
    }

    for account in payment_service.get_accounts() {
        println!("account: {:?}", account);
    }
    Ok(())
}

fn process_transaction(transaction: &TransactionLine, transaction_service: &mut Box<dyn PaymentServiceTrait>) -> Result<()> {
    println!("transaction: {:?}", transaction.format());

    match &transaction.tx_type {
        TransactionType::Deposit => transaction_service.deposit(
            transaction.client_id,
            transaction.tx_id,
            transaction.amount.unwrap()
        ),
        TransactionType::Withdrawal => transaction_service.withdrawal(
            transaction.client_id,
            transaction.tx_id,
            transaction.amount.unwrap()
        ),
        TransactionType::Dispute => transaction_service.dispute(
            transaction.client_id,
            transaction.tx_id
        ),
        TransactionType::Resolve => transaction_service.resolve(
            transaction.client_id,
            transaction.tx_id
        ),
        TransactionType::Chargeback => transaction_service.chargeback(
            transaction.client_id,
            transaction.tx_id
        ),
    }
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
            TransactionType::Deposit    => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
            TransactionType::Dispute    => write!(f, "Dispute"),
            TransactionType::Resolve    => write!(f, "Resolve"),
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


#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::core::entities::account::Account;

    // struct MockPaymentService {}

    // impl MockPaymentService {
    //     fn new() -> MockPaymentService {
    //         MockPaymentService {}
    //     }
    // }

    // impl PaymentServiceTrait for MockPaymentService {
    //     fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
    //         eprintln!("x deposit");
    //         Ok(())
    //     }

    //     fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
    //         eprintln!("x withdrawal");
    //         Ok(())
    //     }

    //     fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
    //         eprintln!("x dispute");
    //         Ok(())
    //     }

    //     fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
    //         eprintln!("x resolve");
    //         Ok(())
    //     }

    //     fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
    //         eprintln!("x chargeback");
    //         Ok(())
    //     }

    //     // fn get_account(&self, client_id: u16) -> Option<&Account> {}

    // }

    // #[test]
    // fn test_calls_deposit() {
    //     let tx_line = TransactionLine {
    //         tx_type: TransactionType::Deposit,
    //         client_id: 1,
    //         tx_id: 1,
    //         amount: Some(1.0),
    //     };

    //     let mut payment_service: Box<dyn PaymentServiceTrait> = Box::new(MockPaymentService::new());

    //     let res = process_transaction(&tx_line, &mut payment_service);
    // }
}
