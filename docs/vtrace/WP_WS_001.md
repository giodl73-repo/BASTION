# WP-WS-001-R1 — empty Rust workspace policy skeleton

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundary: `PB-WS-001`

Predecessor: completed governance-only `WP-VV-001` fixed points; no TST
blocker or proof-input relationship

## 1. Controlled baseline

This exact revision is based on child-repository commit
`f2e67d0fd108c7c61d3e9ed842f14fced8e257d0` and binds:

| Controlled artifact | SHA-256 |
|---|---|
| Accepted `CHANGE_CONTROL.md` / `CHG-BA-TST-BOOT-002` | `ed259afd24fc39ea87d8b21c4b5a535a2b369c5558e9dd6e2f25b1e070deaa3c` |
| Fixed `IMPLEMENTATION_PLAN.md` | `e1ce5cfa603e491d21d7a71859c610316e62fc49caba1c0687c59b8b8baa15c7` |
| Fixed `WORK_PACKAGES.md` | `5fa703f2f93c5748da5f4cb800cbffe0b153747f006d6f116c577b6f90b412e3` |
| Fixed `VERIFICATION.md` | `2d78c946464256cd472b7e93272cf6c5a8ef5699e536e5e7274a92b423281027` |
| Fixed `VALIDATION.md` | `0469934dc1b182438321f095fad9a90666dde09d466bc01007e21aa09ad0db55` |
| Planning fixed-point Pulse 09 | `d8237ae99fa60497066948a31d8c00a5f30675e849451ae5facf61a9a277b781` |
| V&V fixed-point Pulse 10 | `fb1719292d1a5a60dfd5b1c2865a7414ce9274e739d90d067eb62d347772d0e5` |

Any mismatch or content change requires a retained successor revision and new
independent acceptance. This revision closes no hold. All 13 holds remain open.

## 2. Objective and stop condition

Create only a repository-root, policy-bearing virtual Rust workspace with
empty membership and no product or domain semantics. Stop after the four-file
implementation allowlist is proven exact, the selected toolchain and workspace
policy are reproduced, and the required evidence is independently reviewed.

This WP must stop without creating a crate, package target, `src/`, `crates/`,
`tests/`, fixture, generated output, dependency, feature, build script,
procedural macro, native surface, unsafe surface, API, schema, runtime,
deployment, HND output, Taxlane action, release, or official action.

## 3. Exact representation and toolchain policy

The root `Cargo.toml` representation is exactly a virtual manifest with no
`[package]` table and the following policy values:

```toml
[workspace]
members = []
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.95.0"

[workspace.lints.rust]
unsafe_code = "forbid"
```

The root `rust-toolchain.toml` representation is exactly:

```toml
[toolchain]
channel = "1.95.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

The policy selects Rust edition 2024, MSRV 1.95.0, exact Rust toolchain
1.95.0, Cargo resolver 3, and only the `rustfmt` and `clippy` optional
components. It selects zero workspace members, packages, targets,
dependencies, development dependencies, build dependencies, workspace
dependencies, features, patches, replacements, registries, external paths,
git sources, build scripts, procedural macros, native libraries, wrappers,
environment defaults, or unsafe allowances.

`Cargo.lock` is the sole allowed lock surface. It may be created or refreshed
only by the accepted Cargo 1.95.0 invocation. It must contain zero package or
dependency entries and be committed if Cargo creates it. Manual lock editing is
forbidden. No `.cargo/config*` file is permitted in this revision.

Future membership, dependency, feature, profile, lint, lock, configuration,
toolchain, edition, MSRV, resolver, cross-repository, or command changes require
the later accepted WP that owns the change, a successor workspace-policy
revision, compatibility review, and new digest-bound acceptance.

## 4. Exact paths and branch

Implementation is confined to branch `codex/wp-ws-001-bootstrap` in a clean
isolated child-repo worktree. The controlled governance predecessor is exactly
`f2e67d0fd108c7c61d3e9ed842f14fced8e257d0`. The branch base must be the later
local acceptance commit containing this exact WP revision and its accepted
Pulse 11; that commit does not yet exist and must be bound in the separate
entry record before branch creation.

The complete implementation allowlist is:

- `Cargo.toml`;
- `Cargo.lock`; and
- `rust-toolchain.toml`; and
- `tools/ws_policy_gate.ps1`.

The separately allowed control/evidence surfaces are this accepted WP record,
its wave pulse, and
`context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/**`.
They are not Cargo members, product inputs, or substitutes for executed
evidence. No TRACKER file or submodule pointer may change in this WP.

The runner is coordination-only and is not a Cargo member, product tool, or
semantic surface. Any other changed path, hidden workspace member, parent/sibling path,
symlink/reparse escape, external path dependency, generated source, or ambient
Cargo configuration is an immediate failing result.

## 5. Exact command bindings and finite bounds

Every command runs from the BASTION repository root with network access
disabled, no inherited Cargo/Rust wrapper variables, one process at a time, a
60-second wall-clock timeout, a 1 GiB resident-memory ceiling, and a 10 MiB
combined-output ceiling. Exceeding any bound is failure, not a retry or waiver.
Every expected exit code is `0`.

The exact runner is Windows PowerShell `pwsh` 7.6.4 with
`-NoLogo -NoProfile -NonInteractive`. It accepts only the exact `-Mode` values
below, writes no implementation file, emits one canonical JSON result to
standard output, and returns nonzero for an unknown mode or failed assertion.

| Slot | Exact command | Expected result |
|---|---|---|
| `CMD-WS-TOOLCHAIN` | `rustc +1.95.0 --version --verbose` | release is exactly `1.95.0`; host and commit are recorded |
| `CMD-WS-CARGO` | `cargo +1.95.0 --version --verbose` | release is exactly `1.95.0`; host and commit are recorded |
| `CMD-WS-LOCK` | `cargo +1.95.0 generate-lockfile --offline` | lockfile is generated with zero package/dependency entry |
| `CMD-L0-FORMAT` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L0Format` | executed runner emits reasoned `N/A-C` only for zero member/format target |
| `CMD-L0-CHECK` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L0Check` | empty virtual workspace resolves with no package or dependency edge |
| `CMD-L0-FOCUSED-TEST` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L0FocusedTest` | positive/negative policy cases pass without a product-test claim |
| `CMD-L1-WORKSPACE-CHECK` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L1WorkspaceCheck` | whole-workspace policy resolves with zero members |
| `CMD-L1-LINT` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L1Lint` | executed runner emits reasoned `N/A-C` only for zero Rust lint target |
| `CMD-L1-TEST` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L1Test` | executed runner emits reasoned `N/A-C`; no product behavior is claimed |
| `CMD-L1-DOC` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L1Doc` | executed runner emits reasoned `N/A-C`; no product output is created |
| `CMD-L1-STATIC` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L1Static` | JSON reports empty members/packages and exact paths/policy |
| `CMD-L1-SUPPLY-CHAIN` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L1SupplyChain` | JSON reports zero package, resolve, source, dependency, feature, build, native, git, registry, or path edge |
| `CMD-L2-MODEL` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L2Model` | fixed-edge, root-local, empty-graph, and no-semantics inspection passes |
| `CMD-L2-ADVERSARIAL` | `pwsh -NoLogo -NoProfile -NonInteractive -File tools/ws_policy_gate.ps1 -Mode L2Adversarial` | allowlist, ambient configuration, reparse/symlink, and all in-memory negative cases reject |

The executing evidence runner must record the exact process invocation,
environment allowlist, start/end time, peak memory, output byte count, exit
code, stdout/stderr digest, tool/configuration digests, implementation commit,
and the bound baseline digests. Merely naming these commands is `EVT-P`; it is
not execution or exit evidence.

The runner invokes `cargo +1.95.0 metadata --format-version 1 --no-deps
--locked --offline` in every mode and applies all section 6 assertions.
Cargo 1.95.0 correctly returns a nonzero “no package/target” result for
`fmt`, `check`, `clippy`, `test`, `doc`, and `tree` against this zero-member
virtual workspace. Each assigned slot still executes the exact runner; its
target-level result is reasoned `N/A-C`, not a fabricated success. Its proof
is the same exact metadata
result: `packages=[]`, `workspace_members=[]`,
`workspace_default_members=[]`, and `resolve=null`. Adding any member makes
all target-dependent slots required and invalidates this revision.

The runner rejects inherited `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`,
`RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `CARGO_TARGET_DIR`, or Cargo network
configuration; records every inherited `CARGO_*`, `RUST*`, and proxy variable;
uses offline Cargo; and inspects tracked, untracked, ignored, symlink, and
reparse-point paths. It launches Cargo with redirected output, kills the
process tree at 60 seconds, fails above 1 GiB peak process-tree resident
memory or 10 MiB combined output, and includes elapsed time, peak bytes,
output bytes, exit code, tool versions, command, implementation/base digests,
and stdout/stderr SHA-256 values in canonical JSON. The same bounds apply to
the runner; a missing measurement is failure.

## 6. Safe cases and expected posture

No external or person-level fixture is permitted. The positive case is the
exact four-file policy surface above. Independent inspection must also apply
safe, inert manifest snippets in memory for these negative cases, each expected
to reject before promotion: non-empty or glob membership; `[package]` or any
target; dependency/feature/patch/replace/registry table; parent, sibling, git,
or registry source; build script, procedural macro, native link, wrapper, or
unsafe allowance; `.cargo/config*`; extra changed path; toolchain, component,
resolver, edition, or MSRV drift; and lock entries or hand editing.

The policy surface in the preceding paragraph means three root policy files
plus the fourth, coordination-only runner. All negative snippets are exact
in-memory cases executed by `tools/ws_policy_gate.ps1`; no fixture file is
created.

A missing, skipped, unavailable, timed-out, over-bound, stale, or
digest-mismatched result is `held`, never favorable. Empty execution proves
only coordination policy. It supplies no product behavior, semantic result,
producer evidence, hold closure, or authority.

## 7. Evidence destinations and exit gate

Evidence records are immutable and go only to:

| Evidence | Exact destination |
|---|---|
| Planning/acceptance record | `context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/EVT-P.json` |
| L0 record and logs | `context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/EVT-L0.json` and `logs/l0/` below that directory |
| L1 record and logs | `context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/EVT-L1.json` and `logs/l1/` below that directory |
| Fixed-edge/no-semantics record | `context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/EVT-L2.json` and `logs/l2/` below that directory |
| Independent convergence | `context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/EVT-A.json` |

Exit requires the exact four-file implementation allowlist and no other implementation
path; all bound commands at exit `0` within bounds; empty membership and zero
edge/package/dependency results; all negative cases rejecting; `VCL-ALL` with
product-only cases reasoned N/A; present digest-bound `EVT-L0`, `EVT-L1`,
reasoned fixed-edge `EVT-L2`, and `EVT-A`; zero unresolved critical or major
finding; rollback proof; and no semantics, authority, hold closure, or
producer claim. Acceptance and entry do not pre-claim any exit evidence.

## 8. Compatibility, rollback, and invalidation

Compatibility is exact equality to this policy until a separately accepted
successor admits a reviewed member. No implicit Cargo discovery, inherited
configuration, version range, moving channel, or favorable default is
compatible.

Rollback is a reviewed `git revert <exact-WP-WS-001-implementation-commit>` on
the child repository, followed by the same path/digest inspection. The failed
commit, evidence, findings, and dissent remain retained. Reset, history rewrite,
manual deletion, and a mixed TRACKER pointer rollback are forbidden.

Any controlled-input, WP, representation, path, toolchain, command, bound,
fixture/case, expected result, evidence destination, reviewer, decision,
implementation, or evidence digest change invalidates affected acceptance or
evidence and requires a retained successor plus independent re-review.

## 9. Review routing and acceptance state

Required and pending on the same exact revision digest:

- BASTION maintainer/stage controller: accountable acceptance and branch/scope
  decision;
- Scope Keeper: public-aggregate, non-operational, no-authority boundary;
- Role review steward: independent findings, dissent, convergence, and no
  self-approval; and
- Classification & Operational Security and Civilian
  Control/Law/Safety/Readiness: explicit concurrence with the reasoned N/A
  treatment, not a semantic or safety claim.

`PAR-ALL` and product/domain semantic-owner concurrence are reasoned N/A only
because membership is empty and the WP owns coordination policy only.
Classification/security content review is N/A because there is no content;
civilian/law/safety/readiness semantic review is N/A because there is no
semantic behavior. Any member, content, behavior, or semantic change makes
those lanes required and invalidates this revision.

This document is an author proposal only. `WP-WS-001-R1` is not accepted,
entered, implemented, executed, evidenced, completed, or exited until a later
independent digest-bound review and explicit stage-controller decision record
those distinct states.
