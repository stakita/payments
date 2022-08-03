use anyhow::Result;

use crate::repositories::account::AccountRepositoryTrait;
use crate::repositories::transaction::{
    Transaction, TransactionRepositoryTrait, TransactionState, TransactionType,
};

use crate::core::entities::account::Account;

pub trait PaymentServiceTrait {
    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<()>;
    fn get_account<'a>(&'a mut self, _client_id: u16) -> Option<&'a Account> {
        None
    }
    fn get_accounts<'a>(&'a mut self) -> Vec<&'a Account> {
        Vec::new()
    }
    fn get_transaction<'a>(&'a mut self, _tx_id: u32) -> Option<&'a Transaction> {
        None
    }
    fn get_transactions<'a>(&'a mut self) -> Vec<&'a Transaction> {
        Vec::new()
    }
}

pub struct PaymentService {
    tx_store: Box<dyn TransactionRepositoryTrait>,
    ac_store: Box<dyn AccountRepositoryTrait>,
}

impl PaymentService {
    pub fn new(
        tx_store: Box<dyn TransactionRepositoryTrait>,
        ac_store: Box<dyn AccountRepositoryTrait>,
    ) -> PaymentService {
        PaymentService { tx_store, ac_store }
    }
}

impl PaymentServiceTrait for PaymentService {
    fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("deposit");

        // get account, creating it if needed
        let acc = self.ac_store.find_or_create(client_id).unwrap();

        // bail out if account is locked
        if acc.locked {
            return Err(anyhow!("PaymentServiceError::AccountLocked"));
        }

        // store the transaction
        self.tx_store.update(
            tx_id,
            Transaction {
                tx_id,
                tx_type: Transaction::transaction_type_encode(TransactionType::Deposit),
                client_id,
                amount,
                state: Transaction::transaction_state_encode(TransactionState::Normal),
            },
        );

        // update account
        let acc = Account {
            client_id,
            available: acc.available + Account::to_fixed(amount),
            held: acc.held,
            total: acc.total + Account::to_fixed(amount),
            locked: false,
        };
        let _ = self.ac_store.update(client_id, acc);

        Ok(())
    }

    fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("withdrawal");
        // get account
        let acc = match self.ac_store.find(client_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::AccountDoesNotExist")),
        };

        // bail out if account is locked
        if acc.locked {
            return Err(anyhow!("PaymentServiceError::AccountLocked"));
        }

        // bail out if insufficient funds
        if acc.available() < Account::to_fixed(amount) {
            return Err(anyhow!("PaymentServiceError::InsufficientFunds"));
        }

        self.tx_store.update(
            tx_id,
            Transaction {
                tx_id,
                tx_type: Transaction::transaction_type_encode(TransactionType::Withdrawal),
                client_id,
                amount,
                state: Transaction::transaction_state_encode(TransactionState::Normal),
            },
        );

        // update account
        let acc = Account {
            client_id,
            available: acc.available - Account::to_fixed(amount),
            held: acc.held,
            total: acc.total - Account::to_fixed(amount),
            locked: false,
        };
        let _ = self.ac_store.update(client_id, acc);

        Ok(())
    }

    fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
        // get account
        let acc = match self.ac_store.find(client_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::AccountDoesNotExist")),
        };

        // bail out if account is locked
        if acc.locked {
            return Err(anyhow!("PaymentServiceError::AccountLocked"));
        }

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

    fn get_transaction<'a>(&'a mut self, tx_id: u32) -> Option<&'a Transaction> {
        self.tx_store.find(tx_id)
    }

    fn get_transactions<'a>(&'a mut self) -> Vec<&'a Transaction> {
        self.tx_store.find_all()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::repositories::transaction::TransactionRepositoryTrait;

    // struct MockTransactionRepository {
    //     last_inserted: Option<Transaction>,
    // }

    // impl TransactionRepositoryTrait for  MockTransactionRepository {
    //     fn update(&mut self, tx_id: u32, transaction: Transaction) {
    //         self.last_inserted = Some(transaction);
    //     }

    //     fn find(&mut self, tx_id: u32) -> Option<&Transaction> {
    //         None
    //     }

    //     fn find_all(&mut self) -> Vec<&Transaction> {
    //         Vec::new()
    //     }
    // }

    // impl MockTransactionRepository {
    //     fn get_last_inserted(&self) -> Option<&Transaction> {
    //         match &self.last_inserted {
    //             Some(t) => Some(&t),
    //             None => None,
    //         }
    //     }
    // }

    // #[test]
    // fn deposit_creates_in_a_new_account() {
    //     let transaction_repository = Box::new(MockTransaction {});
    //     let account_repository = Box::new(build_account_repository_in_memory());
    //     let mut ps = PaymentService::new(transaction_repository, account_repository);
    //     let client_id = 42;
    //     let expected = Account {
    //         client_id: client_id,
    //         available: 42.42,
    //         held: 0.0,
    //         total: 42.42,
    //         locked: false,
    //     };

    //     let _ = ps.deposit(client_id, 1, 42.42);
    //     let acc = ps.get_account(client_id).unwrap();
    //     assert_eq!(acc, &expected);

    //     let expected2 = Account {
    //         available: 52.53,
    //         total: 52.53,
    //         ..expected
    //     };

    //     let _ = ps.deposit(client_id, 1, 10.11);
    //     let acc = ps.get_account(client_id).unwrap();
    //     assert_eq!(acc, &expected2);

    // }
}
