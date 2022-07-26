
#[derive(PartialEq, Clone, Debug)]
pub struct Account {
    pub client_id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

