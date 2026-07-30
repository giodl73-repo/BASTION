# WP-SEM-003 — Public unclassified aggregate mini-corpus

Status: `implemented; exit evidence pending final program audit`

Date: 2026-07-30

## Scope

Admit and replay one source-labelled GAO public aggregate record, verify closed
provenance and basic arithmetic consistency, and emit a deterministic corpus
artifact without readiness or operational inference.

## Exit evidence

- Official source: GAO-25-107569, Weapon Systems Annual Assessment 2025.
- Feature tests cover official admission, reconciliation, stale/domain/
  operational-field rejection, determinism, and no authority.
- Exact command: `cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-003 fixtures/public/gao-25-107569.fixture`.

No classified/CUI/person data, real-force or operational analysis,
procurement, budget, Taxlane action, rate, or release follows from this work
package.
