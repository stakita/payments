
#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub tx_id: u32,
    pub tx_type: u8,
    pub client_id: u16,
    pub amount: f64,
    pub state: u8,
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
}

pub enum TransactionState {
    Normal,
    Disputed,
    Reversed,
}
