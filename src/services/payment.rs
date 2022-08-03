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
            Transaction::new(
                tx_id,
                Transaction::transaction_type_encode(TransactionType::Deposit),
                client_id,
                amount,
                Transaction::transaction_state_encode(TransactionState::Normal),
            ),
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
            Transaction::new(
                tx_id,
                Transaction::transaction_type_encode(TransactionType::Withdrawal),
                client_id,
                amount,
                Transaction::transaction_state_encode(TransactionState::Normal),
            ),
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
        // handle non-existant transaction
        let tx = match self.tx_store.find(tx_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::TransactionDoesNotExist")),
        };
        // handle incorrect transaction state
        if tx.state != TransactionState::Normal as u8 {
            return Err(anyhow!("PaymentServiceError::InvalidTransactionState"));
        }

        let acc = Account {
            client_id: acc.client_id,
            available: acc.available - tx.amount,
            held: acc.held + tx.amount,
            total: acc.total,
            locked: acc.locked,
        };
        let tx = Transaction {
            state: TransactionState::Disputed as u8,
            ..*tx
        };

        self.ac_store.update(acc.client_id, acc);
        self.tx_store.update(tx.tx_id, tx);

        Ok(())
    }

    fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
        // get account
        let acc = match self.ac_store.find(client_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::AccountDoesNotExist")),
        };
        // bail out if account is locked
        if acc.locked {
            return Err(anyhow!("PaymentServiceError::AccountLocked"));
        }
        // handle non-existant transaction
        let tx = match self.tx_store.find(tx_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::TransactionDoesNotExist")),
        };
        // handle incorrect transaction state
        if tx.state != TransactionState::Disputed as u8 {
            return Err(anyhow!("PaymentServiceError::InvalidTransactionState"));
        }

        let acc = Account {
            client_id: acc.client_id,
            available: acc.available + tx.amount,
            held: acc.held - tx.amount,
            total: acc.total,
            locked: acc.locked,
        };
        let tx = Transaction {
            state: TransactionState::Normal as u8,
            ..*tx
        };

        self.ac_store.update(acc.client_id, acc);
        self.tx_store.update(tx.tx_id, tx);

        Ok(())
    }

    fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<()> {
        // get account
        let acc = match self.ac_store.find(client_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::AccountDoesNotExist")),
        };
        // bail out if account is locked
        if acc.locked {
            return Err(anyhow!("PaymentServiceError::AccountLocked"));
        }
        // handle non-existant transaction
        let tx = match self.tx_store.find(tx_id) {
            Some(a) => a,
            None => return Err(anyhow!("PaymentServiceError::TransactionDoesNotExist")),
        };
        // handle incorrect transaction state
        if tx.state != TransactionState::Disputed as u8 {
            return Err(anyhow!("PaymentServiceError::InvalidTransactionState"));
        }

        let acc = Account {
            client_id: acc.client_id,
            available: acc.available,
            held: acc.held - tx.amount,
            total: acc.total - tx.amount,
            locked: true,
        };
        let tx = Transaction {
            state: TransactionState::Reversed as u8,
            ..*tx
        };

        self.ac_store.update(acc.client_id, acc);
        self.tx_store.update(tx.tx_id, tx);

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
mod tests {}
