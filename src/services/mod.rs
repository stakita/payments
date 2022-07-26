
use anyhow::Result;

use crate::repositories::transaction::{
    Transaction,
    TransactionType,
    TransactionState,
    TransactionRepositoryTrait,
};

use crate::core::entities::account::Account;


pub trait PaymentServiceTrait {
    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn get_account(&self, client_id: u16) -> Account;
}

pub struct PaymentService {
    store: Box<dyn TransactionRepositoryTrait>
}

impl PaymentService {
    pub fn new(store: Box<dyn TransactionRepositoryTrait>) -> PaymentService {
        PaymentService {
            store: store,
        }
    }
}


impl PaymentServiceTrait for PaymentService {

    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("deposit");
        self.store.insert(Transaction{
            tx_id: tx_id,
            tx_type: Transaction::transaction_type_encode(TransactionType::Deposit),
            client_id: client_id,
            amount: amount,
            state: Transaction::transaction_state_encode(TransactionState::Normal),
        });
        Ok(())
    }

    fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("withdrawal");
        self.store.insert(Transaction{
            tx_id: tx_id,
            tx_type: Transaction::transaction_type_encode(TransactionType::Withdrawal),
            client_id: client_id,
            amount: amount,
            state: Transaction::transaction_state_encode(TransactionState::Normal),
        });
        Ok(())
    }

    fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("dispute");
        Ok(())
    }

    fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("resolve");
        Ok(())
    }

    fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("chargeback");
        Ok(())
    }

    fn get_account(&self, client_id: u16) -> Account {
        Account {
            client_id: client_id,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: true,
        }
    }
}

pub mod account {
    pub fn get(client_id: u16) {
        eprintln!("account::get");
    }

    pub fn get_all() {
        eprintln!("account::get_all");
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_can_return_an_account() {
    }
}
