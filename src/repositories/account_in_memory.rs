
use std::collections::BTreeMap;
use itertools::Itertools;
use crate::repositories::account::{
    Account,
    AccountRepositoryTrait
};

pub struct AccountRepositoryInMemory {
    pub store: BTreeMap<u16, Account>,
}

impl AccountRepositoryInMemory {
    pub fn new() -> AccountRepositoryInMemory {
        AccountRepositoryInMemory {
            store: BTreeMap::new(),
        }
    }
}

impl AccountRepositoryTrait for AccountRepositoryInMemory {

        fn insert(&mut self, account: Account) {
            let _ = &self.store.insert(account.client_id, account);
        }

        fn find(&mut self, client_id: u16) -> Option<&Account> {
            self.store.get(&client_id)
        }

        fn find_all(&mut self) -> Vec<&Account> {
            let mut elements = Vec::<&Account>::new();
            for key in self.store.keys().sorted() {
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

        AccountRepositoryTrait::insert(&mut ar, a.clone());
        AccountRepositoryTrait::insert(&mut ar, b.clone());

        // it finds an inserted key
        let res = AccountRepositoryTrait::find(&mut ar, a.client_id).unwrap();
        assert_eq!(res, &a);

        // it finds an inserted key
        let res = AccountRepositoryTrait::find(&mut ar, b.client_id).unwrap();
        assert_eq!(res, &b);

        // it fails to find an invalid key
        let res = AccountRepositoryTrait::find(&mut ar, 68);
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

        AccountRepositoryTrait::insert(&mut ar, a.clone());
        AccountRepositoryTrait::insert(&mut ar, b.clone());
        AccountRepositoryTrait::insert(&mut ar, c.clone());

        let res = AccountRepositoryTrait::find_all(&mut ar);
        println!("res: {:?}", res);

        assert_eq!(res[0], &c);
        assert_eq!(res[1], &a);
        assert_eq!(res[2], &b);
    }

}