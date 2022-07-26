
pub mod in_memory;

// Use the Account structure from core in repository layer as it is identical
pub use crate::core::entities::account::Account;

pub trait AccountRepositoryTrait {
    fn insert(&mut self, account: Account);
    fn find(&mut self, client_id: u16) -> Option<&Account>;
    fn find_all(&mut self) -> Vec<&Account>;
}
