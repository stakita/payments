
// Todo: is this needed?
// enum Transaction {
//     Deposit { client_id: u16, tx_id: u32, amount: f64 },
//     Widthdrawal { client_id: u16, tx_id: u32, amount: f64 },
//     Dispute { client_id: u16, tx_id: u32 },
//     Resolve { client_id: u16, tx_id: u32 },
//     Chargeback { client_id: u16, tx_id: u32 },
// }

pub mod transaction {
    use anyhow::Result;

    pub fn deposit(client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("deposit");
        Ok(())
    }

    pub fn withdrawal(client_id: u16, tx_id: u32, amount: f64) -> Result<()> {
        eprintln!("withdrawal");
        Ok(())
    }

    pub fn dispute(client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("dispute");
        Ok(())
    }

    pub fn resolve(client_id: u16, tx_id: u32) -> Result<()> {
        eprintln!("resolve");
        Ok(())
    }

    pub fn chargeback(client_id: u16, tx_id: u32) -> Result<()> {
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
