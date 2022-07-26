
#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub tx_id: u32,
    pub tx_type: u8,
    pub client_id: u16,
    pub amount: f64,
    pub state: u8,
}

pub trait TransactionRepositoryTrait {
    fn insert(&mut self, transaction: Transaction);
    fn find(&mut self, tx_id: u32) -> Option<&Transaction>;
    // fn find_all(&mut self) -> Vec<&Transaction>;
    fn find_all(&mut self) -> Vec<Option<&Transaction>>;
}
