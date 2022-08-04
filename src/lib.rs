use anyhow::Result;
use csv::{Reader, ReaderBuilder, Trim};
use repositories::{
    account::in_memory::AccountRepositoryInMemory,
    transaction::in_memory::TransactionRepositoryInMemory,
};
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::iter::Iterator;
#[macro_use]
extern crate anyhow;

pub mod core;
pub mod repositories;

pub mod services;
use crate::core::entities::account::Account;
use crate::services::payment::{PaymentService, PaymentServiceTrait};

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<()> {
    let transaction_reader = transaction_line_iter(&config.filename)?;

    // Instantiate here to inject the service into the application functions, specifically process_transaction()
    let transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let account_repository = Box::new(AccountRepositoryInMemory::new());
    let payment_service: Box<dyn PaymentServiceTrait> = Box::new(PaymentService::new(
        transaction_repository,
        account_repository,
    ));

    process_lines(transaction_reader, payment_service)
}

fn transaction_line_iter(filename: &str) -> Result<Reader<File>> {
    let file = File::open(filename)?;
    let reader = ReaderBuilder::new().trim(Trim::All).from_reader(file);

    Ok(reader)
}

fn process_lines(
    mut reader: Reader<File>,
    mut payment_service: Box<dyn PaymentServiceTrait>,
) -> Result<()> {
    let line_offset = 2; // Offset due to bing zero indexed and header line is skipped
    for (i, line_result) in reader.deserialize::<TransactionLine>().enumerate() {
        let transaction = line_result.unwrap();

        transaction.validate().map_err(|error| {
            anyhow!("Error processing input line {}: {}", i + line_offset, error)
        })?;

        let _result = process_transaction(&transaction, &mut payment_service);

        // Handle error based on client requirements.
        // match _result {
        //     Err(error) => {
        //         eprintln!("Error processing input line {}: {}", i + line_offset, error);
        //     }
        //     Ok(_) => (),
        // }
    }

    println!("client, available, held, total, locked");
    for account in payment_service.get_accounts() {
        println!(
            "{}, {:0.4}, {:0.4}, {:0.4}, {}",
            account.client_id,
            Account::from_fixed(account.available),
            Account::from_fixed(account.held),
            Account::from_fixed(account.total),
            account.locked
        );
    }
    Ok(())
}

fn process_transaction(
    transaction: &TransactionLine,
    transaction_service: &mut Box<dyn PaymentServiceTrait>,
) -> Result<()> {
    // println!("transaction: {:?}", transaction.format());

    match &transaction.tx_type {
        TransactionType::Deposit => transaction_service.deposit(
            transaction.client_id,
            transaction.tx_id,
            transaction.amount.unwrap(),
        ),
        TransactionType::Withdrawal => transaction_service.withdrawal(
            transaction.client_id,
            transaction.tx_id,
            transaction.amount.unwrap(),
        ),
        TransactionType::Dispute => {
            transaction_service.dispute(transaction.client_id, transaction.tx_id)
        }
        TransactionType::Resolve => {
            transaction_service.resolve(transaction.client_id, transaction.tx_id)
        }
        TransactionType::Chargeback => {
            transaction_service.chargeback(transaction.client_id, transaction.tx_id)
        }
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
    Chargeback,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
            TransactionType::Dispute => write!(f, "Dispute"),
            TransactionType::Resolve => write!(f, "Resolve"),
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
    // fn format(&self) -> String {
    //     format!(
    //         "TransactionLine {{ type: {}, client: {:4?}, tx: {:8?}, amount: {:?} }}",
    //         self.tx_type, self.client_id, self.tx_id, self.amount
    //     )
    // }

    fn validate(&self) -> Result<()> {
        match self.tx_type {
            TransactionType::Dispute | TransactionType::Resolve | TransactionType::Chargeback => {
                match self.amount {
                    None => Ok(()),
                    _other => Err(anyhow!(
                        "TransactionLine type '{}' cannot have an amounts field",
                        self.tx_type
                    )),
                }
            }
            TransactionType::Deposit | TransactionType::Withdrawal => match self.amount {
                None => Err(anyhow!(
                    "TransactionLine type '{}' must have an amounts field",
                    self.tx_type
                )),
                _other => Ok(()),
            },
        }
    }
}

#[cfg(test)]
mod tests {}
