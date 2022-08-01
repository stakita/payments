
use std::{collections::BTreeMap, fmt::Display};
use crate::fmt::Debug;

pub trait InMemoryDatabaseTrait<K, T> {
    fn update(&mut self, id: K, record: T);
    fn find(&mut self, id: K) -> Option<&T>;
    fn find_or_create(&mut self, id: K) -> &T;
    fn find_all(&mut self) -> Vec<&T>;
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
    K: Ord,
    T: DefaultRecord
{

    fn update(&mut self, client_id: K, account: T) {
        self.store.insert(client_id, account);
    }

    fn find(&mut self, client_id: K) -> Option<&T> {
        self.store.get(&client_id)
    }

    fn find_or_create(&mut self, client_id: K) -> &T {
        self.store.entry(client_id).or_insert_with(|| T::default(client_id))
    }

    fn find_all(&mut self) -> Vec<&T> {
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

    #[derive(Debug)]
    pub struct Account {
        pub client_id: u16,
        pub available: f64,
        pub held: f64,
        pub total: f64,
        pub locked: bool,
    }

    impl<u16, Account> DefaultRecord for Account {
        fn default(key: u16) -> Account {
            Account {
                client_id: key,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: false,
            }
        }
    }

    #[test]
    fn can_create_u16_Account_db() {
        let db = InMemoryDatabase::<u16, Account>::new();

        let a1 = Account {
            client_id: 42,
            available: 42.42,
            held: 0.0,
            total: 42.42,
            locked: false,
        };


        db.update(42, a1);
    }

}