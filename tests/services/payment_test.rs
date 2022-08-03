use payments::repositories::account::in_memory::AccountRepositoryInMemory;
use payments::repositories::transaction::in_memory::TransactionRepositoryInMemory;
use payments::services::payment::{PaymentService, PaymentServiceTrait};

use payments::core::entities::account::Account;
use payments::core::entities::transaction::{Transaction, TransactionState, TransactionType};

fn build_payments_service() -> PaymentService {
    let transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let account_repository = Box::new(AccountRepositoryInMemory::new());
    PaymentService::new(transaction_repository, account_repository)
}

fn build_payments_service_with_locked_account(client_id: u16) -> PaymentService {
    let transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let mut account_repository = Box::new(AccountRepositoryInMemory::new());

    // Create a locked account
    let expected_ac = Account::new(client_id, 42.42, 0.0, 42.42, true);
    account_repository
        .as_mut()
        .store
        .insert(client_id, expected_ac.clone());

    PaymentService::new(transaction_repository, account_repository)
}

fn build_payments_service_with_empty_account(client_id: u16) -> PaymentService {
    let transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let mut account_repository = Box::new(AccountRepositoryInMemory::new());

    // Create a locked account
    let expected_ac = Account::new(client_id, 0.0, 0.0, 0.0, false);
    account_repository
        .as_mut()
        .store
        .insert(client_id, expected_ac.clone());

    PaymentService::new(transaction_repository, account_repository)
}

fn build_payments_service_with_default_account(client_id: u16) -> PaymentService {
    let transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let mut account_repository = Box::new(AccountRepositoryInMemory::new());

    // Create a locked account
    let expected_ac = Account::new(client_id, 50.0, 10.0, 60.0, false);
    account_repository
        .as_mut()
        .store
        .insert(client_id, expected_ac.clone());

    PaymentService::new(transaction_repository, account_repository)
}

#[test]
fn deposit_creates_in_a_new_account() {
    let mut ps = build_payments_service();

    let client_id = 42;
    let expected_ac = Account::new(client_id, 42.42, 0.0, 42.42, false);
    let tx_id = 1;
    let expected_tr = Transaction::new(
        tx_id,
        TransactionType::Deposit as u8,
        client_id,
        42.42,
        TransactionState::Normal as u8,
    );

    let acc = ps.get_account(client_id);
    assert_eq!(acc, None);

    assert_eq!((), ps.deposit(client_id, tx_id, 42.42).unwrap());

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);
    assert_eq!(ps.get_accounts().len(), 1);
    assert_eq!(ps.get_transactions().len(), 1);

    println!("accounts: {:?}", ps.get_accounts());
    println!("transactions: {:?}", ps.get_transactions());
}

#[test]
fn deposit_updates_total_and_available() {
    let client_id = 42;
    let mut ps = build_payments_service_with_empty_account(client_id);

    let expected_ac = Account::new(client_id, 0.0001, 0.0, 0.0001, false);
    let tx_id = 2;
    let expected_tr = Transaction::new(
        tx_id,
        TransactionType::Deposit as u8,
        client_id,
        0.0001,
        TransactionState::Normal as u8,
    );

    assert_eq!((), ps.deposit(client_id, tx_id, 0.0001).unwrap()); // Smallest deposit

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);

    let expected_ac = Account {
        available: Account::to_fixed(0.0002),
        total: Account::to_fixed(0.0002),
        ..expected_ac
    };
    let tx_id = 3;
    let expected_tr = Transaction::new(
        tx_id,
        TransactionType::Deposit as u8,
        client_id,
        0.0001,
        TransactionState::Normal as u8,
    );

    assert_eq!((), ps.deposit(client_id, tx_id, 0.0001).unwrap()); // Smallest deposit

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);

    assert_eq!(ps.get_accounts().len(), 1);
    assert_eq!(ps.get_transactions().len(), 2);

    println!("accounts: {:?}", ps.get_accounts());
    println!("transactions: {:?}", ps.get_transactions());
}

#[test]
fn deposit_does_not_change_locked_account() {
    let client_id = 42;
    let mut ps = build_payments_service_with_locked_account(client_id);

    let initial_ac = ps.get_account(client_id).unwrap().clone();

    let tx_id = 1;
    assert!(ps.deposit(client_id, tx_id, 0.01).is_err());

    assert_eq!(ps.get_account(client_id).unwrap(), &initial_ac); // No change
    assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
}

#[test]
fn withdrawal_updates_total_and_available() {
    let client_id = 42;
    let mut ps = build_payments_service_with_default_account(client_id);

    let expected_ac = Account::new(client_id, 50.0, 10.0, 60.0, false);

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);

    // Succeeds if less than available amount

    let expected_ac = Account::new(client_id, 49.9999, 10.0, 59.9999, false);
    let tx_id = 1;
    let expected_tr = Transaction::new(
        tx_id,
        TransactionType::Withdrawal as u8,
        client_id,
        0.0001,
        TransactionState::Normal as u8,
    );

    assert_eq!((), ps.withdrawal(client_id, tx_id, 0.0001).unwrap());

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);

    assert_eq!(ps.get_accounts().len(), 1);
    assert_eq!(ps.get_transactions().len(), 1);

    // Succeeds if exactly available amount

    let expected_ac = Account::new(client_id, 0.0, 10.0, 10.0, false);

    let tx_id = 2;
    let expected_tr = Transaction::new(
        tx_id,
        TransactionType::Withdrawal as u8,
        client_id,
        49.9999,
        TransactionState::Normal as u8,
    );

    assert_eq!((), ps.withdrawal(client_id, tx_id, 49.9999).unwrap());

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);

    assert_eq!(ps.get_accounts().len(), 1);
    assert_eq!(ps.get_transactions().len(), 2);

    println!("accounts: {:?}", ps.get_accounts());
    println!("transactions: {:?}", ps.get_transactions());
}

#[test]
fn withdrawal_fails_if_insufficient_funds() {
    let client_id = 42;
    let mut ps = build_payments_service_with_default_account(client_id);

    let expected_ac = Account::new(client_id, 50.0, 10.0, 60.0, false);

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);

    // Fail on insufficient funds

    let tx_id = 1;
    assert!(ps.withdrawal(client_id, tx_id, 100.0).is_err());

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac); // unchanged
    assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction

    assert_eq!(ps.get_accounts().len(), 1);
    assert_eq!(ps.get_transactions().len(), 0);

    println!("accounts: {:?}", ps.get_accounts());
    println!("transactions: {:?}", ps.get_transactions());
}

#[test]
fn withdrawal_does_not_change_locked_account() {
    let client_id = 42;
    let mut ps = build_payments_service_with_locked_account(client_id);

    let initial_ac = ps.get_account(client_id).unwrap().clone();

    let tx_id = 1;
    assert!(ps.withdrawal(client_id, tx_id, 0.01).is_err());

    assert_eq!(ps.get_account(client_id).unwrap(), &initial_ac); // No change
    assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
}

#[test]
fn withdrawal_on_non_existant_account_fails() {
    let client_id = 42;
    let mut ps = build_payments_service(); // No accounts

    let tx_id = 1;
    assert!(ps.withdrawal(client_id, tx_id, 0.01).is_err());

    assert_eq!(ps.get_account(client_id), None); // No account
    assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
}

// #[test]
// fn dispute_adjusts_account_amounts_and_transaction_state() {}

// #[test]
// fn dispute_handles_missing_transaction_or_incorrect_state() {
//     let client_id = 42;
//     let mut ps = build_payments_service_with_default_account(client_id);

//     let initial_ac = ps.get_account(client_id).unwrap().clone();

//     let tx_id = 1;
//     assert!(ps.dispute(client_id, tx_id).is_err());

//     assert_eq!(ps.get_account(client_id).unwrap(), &initial_ac); // No change
//     assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
// }

// #[test]
// fn dispute_handles_incorrect_transaction_state() {
//     // let client_id = 42;
//     // let mut ps = build_payments_service(); // No accounts

//     // let transaction_repository = Box::new(TransactionRepositoryInMemory::new());
//     // let mut account_repository = Box::new(AccountRepositoryInMemory::new());

//     // // Create account
//     // let ac = Account::new(client_id, 50.0, 10.0, 60.0, true);
//     // // let tr = Transaction::new(client_id, 50.0, 10.0, 60.0, false);
//     // account_repository
//     //     .as_mut()
//     //     .store
//     //     .insert(client_id, expected_ac.clone());

//     // PaymentService::new(transaction_repository, account_repository)
//     todo!("not written");
// }

// #[test]
// fn dispute_does_not_change_locked_account() {
//     let client_id = 42;
//     let mut ps = build_payments_service_with_locked_account(client_id);

//     let initial_ac = ps.get_account(client_id).unwrap().clone();

//     let tx_id = 1;
//     assert!(ps.dispute(client_id, tx_id).is_err());

//     assert_eq!(ps.get_account(client_id).unwrap(), &initial_ac); // No change
//     assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
// }

// #[test]
// fn dispute_on_non_existant_account_fails() {
//     let client_id = 42;
//     let mut ps = build_payments_service(); // No accounts

//     let tx_id = 1;
//     assert!(ps.dispute(client_id, tx_id).is_err());

//     assert_eq!(ps.get_account(client_id), None); // No account
//     assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
// }

// #[test]
// fn resolve_adjusts_account_amounts_and_transaction_state() {
//     todo!("Need to write");
// }

// #[test]
// fn resolve_handles_missing_transaction_or_incorrect_state() {
//     todo!("Need to write");
// }

// #[test]
// fn chargeback_adjusts_account_amounts_and_transaction_state() {
//     todo!("Need to write");
// }

// #[test]
// fn chargeback_handles_missing_transaction_or_incorrect_state() {
//     todo!("Need to write");
// }
