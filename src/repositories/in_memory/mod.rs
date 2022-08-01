
use std::collections::BTreeMap;
use crate::fmt::Debug;

pub trait InMemoryDatabaseTrait<K, T> {
    fn update(&mut self, id: K, record: T);
    fn find<'a>(&'a mut self, id: K) -> Option<&'a T>;
    fn find_or_create<'a>(&'a mut self, id: K) -> &'a T;
    fn find_all<'a>(&'a mut self) -> Vec<&'a T>;
}

pub struct InMemoryDatabase<K, T> {
    pub store: Box<BTreeMap<K, T>>,
}

impl<K, T> InMemoryDatabase<K, T>
where
    K: Debug,
    T: Debug,
{
    pub fn new() -> InMemoryDatabase<K, T> {
        InMemoryDatabase {
            store: Box::new(BTreeMap::new()),
        }
    }

    pub fn print(&self) {
        for (key, value) in self.store.iter() {
            println!("key: {:?}, value: {:?}", key, value);
        }
    }
}

pub trait DefaultRecord<K, T>: Sized {
    fn default(key: K) -> T;
}


impl<K, T> InMemoryDatabaseTrait<K, T> for InMemoryDatabase<K, T>
where
    K: Ord + Copy,
    T: DefaultRecord<K, T>
{

    fn update(&mut self, client_id: K, account: T) {
        self.store.insert(client_id, account);
    }

    fn find<'a>(&'a mut self, client_id: K) -> Option<&'a T> {
        self.store.get(&client_id)
    }

    fn find_or_create<'a>(&'a mut self, client_id: K) -> &'a T {
        self.store.entry(client_id).or_insert(T::default(client_id))
    }

    fn find_all<'a>(&'a mut self) -> Vec<&'a T> {
        let mut elements = Vec::<&T>::new();
        for key in self.store.keys() {
            elements.push(self.store.get(key).unwrap());
        }
        elements
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    pub use crate::core::entities::account::Account;
    pub use crate::core::entities::transaction::Transaction;

    #[test]
    fn can_create_u16_account_db() {
        let mut db = InMemoryDatabase::<u16, Account>::new();

        let a1 = Account {
            client_id: 42,
            available: 42.42,
            held: 0.0,
            total: 42.42,
            locked: false,
        };

        db.update(42, a1.clone());

        let a2 = Account {
            client_id: 52,
            available: 52.52,
            held: 0.0,
            total: 52.52,
            locked: false,
        };

        db.update(52, a2.clone());

        db.find_or_create(55);

        assert_eq!(db.find(42).unwrap(), &a1);
        assert_eq!(db.find(52).unwrap(), &a2);
        assert_eq!(db.find(55).unwrap(), &Account {
            client_id: 55,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        });

        let res = db.find_all();
        println!("res: {:?}", res);
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn can_create_u32_transaction_db() {
        let mut db = InMemoryDatabase::<u32, Transaction>::new();

        let t1 = Transaction { tx_id: 1111111, tx_type: 0, client_id: 11, amount: 11.11, state: 0 };

        db.update(1111111, t1.clone());

        let t2 = Transaction { tx_id: 2222222, tx_type: 0, client_id: 22, amount: 22.22, state: 0 };

        db.update(2222222, t2.clone());

        assert_eq!(db.find(1111111).unwrap(), &t1);
        assert_eq!(db.find(2222222).unwrap(), &t2);

        let res = db.find_all();
        println!("res: {:?}", res);
        assert_eq!(res.len(), 2);
    }

}