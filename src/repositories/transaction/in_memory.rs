

use crate::repositories::in_memory::{
    InMemoryDatabase,
    DefaultRecord,
};
pub use crate::core::entities::transaction::Transaction;

impl DefaultRecord<u32, Transaction> for Transaction {
    fn default(_key: u32) -> Transaction {
        panic!("Building default transaction doesn't make sense")
        // Transaction { tx_id: key, tx_type: 0, client_id: 0, amount: 0.0, state: 0 }
    }
}

pub fn build_transaction_repository_in_memory() -> InMemoryDatabase<u32, Transaction> {
    InMemoryDatabase::<u32, Transaction>::new()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::in_memory::InMemoryDatabaseTrait;

    #[test]
    fn it_can_insert_and_find() {
        let mut tr = build_transaction_repository_in_memory();

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
        let mut tr = build_transaction_repository_in_memory();

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