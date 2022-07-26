
use std::collections::HashMap;
use itertools::Itertools;
use crate::repositories::transaction::{
    Transaction,
    TransactionRepositoryTrait
};

pub struct TransactionRepositoryInMemory {
    pub store: HashMap<u32, Transaction>,
}

impl TransactionRepositoryInMemory {
    pub fn new() -> TransactionRepositoryInMemory {
        TransactionRepositoryInMemory {
            store: HashMap::new(),
        }
    }
}

impl TransactionRepositoryTrait for TransactionRepositoryInMemory {

        fn insert(&mut self, transaction: Transaction) {
            let _ = &self.store.insert(transaction.tx_id, transaction);
        }

        fn find(&mut self, tx_id: u32) -> Option<&Transaction> {
            self.store.get(&tx_id)
        }

        fn find_all(&mut self) -> Vec<Option<&Transaction>> {
            let mut elements = Vec::<Option<&Transaction>>::new();
            for key in self.store.keys().sorted() {
                elements.push(self.store.get(key));
            }
            elements
        }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_insert_and_find() {
        let mut ar = TransactionRepositoryInMemory::new();
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

        TransactionRepositoryTrait::insert(&mut ar, a.clone());
        TransactionRepositoryTrait::insert(&mut ar, b.clone());

        // it finds an inserted key
        let res = TransactionRepositoryTrait::find(&mut ar, a.tx_id).unwrap();
        assert_eq!(res, &a);

        // it finds an inserted key
        let res = TransactionRepositoryTrait::find(&mut ar, b.tx_id).unwrap();
        assert_eq!(res, &b);

        // it fails to find an invalid key
        let res = TransactionRepositoryTrait::find(&mut ar, 68);
        assert_eq!(res, None);

    }

    #[test]
    fn it_can_insert_and_find_all() {
        let mut ar = TransactionRepositoryInMemory::new();
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

        TransactionRepositoryTrait::insert(&mut ar, a.clone());
        TransactionRepositoryTrait::insert(&mut ar, b.clone());
        TransactionRepositoryTrait::insert(&mut ar, c.clone());

        let res = TransactionRepositoryTrait::find_all(&mut ar);
        // println!("res: {:?}", res);
        for i in 0..res.len() {
            println!("res[{}]: {:?}", i, res.get(i).unwrap().unwrap());
        }

        assert_eq!(res.get(0).unwrap().unwrap(), &c);
        assert_eq!(res.get(1).unwrap().unwrap(), &a);
        assert_eq!(res.get(2).unwrap().unwrap(), &b);
    }

}