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

fn build_payments_service_with_locked_account(client_id: u16, tx_id: u32) -> PaymentService {
    let mut transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let mut account_repository = Box::new(AccountRepositoryInMemory::new());

    // Create a locked account
    let expected_ac = Account::new(client_id, 42.42, 0.0, 42.42, true);
    account_repository
        .as_mut()
        .store
        .insert(client_id, expected_ac.clone());

    // Create a transaction
    let tx1 = Transaction::new(
        tx_id,
        TransactionType::Deposit as u8,
        client_id,
        42.42,
        TransactionState::Normal as u8,
    );
    transaction_repository
        .as_mut()
        .store
        .insert(tx1.tx_id, tx1.clone());

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
    // This test setup has:
    // - 1 account - client_id specifiec by argumnet
    // - 2 transacions:
    //      tx_id 1 - deposit (50.00) - normal state
    //      tx_id 2 - deposit (10.00) - disputed state

    let mut transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let mut account_repository = Box::new(AccountRepositoryInMemory::new());

    // Create a locked account
    let expected_ac = Account::new(client_id, 50.0, 10.0, 60.0, false);
    account_repository
        .as_mut()
        .store
        .insert(client_id, expected_ac.clone());

    // Create corresponding transactions
    let tx1 = Transaction::new(
        1,
        TransactionType::Deposit as u8,
        client_id,
        50.00,
        TransactionState::Normal as u8,
    );
    transaction_repository
        .as_mut()
        .store
        .insert(tx1.tx_id, tx1.clone());
    let tx2 = Transaction::new(
        2,
        TransactionType::Deposit as u8,
        client_id,
        10.00,
        TransactionState::Disputed as u8,
    );
    transaction_repository
        .as_mut()
        .store
        .insert(tx2.tx_id, tx2.clone());

    PaymentService::new(transaction_repository, account_repository)
}

#[test]
fn transactions_on_locked_accounts_fail() {
    let client_id = 42;
    let existing_tx_id = 1;
    let mut ps = build_payments_service_with_locked_account(client_id, existing_tx_id);

    let initial_ac = ps.get_account(client_id).unwrap().clone();
    let initial_tx = ps.get_transaction(existing_tx_id).unwrap().clone();

    let tx_id = 2;
    assert!(ps.deposit(client_id, tx_id, 0.01).is_err());

    let tx_id = 3;
    assert!(ps.withdrawal(client_id, tx_id, 0.01).is_err());

    assert!(ps.dispute(client_id, existing_tx_id).is_err());

    assert!(ps.resolve(client_id, existing_tx_id).is_err());

    assert!(ps.chargeback(client_id, existing_tx_id).is_err());

    assert_eq!(ps.get_account(client_id).unwrap(), &initial_ac); // No change
    assert_eq!(ps.get_transaction(existing_tx_id).unwrap(), &initial_tx); // No change to transaction
}

#[test]
fn transactions_on_non_existant_account_fails() {
    // Aside from deposits, all transactions should fail if there is no corresponding client account
    let client_id = 42;
    let mut ps = build_payments_service(); // No accounts

    let tx_id = 1;
    assert!(ps.withdrawal(client_id, tx_id, 0.01).is_err());

    let tx_id = 1;
    assert!(ps.dispute(client_id, tx_id).is_err());

    assert!(ps.resolve(client_id, tx_id).is_err());

    assert!(ps.chargeback(client_id, tx_id).is_err());

    assert_eq!(ps.get_account(client_id), None); // No account
    assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction
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
fn withdrawal_updates_total_and_available() {
    let client_id = 42;
    let mut ps = build_payments_service_with_default_account(client_id);

    let expected_ac = Account::new(client_id, 50.0, 10.0, 60.0, false);

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);

    // Succeeds if less than available amount

    let expected_ac = Account::new(client_id, 49.9999, 10.0, 59.9999, false);
    let tx_id = 201;
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
    assert_eq!(ps.get_transactions().len(), 3);

    // Succeeds if exactly available amount

    let expected_ac = Account::new(client_id, 0.0, 10.0, 10.0, false);

    let tx_id = 202;
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
    assert_eq!(ps.get_transactions().len(), 4);

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
    let tx_id = 200;
    assert!(ps.withdrawal(client_id, tx_id, 100.0).is_err());

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac); // unchanged
    assert_eq!(ps.get_transaction(tx_id), None); // No stored transaction

    assert_eq!(ps.get_accounts().len(), 1);
    assert_eq!(ps.get_transactions().len(), 2);

    println!("accounts: {:?}", ps.get_accounts());
    println!("transactions: {:?}", ps.get_transactions());
}

#[test]
fn dispute_adjusts_account_amounts_and_transaction_state() {
    let mut ps = build_payments_service();

    let client_id = 42;

    let tx_id = 1;
    assert!(!ps.deposit(client_id, tx_id, 42.42).is_err());

    assert!(!ps.dispute(client_id, tx_id).is_err());

    let expected_ac = Account::new(client_id, 0.00, 42.42, 42.42, false);
    let expected_tr = Transaction::new(
        1,
        TransactionType::Deposit as u8,
        client_id,
        42.42,
        TransactionState::Disputed as u8,
    );

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);
}

#[test]
fn dispute_handles_incorrect_transaction_state() {
    let client_id = 42;

    let mut transaction_repository = Box::new(TransactionRepositoryInMemory::new());
    let mut account_repository = Box::new(AccountRepositoryInMemory::new());

    // Create a locked account
    let ac = Account::new(client_id, 50.0, 10.0, 60.0, false);
    account_repository
        .as_mut()
        .store
        .insert(client_id, ac.clone());

    // Create a disputed transaction
    let tx1 = Transaction::new(
        1,
        TransactionType::Deposit as u8,
        client_id,
        10.0,
        TransactionState::Disputed as u8,
    );
    transaction_repository
        .as_mut()
        .store
        .insert(tx1.tx_id, tx1.clone());

    // Create a reversed transaction
    let tx2 = Transaction::new(
        2,
        TransactionType::Deposit as u8,
        client_id,
        10.0,
        TransactionState::Reversed as u8,
    );
    transaction_repository
        .as_mut()
        .store
        .insert(tx2.tx_id, tx2.clone());

    let mut ps = PaymentService::new(transaction_repository, account_repository);

    assert!(ps.dispute(client_id, tx1.tx_id).is_err());
    assert!(ps.dispute(client_id, tx2.tx_id).is_err());

    assert_eq!(ps.get_account(client_id).unwrap(), &ac); // No change
    assert_eq!(ps.get_transaction(tx1.tx_id).unwrap(), &tx1); // No change
    assert_eq!(ps.get_transaction(tx2.tx_id).unwrap(), &tx2); // No change
}

#[test]
fn dispute_on_non_existant_transaction_fails() {
    let client_id = 42;
    let mut ps = build_payments_service_with_default_account(client_id);

    let tx_id = 201;

    assert!(ps.dispute(client_id, tx_id).is_err());
}

#[test]
fn resolve_adjusts_account_amounts_and_transaction_state() {
    let mut ps = build_payments_service();

    let client_id = 42;

    let tx_id = 1;
    assert!(!ps.deposit(client_id, tx_id, 42.42).is_err());
    assert!(!ps.dispute(client_id, tx_id).is_err());

    assert!(!ps.resolve(client_id, tx_id).is_err());

    let expected_ac = Account::new(client_id, 42.42, 0.00, 42.42, false);
    let expected_tr = Transaction::new(
        1,
        TransactionType::Deposit as u8,
        client_id,
        42.42,
        TransactionState::Normal as u8,
    );

    assert_eq!(ps.get_account(client_id).unwrap(), &expected_ac);
    assert_eq!(ps.get_transaction(tx_id).unwrap(), &expected_tr);
}

#[test]
fn resolve_handles_incorrect_transaction_state() {
    let client_id = 42;
    let mut ps = build_payments_service_with_default_account(client_id);

    let normal_tx_id = 1;
    assert!(ps.resolve(client_id, normal_tx_id).is_err());
}

#[test]
fn resolve_on_non_existant_transaction_fails() {
    let client_id = 42;
    let mut ps = build_payments_service_with_default_account(client_id);

    let tx_id = 888;
    assert!(ps.resolve(client_id, tx_id).is_err());
}

// #[test]
// fn chargeback_adjusts_account_amounts_and_transaction_state() {}

// #[test]
// fn chargeback_handles_incorrect_transaction_state() {}

// #[test]
// fn chargeback_on_non_existant_transaction_fails() {}
