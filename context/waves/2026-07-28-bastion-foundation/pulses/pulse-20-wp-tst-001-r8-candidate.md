# Pulse 20 — WP-TST-001 retained R8 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R7 governance commit | `3550c5deece2ec97207fbe8c1b4dda4c44d62a97` |
| Retained R7 candidate SHA-256 / blob | `e9b2c5e82a31eb6ee172f35fe06b2db46f3affcebae0f8b5391264cca59644ff` / `89ab301d7e3596fbb32678b84c1356a41b3f2bc0` |
| Retained R7 pulse SHA-256 / blob | `ec12c55fa7a08d3e764c596b25ce5af96daafcef9a0a314eb9e2cded31aeba13` / `97249bc4a20bc7dd4dff4950dd169c78cf25590a` |
| R8 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R8` |
| R8 candidate SHA-256 | `3165853787462ede6a39c154b060accd8d4ab43e83a36ea1373e5d01aaf86de7` |
| R8 candidate Git blob | `41a279ad001ef327c8beb2b76d1ac5d2cb84e540` |

R1 through R7 remain immutable, unaccepted review history. R8 is a proposed
successor and does not rewrite their records, findings, or execution claims.

## Retained R7 finding disposition

| Finding | R7 disposition | R8 remediation |
|---|---|---|
| `BA-TST-R7-M01` | retained major; R7 not accepted | immutable `allocated_targets` are separated from same-order, same-cardinality `target_results`; each target is exactly `not_run|passed|failed|held` with closed optional reason/output-pointer rules, five derived counts, and honest attempted-prefix/remaining-not-run custody across preflight failure, command-not-started, timeout, bound kill, crash, and unexpected exit. |
| `BA-TST-R7-M02` | retained major; R7 not accepted | preflight freezes literal Git argv and command-line config for exact A/M/D/R, porcelain-v2, binary diff, tree inventory, and every blob read; renames are exact-content-only and unlimited; raw stdout/stderr hashes are retained per invocation; local config and repository info inputs are bound before/after; porcelain/raw/tree grammars are closed; seven observation lists use uint64-total counts, an explicit count-overflow terminal variant, bounded retained prefixes, and full-set streaming digests without an arbitrary smaller cap. |

## Cross-review closure

`L1Static` now executes the exact `static_surface` Cargo test before its exact
runner assertions, and its normative argv and `[cargo,test_gate]` tool sequence
match the command prose. `L0Format` and `L1Clippy` retain exact Cargo plus
rustfmt/clippy tuples. Candidate-wide author/controller identity custody and
per-reviewer authentication digests make independence a derived collision and
lane-bijection result, not an asserted enum.

## Preserved candidate boundary

R8 retains R7's exact implementation allowlist, two-node/zero-edge package
shape, four synthetic fixtures, 16 modes, 123 canonical identities, 148 exact
forward/reverse transpose edges, 38 CR identities, 63 CR edges, and 22 indexed
review lanes. Section 7 is byte-identical to committed R7. Evidence remains
create-new, bounded, embedded, pointer-local, digest-bound, and non-authorizing.

Mode evidence advances to `test-gate-evidence.v5`; set evidence advances to
`test-gate-evidence-set.v4`. Acceptance must bind the R8 bytes above and prior
inputs, never its own future commit/digest; later entry must bind committed
acceptance and cannot manufacture execution or authority.

## Required independent review and no authority

All 22 exact lanes must independently bind the R8 digest above. Any changed
digest, dishonest partial attempt, unretained Git invocation, configuration or
count ambiguity, author/controller collision, duplicate cross-lane reviewer,
unresolved critical/major finding, open defer/dissent/conflict, failed
assurance, schema drift, or authority ambiguity requires another retained
successor.

This pulse does not accept or enter the WP, implement or execute a command,
create evidence, close a held pair, accept product evidence, authorize a
producer or official use, emit HND/TERM/REL/Taxlane state, publish, push, or
release.

Disposition: **R1–R7 retained; R7 findings and cross-review gaps remediated only
in proposed R8; independent acceptance review required; not accepted; not
entered**.
