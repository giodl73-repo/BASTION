# WP-SEM-003 — Public unclassified aggregate mini-corpus

Status: `implemented; bounded exit evidence recorded`

Date: 2026-07-30

## Scope

Admit and replay one source-labelled GAO public aggregate record, verify closed
provenance and basic arithmetic consistency, and emit a deterministic corpus
artifact without readiness or operational inference.

## Exit evidence

- Official source: GAO-26-108457, Weapon Systems Annual Assessment 2026.
- Feature tests cover official admission, reconciliation, stale/domain/
  operational-field rejection, determinism, and no authority.
- Exact command: `cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-003 fixtures/public/gao-26-108457.fixture`.

No classified/CUI/person data, real-force or operational analysis,
procurement, budget, Taxlane action, rate, or release follows from this work
package.
