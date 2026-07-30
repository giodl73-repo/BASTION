# WP-SEM-001 — Safe-synthetic readiness-package slice

Status: `implemented; bounded exit evidence recorded`

Date: 2026-07-30

## Single semantic scope

This work package authorizes exactly one semantic vertical slice: ingest one
closed safe-synthetic, aggregate, unclassified, non-operational readiness
fixture; assess nine visible readiness/support facets for a baseline and one
bounded support alternative; and emit one deterministic research-only
artifact.

The slice may implement readiness-package meaning only to the minimum needed
for this end-to-end assessment. It does not authorize a general corpus,
classified/CUI/person data, real unit or platform detail, force selection,
deployment, targeting, vulnerability analysis, acquisition recommendation,
procurement, budget, savings claim, handoff, Taxlane action, or release.

## Exact implementation paths

- `Cargo.toml`
- `Cargo.lock`
- `crates/bastion-readiness-slice/Cargo.toml`
- `crates/bastion-readiness-slice/src/lib.rs`
- `crates/bastion-readiness-slice/src/main.rs`
- `fixtures/synthetic/readiness-package.fixture`
- `README.md`
- this work-package record

## Required behavior

- Closed, dependency-free parsing rejects unknown, operational, targeting, or
  person-shaped fields.
- Personnel, training, assets, maintenance, spares, logistics, suppliers,
  interoperability, and safety remain separately visible.
- Current values may pass or identify concern; missing and stale values hold
  and suppress the readiness floor rather than becoming zero.
- The reported floor is the minimum observed current facet, not mission-success
  probability or a deployment recommendation.
- The bounded alternative changes support facets only.
- Output is deterministic JSON with explicit no-authority fields.

## Verification and exit

Run format, workspace compilation, Clippy with warnings denied, all tests,
documentation, and the exact CLI example. Exit requires the synthetic fixture
to identify the supplier baseline bottleneck, show bounded support improvement,
retain every facet and negative hold behavior, replay deterministically, and
create zero operational, procurement, fiscal, Taxlane, or release authority.

Rollback is a revert of the single implementation commit. No TRACKER pointer
change belongs in that commit.

## Exit evidence — 2026-07-30

- Workspace format, compilation, Clippy with warnings denied, tests, and
  documentation completed successfully with Rust 1.95.0 offline and locked.
- Tests: 21 passed, 0 failed (16 review-kernel; 5 semantic-slice).
- The replayable fixture identifies supplier resilience as the baseline
  bottleneck, moves the transparent synthetic floor from 60% to 78%, and
  improves exactly five support facets without hiding any of the nine facets.
- Parser and negative-state tests reject operational, targeting, and
  person-shaped fields and suppress the floor for missing or stale evidence.
- Canonical-output tests prove deterministic replay and explicit false values
  for operational planning, deployment advice, targeting, procurement,
  budget allocation, and Taxlane action authority.
