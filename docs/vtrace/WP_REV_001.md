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
- `RecordBinding`: stable record ID, caller-supplied content `Digest256`,
  `u64` version, and optional exact predecessor record ID/digest/version. A
  successor never overwrites or reuses its predecessor identity. The digest
  domain is the externally canonical record payload excluding this binding, so
  it is not a self-referential hash; the crate validates and carries the bond.
- `RequiredSet`: its own `RecordBinding` plus an exact bytewise-sorted unique
  ID set. Separate sets exist for roles, assurance gates, evidence methods,
  derivations, negative cases, unresolved questions, and trace links.
- `ReviewPolicy`: its own `RecordBinding`, the seven exact `RequiredSet`
  bindings above, and role-corpus version, frozen independently before packet
  assembly.
- `FrozenSubject`: subject ID, producer ID, subject digest, context digest,
  security-admission digest, review generation, admitted posture ID, and exact
  review-policy ID/digest/version.
- `EvidenceState`: `Planned`, `Absent`, `Executed`, `Passed`, `Failed`,
  `Stale`, `Conflicted`, `Held`, `Rejected`, or `Superseded`.
- `EvidenceRecord`: `RecordBinding`, all three subject/context/admission
  digests, method ID, and state; never a product value.
- `Severity`: `Critical`, `Major`, `Minor`, or `Editorial`.
- `FindingDisposition`: `Open`, `Remediated`, `Deferred`, `AcceptedRisk`, or
  `Rejected`.
- `Finding`: `RecordBinding`, frozen subject ID/digest/context/admission/
  generation, policy digest, role, severity, affected claim, evidence IDs,
  disposition, owner, destination, controlled closure-condition ID,
  independence, and retained dissent IDs. Closure content stays external.
- `RoleDecision` and `AssuranceDecision`: their own `RecordBinding`, stable
  role/gate ID, independent reviewer ID, policy digest, frozen subject
  ID/digest/context/admission/generation, and `Pass`, `Hold`, or `Reject`.
- `EvidenceConflict`: `RecordBinding`, both evidence IDs/digests, highest plausible
  severity, owner, controlled resolution-trigger ID, and open/resolved state.
  Trigger content remains external and unrepresentable here.
- `TraceLink`: `RecordBinding`, parent/child IDs and digests, owning stage, gate posture,
  evidence state, invalidation/supersession relation, and explicit next-stage
  non-authorizations; no domain value.
- `ReviewDisposition`: `PassRecommended`, `Hold`, or `Reject`. It is a review
  result only and is never a fixed-point or authority state.
- `AcceptanceReceipt`: input-only external stage-governance record containing
  its own `RecordBinding`, exact accepted review-decision binding, stage-
  controller role ID, and governance-only acceptance posture. The evaluator
  validates a supplied bond but cannot create one.
- `PriorReviewSnapshot`: complete immutable prior decision binding and
  disposition, subject/context/admission/policy bonds, external
  `AcceptanceReceipt` when accepted, and exact retained prior finding, defer,
  dissent, negative-evidence, conflict, and trace bindings.
- `ReviewPacket`: its own `RecordBinding`, frozen subject, independently frozen
  `ReviewPolicy`, derivation record bindings, negative-case record bindings,
  unresolved-question record bindings, evidence, conflicts, findings, trace
  links, role decisions, assurance decisions, dissent record bindings, and
  optional `PriorReviewSnapshot`. Every controlled packet list must equal its
  separately digest-bound `RequiredSet`; caller-selected lists cannot narrow
  or widen applicability.
- `BlockerCode`: exhaustive stable reasons including digest/admission mismatch,
  self-approval, reviewer conflict, missing role/assurance/trace, failed gate,
  evidence-free pass, absent/failed/stale/conflicted evidence, incomplete
  finding/defer, orphan/duplicate trace, open conflict, open critical/major,
  false approval, invalid bound, and prohibited input surface.
- `ReviewDecision`: its own caller-supplied prospective `RecordBinding`,
  `ReviewDisposition`, unchanged subject/context/admission/policy digests,
  optional exact predecessor acceptance binding without a supersession claim,
  and
  deterministically sorted blocker, finding, conflict, trace, and dissent
  bindings. The crate validates and carries external content digests; it does
  not generate or claim to cryptographically verify them.

The evaluator borrows immutable input, creates a new decision, performs no
I/O, reads no ambient state, and exposes no producer mutation. `Pass` requires
current passed evidence, exact subject/context/security-admission/policy
agreement, distinct producer/reviewer identities, exact equality between the
  independently frozen policy sets and all seven supplied controlled sets,
zero failed gate or open conflict, zero incomplete
defer, and zero unresolved critical/major finding. Minor/editorial findings remain visible and
explicitly dispositioned. The model has no free-form text, content, product,
HND/terminal/release-request, or authority-effect field. Identifiers are opaque
references whose external meaning is never interpreted as evidence or authority.

## 5. Determinism and finite bounds

Constructors enforce before allocation-dependent work:

The same per-class maxima apply separately to the current packet and its one
optional immutable prior snapshot; deeper history is represented by digest
links and is not recursively loaded.

- at most 1,024 evidence rows, 1,024 findings, 1,024 trace links, 1,024 dissent
  IDs, 1,024 derivations, 1,024 negative cases, 1,024 unresolved questions,
  256 conflicts, 128 required roles, 128 role decisions, 32 assurance gates,
  32 assurance decisions, and 128 evidence/dissent references per finding;
- each identifier at most 128 bytes; there is no free-form text or payload
  field, and closure/trigger content is referenced only by `Identifier`;
- generation is `u64` and never incremented inside review;
- every successor version must be strictly greater than its exact predecessor
  version; predecessor ID/digest substitution, broken chains, and in-place
  reuse reject before convergence evaluation;
- no recursion, threads, async, retry, randomness, locale, clock, filesystem
  order, map/set iteration, parser, or unsafe code; and
- canonical bytewise ascending stable-ID output, rejecting duplicate IDs
  before convergence evaluation.

The evaluator uses bounded loops over caller-owned slices and result vectors.
The accepted executable memory bound is the evidence runner's enforced 1 GiB
process-tree ceiling; no lower unmeasured heap claim is made. Evidence commands
also enforce 60 seconds wall time and 10 MiB combined generated output per run.

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
| `CMD-L1-DOC` | `L1Doc` | `cargo +1.95.0 doc --workspace --locked --offline --no-deps`, then `cargo +1.95.0 test -p bastion-review --doc --locked --offline` |
| `CMD-L1-STATIC` | `L1Static` | fail on unsafe/FFI, panic escapes, unwrap/expect/todo/unimplemented, recursion, I/O, ambient state, any free-form/content/product/authority-effect field, forbidden paths, or producer dependency on `bastion-review` |
| `CMD-L1-SUPPLY-CHAIN` | `L1SupplyChain` | Cargo metadata/lock assertion: one workspace package, zero external/transitive dependency, feature, build, proc-macro, native, registry, git, or path dependency |
| `CMD-L2-CONTRACT-MATRIX` | `L2Contract` | execute every required TEST/TRACE identity/digest/version/policy/acceptance-separation bootstrap partition |
| `CMD-L2-MODEL` | `L2Model` | exhaustive bounded review-disposition/trace/successor cases, external-acceptance separation, append preservation, and deterministic permutation equality |
| `CMD-L2-ADVERSARIAL` | `L2Adversarial` | stale/mismatch, self-approval, conflict, missing role/gate/trace, false-pass, duplicate/oversize, advocacy/classified-appeal substitution, dissent/defer attacks |
| `CMD-L2-NO-EMISSION` | `L2NoAuthority` | prove no free-form or product payload, I/O, HND/TERM/REL/Taxlane output/request, product value, or authority-returning surface |

All modes emit `review-gate-evidence.v1` JSON binding repository, WP, mode,
implementation/acceptance/WP/runner/toolchain/manifest/lock digests, exact argv
and sanitized environment, bounds, timings, exit/result, captured stream
hashes, assertions, and negative cases.

## 7. Required cases and acceptance gate

Positive cases: complete frozen security-admitted packet with derivations,
negative cases, and unresolved questions; independently frozen applicability
policy; exact equality for all seven policy-bound sets; both assurance gates; current passed
evidence; exact trace; explicitly
dispositioned minor/editorial item; retained negative result and dissent;
resolved conflict with retained predecessor; successor recommendation with an
exact externally accepted prior snapshot and replay of prior current state;
append-preserved prior finding/defer/dissent/negative
evidence; deterministic permutation; immutable repeat evaluation; valid
Identifier and Digest256 lower/upper constructor boundaries.

Fail-closed cases: subject/context/admission mismatch; zero or non-current
evidence; omitted or extra member in each evidence/derivation/negative-case/
unresolved-question/trace/role/assurance set; required-set binding or digest
substitution; each derivation/negative-case/unresolved-question bound plus one;
self-approval/conflict; missing/duplicate role; missing/failed gate;
policy ID/digest/version mismatch; context/admission-only substitution in
evidence, finding, role, or assurance records;
record ID/content-digest/version/predecessor substitution; non-monotone/broken
successor; missing/mismatched/forged external acceptance receipt; attempted
caller assertion of accepted/superseded state (structurally absent);
incomplete finding/defer; open conflict; open critical/major; orphan/duplicate/
stale trace; planned-as-executed evidence; false approval; duplicate ID; every
bound plus one; empty/oversize/non-ASCII/whitespace/control/illegal-character
Identifier; short/long/uppercase/non-hex Digest256; erased or rewritten
historical finding/defer/dissent/negative result or prior snapshot; advocacy, credentials, or
inaccessible classified appeal represented as evidence; payload insertion
through every identifier constructor; attempted HND/TERM/REL/Taxlane/
official-use field or effect (structurally absent);
subject mutation; dependency/unsafe/I/O/producer-backedge insertion.

Exit requires all 13 command identities to pass at one implementation digest,
all evidence retained under `EVID-WP-REV-001`, the exact Independent Test,
Role review, eight parliament, seven stakeholder, Scope, Citation, Numeracy,
Methodology, Classification/Operational Security, and Civilian Control/Law/
Safety/Readiness decisions, zero unresolved critical/major finding, and an
exact acceptance pulse. REV evidence proves only this substrate, passes no
producer, emits no terminal receipt, and does not close `TBD-TST-001` /
`SPEC-UNK-TST-001` alone. Rollback atomically reverts the exact implementation
commit, including crate, workspace member, Cargo.lock entry, and runner,
retains evidence/dissent, and returns REV to absent without altering WS.

Disposition: **exact WP candidate; independent acceptance required before
entry or implementation**.
