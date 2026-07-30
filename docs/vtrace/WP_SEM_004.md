# WP-SEM-004 — Aggregate stress and recovery scenarios

Status: `implemented; bounded exit evidence recorded`

SEM-004 compares baseline, disruption, and recovery support conditions with an
explicit successor version and recovery horizon. Supplier, workforce,
maintenance, and spares remain separate, and no mission-success probability is
inferred.

Command: `cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-004 fixtures/synthetic/stress-recovery.fixture`.

No real-force, operational, procurement, budget, Taxlane, or release authority
is created.
