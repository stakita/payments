use payments::services::payment::{PaymentService, PaymentServiceTrait};
use payments::repositories::transaction::in_memory::build_transaction_repository_in_memory;
use payments::repositories::account::in_memory::build_account_repository_in_memory;

use payments::core::entities::account::Account;


#[test]
fn deposit_creates_in_a_new_account() {
    let transaction_repository = Box::new(build_transaction_repository_in_memory());
    let account_repository = Box::new(build_account_repository_in_memory());
    let mut ps = PaymentService::new(transaction_repository, account_repository);
    let client_id = 42;
    let expected = Account {
        client_id: client_id,
        available: 42.42,
        held: 0.0,
        total: 42.42,
        locked: false,
    };

    let _ = ps.deposit(client_id, 1, 42.42);
    let acc = ps.get_account(client_id).unwrap();
    assert_eq!(acc, &expected);

    let expected2 = Account {
        available: 52.53,
        total: 52.53,
        ..expected
    };

    let _ = ps.deposit(client_id, 1, 10.11);
    let acc = ps.get_account(client_id).unwrap();
    assert_eq!(acc, &expected2);

}
