use crate::core::FIXED_DECIMAL_SCALING;

#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub tx_id: u32,
    pub tx_type: u8,
    pub client_id: u16,
    pub amount: i128,
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

impl Transaction {
    pub fn new(tx_id: u32, tx_type: u8, client_id: u16, amount: f64, state: u8) -> Transaction {
        if tx_type != TransactionType::Deposit as u8 && tx_type != TransactionType::Withdrawal as u8
        {
            panic!("Invalid TransactionType");
        }
        if state != TransactionState::Normal as u8
            && state != TransactionState::Disputed as u8
            && state != TransactionState::Reversed as u8
        {
            panic!("Invalid TransactionState");
        }
        Transaction {
            tx_id,
            tx_type,
            client_id,
            amount: Self::to_fixed(amount),
            state,
        }
    }

    pub fn to_fixed(value: f64) -> i128 {
        (value * FIXED_DECIMAL_SCALING as f64).round() as i128
    }

    pub fn from_fixed(value: i128) -> f64 {
        (value as f64 / FIXED_DECIMAL_SCALING as f64).round() as f64
    }
}
