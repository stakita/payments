# payments
Toy payments engine

Assumptions:
* Floating point representation with fixed point rendering should be sufficient
** Assume normal real world amounts - transaction typically less than GDP of USA $25e12
*** May need to revisit this assumption if inflation continues at current trends
** Could move to a fixed point representation


Test cases:
* dispute
** on a tx that doesn't exist - noop
** on a tx already under dispute
* resolve
** on a tx that doesn't exist - noop
** on a tx not under dispute
* chargeback
** on a tx that doesn't exist - noop
** on an acount already frozen
