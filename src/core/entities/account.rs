use crate::core::FIXED_DECIMAL_SCALING;

#[derive(PartialEq, Clone, Debug)]
pub struct Account {
    pub client_id: u16,
    pub available: i128,
    pub held: i128,
    pub total: i128,
    pub locked: bool,
}

impl Account {
    pub fn new(client_id: u16, available: f64, held: f64, total: f64, locked: bool) -> Account {
        Account {
            client_id,
            available: Self::to_fixed(available),
            held: Self::to_fixed(held),
            total: Self::to_fixed(total),
            locked,
        }
    }

    pub fn to_fixed(value: f64) -> i128 {
        (value * FIXED_DECIMAL_SCALING as f64).round() as i128
    }

    pub fn from_fixed(value: i128) -> f64 {
        (value as f64 / FIXED_DECIMAL_SCALING as f64).round() as f64
    }

    pub fn available(&self) -> i128 {
        self.available
    }

    pub fn available_f64(&self) -> f64 {
        Self::from_fixed(self.available)
    }
}
