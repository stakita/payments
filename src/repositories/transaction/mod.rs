
pub mod in_memory;

// Use the Transaction structure and related from core in repository layer as it is identical
pub use crate::core::entities::transaction::{
    Transaction,
    TransactionType,
    TransactionState,
};


impl Transaction {
    pub fn transaction_type_encode(type_enum: TransactionType) -> u8 {
        match type_enum {
            TransactionType::Deposit => 0,
            TransactionType::Withdrawal => 1,
        }
    }

    pub fn transaction_type_decode(value: u8) -> TransactionType {
        match value {
            0 => TransactionType::Deposit,
            1 => TransactionType::Withdrawal,
            _other => panic!("Unexpected transaction type encoding")
        }
    }

    pub fn transaction_state_encode(state_enum: TransactionState) -> u8 {
        match state_enum {
            TransactionState::Normal => 0,
            TransactionState::Disputed => 1,
            TransactionState::Reversed => 2,
        }
    }

    pub fn transaction_state_decode(value: u8) -> TransactionState {
        match value {
            0 => TransactionState::Normal,
            1 => TransactionState::Disputed,
            2 => TransactionState::Reversed,
            3_u8..=u8::MAX => panic!("Unexpected transaction state encoding")
        }
    }
}


pub trait TransactionRepositoryTrait {
    fn insert(&mut self, transaction: Transaction);
    fn find(&mut self, tx_id: u32) -> Option<&Transaction>;
    fn find_all(&mut self) -> Vec<&Transaction>;
}
