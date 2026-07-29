# WP-REV-001-R1 — independent test, trace, and convergence substrate

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundaries: `PB-REV-001` plus configuration-only membership touch to
`PB-WS-001`

## 1. Controlled baseline and authority

This exact revision is based on child-repository commit
`f862e336035c889517d008b3488da4c60daa70f2` and binds the accepted governance
successors recorded by Pulse 13 SHA-256
`6998b464a8239aa269434abd9937766dfa27ca4d7eb775927e44fd7a8790152a`.

| Controlled artifact | SHA-256 |
|---|---|
| `PACKAGE_BOUNDARIES.md` / `BA-PB-SUCC-002` | `04f3923bf09f14b57cd02006dcb5e17abb4cac8981fced4c4749f0d446a71ca6` |
| `IMPLEMENTATION_PLAN.md` / `BA-PLAN-SUCC-003` | `d8db35026e02796bc2bba5e034aa4981394ae1d9a4574943c82f4705bd64d9aa` |
| `WORK_PACKAGES.md` / `BA-WP-SUCC-003` | `596761dd75e2d9b0721c7ecabb8086ee58682513b5a2c7d2a94ab1f7a79ea129` |
| `VERIFICATION.md` / `BA-VER-SUCC-002` | `e25d1999a72575bd8e6c920cd450f50146754e08262e06127f64ff5c732fe080` |
| `VALIDATION.md` / `BA-VAL-SUCC-002` | `325be52b0ba258f97415709c040880f4282452d4ea3d609855dacd31a2bb1016` |
| `CHANGE_CONTROL.md` / `BA-CC-SUCC-003` | `13cbfc8990ae42a5719f3945fe9dd7a46076def0db9be1f12ce4316c2dc57190` |

Any mismatch holds entry. All 13 held pairs remain open. The open TST pair is
a fail-closed proof input for this pure scaffold and is not closed by WP
acceptance or implementation. This WP authorizes no classified/CUI/person or
operational content, targeting, vulnerability detail, military or procurement
decision, budget, rate, allocation, Taxlane state, HND/TERM/REL output,
official use, external action, publication, or release.

## 2. Objective and stop condition

Create the dependency-free `bastion-review` Rust library that evaluates a
frozen, security-admitted digest-bound review packet without mutating its
subject. Stop after the crate deterministically returns only typed `pass`,
`hold`, or `reject` control dispositions, retains findings, conflicts,
negative evidence, and dissent, and passes the exact bounded evidence suite.

This bootstrap does not implement quantitative reproduction, domain analysis,
SOURCE admission, HND or terminal receipt behavior, a product parser,
serializer, network/filesystem adapter, cryptographic digest generator,
boundary-test crate, external fixture, runner, Taxlane adapter, or release
path. It can pass no producer and cannot create operational authority.

## 3. Exact implementation surface

The implementation allowlist is exact:

1. root `Cargo.toml`: add only member `crates/bastion-review` while preserving
   every accepted workspace policy;
2. `Cargo.lock`: Cargo-generated zero-external-dependency package entry only;
3. `crates/bastion-review/Cargo.toml`;
4. `crates/bastion-review/src/lib.rs`;
5. `tools/review_gate.ps1`, a coordination/evidence runner only; and
6. separately committed governance, pulse, review, and evidence records under
   `docs/vtrace/` and the existing foundation wave.

The package manifest fixes `name = "bastion-review"`, `version = "0.1.0"`,
workspace edition/rust-version/lints, `publish = false`, and no dependency,
feature, build script, binary, example, benchmark, procedural macro, native
surface, or unsafe allowance.

Implementation occurs on branch `codex/wp-rev-001` from a clean isolated
child worktree after separate acceptance and entry. No TRACKER file or
submodule pointer is part of this WP.

## 4. Exact public concepts and decisions

All public types are non-operational, domain-neutral review-control types:

- `Identifier`: 1–128 bytes of ASCII alphanumeric plus `.`, `_`, `-`, `:`,
  and `/`; no empty, whitespace, control, or non-ASCII value.
- `Digest256`: exactly 64 lowercase ASCII hexadecimal characters; validates a
  caller-supplied identity and does not hash content.
- `FrozenSubject`: subject ID, producer ID, subject digest, context digest,
  security-admission digest, and review generation.
- `EvidenceState`: `Planned`, `Absent`, `Executed`, `Passed`, `Failed`,
  `Stale`, `Conflicted`, `Held`, `Rejected`, or `Superseded`.
- `EvidenceRecord`: stable ID, all three subject/context/admission digests,
  method ID, state, and optional predecessor ID; never a product value.
- `Severity`: `Critical`, `Major`, `Minor`, or `Editorial`.
- `FindingDisposition`: `Open`, `Remediated`, `Deferred`, `AcceptedRisk`, or
  `Rejected`.
- `Finding`: stable ID, reviewed digest, role, severity, affected claim,
  evidence IDs, disposition, owner, destination, substantive closure,
  independence, and retained dissent IDs.
- `RoleDecision` and `AssuranceDecision`: stable role/gate ID, independent
  reviewer ID, exact digests, and `Pass`, `Hold`, or `Reject` disposition.
- `EvidenceConflict`: stable ID, both evidence IDs/digests, highest plausible
  severity, owner, resolution trigger, and open/resolved state.
- `TraceLink`: stable parent/child IDs and digests, owning stage, gate posture,
  evidence state, invalidation/supersession relation, and explicit next-stage
  non-authorizations; no domain value.
- `ReviewPacket`: frozen subject, evidence, conflicts, findings, trace links,
  required roles/decisions, required assurance gates/decisions, and dissent.
- `BlockerCode`: exhaustive stable reasons including digest/admission mismatch,
  self-approval, reviewer conflict, missing role/assurance/trace, failed gate,
  evidence-free pass, absent/failed/stale/conflicted evidence, incomplete
  finding/defer, orphan/duplicate trace, open conflict, open critical/major,
  false approval, invalid bound, and prohibited authority request.
- `ReviewDecision`: `Pass`, `Hold`, or `Reject`, bound to unchanged digests and
  containing deterministically sorted blocker, finding, conflict, trace, and
  dissent IDs.

The evaluator borrows immutable input, creates a new decision, performs no
I/O, reads no ambient state, and exposes no producer mutation. `Pass` requires
current passed evidence, exact subject/context/security-admission agreement,
distinct producer/reviewer identities, every required role, assurance, and
trace link, zero failed gate or open conflict, zero incomplete defer, and zero
unresolved critical/major finding. Minor/editorial findings remain visible and
explicitly dispositioned. Advocacy, credentials, classified appeal, HND/
terminal/release request, or operational content is rejected rather than
converted into evidence or authority.

## 5. Determinism and finite bounds

Constructors enforce before allocation-dependent work:

- at most 1,024 evidence rows, 1,024 findings, 1,024 trace links, 1,024 dissent
  IDs, 256 conflicts, 128 required roles, 128 role decisions, 32 assurance
  gates, 32 assurance decisions, and 128 evidence/dissent references per
  finding;
- each identifier at most 128 bytes and closure/trigger text at most 4,096
  ASCII bytes; aggregate caller-supplied review text at most 1 MiB;
- generation is `u64` and never incremented inside review;
- no recursion, threads, async, retry, randomness, locale, clock, filesystem
  order, map/set iteration, parser, or unsafe code; and
- canonical bytewise ascending stable-ID output, rejecting duplicate IDs
  before convergence evaluation.

The evaluator uses bounded loops over caller-owned slices and result vectors.
One maximum-cardinality evaluation must stay below 64 MiB additional heap.
Evidence commands enforce 60 seconds wall time, 1 GiB process-tree memory,
and 10 MiB combined generated output per run.

## 6. Exact command bindings

`tools/review_gate.ps1` is a non-product successor verification runner. It
inherits the accepted `ws_policy_gate.ps1` fail-closed environment allowlist,
Windows Job Object enforcement, canonical JSON evidence, and toolchain checks.
Its SHA-256 is fixed at implementation review before evidence runs. Invocation:

`pwsh -NoLogo -NoProfile -NonInteractive -File tools/review_gate.ps1 -Mode <MODE>`

| Identity | MODE | Exact internal command/assertion |
|---|---|---|
| `CMD-L0-FORMAT` | `L0Format` | `cargo +1.95.0 fmt --all -- --check` |
| `CMD-L0-CHECK` | `L0Check` | `cargo +1.95.0 check -p bastion-review --locked --offline --all-targets` |
| `CMD-L0-FOCUSED-TEST` | `L0FocusedTest` | `cargo +1.95.0 test -p bastion-review --locked --offline` |
| `CMD-L1-WORKSPACE-CHECK` | `L1WorkspaceCheck` | `cargo +1.95.0 check --workspace --locked --offline --all-targets` |
| `CMD-L1-LINT` | `L1Clippy` | `cargo +1.95.0 clippy --workspace --locked --offline --all-targets -- -D warnings` |
| `CMD-L1-TEST` | `L1Test` | `cargo +1.95.0 test --workspace --locked --offline` |
| `CMD-L1-DOC` | `L1Doc` | `cargo +1.95.0 doc --workspace --locked --offline --no-deps` plus doctests |
| `CMD-L1-STATIC` | `L1Static` | fail on unsafe/FFI, panic escapes, unwrap/expect/todo/unimplemented, recursion, I/O, ambient state, operational terms/payloads, forbidden paths, or producer dependency on `bastion-review` |
| `CMD-L1-SUPPLY-CHAIN` | `L1SupplyChain` | Cargo metadata/lock assertion: one workspace package, zero external/transitive dependency, feature, build, proc-macro, native, registry, git, or path dependency |
| `CMD-L2-CONTRACT-MATRIX` | `L2Contract` | execute every required `CONTRACT-TEST-001` and `CONTRACT-TRACE-001` bootstrap partition |
| `CMD-L2-MODEL` | `L2Model` | exhaustive bounded state/trace/convergence cases and deterministic permutation equality |
| `CMD-L2-ADVERSARIAL` | `L2Adversarial` | stale/mismatch, self-approval, conflict, missing role/gate/trace, false-pass, duplicate/oversize, advocacy/classified-appeal substitution, dissent/defer attacks |
| `CMD-L2-NO-EMISSION` | `L2NoAuthority` | prove no I/O, operational payload, HND/TERM/REL/Taxlane output, product value, or authority-returning surface |

All modes emit `review-gate-evidence.v1` JSON binding repository, WP, mode,
implementation/acceptance/WP/runner/toolchain/manifest/lock digests, exact argv
and sanitized environment, bounds, timings, exit/result, captured stream
hashes, assertions, and negative cases.

## 7. Required cases and acceptance gate

Positive cases: complete frozen security-admitted packet; independent roles;
both assurance gates; current passed evidence; exact trace; explicitly
dispositioned minor/editorial item; retained negative result and dissent;
resolved conflict with retained predecessor; deterministic permutation; and
immutable repeat evaluation.

Fail-closed cases: subject/context/admission mismatch; zero or non-current
evidence; self-approval/conflict; missing/duplicate role; missing/failed gate;
incomplete finding/defer; open conflict; open critical/major; orphan/duplicate/
stale trace; planned-as-executed evidence; false approval; duplicate ID; every
bound plus one; erased dissent/negative result; advocacy, credentials, or
inaccessible classified appeal as evidence; operational/targeting/
vulnerability content marker; HND/TERM/REL/Taxlane/official-use request;
subject mutation; dependency/unsafe/I/O/producer-backedge insertion.

Exit requires all 13 command identities to pass at one implementation digest,
all evidence retained under `EVID-WP-REV-001`, the exact Independent Test,
Role review, eight parliament, seven stakeholder, Scope, Citation, Numeracy,
Methodology, Classification/Operational Security, and Civilian Control/Law/
Safety/Readiness decisions, zero unresolved critical/major finding, and an
exact acceptance pulse. REV evidence proves only this substrate, passes no
producer, emits no terminal receipt, and does not close `TBD-TST-001` /
`SPEC-UNK-TST-001` alone. Rollback atomically reverts crate/member/runner,
retains evidence/dissent, and returns REV to absent without altering WS.

Disposition: **exact WP candidate; independent acceptance required before
entry or implementation**.
