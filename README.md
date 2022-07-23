# Payments

A toy payments engine

## Assumptions:

* Representation of dollar values
  
  * With 4 decimal places precision on amounts, this implies 10k distinct steps per dollar - minimum resolvable amount is 0.0001 dollars
  
  * Assuming normal real world amounts - expect transactions to be less \$1 billion (\$1e9) - may need to revisit this assumption if inflation continues at current trends
  
  * This means we need to resolve steps: `1e9 * 1e4 = 1e13` - this can be represented in `ceil(log2(1e13)) = 44 bits`
  
  * This is less the mantissa for a 64-bit floating point number (55 bits) so we should be able to repesent this range accurately in that representation
  
  * If requirements dicatate a larger maximum transaction amount, switching to a fixed point representation (e.g `u64` for dollar amount + `u16` for decimal amount) would allow for accuracy over a wider range
  
  * This could be an issue for the stored amounts fields on accounts, which could also be updated to be fixed point representation
  
  * Will be using `f64` for amount represenations for this project for both transactions and account values

* Dispute resolution operations (dispute, resolve, chargeback) can only occur on deposit or withdrawal operations
  
  * In the case of widthdrawals, the disupted amount has a negative magnitude

## Design considerations

* Because the dispute, resolve and chargeback operations don't store amounts, we need to be able to reference the originating deposit or withdrawal transactions at any time after creation
  
  * The current specification states the transaction ID is a `u32` integer which implies `2^32` (~4 billion) records which is probably within the bounds of a single systems memory configuration (for development), but an obvious change to the system would be to make these transaction IDs opaque data blobs (say hashes) or `u64` integers which would require a dedicated data store
  
  * A key-value or relational database could implement this, however we use a memory backed store with an abstracted inteface to allow simpler implementation during development with the option of changing out the repository implementation at some point in the future

* The CLI client is the driving application of the core entities business logic, however the service level interfaces should be set up for possible integration into a web gateway or as a consumer of a streaming feed (e.g. Kafka)
  
  * Transactions processed at the service level should have observable outcomes in response to a request to process a transaction:
    
    1. Success - the transaction was processed normally
    
    2. Error - the transaction either:
       
       * Was ignored  - due to incorrect transaction-client specification
       
       * Failed - due to insufficient funds
    
    3. Panic - these indicate some fundamental system failure and should be caught and translated (e.g. 5xx HTTP status in the case of a web client)

## Transaction States

A key consideration of the system is the state of the transaction. This can be summarized by the following state transition diagram:

```mermaid
stateDiagram-v2
state "Transaction States" as title

state title {

    [*] --> Normal
    Normal --> Disputed : DISPUTE
    Disputed --> Normal : RESOLVE
    Disputed --> Reversed : CHARGEBACK
}
```

## Entities

The data storage interface should have storage for two record types:

1. **Transactions** - for tracking transaction status

2. **Accounts** - for tracking the account status

### Transaction Records

Transactions store the core data for a processed transaction (type, client_id, tx_id, amount), as well as the state of the transaction in response to dispute, resolve and chargeback operations.

There are two transaction types that result in immediate account changes (`deposit`, `withdrawal`), and three types that modify transaction state and modify accounts as a second order effect (`dispute`, `resolve`, `chargeback`). This means that we need to store `deposit` and `withdrawal` transactions as primary storage entities, and the processing of `dispute`, `resolve`, `chargeback` can update state on these entities.

The transaction record should have the following operational elements:

| Name        | Type  | Description                                                                                                                                                                                       |
| ----------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tx_id`     | `u32` | ID of the transaction - unique primary key of the record set                                                                                                                                      |
| `tx_type`   | `u8`  | Enumeration of the transaction type encoded as an integer where:<br/>`0` = Deposit<br/>`1` = Withdrawal                                                                                           |
| `client_id` | `u16` |                                                                                                                                                                                                   |
| `amount`    | `f64` |                                                                                                                                                                                                   |
| `state`     | `u8`  | Enumeration of the transaction state where:<br/>`0` = Normal - undisputed or resolved transaction)<br/>`1` = Dispute - transaction is disputed<br/>`2` = Reversed - transaction has been reversed |

#### Transaction States

A key consideration of the system is the state of the transaction. This can be summarized by the following state transition diagram:

```mermaid
stateDiagram-v2
state "Transaction States" as title

state title {

    [*] --> Normal
    Normal --> Disputed : DISPUTE
    Disputed --> Normal : RESOLVE
    Disputed --> Reversed : CHARGEBACK
}
```

### Account Records

Account records store the state and total tallys of a client account as well as the locked state of the account. An account in locked state will reject any further transactional updates (deposit, withdraw, dispute, resolve and chargeback).

The state of the account must be considered during the processing of a transaction.

| Name        | Type   | Description                                                                                                       |
| ----------- | ------ | ----------------------------------------------------------------------------------------------------------------- |
| `client_id` | `u16`  | ID of the client account - unique primary key of the record set                                                   |
| `available` | `f64`  | Total funds available to transact of the account                                                                  |
| `held`      | `f64`  | Total funds held in dispute for the account                                                                       |
| `total`     | `f64`  | Total funds available or held for the account                                                                     |
| `locked`    | `bool` | State of the account - locked accounts will reject any further transactional state changes (i.e. any transaction) |

## Test cases:

- dispute
  - on a tx that doesn't exist - noop
  - on a tx already under dispute
  - on a transaction where client_id doesn't match
- resolve
  - on a tx that doesn't exist - noop
  - on a tx not under dispute
- chargeback
  - on a tx that doesn't exist - noop
  - on an acount already frozen
- disputes work
  - on deposits
  - on withdrawals
  - account can be disputed multiple times if not locked
  - held totals are updated with multiple unresolved disputes
- locked accounts
  * deposits and withdrawals are rejected/ignored for locked accounts
  - further disputes/resolve/chargeback operations are rejected for locked accounts
- Disputed amount takes account below zero
- deposits and withdrawals can't occur on a locked account