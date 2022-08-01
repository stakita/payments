
use anyhow::Result;

use crate::repositories::in_memory::InMemoryDatabaseTrait;
use crate::repositories::transaction::{
    Transaction,
    TransactionType,
    TransactionState,
};

use crate::core::entities::account::Account;


pub trait PaymentServiceTrait {
    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn get_account<'a>(&'a mut self, client_id: u16) -> Option<&'a Account> {
        None
    }
    fn get_accounts<'a>(&'a mut self) -> Vec<&'a Account> {
        Vec::new()
    }
}

pub struct PaymentService {
    tx_store: Box<dyn InMemoryDatabaseTrait<u32, Transaction>>,
    ac_store: Box<dyn InMemoryDatabaseTrait<u16, Account>>,
}

impl PaymentService {
    pub fn new(tx_store: Box<dyn InMemoryDatabaseTrait<u32, Transaction>>, ac_store: Box<dyn InMemoryDatabaseTrait<u16, Account>>) -> PaymentService {
        PaymentService {
            tx_store,
            ac_store,
        }
    }
}


impl PaymentServiceTrait for PaymentService {

    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("deposit");

        // Check if account exists
            // If account locked: skip transaction
            // else continue
        // else
            // create
        // get account, creating it if needed
        let acc = self.ac_store.find_or_create(client_id);

        // store the transaction
        self.tx_store.update(tx_id, Transaction{
            tx_id,
            tx_type: Transaction::transaction_type_encode(TransactionType::Deposit),
            client_id,
            amount,
            state: Transaction::transaction_state_encode(TransactionState::Normal),
        });

        // update account
        let acc = Account {
            client_id,
            available: acc.available + amount,
            held: acc.held,
            total: acc.total + amount,
            locked: false,
        };
        let _ = self.ac_store.update(client_id, acc);

        Ok(())
    }

    fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("withdrawal");
        self.tx_store.update(tx_id, Transaction{
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

    fn get_account<'a>(&'a mut self, client_id: u16) -> Option<&'a Account> {
        self.ac_store.find(client_id)
    }

    fn get_accounts<'a>(&'a mut self) -> Vec<&'a Account> {
        self.ac_store.find_all()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::transaction::in_memory::build_transaction_repository_in_memory;
    use crate::repositories::account::in_memory::build_account_repository_in_memory;

    #[test]
    fn deposit_creates_in_a_new_account() {
        let transaction_repository = Box::new(build_transaction_repository_in_memory());
        let account_repository = Box::new(build_account_repository_in_memory());
        let mut ps = PaymentService::new(transaction_repository, account_repository);
        let client_id = 42;
        let expected = Account {
            client_id: client_id,
            available: 42.42,
            held: 0.0,
            total: 42.42,
            locked: false,
        };

        let _ = ps.deposit(client_id, 1, 42.42);
        let acc = ps.get_account(client_id).unwrap();
        assert_eq!(acc, &expected);

        let expected2 = Account {
            available: 52.53,
            total: 52.53,
            ..expected
        };

        let _ = ps.deposit(client_id, 1, 10.11);
        let acc = ps.get_account(client_id).unwrap();
        assert_eq!(acc, &expected2);

    }
}
