
#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub tx_id: u32,
    pub tx_type: u8,
    pub client_id: u16,
    pub amount: f64,
    pub state: u8,
}

pub enum TransactionType {
    Deposit = 0,
    Withdrawal = 1,
}

pub enum TransactionState {
    Normal = 0,
    Disputed = 1,
    Reversed = 2,
}
