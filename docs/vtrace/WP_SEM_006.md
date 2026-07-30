# WP-SEM-006 — Reconciled lifecycle accounting

Status: `implemented; exit evidence pending final program audit`

SEM-006 reconciles a fictional authorized resource envelope across acquisition,
personnel, operations, maintenance, spares, infrastructure, transition, and
unallocated custody over an explicit horizon. Units, price basis, and residual
must reconcile exactly.

Command: `cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-006 fixtures/synthetic/lifecycle-accounting.fixture`.

The fixture is illustrative accounting, not a budget or savings estimate. No
procurement, allocation, Taxlane, rate, operational, or release authority is
created.
