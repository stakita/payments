
use std::collections::BTreeMap;
use crate::repositories::transaction::{
    Transaction,
    TransactionRepositoryTrait
};

pub struct TransactionRepositoryInMemory {
    pub store: BTreeMap<u32, Transaction>,
}

impl TransactionRepositoryInMemory {
    pub fn new() -> TransactionRepositoryInMemory {
        TransactionRepositoryInMemory {
            store: BTreeMap::new(),
        }
    }

    pub fn print(&self) {
        for (key, value) in &self.store {
            println!("key: {}, value: {:?}", key, value);
        }
    }
}

impl TransactionRepositoryTrait for TransactionRepositoryInMemory {
        fn insert(&mut self, transaction: Transaction) {
            let _ = &self.store.insert(transaction.tx_id, transaction);
            self.print();
        }

        fn find(&mut self, tx_id: u32) -> Option<&Transaction> {
            self.store.get(&tx_id)
        }

        fn find_all(&mut self) -> Vec<&Transaction> {
            let mut elements = Vec::<&Transaction>::new();
            for key in self.store.keys() {
                elements.push(self.store.get(key).unwrap());
            }
            elements
        }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_insert_and_find() {
        let mut tr = TransactionRepositoryInMemory::new();
        let a = Transaction {
            tx_id: 1600002,
            tx_type: 1,
            client_id: 42,
            amount: 1.23,
            state: 0,
        };
        let b = Transaction {
            tx_id: 1600004,
            tx_type: 1,
            client_id: 21,
            amount: 2.23,
            state: 1,
        };

        TransactionRepositoryTrait::insert(&mut tr, a.clone());
        TransactionRepositoryTrait::insert(&mut tr, b.clone());

        // it finds an inserted key
        let res = TransactionRepositoryTrait::find(&mut tr, a.tx_id).unwrap();
        assert_eq!(res, &a);

        // it finds an inserted key
        let res = TransactionRepositoryTrait::find(&mut tr, b.tx_id).unwrap();
        assert_eq!(res, &b);

        // it fails to find an invalid key
        let res = TransactionRepositoryTrait::find(&mut tr, 68);
        assert_eq!(res, None);

    }

    #[test]
    fn it_can_insert_and_find_all_sorted() {
        let mut tr = TransactionRepositoryInMemory::new();
        let a = Transaction {
            tx_id: 1600042,
            tx_type: 1,
            client_id: 42,
            amount: 1.23,
            state: 0,
        };
        let b = Transaction {
            tx_id: 1600056,
            tx_type: 1,
            client_id: 21,
            amount: 2.23,
            state: 1,
        };
        let c = Transaction {
            tx_id: 16,
            tx_type: 0,
            client_id: 1,
            amount: 700.11,
            state: 2,
        };

        TransactionRepositoryTrait::insert(&mut tr, a.clone());
        TransactionRepositoryTrait::insert(&mut tr, b.clone());
        TransactionRepositoryTrait::insert(&mut tr, c.clone());

        let res = TransactionRepositoryTrait::find_all(&mut tr);

        println!("res: {:?}", res);

        assert_eq!(res[0], &c);
        assert_eq!(res[1], &a);
        assert_eq!(res[2], &b);
    }

}