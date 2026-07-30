# WP-SEM-002 — Safe-synthetic readiness remedy comparison

Status: `implemented; bounded exit evidence recorded`

Date: 2026-07-30

## Single semantic scope

This work package authorizes exactly one semantic vertical slice: ingest one
closed safe-synthetic, aggregate, unclassified, non-operational readiness
fixture; compare inventory, maintenance/spares, and supplier/logistics remedy
classes; and expose each remedy's readiness-floor change, remaining
bottleneck, synthetic resource requirement, lead time, and transition burden.

It does not authorize a public corpus, classified/CUI/person data, real unit or
platform detail, force selection, deployment, targeting, vulnerability
analysis, procurement recommendation, budget, monetized cost, savings claim,
handoff, Taxlane action, or release.

## Exact implementation paths

- `Cargo.toml`
- `Cargo.lock`
- `crates/bastion-remedy-slice/Cargo.toml`
- `crates/bastion-remedy-slice/src/lib.rs`
- `crates/bastion-remedy-slice/src/main.rs`
- `fixtures/synthetic/readiness-remedies.fixture`
- `PRODUCT_PLAN.md`
- `docs/vtrace/IMPLEMENTATION_PLAN.md`
- `docs/vtrace/WORK_PACKAGES.md`
- `README.md`
- this work-package record

## Required behavior

- Closed, bounded, dependency-free parsing rejects unknown, operational,
  targeting, procurement-instruction, or person-shaped fields.
- All nine readiness facets remain visible for baseline and every remedy.
- A missing or stale facet, resource requirement, lead time, or transition
  burden holds that remedy; missing never becomes zero.
- Readiness floor and remaining bottleneck are computed transparently; safety
  regression holds a remedy even if another facet improves.
- Resource index, lead time, and transition burden remain distinct and cannot
  be collapsed into a recommendation or monetary savings claim.
- Output is deterministic JSON with explicit no operational, procurement,
  budget, Taxlane, or release authority.

## Verification and exit

Run format, workspace compilation, Clippy with warnings denied, all tests,
documentation, and the exact CLI example. Exit requires a replayable fixture
to demonstrate that additional inventory can leave the limiting constraint
unchanged, compare at least three remedy classes, retain all resource and
safety limits, exercise held evidence and prohibited-field rejection, and emit
no selected or recommended remedy.

Rollback is a revert of the single implementation commit. No TRACKER pointer
change belongs in that commit.

## Exit evidence — 2026-07-30

- Workspace format, compilation, Clippy with warnings denied, tests, and
  documentation completed successfully with Rust 1.95.0 offline and locked.
- Tests: 26 passed, 0 failed (16 review-kernel; 5 readiness-slice; 5
  remedy-comparison-slice).
- The replayable fictional comparison shows inventory expansion leaving the
  60% supplier-constrained floor unchanged, maintenance/spares raising it to
  65%, and supplier/logistics raising it to 80%.
- Every remedy retains all nine readiness facets plus its synthetic resource
  index, lead time, transition burden, safety posture, and remaining
  bottleneck; no remedy is selected or recommended.
- Negative tests prove missing limits and safety regression hold a remedy,
  prohibited fields are rejected, output is deterministic, and operational,
  procurement, budget, monetized-savings, Taxlane, and release authority all
  remain false.
