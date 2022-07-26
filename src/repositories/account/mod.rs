
pub mod in_memory;

#[derive(PartialEq, Clone, Debug)]
pub struct Account {
    pub client_id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

pub trait AccountRepositoryTrait {
    fn insert(&mut self, account: Account);
    fn find(&mut self, client_id: u16) -> Option<&Account>;
    fn find_all(&mut self) -> Vec<&Account>;
}
