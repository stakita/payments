mod service {

    enum Transaction {
        Deposit { client_id: u16, tx_id: u32, amount: f64 },
        Widthdrawal { client_id: u16, tx_id: u32, amount: f64 },
        Dispute { client_id: u16, tx_id: u32 },
        Resolve { client_id: u16, tx_id: u32 },
        Chargeback client_id: u16, tx_id: u32 },
    }

    mod transaction {
        fn deposit(client_id: u16, tx_id: u32, amount: f64) {
        }

        fn withdrawal(client_id: u16, tx_id: u32, amount: f64) {
        }

        fn dispute(client_id: u16, tx_id: u32) {
        }

        fn resolve(client_id: u16, tx_id: u32) {
        }

        fn chargeback(client_id: u16, tx_id: u32) {
        }
    }

    mod account {
        fn get(client_id: u16) {

        }

        fn get_all() {

        }
    }
}