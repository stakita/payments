# payments
Toy payments engine

Assumptions:
* Floating point representation with fixed point rendering should be sufficient
** Assume normal real world amounts - transaction typically less than GDP of USA $25e12
*** May need to revisit this assumption if inflation continues at current trends
** Could move to a fixed point representation

* Dispute resolution operations (dispute, resolve, chargeback) can only occur on deposit or withdrawal operations. In the case of widthdrawals, the disupted amount has a negative magnitude.
* Because the dispute, resolve and chargeback operations don't store amounts, we need to be able to reference the originating deposit or withdrawal transactions at any time after creation
** This could be done with a database backed store. A key-value or relational database could implement this, however we use a memory backed store with an abstracted inteface to allow simpler implementation during development with the option of changing out the repository implementation at some point in the future.
*** at the moment the transaction count is a u32 integer which implies ~4 billion (2^32) records, but an obvious change to the system would be to make these transaction IDs opaque data blobs (say hashes) or u64 integers.


Test cases:
* dispute
** on a tx that doesn't exist - noop
** on a tx already under dispute
** on a transaction where client_id doesn't match
* resolve
** on a tx that doesn't exist - noop
** on a tx not under dispute
* chargeback
** on a tx that doesn't exist - noop
** on an acount already frozen
* disputes work
** on deposits
** on withdrawals
** account can be disputed multiple times if not locked
* Disputed amount takes account below zero
* deposits and withdrawals can't occur on a locked account