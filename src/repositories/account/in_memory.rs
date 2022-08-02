use std::collections::BTreeMap;

use super::AccountRepositoryTrait;
use crate::core::entities::account::Account;

pub struct AccountRepositoryInMemory {
    pub store: Box<BTreeMap<u16, Account>>,
}

impl AccountRepositoryInMemory {
    pub fn new() -> AccountRepositoryInMemory {
        AccountRepositoryInMemory {
            store: Box::new(BTreeMap::new()),
        }
    }

    pub fn default(key: u16) -> Account {
        Account {
            client_id: key,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn print(&self) {
        for (key, value) in self.store.iter() {
            println!("key: {:?}, value: {:?}", key, value);
        }
    }
}

impl AccountRepositoryTrait for AccountRepositoryInMemory {
    fn update(&mut self, client_id: u16, account: Account) {
        self.store.insert(client_id, account);
    }

    fn find(&mut self, client_id: u16) -> Option<&Account> {
        self.store.get(&client_id)
    }

    fn find_or_create(&mut self, client_id: u16) -> Option<&Account> {
        Some(self.store.entry(client_id).or_insert(AccountRepositoryInMemory::default(client_id)))
    }

    fn find_all(&mut self) -> Vec<&Account> {
        let mut elements = Vec::<&Account>::new();
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
        let mut ar = AccountRepositoryInMemory::new();

        let a = Account {
            client_id: 42,
            available: 1.23,
            held: 0.0,
            total: 1.23,
            locked: false,
        };
        let b = Account {
            client_id: 21,
            available: 0.23,
            held: 0.0,
            total: 0.23,
            locked: false,
        };

        ar.update(a.client_id, a.clone());
        ar.update(b.client_id, b.clone());

        // it finds an inserted key
        let res = ar.find(a.client_id).unwrap();
        assert_eq!(res, &a);

        // it finds an inserted key
        let res = ar.find(b.client_id).unwrap();
        assert_eq!(res, &b);

        // it fails to find an invalid key
        let res = ar.find(68);
        assert_eq!(res, None);

    }

    #[test]
    fn it_can_insert_and_find_all_sorted() {
        let mut ar = AccountRepositoryInMemory::new();

        let a = Account {
            client_id: 42,
            available: 1.23,
            held: 0.0,
            total: 1.23,
            locked: false,
        };
        let b = Account {
            client_id: 420,
            available: 10.23,
            held: 0.0,
            total: 10.23,
            locked: true,
        };
        let c = Account {
            client_id: 1,
            available: 1.11,
            held: 1.22,
            total: 1.33,
            locked: true,
        };

        ar.update(a.client_id, a.clone());
        ar.update(b.client_id, b.clone());
        ar.update(c.client_id, c.clone());

        let res = ar.find_all();
        println!("res: {:?}", res);

        assert_eq!(res[0], &c);
        assert_eq!(res[1], &a);
        assert_eq!(res[2], &b);
    }

    #[test]
    fn it_can_create_a_new_account() {
        let mut ar = AccountRepositoryInMemory::new();

        let expected = Account {
            client_id: 42,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };

        let account = ar.find_or_create(42).unwrap();

        assert_eq!(account, &expected);
    }

    #[test]
    fn it_can_update_an_existing_account() {
        let mut ar = AccountRepositoryInMemory::new();

        let initial = Account {
            client_id: 42,
            available: 42.42,
            held: 0.0,
            total: 42.42,
            locked: false,
        };

        let update = Account {
            client_id: 42,
            available: 20.23,
            held: 3.0,
            total: 23.23,
            locked: false,
        };

        let expected = update.clone();

        ar.update(42, initial);
        ar.update(42, update);

        let client_id = 42;
        let account = match ar.find(client_id) {
            Some(a) => a,
            None => {
                ar.update(client_id, AccountRepositoryInMemory::default(client_id));
                ar.find(client_id).unwrap()
            }
        };

        assert_eq!(account, &expected);
    }
}
