use std::collections::BTreeMap;

use super::TransactionRepositoryTrait;
use crate::core::entities::transaction::Transaction;

pub struct TransactionRepositoryInMemory {
    pub store: Box<BTreeMap<u32, Transaction>>,
}

impl TransactionRepositoryInMemory {
    pub fn new() -> TransactionRepositoryInMemory {
        TransactionRepositoryInMemory {
            store: Box::new(BTreeMap::new()),
        }
    }

    pub fn print(&self) {
        for (key, value) in self.store.iter() {
            println!("key: {:?}, value: {:?}", key, value);
        }
    }
}

impl TransactionRepositoryTrait for TransactionRepositoryInMemory {
    fn update(&mut self, tx_id: u32, transaction: Transaction) {
        self.store.insert(tx_id, transaction);
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

        let a = Transaction::new(1600002, 1, 42, 1.23, 0);
        let b = Transaction::new(1600004, 1, 21, 2.23, 1);

        tr.update(a.tx_id, a.clone());
        tr.update(b.tx_id, b.clone());

        // it finds an inserted key
        let res = tr.find(a.tx_id).unwrap();
        assert_eq!(res, &a);

        // it finds an inserted key
        let res = tr.find(b.tx_id).unwrap();
        assert_eq!(res, &b);

        // it fails to find an invalid key
        let res = tr.find(68);
        assert_eq!(res, None);

        let res = tr.find_all();
        println!("res: {:?}", res);
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn it_can_insert_and_find_all_sorted() {
        let mut tr = TransactionRepositoryInMemory::new();

        let a = Transaction::new(1600042, 1, 42, 1.23, 0);
        let b = Transaction::new(1600056, 1, 21, 2.23, 1);
        let c = Transaction::new(16, 0, 1, 700.11, 2);

        tr.update(a.tx_id, a.clone());
        tr.update(b.tx_id, b.clone());
        tr.update(c.tx_id, c.clone());

        let res = tr.find_all();
        println!("res: {:?}", res);
        assert_eq!(res.len(), 3);

        assert_eq!(res[0], &c);
        assert_eq!(res[1], &a);
        assert_eq!(res[2], &b);
    }
}
