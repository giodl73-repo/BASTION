# WP-TST-001-R3 — linearly entered boundary-test and fixture-custody bootstrap

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundaries: `PB-TST-001` and assigned `PB-FIX-001`, plus
configuration-only membership integration in `PB-WS-001`

Logical WP predecessor: accepted `WP-WS-001` exit only. R1 commit
`62116481b7b3e7d671517b6053c8cc3f20f93fce` and R2 commit
`21c8066445c72358a444c0b506422ec3b9dc63e0` are retained governance history.
After R3 acceptance, the entry commit and its direct implementation successor
must remain on the current governance/main lineage. Accepted REV is only a
context co-member: workspace co-membership and Git ancestry are explicitly not
WP-predecessor or dependency relationships.

## 1. Controlled baseline and custody

The future acceptance commit must descend from R3 on current main; the entry
commit must be its direct governance successor; and the one implementation
commit must be the direct child of entry. Accepted `WP-WS-001` exit
`cd1f1d75ec312789fed63a265219d8ad9069a17a` remains the sole logical WP
predecessor. Any nonlinear implementation ancestry, dirty unrelated path,
predecessor digest, or Cargo edge holds acceptance and entry.

| Controlled artifact | Exact identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R1 governance commit (not accepted) | `62116481b7b3e7d671517b6053c8cc3f20f93fce` |
| Retained R1 WP SHA-256 | `93ea15ea87b140b7e45ae67db5a4133e24e8f18778db1ce41a891042b1157554` |
| Retained R2 governance commit (not accepted) | `21c8066445c72358a444c0b506422ec3b9dc63e0` |
| Retained R2 WP SHA-256 / blob | `4ecd246d67bb5d07c94496a9975c99cdc8488295e8e74235be29391b3725e146` / `47687aff86c392b7e30b237de1015b9d304d4fc4` |
| Current R3 governance-line base before acceptance | `21c8066445c72358a444c0b506422ec3b9dc63e0` |
| Context-only accepted REV exit | `ab227cc06f15299b594cfe2be99915bd93c4c081` |
| Context-only accepted REV implementation commit / SHA-256 | `5c4e96306d3c463a44be7621371759da8bca399b` / `c5c2df1178568cd49b5d721cd01cba7cce3371e049528e07bad30d6b3324ea72` |
| Context-only accepted REV evidence-set SHA-256 / tree | `b95beff569794125018f2fde3d4d3317ed32278dfcfb1fc22a7d25cf51226bd9` / `d554c8c0c3d534aa96924f085a4dc007b25e3a3c` |
| `PACKAGE_BOUNDARIES.md` | `04f3923bf09f14b57cd02006dcb5e17abb4cac8981fced4c4749f0d446a71ca6` |
| `IMPLEMENTATION_PLAN.md` | `d8db35026e02796bc2bba5e034aa4981394ae1d9a4574943c82f4705bd64d9aa` |
| `WORK_PACKAGES.md` | `596761dd75e2d9b0721c7ecabb8086ee58682513b5a2c7d2a94ab1f7a79ea129` |
| `VERIFICATION.md` | `e25d1999a72575bd8e6c920cd450f50146754e08262e06127f64ff5c732fe080` |
| `VALIDATION.md` | `325be52b0ba258f97415709c040880f4282452d4ea3d609855dacd31a2bb1016` |
| `CHANGE_CONTROL.md` | `13cbfc8990ae42a5719f3945fe9dd7a46076def0db9be1f12ce4316c2dc57190` |
| WS workspace manifest | bound by the accepted WS-exit tree |
| WS lockfile | bound by the accepted WS-exit tree |

REV records may be inspected only to prove the co-member is unchanged and has
zero edges to or from TST. No REV source, binary, result, or digest is a TST
input or pass prerequisite. This candidate may not alter REV, manufacture a
producer packet, claim a producer pass, or treat any recommendation as
acceptance.

## 2. Objective and hard boundary

The smallest coherent result is an independently owned, deterministic Rust
integration-test package and an inert synthetic fixture-custody scaffold. It
proves only that:

1. TST is dependency-free: the two-node workspace graph has no edge in either
   direction, TST-only selection does not select REV, and removing REV from an
   in-memory graph projection leaves the TST node and command invariant;
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

The root files may change only to add the test package after the unchanged REV
co-member and add the corresponding dependency-free local lock entry on the
linear entry/implementation line. Evidence uses create-new,
successor-addressed paths only:

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

The root member list is exactly
`["crates/bastion-boundary-tests", "crates/bastion-review"]`. Cargo metadata
must show exactly those two zero-dependency nodes and zero edges. No product or
REV target may depend on `PB-TST-001` or `PB-FIX-001`; fixtures are compile-time
test inputs only through `include_bytes!`, never runtime, product, build-script,
or generated inputs.

Test support uses only `std`; it must not import, link, execute, or otherwise
depend on a REV surface. Static custody may inspect the unchanged REV digest,
metadata node, and absence of edges, but never use REV behavior as TST input.
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
fixture_id<TAB>version<TAB>predecessor_id<TAB>predecessor_digest<TAB>predecessor_version<TAB>supersession_state<TAB>path<TAB>sha256<TAB>class<TAB>source_posture<TAB>source_id<TAB>source_digest<TAB>custodian_id<TAB>custody_id<TAB>custody_digest<TAB>purpose_id<TAB>expected_posture<TAB>expected_reason_id<TAB>proof_input_hold
```

The four rows are:

| Fixture ID | File / class | Source ID / exact source digest | Custody ID | Purpose / expected reason | Expected posture | Hold |
|---|---|---|---|---|---|---|
| `FIX-TST-BOOT-001` | `cases/valid.fixture` / `valid-custody` | `SRC-TST-BOOT-001` / `3d41a285934e097c1c806401c27997311df0bd9236bcbb262fc22be68dd5d360` | `CUSTODY-TST-001` | `PURPOSE-TST-CUSTODY-001` / `REASON-TST-HARNESS-ONLY-001` | `accepted-for-harness-only` | `TBD-TST-001` |
| `FIX-TST-BOOT-002` | `cases/absent.fixture` / `explicit-absence` | `SRC-TST-BOOT-002` / `69041cd1d2687c80252fe90d38559d350c4141ceb97bfb57c93df9e856d184d3` | `CUSTODY-TST-002` | `PURPOSE-TST-ABSENCE-001` / `REASON-SRC-ABSENT-001` | `held` | `TBD-SRC-001` |
| `FIX-TST-BOOT-003` | `cases/stale.fixture` / `stale-binding` | `SRC-TST-BOOT-003` / `d94e4b91e6e30a8106c3ffb0e48dcb9e6bbdbed55d8c0960bcee84ef1fb69bb0` | `CUSTODY-TST-003` | `PURPOSE-TST-STALE-001` / `REASON-TST-STALE-001` | `rejected` | `TBD-TST-001` |
| `FIX-TST-BOOT-004` | `cases/deny-marker.fixture` / `safe-denial-marker` | `SRC-TST-BOOT-004` / `c41e496c0790862b1a3c790703635b44e8862d7c10e95851fd052ebd778a0376` | `CUSTODY-TST-004` | `PURPOSE-TST-DENY-001` / `REASON-SEC-DENY-001` | `rejected-safe` | `TBD-SEC-001` |

Each source digest is SHA-256 over exact canonical UTF-8/LF/no-BOM bytes
`schema=synthetic-fixture-source.v1<LF>`,
`source_id=<source_id><LF>`, and
`source_posture=synthetic-inert<LF>` in that order.

Each fixture file is UTF-8 without BOM, LF-only ASCII with exactly twelve
single-valued `key=value` rows: `fixture_id`, `version`, `predecessor_id`,
`predecessor_digest`, `predecessor_version`, `supersession_state`, `class`,
`source_posture`, `custody_id`, `purpose_id`, `expected_reason_id`, and `token`.
Keys occur in that order and only once. Version 1 uses predecessor ID `none`,
digest exactly 64 zeroes, predecessor version `0`, and state `current`.
`source_posture` is exactly `synthetic-inert`; `source_id` is the stable safe-
synthetic recipe identity and `source_digest` binds its exact canonical recipe
bytes. `custodian_id` is exactly `ACT-TST`.

`custody_digest` is SHA-256 over the following exact UTF-8, LF-only,
ASCII, no-BOM canonical bytes, with no final omitted field, interpolation, or
normalization:

```text
schema=test-fixture-custody.v1<LF>
custodian_id=<custodian_id><LF>
custody_id=<custody_id><LF>
fixture_id=<fixture_id><LF>
version=<base-10 version><LF>
source_posture=<source_posture><LF>
source_id=<source_id><LF>
source_digest=<source_digest><LF>
purpose_id=<purpose_id><LF>
expected_posture=<expected_posture><LF>
expected_reason_id=<expected_reason_id><LF>
proof_input_hold=<proof_input_hold><LF>
```

Only those existing manifest values, in that order, enter the custody preimage;
`custody_digest` itself, path, fixture-byte digest, class, predecessor, and
supersession fields are excluded, preventing self-reference. The schema tag,
keys, `=`, values, and final LF all count as bytes.
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
negative missing/source-substitution/custody-substitution/digest-mismatch
variants and a version-2 replay successor in memory.
Replay must reproduce the same verdict, identify version 1 by exact ID/version/
digest, mark version 1 superseded without deleting it, and make version 2 the
only current row. Any expected-posture or custody change requires that exact
successor operation and new review; deletion, overwrite, ambiguous current
state, missing source/custody field, changed canonical order, hand edit, silent
golden replacement, or quarantine rejects. Successor and replay tests recompute
both source and custody digests, retain the predecessor bytes/digests, reject a
copied old digest, and reproduce the same verdict from identical canonical
bytes.

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
| `CMD-L1-SUPPLY-CHAIN` | `L1SupplyChain` | `cargo +1.95.0 metadata --locked --offline --no-deps --format-version 1`, then assert exactly `bastion-boundary-tests` and unchanged `bastion-review`, both zero-dependency nodes, zero graph edges, and invariant TST node after in-memory REV-node removal |
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

`L0FocusedTest` and every L2 command select only
`bastion-boundary-tests`; Cargo resolution must show that REV is not selected,
built, linked, or executed. `L1Static` binds the unchanged accepted REV source
digest and proves no TST/REV import or path reference. `L1SupplyChain` compares
the TST node before and after non-mutating in-memory removal of the REV metadata
node and requires byte-identical TST package/target/dependency data. Full
workspace L1 commands additionally prove unchanged REV behavior, but that
result cannot satisfy, alter, or gate a TST assertion.

## 7. Required bootstrap cases

`source_spine` must prove the bounded bootstrap-only chain
`linear accepted entry -> inert fixture custody -> isolated test verdict ->
non-authoritative evidence record`. Every node and edge is digest-bound; a
missing, reordered, substituted, reverse, dependency, producer, HND, Taxlane, or
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

### 7.1 Exact canonical forward trace

`support/mod.rs` contains one canonical, bytewise-sorted trace manifest. Each
row below has exactly one controlled identity, one target/assertion, and one
consuming evidence mode. Paired identities, aliases, ranges, shorthand, and
names ending in `bootstrap` are invalid. Every row binds the exact digest of
its controlled source and every evidence record binds the trace-manifest
SHA-256.

| Canonical controlled identity | Exact target / assertion | Evidence mode |
|---|---|---|
| `BASTION-REQ-TST-001` | `source_spine::trace_bastion_req_tst_001` | `L2SourceSpine` |
| `BASTION-REQ-TST-002` | `property_cases::trace_bastion_req_tst_002` | `L2Property` |
| `BASTION-REQ-TST-003` | `model_cases::trace_bastion_req_tst_003` | `L2Model` |
| `BASTION-REQ-TST-004` | `contract_matrix::trace_bastion_req_tst_004` | `L2Contract` |
| `BASTION-REQ-TST-005` | `hold_closure::trace_bastion_req_tst_005` | `L2HoldClosure` |
| `BASTION-REQ-TST-006` | `adversarial_cases::trace_bastion_req_tst_006` | `L2Adversarial` |
| `BASTION-REQ-REL-001` | `no_authority_surface::trace_bastion_req_rel_001` | `L2NoAuthority` |
| `BASTION-REQ-REL-002` | `adversarial_cases::trace_bastion_req_rel_002` | `L2Adversarial` |
| `BASTION-REQ-REL-003` | `no_authority_surface::trace_bastion_req_rel_003` | `L2NoAuthority` |
| `SPEC-TST-001` | `source_spine::trace_spec_tst_001` | `L2SourceSpine` |
| `SPEC-TST-002` | `property_cases::trace_spec_tst_002` | `L2Property` |
| `SPEC-TST-003` | `model_cases::trace_spec_tst_003` | `L2Model` |
| `SPEC-TST-004` | `contract_matrix::trace_spec_tst_004` | `L2Contract` |
| `SPEC-TST-005` | `hold_closure::trace_spec_tst_005` | `L2HoldClosure` |
| `SPEC-TST-006` | `adversarial_cases::trace_spec_tst_006` | `L2Adversarial` |
| `SPEC-REL-001` | `no_authority_surface::trace_spec_rel_001` | `L2NoAuthority` |
| `SPEC-REL-002` | `adversarial_cases::trace_spec_rel_002` | `L2Adversarial` |
| `SPEC-REL-003` | `no_authority_surface::trace_spec_rel_003` | `L2NoAuthority` |
| `SPEC-NF-001` | `adversarial_cases::trace_spec_nf_001` | `L2Adversarial` |
| `SPEC-NF-002` | `no_authority_surface::trace_spec_nf_002` | `L2NoAuthority` |
| `SPEC-NF-003` | `no_authority_surface::trace_spec_nf_003` | `L2NoAuthority` |
| `SPEC-NF-004` | `property_cases::trace_spec_nf_004` | `L2Property` |
| `SPEC-NF-005` | `property_cases::trace_spec_nf_005` | `L2Property` |
| `SPEC-NF-006` | `model_cases::trace_spec_nf_006` | `L2Model` |
| `SPEC-NF-007` | `property_cases::trace_spec_nf_007` | `L2Property` |
| `SPEC-NF-008` | `contract_matrix::trace_spec_nf_008` | `L2Contract` |
| `SPEC-NF-009` | `model_cases::trace_spec_nf_009` | `L2Model` |
| `SPEC-NF-010` | `source_spine::trace_spec_nf_010` | `L2SourceSpine` |
| `DES-TEST-001` | `contract_matrix::trace_des_test_001` | `L2Contract` |
| `DES-REL-001` | `no_authority_surface::trace_des_rel_001` | `L2NoAuthority` |
| `CONTRACT-TEST-001` | `contract_matrix::trace_contract_test_001` | `L2Contract` |
| `CONTRACT-REL-001` | `no_authority_surface::trace_contract_rel_001` | `L2NoAuthority` |
| `CR-002` | `static_surface::trace_cr_002` | `L1Static` |
| `CR-003` | `test_gate::trace_cr_003` | `L1SupplyChain` |
| `CR-004` | `static_surface::trace_cr_004` | `L1Static` |
| `CR-005` | `test_gate::trace_cr_005` | `L1SupplyChain` |
| `CR-006` | `static_surface::trace_cr_006` | `L1Static` |
| `CR-008` | `test_gate::trace_cr_008` | `L1SupplyChain` |
| `CR-009` | `static_surface::trace_cr_009` | `L1Static` |
| `CR-010` | `test_gate::trace_cr_010` | `L1SupplyChain` |
| `CR-011` | `static_surface::trace_cr_011` | `L1Static` |
| `CR-012` | `test_gate::trace_cr_012` | `L1SupplyChain` |
| `CR-013` | `static_surface::trace_cr_013` | `L1Static` |
| `CR-014` | `test_gate::trace_cr_014` | `L1SupplyChain` |
| `CR-015` | `static_surface::trace_cr_015` | `L1Static` |
| `CR-016` | `test_gate::trace_cr_016` | `L1SupplyChain` |
| `CR-017` | `static_surface::trace_cr_017` | `L1Static` |
| `CR-018` | `test_gate::trace_cr_018` | `L1SupplyChain` |
| `CR-019` | `static_surface::trace_cr_019` | `L1Static` |
| `CR-020` | `test_gate::trace_cr_020` | `L1SupplyChain` |
| `CR-021` | `static_surface::trace_cr_021` | `L1Static` |
| `CR-022` | `test_gate::trace_cr_022` | `L1SupplyChain` |
| `CR-023` | `static_surface::trace_cr_023` | `L1Static` |
| `CR-024` | `test_gate::trace_cr_024` | `L1SupplyChain` |
| `CR-025` | `static_surface::trace_cr_025` | `L1Static` |
| `CR-026` | `test_gate::trace_cr_026` | `L1SupplyChain` |
| `CR-027` | `static_surface::trace_cr_027` | `L1Static` |
| `CR-028` | `test_gate::trace_cr_028` | `L1SupplyChain` |
| `CR-029` | `static_surface::trace_cr_029` | `L1Static` |
| `CR-030` | `test_gate::trace_cr_030` | `L1SupplyChain` |
| `CR-031` | `static_surface::trace_cr_031` | `L1Static` |
| `CR-032` | `test_gate::trace_cr_032` | `L1SupplyChain` |
| `CR-033` | `static_surface::trace_cr_033` | `L1Static` |
| `CR-034` | `test_gate::trace_cr_034` | `L1SupplyChain` |
| `CR-035` | `static_surface::trace_cr_035` | `L1Static` |
| `CR-036` | `test_gate::trace_cr_036` | `L1SupplyChain` |
| `CR-037` | `static_surface::trace_cr_037` | `L1Static` |
| `CR-038` | `test_gate::trace_cr_038` | `L1SupplyChain` |
| `CR-039` | `static_surface::trace_cr_039` | `L1Static` |
| `CR-040` | `test_gate::trace_cr_040` | `L1SupplyChain` |
| `VCL-01` | `source_spine::trace_vcl_01` | `L2SourceSpine` |
| `VCL-02` | `contract_matrix::trace_vcl_02` | `L2Contract` |
| `VCL-03` | `model_cases::trace_vcl_03` | `L2Model` |
| `VCL-04` | `property_cases::trace_vcl_04` | `L2Property` |
| `VCL-05` | `hold_closure::trace_vcl_05` | `L2HoldClosure` |
| `VCL-06` | `adversarial_cases::trace_vcl_06` | `L2Adversarial` |
| `VCL-07` | `no_authority_surface::trace_vcl_07` | `L2NoAuthority` |
| `VCL-08` | `no_authority_surface::trace_vcl_08` | `L2NoAuthority` |
| `VCL-09` | `static_surface::trace_vcl_09` | `L1Static` |
| `VCL-10` | `source_spine::trace_vcl_10` | `L2SourceSpine` |
| `VAL-SCOPE` | `source_spine::trace_val_scope` | `L2SourceSpine` |
| `VAL-ASSURANCE` | `hold_closure::trace_val_assurance` | `L2HoldClosure` |
| `ACT-CIV` | `no_authority_surface::trace_act_civ` | `L2NoAuthority` |
| `ACT-RDY` | `source_spine::trace_act_rdy` | `L2SourceSpine` |
| `ACT-ACQ` | `source_spine::trace_act_acq` | `L2SourceSpine` |
| `ACT-LOG` | `source_spine::trace_act_log` | `L2SourceSpine` |
| `ACT-ALLY` | `source_spine::trace_act_ally` | `L2SourceSpine` |
| `ACT-FIN` | `source_spine::trace_act_fin` | `L2SourceSpine` |
| `ACT-PPL` | `source_spine::trace_act_ppl` | `L2SourceSpine` |
| `ACT-TST` | `source_spine::trace_act_tst` | `L2SourceSpine` |
| `ACT-SRC` | `contract_matrix::trace_act_src` | `L2Contract` |
| `ACT-LAW` | `no_authority_surface::trace_act_law` | `L2NoAuthority` |
| `ACT-EXT` | `no_authority_surface::trace_act_ext` | `L2NoAuthority` |
| `.roles/parliament/civilian-strategy-force-planner.md` | `no_authority_surface::trace_role_parliament_civilian_strategy_force_planner` | `L2NoAuthority` |
| `.roles/parliament/operational-readiness.md` | `source_spine::trace_role_parliament_operational_readiness` | `L2SourceSpine` |
| `.roles/parliament/acquisition-industrial-base.md` | `source_spine::trace_role_parliament_acquisition_industrial_base` | `L2SourceSpine` |
| `.roles/parliament/logistics-sustainment.md` | `source_spine::trace_role_parliament_logistics_sustainment` | `L2SourceSpine` |
| `.roles/parliament/defense-comptroller.md` | `source_spine::trace_role_parliament_defense_comptroller` | `L2SourceSpine` |
| `.roles/parliament/service-member-family.md` | `source_spine::trace_role_parliament_service_member_family` | `L2SourceSpine` |
| `.roles/parliament/independent-test-oversight.md` | `source_spine::trace_role_parliament_independent_test_oversight` | `L2SourceSpine` |
| `.roles/parliament/alliance-interoperability.md` | `source_spine::trace_role_parliament_alliance_interoperability` | `L2SourceSpine` |
| `.roles/panel-reviewer/panel.md` | `property_cases::trace_role_panel_reviewer_panel` | `L2Property` |
| `Role review steward` | `hold_closure::trace_role_review_steward` | `L2HoldClosure` |
| `.roles/editorial/citation-auditor.md` | `source_spine::trace_role_editorial_citation_auditor` | `L2SourceSpine` |
| `.roles/editorial/scope-keeper.md` | `no_authority_surface::trace_role_editorial_scope_keeper` | `L2NoAuthority` |
| `.roles/editorial/numeracy-checker.md` | `property_cases::trace_role_editorial_numeracy_checker` | `L2Property` |
| `.roles/stakeholders/service-member-family.md` | `no_authority_surface::trace_role_stakeholders_service_member_family` | `L2NoAuthority` |
| `.roles/stakeholders/mission-user.md` | `no_authority_surface::trace_role_stakeholders_mission_user` | `L2NoAuthority` |
| `.roles/stakeholders/depot-logistics-workforce.md` | `no_authority_surface::trace_role_stakeholders_depot_logistics_workforce` | `L2NoAuthority` |
| `.roles/stakeholders/prime-small-supplier.md` | `no_authority_surface::trace_role_stakeholders_prime_small_supplier` | `L2NoAuthority` |
| `.roles/stakeholders/installation-community.md` | `no_authority_surface::trace_role_stakeholders_installation_community` | `L2NoAuthority` |
| `.roles/stakeholders/ally-partner.md` | `no_authority_surface::trace_role_stakeholders_ally_partner` | `L2NoAuthority` |
| `.roles/stakeholders/taxpayer-oversight.md` | `no_authority_surface::trace_role_stakeholders_taxpayer_oversight` | `L2NoAuthority` |
| `.roles/assurance/classification-operational-security.md` | `adversarial_cases::trace_role_assurance_classification_operational_security` | `L2Adversarial` |
| `.roles/assurance/civilian-control-law-safety-readiness.md` | `no_authority_surface::trace_role_assurance_civilian_control_law_safety_readiness` | `L2NoAuthority` |
| `SPEC-UNK-SEC-001` | `hold_closure::trace_spec_unk_sec_001` | `L2HoldClosure` |
| `TBD-SEC-001` | `hold_closure::trace_tbd_sec_001` | `L2HoldClosure` |
| `SPEC-UNK-SRC-001` | `hold_closure::trace_spec_unk_src_001` | `L2HoldClosure` |
| `TBD-SRC-001` | `hold_closure::trace_tbd_src_001` | `L2HoldClosure` |
| `SPEC-UNK-TST-001` | `hold_closure::trace_spec_unk_tst_001` | `L2HoldClosure` |
| `TBD-TST-001` | `hold_closure::trace_tbd_tst_001` | `L2HoldClosure` |
| `SPEC-UNK-REL-001` | `hold_closure::trace_spec_unk_rel_001` | `L2HoldClosure` |
| `TBD-REL-001` | `hold_closure::trace_tbd_rel_001` | `L2HoldClosure` |

The forward identity count is mechanically derived as
`9 requirements + 19 specifications + 4 design/contract identities + 38 CR
identities + 10 VCL identities + 2 validation identities + 11 actor identities
+ 22 lane identities + 8 separate unknown/TBD identities = 123`. The 38 CR
rows are exactly `CR-002` through `CR-006` and `CR-008` through `CR-040`,
enumerated above; `CR-001` and `CR-007` are absent. R2's reviewer-side
59-ID presence check was only a partial sampling assertion, is retired, and
confers no completeness claim or authority in R3.

### 7.2 Exact canonical reverse transpose

The reverse representation has one row per consuming edge and repeats each
canonical identity and target/assertion verbatim. Because every forward identity
has exactly one consuming mode, the mechanically derived edge count is exactly
123. Per-mode counts are `L1Static`=20, `L1SupplyChain`=19, `L2SourceSpine`=21, `L2Contract`=7, `L2Property`=8, `L2Model`=5, `L2Adversarial`=7, `L2HoldClosure`=13, `L2NoAuthority`=23.

| Evidence mode | Canonical controlled identity | Exact target / assertion |
|---|---|---|
| `L1Static` | `CR-002` | `static_surface::trace_cr_002` |
| `L1Static` | `CR-004` | `static_surface::trace_cr_004` |
| `L1Static` | `CR-006` | `static_surface::trace_cr_006` |
| `L1Static` | `CR-009` | `static_surface::trace_cr_009` |
| `L1Static` | `CR-011` | `static_surface::trace_cr_011` |
| `L1Static` | `CR-013` | `static_surface::trace_cr_013` |
| `L1Static` | `CR-015` | `static_surface::trace_cr_015` |
| `L1Static` | `CR-017` | `static_surface::trace_cr_017` |
| `L1Static` | `CR-019` | `static_surface::trace_cr_019` |
| `L1Static` | `CR-021` | `static_surface::trace_cr_021` |
| `L1Static` | `CR-023` | `static_surface::trace_cr_023` |
| `L1Static` | `CR-025` | `static_surface::trace_cr_025` |
| `L1Static` | `CR-027` | `static_surface::trace_cr_027` |
| `L1Static` | `CR-029` | `static_surface::trace_cr_029` |
| `L1Static` | `CR-031` | `static_surface::trace_cr_031` |
| `L1Static` | `CR-033` | `static_surface::trace_cr_033` |
| `L1Static` | `CR-035` | `static_surface::trace_cr_035` |
| `L1Static` | `CR-037` | `static_surface::trace_cr_037` |
| `L1Static` | `CR-039` | `static_surface::trace_cr_039` |
| `L1Static` | `VCL-09` | `static_surface::trace_vcl_09` |
| `L1SupplyChain` | `CR-003` | `test_gate::trace_cr_003` |
| `L1SupplyChain` | `CR-005` | `test_gate::trace_cr_005` |
| `L1SupplyChain` | `CR-008` | `test_gate::trace_cr_008` |
| `L1SupplyChain` | `CR-010` | `test_gate::trace_cr_010` |
| `L1SupplyChain` | `CR-012` | `test_gate::trace_cr_012` |
| `L1SupplyChain` | `CR-014` | `test_gate::trace_cr_014` |
| `L1SupplyChain` | `CR-016` | `test_gate::trace_cr_016` |
| `L1SupplyChain` | `CR-018` | `test_gate::trace_cr_018` |
| `L1SupplyChain` | `CR-020` | `test_gate::trace_cr_020` |
| `L1SupplyChain` | `CR-022` | `test_gate::trace_cr_022` |
| `L1SupplyChain` | `CR-024` | `test_gate::trace_cr_024` |
| `L1SupplyChain` | `CR-026` | `test_gate::trace_cr_026` |
| `L1SupplyChain` | `CR-028` | `test_gate::trace_cr_028` |
| `L1SupplyChain` | `CR-030` | `test_gate::trace_cr_030` |
| `L1SupplyChain` | `CR-032` | `test_gate::trace_cr_032` |
| `L1SupplyChain` | `CR-034` | `test_gate::trace_cr_034` |
| `L1SupplyChain` | `CR-036` | `test_gate::trace_cr_036` |
| `L1SupplyChain` | `CR-038` | `test_gate::trace_cr_038` |
| `L1SupplyChain` | `CR-040` | `test_gate::trace_cr_040` |
| `L2SourceSpine` | `BASTION-REQ-TST-001` | `source_spine::trace_bastion_req_tst_001` |
| `L2SourceSpine` | `SPEC-TST-001` | `source_spine::trace_spec_tst_001` |
| `L2SourceSpine` | `SPEC-NF-010` | `source_spine::trace_spec_nf_010` |
| `L2SourceSpine` | `VCL-01` | `source_spine::trace_vcl_01` |
| `L2SourceSpine` | `VCL-10` | `source_spine::trace_vcl_10` |
| `L2SourceSpine` | `VAL-SCOPE` | `source_spine::trace_val_scope` |
| `L2SourceSpine` | `ACT-RDY` | `source_spine::trace_act_rdy` |
| `L2SourceSpine` | `ACT-ACQ` | `source_spine::trace_act_acq` |
| `L2SourceSpine` | `ACT-LOG` | `source_spine::trace_act_log` |
| `L2SourceSpine` | `ACT-ALLY` | `source_spine::trace_act_ally` |
| `L2SourceSpine` | `ACT-FIN` | `source_spine::trace_act_fin` |
| `L2SourceSpine` | `ACT-PPL` | `source_spine::trace_act_ppl` |
| `L2SourceSpine` | `ACT-TST` | `source_spine::trace_act_tst` |
| `L2SourceSpine` | `.roles/parliament/operational-readiness.md` | `source_spine::trace_role_parliament_operational_readiness` |
| `L2SourceSpine` | `.roles/parliament/acquisition-industrial-base.md` | `source_spine::trace_role_parliament_acquisition_industrial_base` |
| `L2SourceSpine` | `.roles/parliament/logistics-sustainment.md` | `source_spine::trace_role_parliament_logistics_sustainment` |
| `L2SourceSpine` | `.roles/parliament/defense-comptroller.md` | `source_spine::trace_role_parliament_defense_comptroller` |
| `L2SourceSpine` | `.roles/parliament/service-member-family.md` | `source_spine::trace_role_parliament_service_member_family` |
| `L2SourceSpine` | `.roles/parliament/independent-test-oversight.md` | `source_spine::trace_role_parliament_independent_test_oversight` |
| `L2SourceSpine` | `.roles/parliament/alliance-interoperability.md` | `source_spine::trace_role_parliament_alliance_interoperability` |
| `L2SourceSpine` | `.roles/editorial/citation-auditor.md` | `source_spine::trace_role_editorial_citation_auditor` |
| `L2Contract` | `BASTION-REQ-TST-004` | `contract_matrix::trace_bastion_req_tst_004` |
| `L2Contract` | `SPEC-TST-004` | `contract_matrix::trace_spec_tst_004` |
| `L2Contract` | `SPEC-NF-008` | `contract_matrix::trace_spec_nf_008` |
| `L2Contract` | `DES-TEST-001` | `contract_matrix::trace_des_test_001` |
| `L2Contract` | `CONTRACT-TEST-001` | `contract_matrix::trace_contract_test_001` |
| `L2Contract` | `VCL-02` | `contract_matrix::trace_vcl_02` |
| `L2Contract` | `ACT-SRC` | `contract_matrix::trace_act_src` |
| `L2Property` | `BASTION-REQ-TST-002` | `property_cases::trace_bastion_req_tst_002` |
| `L2Property` | `SPEC-TST-002` | `property_cases::trace_spec_tst_002` |
| `L2Property` | `SPEC-NF-004` | `property_cases::trace_spec_nf_004` |
| `L2Property` | `SPEC-NF-005` | `property_cases::trace_spec_nf_005` |
| `L2Property` | `SPEC-NF-007` | `property_cases::trace_spec_nf_007` |
| `L2Property` | `VCL-04` | `property_cases::trace_vcl_04` |
| `L2Property` | `.roles/panel-reviewer/panel.md` | `property_cases::trace_role_panel_reviewer_panel` |
| `L2Property` | `.roles/editorial/numeracy-checker.md` | `property_cases::trace_role_editorial_numeracy_checker` |
| `L2Model` | `BASTION-REQ-TST-003` | `model_cases::trace_bastion_req_tst_003` |
| `L2Model` | `SPEC-TST-003` | `model_cases::trace_spec_tst_003` |
| `L2Model` | `SPEC-NF-006` | `model_cases::trace_spec_nf_006` |
| `L2Model` | `SPEC-NF-009` | `model_cases::trace_spec_nf_009` |
| `L2Model` | `VCL-03` | `model_cases::trace_vcl_03` |
| `L2Adversarial` | `BASTION-REQ-TST-006` | `adversarial_cases::trace_bastion_req_tst_006` |
| `L2Adversarial` | `BASTION-REQ-REL-002` | `adversarial_cases::trace_bastion_req_rel_002` |
| `L2Adversarial` | `SPEC-TST-006` | `adversarial_cases::trace_spec_tst_006` |
| `L2Adversarial` | `SPEC-REL-002` | `adversarial_cases::trace_spec_rel_002` |
| `L2Adversarial` | `SPEC-NF-001` | `adversarial_cases::trace_spec_nf_001` |
| `L2Adversarial` | `VCL-06` | `adversarial_cases::trace_vcl_06` |
| `L2Adversarial` | `.roles/assurance/classification-operational-security.md` | `adversarial_cases::trace_role_assurance_classification_operational_security` |
| `L2HoldClosure` | `BASTION-REQ-TST-005` | `hold_closure::trace_bastion_req_tst_005` |
| `L2HoldClosure` | `SPEC-TST-005` | `hold_closure::trace_spec_tst_005` |
| `L2HoldClosure` | `VCL-05` | `hold_closure::trace_vcl_05` |
| `L2HoldClosure` | `VAL-ASSURANCE` | `hold_closure::trace_val_assurance` |
| `L2HoldClosure` | `Role review steward` | `hold_closure::trace_role_review_steward` |
| `L2HoldClosure` | `SPEC-UNK-SEC-001` | `hold_closure::trace_spec_unk_sec_001` |
| `L2HoldClosure` | `TBD-SEC-001` | `hold_closure::trace_tbd_sec_001` |
| `L2HoldClosure` | `SPEC-UNK-SRC-001` | `hold_closure::trace_spec_unk_src_001` |
| `L2HoldClosure` | `TBD-SRC-001` | `hold_closure::trace_tbd_src_001` |
| `L2HoldClosure` | `SPEC-UNK-TST-001` | `hold_closure::trace_spec_unk_tst_001` |
| `L2HoldClosure` | `TBD-TST-001` | `hold_closure::trace_tbd_tst_001` |
| `L2HoldClosure` | `SPEC-UNK-REL-001` | `hold_closure::trace_spec_unk_rel_001` |
| `L2HoldClosure` | `TBD-REL-001` | `hold_closure::trace_tbd_rel_001` |
| `L2NoAuthority` | `BASTION-REQ-REL-001` | `no_authority_surface::trace_bastion_req_rel_001` |
| `L2NoAuthority` | `BASTION-REQ-REL-003` | `no_authority_surface::trace_bastion_req_rel_003` |
| `L2NoAuthority` | `SPEC-REL-001` | `no_authority_surface::trace_spec_rel_001` |
| `L2NoAuthority` | `SPEC-REL-003` | `no_authority_surface::trace_spec_rel_003` |
| `L2NoAuthority` | `SPEC-NF-002` | `no_authority_surface::trace_spec_nf_002` |
| `L2NoAuthority` | `SPEC-NF-003` | `no_authority_surface::trace_spec_nf_003` |
| `L2NoAuthority` | `DES-REL-001` | `no_authority_surface::trace_des_rel_001` |
| `L2NoAuthority` | `CONTRACT-REL-001` | `no_authority_surface::trace_contract_rel_001` |
| `L2NoAuthority` | `VCL-07` | `no_authority_surface::trace_vcl_07` |
| `L2NoAuthority` | `VCL-08` | `no_authority_surface::trace_vcl_08` |
| `L2NoAuthority` | `ACT-CIV` | `no_authority_surface::trace_act_civ` |
| `L2NoAuthority` | `ACT-LAW` | `no_authority_surface::trace_act_law` |
| `L2NoAuthority` | `ACT-EXT` | `no_authority_surface::trace_act_ext` |
| `L2NoAuthority` | `.roles/parliament/civilian-strategy-force-planner.md` | `no_authority_surface::trace_role_parliament_civilian_strategy_force_planner` |
| `L2NoAuthority` | `.roles/editorial/scope-keeper.md` | `no_authority_surface::trace_role_editorial_scope_keeper` |
| `L2NoAuthority` | `.roles/stakeholders/service-member-family.md` | `no_authority_surface::trace_role_stakeholders_service_member_family` |
| `L2NoAuthority` | `.roles/stakeholders/mission-user.md` | `no_authority_surface::trace_role_stakeholders_mission_user` |
| `L2NoAuthority` | `.roles/stakeholders/depot-logistics-workforce.md` | `no_authority_surface::trace_role_stakeholders_depot_logistics_workforce` |
| `L2NoAuthority` | `.roles/stakeholders/prime-small-supplier.md` | `no_authority_surface::trace_role_stakeholders_prime_small_supplier` |
| `L2NoAuthority` | `.roles/stakeholders/installation-community.md` | `no_authority_surface::trace_role_stakeholders_installation_community` |
| `L2NoAuthority` | `.roles/stakeholders/ally-partner.md` | `no_authority_surface::trace_role_stakeholders_ally_partner` |
| `L2NoAuthority` | `.roles/stakeholders/taxpayer-oversight.md` | `no_authority_surface::trace_role_stakeholders_taxpayer_oversight` |
| `L2NoAuthority` | `.roles/assurance/civilian-control-law-safety-readiness.md` | `no_authority_surface::trace_role_assurance_civilian_control_law_safety_readiness` |

`source_spine` compares the exact 123 forward identities and edges against
these exact 123 reverse identities and edges after bytewise sorting. It rejects
an absent/orphan row, unlisted extra, alias, range, shorthand, paired cell,
duplicate identity, duplicate edge, count error, controlled-source digest
mismatch, target/assertion mismatch, mode mismatch, spelling change, or
non-transpose. `L1Static` and `L1SupplyChain` execute and retain their exact
reverse CR/VCL edges; the transpose is not limited to L2 modes.

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
entry decision. Entry requires the exact accepted R3 and acceptance-pulse
digests on current main. A clean isolated worktree starts at that acceptance;
the entry commit is its direct child, and the one atomic implementation commit
is the direct child of entry. Entry binds both commit IDs before execution,
the exact allowlist, unchanged REV bytes, and no unrelated change. WS remains
the sole logical WP predecessor; current-main ancestry and co-membership do not
create another WP predecessor.

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
fixtures, and runner and restores the accepted entry root manifest and lock
bytes, including the unchanged REV co-member. It
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
