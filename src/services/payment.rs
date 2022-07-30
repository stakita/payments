
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
    fn get_account(&self, client_id: u16) -> Option<&Account> {
        None
    }
}

pub struct PaymentService {
    tx_store: Box<dyn TransactionRepositoryTrait>
}

impl PaymentService {
    pub fn new(tx_store: Box<dyn TransactionRepositoryTrait>) -> PaymentService {
        PaymentService {
            tx_store,
        }
    }
}


impl PaymentServiceTrait for PaymentService {

    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("deposit");
        self.tx_store.insert(Transaction{
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
        self.tx_store.insert(Transaction{
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

    // fn get_account(&self, client_id: u16) -> Option<&Account> {
    // }

    // fn get_accounts(self) -> Vec<Account> {
    // }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::transaction::in_memory::TransactionRepositoryInMemory;

    #[test]
    fn it_can_return_an_account() {
        let transaction_repository = TransactionRepositoryInMemory::new();
        let mut payment_service: Box<dyn PaymentServiceTrait> = Box::new(PaymentService::new(Box::new(transaction_repository)));
        payment_service.deposit(42, 4242, 42.42);
        let acc = payment_service.get_account(42).unwrap();
        assert_eq!(acc, &Account {
            client_id: 42,
            available: 42.42,
            held: 0.0,
            total: 42.42,
            locked: false,
        });
    }
}
