
use std::collections::BTreeMap;

use crate::repositories::in_memory::{
    InMemoryDatabaseTrait,
    InMemoryDatabase,
    DefaultRecord,
};
pub use crate::core::entities::account::Account;

use super::AccountRepositoryTrait;

impl DefaultRecord<u16, Account> for Account {
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

// use crate::repositories::account::{
//     Account,
//     AccountRepositoryTrait
// };

pub struct AccountRepositoryInMemory {
    // pub store: Box<BTreeMap<u16, Account>>,
    pub store: Box<InMemoryDatabase<u16, Account>>,
}

impl AccountRepositoryInMemory {
    pub fn new() -> AccountRepositoryInMemory {
        AccountRepositoryInMemory {
            store: Box::new(InMemoryDatabase::<u16, Account>::new()),
        }
    }

    pub fn print(&mut self) {
        for entry in self.store.find_all() {
            println!("{:?}", entry);
        }
    }
}

impl AccountRepositoryTrait for AccountRepositoryInMemory {

        // fn insert(&mut self, account: Account) {
        //     let _ = &self.store.insert(account.client_id, account);
        // }

        fn update(&mut self, client_id: u16, account: Account) {
            self.store.update(client_id, account);
        }

        fn find(&mut self, client_id: u16) -> Option<&Account> {
            self.store.find(client_id)
        }

        fn find_or_create(&mut self, client_id: u16) -> &Account {
            self.store.find_or_create(client_id)
        }

        fn find_all(&mut self) -> Vec<&Account> {
            self.store.find_all()
        }
}

pub fn build_account_repository_in_memory() -> InMemoryDatabase<u16, Account> {
    InMemoryDatabase::<u16, Account>::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_insert_and_find() {
        let mut ar = build_account_repository_in_memory();

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

    // #[test]
    // fn it_can_insert_and_find_all_sorted() {
    //     let mut ar = AccountRepositoryInMemory::new();
    //     let a = Account {
    //         client_id: 42,
    //         available: 1.23,
    //         held: 0.0,
    //         total: 1.23,
    //         locked: false,
    //     };
    //     let b = Account {
    //         client_id: 420,
    //         available: 10.23,
    //         held: 0.0,
    //         total: 10.23,
    //         locked: true,
    //     };
    //     let c = Account {
    //         client_id: 1,
    //         available: 1.11,
    //         held: 1.22,
    //         total: 1.33,
    //         locked: true,
    //     };

    //     AccountRepositoryTrait::update(&mut ar, a.client_id, a.clone());
    //     AccountRepositoryTrait::update(&mut ar, b.client_id, b.clone());
    //     AccountRepositoryTrait::update(&mut ar, c.client_id, c.clone());

    //     let res = AccountRepositoryTrait::find_all(&mut ar);
    //     println!("res: {:?}", res);

    //     assert_eq!(res[0], &c);
    //     assert_eq!(res[1], &a);
    //     assert_eq!(res[2], &b);
    // }

    // #[test]
    // fn it_can_create_a_new_account() {
    //     let mut ar = AccountRepositoryInMemory::new();
    //     let expected = Account {
    //         client_id: 42,
    //         available: 0.0,
    //         held: 0.0,
    //         total: 0.0,
    //         locked: false,
    //     };

    //     let account = ar.find_or_create(42);

    //     assert_eq!(account, &expected);
    // }

    // #[test]
    // fn it_can_update_an_existing_account() {
    //     let mut ar = AccountRepositoryInMemory::new();

    //     let initial = Account {
    //         client_id: 42,
    //         available: 42.42,
    //         held: 0.0,
    //         total: 42.42,
    //         locked: false,
    //     };

    //     let update = Account {
    //         client_id: 42,
    //         available: 20.23,
    //         held: 3.0,
    //         total: 23.23,
    //         locked: false,
    //     };

    //     let expected = update.clone();

    //     ar.update(42, initial);
    //     ar.update(42, update);

    //     let account = ar.find_or_create(42);



    //     assert_eq!(account, &expected);
    // }
}
