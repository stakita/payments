
pub mod in_memory;

// Use the Account structure from core in repository layer as it is identical
pub use crate::core::entities::account::Account;

impl Account {
    pub fn build_default_account(client_id: u16) -> Account {
        Account {
            client_id: client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}

pub trait AccountRepositoryTrait {
    fn insert(&mut self, account: Account);
    fn update(&mut self, client_id: u16, account: Account);
    fn find(&mut self, client_id: u16) -> Option<&Account>;
    fn find_or_create(&mut self, client_id: u16) -> &Account;
    fn find_all(&mut self) -> Vec<&Account>;
}
