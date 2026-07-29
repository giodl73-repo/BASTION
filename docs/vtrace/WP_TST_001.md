# WP-TST-001-R2 — WS-rooted boundary-test and append-only fixture bootstrap

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundaries: `PB-TST-001` and assigned `PB-FIX-001`, plus
configuration-only membership integration in `PB-WS-001`

Implementation predecessor: accepted `WP-WS-001` exit only. R1 commit
`62116481b7b3e7d671517b6053c8cc3f20f93fce` is retained governance history,
not an executable predecessor. REV is context-only: it is neither a dependency,
entry precondition, executable input, nor authority, and the tests prove the
absence of both `TST -> REV` and `REV -> TST` edges.

## 1. Controlled baseline and custody

The executable line must descend from accepted `WP-WS-001` exit
`cd1f1d75ec312789fed63a265219d8ad9069a17a`. Governance may retain the R1 and
R2 records separately, but neither changes executable ancestry. Any different
ancestry, dirty unrelated path, predecessor digest, or dependency edge holds
acceptance and entry.

| Controlled artifact | Exact identity |
|---|---|
| Sole executable predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R1 governance commit (not accepted) | `62116481b7b3e7d671517b6053c8cc3f20f93fce` |
| Retained R1 WP SHA-256 | `93ea15ea87b140b7e45ae67db5a4133e24e8f18778db1ce41a891042b1157554` |
| `PACKAGE_BOUNDARIES.md` | `04f3923bf09f14b57cd02006dcb5e17abb4cac8981fced4c4749f0d446a71ca6` |
| `IMPLEMENTATION_PLAN.md` | `d8db35026e02796bc2bba5e034aa4981394ae1d9a4574943c82f4705bd64d9aa` |
| `WORK_PACKAGES.md` | `596761dd75e2d9b0721c7ecabb8086ee58682513b5a2c7d2a94ab1f7a79ea129` |
| `VERIFICATION.md` | `e25d1999a72575bd8e6c920cd450f50146754e08262e06127f64ff5c732fe080` |
| `VALIDATION.md` | `325be52b0ba258f97415709c040880f4282452d4ea3d609855dacd31a2bb1016` |
| `CHANGE_CONTROL.md` | `13cbfc8990ae42a5719f3945fe9dd7a46076def0db9be1f12ce4316c2dc57190` |
| WS workspace manifest | bound by the accepted WS-exit tree |
| WS lockfile | bound by the accepted WS-exit tree |

REV records may be inspected only as non-normative historical context. No REV
source, binary, package metadata, result, or digest is read by a command or
counted as evidence. This candidate may not alter REV, manufacture a producer
packet, claim a producer pass, or treat any recommendation as acceptance.

## 2. Objective and hard boundary

The smallest coherent result is an independently owned, deterministic Rust
integration-test package and an inert synthetic fixture-custody scaffold. It
proves only that:

1. the test executable is constructible from WS alone and no TST/FIX edge
   enters a product or REV target;
2. fixture identity, digest, expected posture, history, and safe content class
   are explicit and bounded;
3. malformed, stale, substituted, oversized, unsafe-marker, silent-golden,
   reverse-edge, and authority-bearing cases fail closed; and
4. exact commands can retain reproducible bootstrap evidence without creating
   product meaning.

The scaffold contains no semantic producer, corpus, parser for untrusted
content, generator, runtime service, product library, public API, operational
model, handoff adapter, release path, or official result. It may not contain
classified information, CUI, person-level service data, sensitive operational
data, targeting or operational-planning content, exploitable vulnerability
detail, credentials, or a real-world unit, person, asset, location, mission, or
event. Safe denial tokens are inert identifiers, never examples of prohibited
payloads.

`TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001`, and `TBD-REL-001` remain open and
are proof inputs only. This WP, its acceptance, entry, fixture definitions,
execution, evidence, or exit cannot close them. Every semantic producer and
later GEN/DOC/INT work remains blocked by its accepted hold transpose.

## 3. Exact future implementation allowlist

Implementation is forbidden unless a later pulse independently accepts this
exact WP digest and another later pulse enters it. Entry must create an
isolated branch/worktree from the accepted entry commit. The implementation
commit may add or change only:

```text
Cargo.toml
Cargo.lock
crates/bastion-boundary-tests/Cargo.toml
crates/bastion-boundary-tests/tests/support/mod.rs
crates/bastion-boundary-tests/tests/source_spine.rs
crates/bastion-boundary-tests/tests/contract_matrix.rs
crates/bastion-boundary-tests/tests/property_cases.rs
crates/bastion-boundary-tests/tests/model_cases.rs
crates/bastion-boundary-tests/tests/adversarial_cases.rs
crates/bastion-boundary-tests/tests/hold_closure.rs
crates/bastion-boundary-tests/tests/no_authority_surface.rs
crates/bastion-boundary-tests/tests/static_surface.rs
fixtures/bootstrap/manifest.tsv
fixtures/bootstrap/cases/valid.fixture
fixtures/bootstrap/cases/absent.fixture
fixtures/bootstrap/cases/stale.fixture
fixtures/bootstrap/cases/deny-marker.fixture
tools/test_gate.ps1
```

The root files may change only to add the test package as the sole workspace
member and the corresponding local lock entry on the WS-rooted executable
line. Evidence uses create-new, successor-addressed paths only:

```text
context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/runs/<Mode>/EVID-WP-TST-001-<Mode>-vNNNN.json
context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/sets/EVID-WP-TST-001-SET-vNNNN.json
```

No other crate, source, fixture, generated, documentation, role, wave,
TRACKER, submodule, CI, release, or configuration path is permitted. The
implementation commit is one atomic commit directly after the accepted entry
commit. Evidence is a separate later commit.

## 4. Exact package and dependency shape

`crates/bastion-boundary-tests/Cargo.toml` must define package
`bastion-boundary-tests` version `0.1.0`, `edition.workspace = true`,
`rust-version.workspace = true`, `publish = false`, `autotests = false`, and
workspace lints. It has:

- no library, binary, example, benchmark, build script, feature, normal
  dependency, external dependency, registry/git dependency, proc macro, native
  source, or unsafe allowance;
- zero normal, development, build, target-specific, workspace-inherited,
  external, registry, git, or path dependencies; and
- exactly eight explicit integration-test targets named `source_spine`,
  `contract_matrix`, `property_cases`, `model_cases`, `adversarial_cases`,
  `hold_closure`, `no_authority_surface`, and `static_surface`, each mapped to
  its same-named allowlisted file.

The root member list is exactly `["crates/bastion-boundary-tests"]` on the
WS-rooted executable line. Cargo metadata must show one package and zero
dependency edges. No product or REV target may depend on `PB-TST-001` or
`PB-FIX-001`; fixtures are compile-time
test inputs only through `include_bytes!`, never runtime, product, build-script,
or generated inputs.

Test support uses only `std`; it must not import, link, execute, inspect, or
otherwise depend on a REV surface.
It performs no runtime filesystem, network, environment, process, thread,
clock, locale, retry, randomness, or recursive operation. It exposes no public
library surface and contains no product value or decision. Explicit
`unwrap`, `expect`, `panic!`, `todo!`, and `unimplemented!` are forbidden;
ordinary Rust test assertion macros remain the test verdict mechanism.

## 5. Exact fixture-custody scaffold

`manifest.tsv` is UTF-8 without BOM, LF-only, ASCII, and contains one header
followed by exactly four current version-1 rows in ascending fixture-ID order.
Its exact columns are:

```text
fixture_id<TAB>version<TAB>predecessor_id<TAB>predecessor_digest<TAB>predecessor_version<TAB>supersession_state<TAB>path<TAB>sha256<TAB>class<TAB>source_posture<TAB>custodian_id<TAB>custody_id<TAB>custody_digest<TAB>purpose_id<TAB>expected_posture<TAB>expected_reason_id<TAB>proof_input_hold
```

The four rows are:

| Fixture ID | File | Class | Custody ID | Purpose / expected reason | Expected posture | Hold |
|---|---|---|---|---|---|---|
| `FIX-TST-BOOT-001` | `cases/valid.fixture` | `valid-custody` | `CUSTODY-TST-001` | `PURPOSE-TST-CUSTODY-001` / `REASON-TST-HARNESS-ONLY-001` | `accepted-for-harness-only` | `TBD-TST-001` |
| `FIX-TST-BOOT-002` | `cases/absent.fixture` | `explicit-absence` | `CUSTODY-TST-002` | `PURPOSE-TST-ABSENCE-001` / `REASON-SRC-ABSENT-001` | `held` | `TBD-SRC-001` |
| `FIX-TST-BOOT-003` | `cases/stale.fixture` | `stale-binding` | `CUSTODY-TST-003` | `PURPOSE-TST-STALE-001` / `REASON-TST-STALE-001` | `rejected` | `TBD-TST-001` |
| `FIX-TST-BOOT-004` | `cases/deny-marker.fixture` | `safe-denial-marker` | `CUSTODY-TST-004` | `PURPOSE-TST-DENY-001` / `REASON-SEC-DENY-001` | `rejected-safe` | `TBD-SEC-001` |

Each fixture file is UTF-8 without BOM, LF-only ASCII with exactly twelve
single-valued `key=value` rows: `fixture_id`, `version`, `predecessor_id`,
`predecessor_digest`, `predecessor_version`, `supersession_state`, `class`,
`source_posture`, `custody_id`, `purpose_id`, `expected_reason_id`, and `token`.
Keys occur in that order and only once. Version 1 uses predecessor ID `none`,
digest exactly 64 zeroes, predecessor version `0`, and state `current`.
`source_posture` is exactly `synthetic-inert`; `custodian_id` in the manifest
is exactly `ACT-TST` and `custody_digest` binds the canonical custody record.
Tokens use only ASCII
upper-case letters, digits, underscore, colon, and hyphen and convey no
real-world content. The manifest SHA-256 is lower-case hex over exact fixture
bytes. The test harness never interprets a token as evidence, semantics, or
authority.

Bounds are hard failures before favorable evaluation:

- manifest: at most 16 KiB and 32 rows;
- each fixture: at most 4 KiB; all fixture bytes together at most 32 KiB;
- each field/reference: non-empty and at most 128 bytes;
- fixture version: positive `u64`;
- path: exact allowlisted relative path, with no absolute, parent, alternate
  separator, drive, URI, symlink, or normalization behavior; and
- manifest and fixture IDs unique; `supersession_state` is exactly `current`
  or `superseded`, with one current row per lineage; lower-case 64-hex digest exact; no ignored,
  extra, duplicate, reordered, or silently defaulted field.

The committed rows are positive custody representatives. Tests derive bounded
negative missing/mismatch variants and a version-2 replay successor in memory.
Replay must reproduce the same verdict, identify version 1 by exact ID/version/
digest, mark version 1 superseded without deleting it, and make version 2 the
only current row. Any expected-posture or custody change requires that exact
successor operation and new review; deletion, overwrite, ambiguous current
state, hand edit, silent golden replacement, or quarantine rejects.

## 6. Exact runner, commands, and execution bounds

`tools/test_gate.ps1` is a non-product verification runner with the same
supervisor/worker separation as the accepted REV runner. Invocation is:

```text
pwsh -NoLogo -NoProfile -NonInteractive -File tools/test_gate.ps1 -Mode <MODE>
```

The supervisor never deletes, overwrites, renames, or quarantines evidence. It
allocates the next unused positive four-digit mode version with create-new
semantics, creates a Windows Job Object, sanitizes the environment to exactly
`APPDATA`, `CARGO_HOME`, `COMSPEC`, `HOME`, `LOCALAPPDATA`, `PATH`, `PATHEXT`,
`PSModulePath`, `RUSTUP_HOME`, `SYSTEMROOT`, `TEMP`, `TMP`, `USERPROFILE`, and
`WINDIR` when non-empty, and publishes pass evidence only after bounded stream
capture, zero worker exit, and all postconditions. Every non-pass attempt writes
a typed fail record; if create-new retention itself fails, the mode has no
evidence and the supervising reproduction retains that incident before retry.
Command stdout and stderr share one streaming budget
across the complete mode.

| Identity | MODE | Exact internal command/assertion |
|---|---|---|
| `CMD-L0-FORMAT` | `L0Format` | `cargo +1.95.0 fmt --all -- --check` |
| `CMD-L0-CHECK` | `L0Check` | `cargo +1.95.0 check -p bastion-boundary-tests --locked --offline --all-targets` |
| `CMD-L0-FOCUSED-TEST` | `L0FocusedTest` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline` |
| `CMD-L1-WORKSPACE-CHECK` | `L1WorkspaceCheck` | `cargo +1.95.0 check --workspace --locked --offline --all-targets` |
| `CMD-L1-LINT` | `L1Clippy` | `cargo +1.95.0 clippy --workspace --locked --offline --all-targets -- -D warnings` |
| `CMD-L1-TEST` | `L1Test` | `cargo +1.95.0 test --workspace --locked --offline` |
| `CMD-L1-DOC` | `L1Doc` | `cargo +1.95.0 doc --workspace --locked --offline --no-deps`, then `cargo +1.95.0 test --workspace --doc --locked --offline` |
| `CMD-L1-STATIC` | `L1Static` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test static_surface`, then exact source/path/dependency-direction assertions |
| `CMD-L1-SUPPLY-CHAIN` | `L1SupplyChain` | `cargo +1.95.0 metadata --locked --offline --no-deps --format-version 1`, then assert the exact one-package graph and zero dependency edges |
| `CMD-L2-SOURCE-SPINE` | `L2SourceSpine` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test source_spine` |
| `CMD-L2-CONTRACT-MATRIX` | `L2Contract` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test contract_matrix` |
| `CMD-L2-PROPERTY` | `L2Property` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test property_cases` |
| `CMD-L2-MODEL` | `L2Model` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test model_cases` |
| `CMD-L2-ADVERSARIAL` | `L2Adversarial` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test adversarial_cases` |
| `CMD-L2-HOLD-CLOSURE` | `L2HoldClosure` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test hold_closure` |
| `CMD-L2-NO-EMISSION` | `L2NoAuthority` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test no_authority_surface` |

Every mode has a 60-second wall limit, 1 GiB process-tree memory limit, and
10 MiB combined stdout/stderr limit. The runner itself performs no network
access and all Cargo commands are `--locked --offline`. It binds and verifies
one implementation commit, implementation/test/fixture-manifest/WP/
acceptance/runner/root-manifest/lock/WS predecessor digest, exact argv,
sanitized-environment digest, start/end/duration, bounds, per-command exit and
stream hashes/bytes, combined bytes, assertions, executed case target, and
result in canonical `test-gate-evidence.v2` JSON. All 16 modes must pass at
one identical binding. A zero-test target, skipped target, missing field,
mutation during a run, mismatched digest, or output after supervisor failure
is a failure.

## 7. Required bootstrap cases

`source_spine` must prove the bounded bootstrap-only chain
`accepted WS binding -> inert fixture custody -> isolated test verdict ->
non-authoritative evidence record`. Every node and edge is digest-bound; a
missing, reordered, substituted, reverse, REV, producer, HND, Taxlane, or
release edge rejects. The target proves no semantic source or product result
exists and that no TST-to-REV or REV-to-TST edge exists.

`contract_matrix` must prove the exact four-row manifest, header/field order,
fixture byte digests, exact allowed paths, unique IDs, four expected postures,
custody/predecessor/supersession/reason fields, compile-time fixture custody,
zero dependency edges, and absence of a product target. It also proves
lower/upper accepted field and byte bounds.

`property_cases` must exhaustively exercise finite deterministic partitions,
not random generation: every fixture class and expected posture; empty, one,
maximum, and maximum-plus-one field/row/byte bounds; ascending and permuted
input order; all accepted token characters and each rejected character class;
and repeat evaluation. It must retain the first bounded counterexample and
never infer a favorable value on exhaustion.

`model_cases` must prove deterministic equality under input permutation,
explicit absence remains held, stale remains rejected, denial remains safely
rejected, predecessor/version changes create immutable successors, prior
fixture/history bindings remain retained, and repeat evaluation is identical.

`adversarial_cases` must fail closed for: missing/extra/duplicate/reordered
manifest rows or fields; empty/oversized/illegal/non-ASCII field; short, long,
upper-case, non-hex, or substituted digest; fixture/manifest mismatch;
absolute, traversal, alternate-separator, drive, URI, symlink, or unallowlisted
path; zero/non-monotone/broken predecessor; stale-as-current; absent-as-valid;
unsafe-content marker substitution; prohibited payload-shaped insertion;
silent expected-posture/golden update; fixture byte/row totals plus one; test
target omission; normal/product dependency on TST or FIX; unexpected package,
dependency, feature, build script, proc macro, native source, or registry/git
source; runtime I/O/ambient state; and any implementation path outside the
allowlist.

`hold_closure` must execute the exact bootstrap hold assertions. It proves
`TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001`, and `TBD-REL-001` are present as
open proof inputs; no fixture state, test verdict, REV recommendation,
evidence tier, role majority, or exit recommendation can represent `closed`;
and missing or favorably rewritten hold state rejects. Its passing result is
explicit evidence of non-closure, never hold closure.

`no_authority_surface` must prove that the package and fixtures cannot encode
or return a producer result, operational/force/procurement/budget/allocation/
rate decision, readiness or savings claim, HND pack, terminal acceptance,
Taxlane state, release request/artifact, official-use result, or external
action. A test pass is evidence only and grants no acceptance or authority.

`static_surface` must prove the exact eight test targets, support-only module,
four fixtures, manifest, zero dependencies and no REV edge, no public library/binary,
no unsafe/FFI/runtime I/O/ambient state/thread/randomness/recursion, no hidden
path, and no product-to-test/fixture or generated-source edge.

The full L0/L1 runs must also prove formatting, compilation, all tests, docs,
lint with warnings denied, offline supply-chain closure, and exact rollback
surface. Case labels in evidence may state only an assertion or test target
that actually executed.

### 7.1 Digest-bound forward trace

`support/mod.rs` contains one canonical, bytewise-sorted trace manifest. Every
row binds its controlled-source digest, exact source IDs, target/assertion IDs,
and evidence modes. Every evidence record binds the manifest SHA-256.

| Controlled source IDs | Exact target/assertion | Evidence mode(s) |
|---|---|---|
| `BASTION-REQ-TST-001`; `SPEC-TST-001` | `source_spine::frozen_digest_packet`; `contract_matrix::digest_binding` | `L2SourceSpine`, `L2Contract` |
| `BASTION-REQ-TST-002`; `SPEC-TST-002` | `property_cases::bounded_reproduction`; `model_cases::repeat_identity`; `adversarial_cases::adverse_replay` | `L2Property`, `L2Model`, `L2Adversarial` |
| `BASTION-REQ-TST-003`; `SPEC-TST-003` | `model_cases::retained_negative_history`; `hold_closure::nonpass_retention` | `L2Model`, `L2HoldClosure` |
| `BASTION-REQ-TST-004`; `SPEC-TST-004` | `contract_matrix::full_evidence_schema` | `L2Contract` |
| `BASTION-REQ-TST-005`; `SPEC-TST-005` | `hold_closure::convergence_blocks` | `L2HoldClosure` |
| `BASTION-REQ-TST-006`; `SPEC-TST-006` | `adversarial_cases::substitution_cannot_approve` | `L2Adversarial` |
| `BASTION-REQ-REL-001`; `SPEC-REL-001` | `no_authority_surface::no_publish_or_approval` | `L2NoAuthority` |
| `BASTION-REQ-REL-002`; `SPEC-REL-002` | `adversarial_cases::release_contract_unavailable`; `no_authority_surface::no_release_artifact` | `L2Adversarial`, `L2NoAuthority` |
| `BASTION-REQ-REL-003`; `SPEC-REL-003` | `source_spine::context_retained_without_output`; `no_authority_surface::no_external_output` | `L2SourceSpine`, `L2NoAuthority` |
| `SPEC-NF-001` | `adversarial_cases::bounded_safe_failure` | `L2Adversarial` |
| `SPEC-NF-002` | `no_authority_surface::authority_and_security_absent` | `L2NoAuthority` |
| `SPEC-NF-003` | `no_authority_surface::no_readiness_claim` | `L2NoAuthority` |
| `SPEC-NF-004` | `property_cases::finite_tail_partitions` | `L2Property` |
| `SPEC-NF-005` | `property_cases::no_incidence_or_uncertainty_inference` | `L2Property` |
| `SPEC-NF-006` | `model_cases::accounting_successor_history` | `L2Model` |
| `SPEC-NF-007` | `property_cases::determinism_and_bounds` | `L2Property` |
| `SPEC-NF-008` | `contract_matrix::fixture_and_support_custody` | `L2Contract` |
| `SPEC-NF-009` | `model_cases::immutable_rejection_no_emission` | `L2Model`, `L2NoAuthority` |
| `SPEC-NF-010` | `source_spine::trace_and_review_state` | `L2SourceSpine` |
| `DES-TEST` bootstrap; `CONTRACT-TEST` bootstrap | `contract_matrix::test_bootstrap_contract`; all seven L2 targets | `L2SourceSpine`, `L2Contract`, `L2Property`, `L2Model`, `L2Adversarial`, `L2HoldClosure`, `L2NoAuthority` |
| `DES-REL` bootstrap; `CONTRACT-REL` bootstrap | `no_authority_surface::release_bootstrap_no_output`; `adversarial_cases::release_misuse_rejected` | `L2Adversarial`, `L2NoAuthority` |
| `CR-002..CR-006`; `CR-008..CR-040` (exactly 38; `CR-001` and `CR-007` excluded) | `static_surface::exact_cr_allocation`; relevant seven L2 assertions above | `L1Static`, `L1SupplyChain`, all seven L2 modes |
| `VCL-01` | `source_spine::identity_allocation_trace_custody_digest` | `L2SourceSpine` |
| `VCL-02` | `contract_matrix::positive_negative_unauthorized_consumer` | `L2Contract` |
| `VCL-03` | `model_cases::typed_state_transition_dag_successor_invalid_edge` | `L2Model` |
| `VCL-04` | `property_cases::property_deterministic_reproduction` | `L2Property` |
| `VCL-05` | `hold_closure::thirteen_hold_missing_null_na_no_default` | `L2HoldClosure` |
| `VCL-06` | `adversarial_cases::prohibited_composition_minimization_safe_failure` | `L2Adversarial` |
| `VCL-07` | `no_authority_surface::civilian_law_safety_readiness_stakeholder_distribution_burden` | `L2NoAuthority` |
| `VCL-08` | `no_authority_surface::accounting_path_delivery_hnd_term_taxlane_rel_no_authority` | `L2NoAuthority` |
| `VCL-09` | `static_surface::quality_dependency_support_isolation_generated_custody_bounds` | `L1Static`, `L1SupplyChain` |
| `VCL-10` | `source_spine::evidence_truth_review_dissent_validation_rollback_compat_history` | `L2SourceSpine` |
| `VAL-SCOPE`; `ACT-TST`, `ACT-SRC`, `ACT-EXT`, `ACT-LAW`, `ACT-CIV`; `ACT-PAR-ALL` exactly = `ACT-CIV`, `ACT-RDY`, `ACT-ACQ`, `ACT-LOG`, `ACT-ALLY`, `ACT-FIN`, `ACT-PPL`, `ACT-TST` | `source_spine::validation_scope_actors`; all seven L2 targets | all seven L2 modes |
| `VAL-ASSURANCE`; `PAR-ALL`, Methodology Panel, Role review steward, Citation Auditor, Scope Keeper, Numeracy Checker, seven stakeholder lenses, Classification & Operational Security, Civilian Control/Law/Safety/Readiness | `source_spine::validation_assurance_lanes` | `L2SourceSpine`, `L2HoldClosure`, `L2NoAuthority` |
| `SPEC-UNK-SEC-001`; `TBD-SEC-001` | `hold_closure::sec_open_proof_input`; no closure/default | `L2HoldClosure`, `L2Adversarial` |
| `SPEC-UNK-SRC-001`; `TBD-SRC-001` | `hold_closure::src_open_proof_input`; absence remains held | `L2HoldClosure`, `L2SourceSpine` |
| `SPEC-UNK-TST-001`; `TBD-TST-001` | `hold_closure::tst_open_proof_input`; verdict cannot close | `L2HoldClosure`, `L2Contract` |
| `SPEC-UNK-REL-001`; `TBD-REL-001` | `hold_closure::rel_open_proof_input`; release remains unavailable | `L2HoldClosure`, `L2NoAuthority` |

### 7.2 Exact reverse transpose

| Evidence mode | Exact controlled rows consumed |
|---|---|
| `L2SourceSpine` | `TST-001`, `REL-003`, `NF-010`, `DES/CONTRACT-TEST`, `VCL-01`, `VCL-10`, `VAL-SCOPE`, `VAL-ASSURANCE`, `UNK/TBD-SRC` |
| `L2Contract` | `TST-001`, `TST-004`, `NF-008`, `DES/CONTRACT-TEST`, `VCL-02`, `VAL-SCOPE`, `UNK/TBD-TST` |
| `L2Property` | `TST-002`, `NF-004`, `NF-005`, `NF-007`, `DES/CONTRACT-TEST`, `VCL-04`, `VAL-SCOPE` |
| `L2Model` | `TST-002`, `TST-003`, `NF-006`, `NF-009`, `DES/CONTRACT-TEST`, `VCL-03`, `VAL-SCOPE` |
| `L2Adversarial` | `TST-002`, `TST-006`, `REL-002`, `NF-001`, `DES/CONTRACT-TEST`, `DES/CONTRACT-REL`, `VCL-06`, `VAL-SCOPE`, `UNK/TBD-SEC` |
| `L2HoldClosure` | `TST-003`, `TST-005`, `DES/CONTRACT-TEST`, `VCL-05`, `VAL-SCOPE`, `VAL-ASSURANCE`, all four `UNK/TBD` pairs |
| `L2NoAuthority` | `REL-001..003`, `NF-002`, `NF-003`, `NF-009`, `DES/CONTRACT-TEST`, `DES/CONTRACT-REL`, `VCL-07`, `VCL-08`, `VAL-SCOPE`, `VAL-ASSURANCE`, `UNK/TBD-REL` |

Every reverse row also consumes the exact 38-CR allocation. The trace test
compares forward and reverse sets exactly and rejects an absent/orphan row,
unlisted extra, duplicate ID, duplicate edge, count error, digest mismatch,
target mismatch, assertion mismatch, evidence-mode mismatch, or non-transpose.

## 8. Evidence, independent review, and roles

Evidence is retained only under `EVID-WP-TST-001`. Every attempt, including a
runner/supervisor failure, creates its own immutable mode successor. Version 1
uses null predecessor ID/digest and predecessor version `0`; every later
version names the exact preceding ID, SHA-256, and version. No record is
deleted, overwritten, quarantined, or reused. A correction or retry is another
successor and retains failed, conflicted, invalid, rejected, and superseded
history.

Each of the 16 canonical `test-gate-evidence.v2` mode records contains all
fields required by `VERIFICATION.md` lines 100–120, without omission:

1. evidence ID/version/schema, tier/status, producer WP, and exact producing
   commit;
2. exact requirement/specification/DES/contract/CR/hold/VCL coverage;
3. resolved command ID/mode/argv, tool versions/configuration digests, and
   sanitized-environment digest;
4. every public/synthetic fixture ID/version/source/custody/purpose/input
   digest/predecessor/supersession state;
5. finite row/byte/time/memory/stream bounds, deterministic order, and explicit
   seed/clock/locale controls (`disabled` where prohibited);
6. expected/actual typed posture, exit state, stdout/stderr/output/evidence
   digests, and retained failure/counterexample objects;
7. author, independent reviewer, parliament/domain/formal-assurance decisions,
   findings, dissent, conflicts, and dispositions; and
8. predecessor digests, invalidation triggers, rollback/reproduction pointers,
   immutable history, and successor fields.

Fields exist even when empty: typed empty arrays plus an exact not-applicable
reason replace omission. Each record carries a payload SHA-256 over canonical
bytes with its digest field omitted; the set independently binds the complete
record bytes. It also binds WS/WP/acceptance/implementation/trace/fixture/
runner/root/lock digests, executed
target/assertion IDs, start/end/duration, bounds, per-command stream digests and
bytes, and result. A worker cannot publish pass; the supervisor creates a
record only after postcondition validation. Missing, planned, absent, stale,
conflicted, held, failed, rejected, zero-test, or schema-incomplete evidence
cannot count as pass.

After the 16 mode files are committed, a separate create-new set record lists
their exact paths/digests in mode order, binds that evidence commit and Git
tree, and records the aggregate SHA-256 over each
`<relative-path><TAB><sha256><LF>` line in bytewise path order. The set record
carries the same history/reproduction/review fields. Independent review binds
the later final evidence-directory Git tree including the set record, avoiding
a self-referential tree digest. Independent reproduction creates a new complete
16-record/set successor and never mutates the producer set.

All decisions bind identical per-mode digests, aggregate digest, preceding
evidence tree, final custody tree, WP, implementation, fixture-manifest,
runner, environment, WS-predecessor, and policy digests. Required lanes are:

- all eight exact parliament roles (`PAR-ALL`), with Independent Test &
  Oversight as test owner and no author/self-review;
- Methodology Panel and Role review steward;
- Citation Auditor, Scope Keeper, and Numeracy Checker;
- all seven stakeholder lenses, limited to detecting accidental semantic,
  burden, people, supplier, community, alliance, or taxpayer claims; and
- Classification & Operational Security and Civilian Control, Law, Safety &
  Readiness as independent conjunctive assurance gates.

Each lane records `pass`, `finding`, or `defer`, retains dissent, and names an
owner/destination/closure condition for every non-pass. No majority, other
lane, bootstrap test, or stage controller can waive a failed assurance gate.
Zero unresolved critical or major finding and zero open evidence conflict are
mandatory for an exit recommendation.

## 9. Entry, stop, exit, and authority

Acceptance of this candidate, if it occurs, authorizes only a later separate
entry decision. Entry requires the exact accepted WP and acceptance-pulse
digests, a clean isolated worktree rooted at the accepted WS exit, the exact
allowlist, one atomic
implementation commit, and no unrelated change.

Stop immediately on any baseline, path, package, dependency, target, fixture,
digest, schema, case, command, environment, bound, role, history, content,
hold, or authority deviation. Preserve the failure and request a successor WP;
do not broaden the allowlist or weaken an expected rejection in place.

Exit requires all 16 exact modes retained at one implementation digest, exact
fixture inventory, independent reproduction, exact evidence-set aggregate and
Git tree, all required current role decisions, no unresolved critical/major
finding, and a separate independent review. External stage governance alone
may later accept the bootstrap exit.

Even an accepted exit proves only isolated harness and safe fixture custody.
It does not accept a producer, produce product evidence, close any held pair,
emit HND/TERM/REL/Taxlane state, authorize official or operational use, or
permit publication. Any later proposal to close `TBD-TST-001` /
`SPEC-UNK-TST-001` requires its own exact retained evidence review and explicit
governance decision against all eight planned `VER-*` identities.

## 10. Rollback and reopen

Rollback is one atomic successor tree change that removes the exact TST package,
fixtures, and runner and restores the WS-exit root manifest and lock bytes. It
never deletes evidence: mode/set records, implementation/evidence commits,
failed results, counterexamples, findings, dissent, reproduction records, and
the rollback decision remain recoverable in Git history and digest-bound.

Reopen this WP before entry for any changed baseline, representation,
allowlist, dependency direction, fixture inventory/schema/content class,
expected posture, test target, command, runner behavior, resource bound,
evidence schema/destination, reviewer set, stop/exit rule, or rollback. After
entry, any such change requires stop, retained evidence, and a separately
accepted successor; it is not an implementation detail.

Disposition: **exact WP candidate for independent review only; not accepted;
not entered; no implementation or authority**.
