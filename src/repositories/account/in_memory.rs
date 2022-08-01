
use crate::repositories::in_memory::{
    InMemoryDatabase,
    DefaultRecord,
};
pub use crate::core::entities::account::Account;

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

pub fn build_account_repository_in_memory() -> InMemoryDatabase<u16, Account> {
    InMemoryDatabase::<u16, Account>::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::in_memory::InMemoryDatabaseTrait;

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

    #[test]
    fn it_can_insert_and_find_all_sorted() {
        let mut ar = build_account_repository_in_memory();

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
        let mut ar = build_account_repository_in_memory();

        let expected = Account {
            client_id: 42,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };

        let account = ar.find_or_create(42);

        assert_eq!(account, &expected);
    }

    #[test]
    fn it_can_update_an_existing_account() {
        let mut ar = build_account_repository_in_memory();

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

        let account = ar.find_or_create(42);



        assert_eq!(account, &expected);
    }
}
