
use anyhow::Result;

pub trait TransactionServiceTrait {
    fn deposit(&self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn withdrawal(&self, client_id: u16, tx_id: u32, amount: f64) -> Result<()>;
    fn dispute(&self, client_id: u16, tx_id: u32) -> Result<()>;
    fn resolve(&self, client_id: u16, tx_id: u32) -> Result<()>;
    fn chargeback(&self, client_id: u16, tx_id: u32) -> Result<()>;
}

pub struct TransactionService;

impl TransactionService {
    pub fn new() -> TransactionService {
        TransactionService {}
    }
}


impl TransactionServiceTrait for TransactionService {

    fn deposit(&self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("deposit");
        Ok(())
    }

    fn withdrawal(&self, client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("withdrawal");
        Ok(())
    }

    fn dispute(&self, client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("dispute");
        Ok(())
    }

    fn resolve(&self, client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("resolve");
        Ok(())
    }

    fn chargeback(&self, client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("chargeback");
        Ok(())
    }
}

pub mod account {
    pub fn get(client_id: u16) {
        eprintln!("account::get");
    }

    pub fn get_all() {
        eprintln!("account::get_all");
    }
}
