# WP-TST-001-R6 — closed-schema boundary-test and fixture-custody bootstrap

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundaries: `PB-TST-001` and assigned `PB-FIX-001`, plus
configuration-only membership integration in `PB-WS-001`

Logical WP predecessor: accepted `WP-WS-001` exit only. R1 commit
`62116481b7b3e7d671517b6053c8cc3f20f93fce` and R2 commit
`21c8066445c72358a444c0b506422ec3b9dc63e0` are retained governance history.
After R6 acceptance, the entry commit and its direct implementation successor
must remain on the current governance/main lineage. Accepted REV is only a
context co-member: workspace co-membership and Git ancestry are explicitly not
WP-predecessor or dependency relationships.

## 1. Controlled baseline and custody

The future acceptance commit must descend from R6 on current main; the entry
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
| Retained R3 governance commit (not accepted) | `ae64448e98744668e3b80e3411255503bfbdd4ae` |
| Retained R3 WP SHA-256 / blob | `76f259e3189cbb53be5e88b84dc922a13673ec52572efbe842f55fe85a67c2ae` / `655f38734b4f52ed7ff740fd2117c3cd5916f977` |
| Retained R4 governance commit (not accepted) | `b919512fb73472149afea5a55d1a022bf6aec8da` |
| Retained R4 WP SHA-256 / blob | `eaff0bd15d34afb533306ab5a4a967cb672149422e14b634ae263fea512f4f70` / `18e616868d9f94b97264e4b744961d85b6442f3d` |
| Retained R5 governance commit (not accepted) | `77e0abb94a427a1f824e4f5659e580b1aae74137` |
| Retained R5 WP SHA-256 / blob | `c618af6d61d05c51fe689a791f7a8bc9f2ed908c4c42e7e48dd07badec2a633d` / `42f8ff4bd0e9350ac269b0a3a137209b1be1f120` |
| Current R6 governance-line base before acceptance | `77e0abb94a427a1f824e4f5659e580b1aae74137` |
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
| `CMD-L1-SUPPLY-CHAIN` | `L1SupplyChain` | `cargo +1.95.0 metadata --locked --offline --no-deps --format-version 1`, then assert exactly `bastion-boundary-tests` and unchanged `bastion-review`, both zero-dependency nodes, zero graph edges, invariant TST after in-memory REV-node removal, zero third-party license subjects, and no registry/git/advisory dependency surface |
| `CMD-L2-SOURCE-SPINE` | `L2SourceSpine` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test source_spine` |
| `CMD-L2-CONTRACT-MATRIX` | `L2Contract` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test contract_matrix` |
| `CMD-L2-PROPERTY` | `L2Property` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test property_cases` |
| `CMD-L2-MODEL` | `L2Model` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test model_cases` |
| `CMD-L2-ADVERSARIAL` | `L2Adversarial` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test adversarial_cases` |
| `CMD-L2-HOLD-CLOSURE` | `L2HoldClosure` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test hold_closure` |
| `CMD-L2-NO-EMISSION` | `L2NoAuthority` | `cargo +1.95.0 test -p bastion-boundary-tests --locked --offline --test no_authority_surface` |

After all 16 mode records exist, the supervisor's non-mode set-assembly command
is identity `CMD-TST-EVIDENCE-SET` with exact argv
`pwsh -NoLogo -NoProfile -NonInteractive -File tools/test_gate.ps1 -AssembleSet`.
It executes no Cargo target and only validates, projects, hashes, and creates the
next unused set record under the same wall/memory/combined-stream bounds.

Every mode has a 60-second wall limit, 1 GiB process-tree memory limit, and
10 MiB combined stdout/stderr limit. The runner itself performs no network
access and all Cargo commands are `--locked --offline`. It binds and verifies
one implementation commit, implementation/test/fixture-manifest/WP/
acceptance/runner/root-manifest/lock/WS predecessor digest, exact argv,
sanitized-environment digest, start/end/duration, bounds, per-command exit and
stream hashes/bytes, combined bytes, assertions, executed case target, and
result in canonical `test-gate-evidence.v3` JSON. All 16 modes must pass at
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

### 7.1 Exact canonical forward edges

`support/mod.rs` contains one canonical, bytewise-sorted trace manifest. Each
row below is one edge with exactly one controlled identity, one assertion that
actually executes, and one consuming evidence mode. An identity repeats only
when distinct modes execute distinct allocated obligations. Paired cells,
aliases, ranges, shorthand, count-driven assignment, and names ending in
`bootstrap` are invalid. Every edge binds the exact controlled-source digest,
and every evidence record binds the trace-manifest SHA-256.

| Canonical controlled identity | Exact executed target / assertion | Evidence mode |
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
| `CR-002` | `contract_matrix::cr_002_logical_contract` | `L2Contract` |
| `CR-002` | `source_spine::cr_002_logical_responsibility` | `L2SourceSpine` |
| `CR-003` | `adversarial_cases::cr_003_typed_failure_rejection` | `L2Adversarial` |
| `CR-003` | `contract_matrix::cr_003_typed_branch_totality` | `L2Contract` |
| `CR-004` | `adversarial_cases::cr_004_exhaustion_failure` | `L2Adversarial` |
| `CR-004` | `property_cases::cr_004_finite_bounds_progress` | `L2Property` |
| `CR-005` | `static_surface::cr_005_call_graph_depth` | `L1Static` |
| `CR-005` | `adversarial_cases::cr_005_termination_violation` | `L2Adversarial` |
| `CR-006` | `adversarial_cases::cr_006_hidden_failure_scan` | `L2Adversarial` |
| `CR-006` | `model_cases::cr_006_invalid_state` | `L2Model` |
| `CR-008` | `adversarial_cases::cr_008_default_fallback_rejection` | `L2Adversarial` |
| `CR-008` | `hold_closure::cr_008_missing_default_hold` | `L2HoldClosure` |
| `CR-009` | `contract_matrix::cr_009_typed_family_exhaustiveness` | `L2Contract` |
| `CR-009` | `model_cases::cr_009_typed_state_exhaustiveness` | `L2Model` |
| `CR-010` | `no_authority_surface::cr_010_release_exception_no_output` | `L2NoAuthority` |
| `CR-010` | `property_cases::cr_010_universal_admission_bypass` | `L2Property` |
| `CR-011` | `model_cases::cr_011_replay_identity` | `L2Model` |
| `CR-011` | `property_cases::cr_011_order_invariance` | `L2Property` |
| `CR-011` | `source_spine::cr_011_digest_reproduction_order` | `L2SourceSpine` |
| `CR-012` | `static_surface::cr_012_ambient_state_absence` | `L1Static` |
| `CR-012` | `property_cases::cr_012_schedule_equivalence` | `L2Property` |
| `CR-013` | `model_cases::cr_013_immutable_successor_acyclic` | `L2Model` |
| `CR-014` | `static_surface::cr_014_consumer_direction` | `L1Static` |
| `CR-014` | `test_gate::cr_014_fixed_dependency_graph` | `L1SupplyChain` |
| `CR-015` | `adversarial_cases::cr_015_prohibited_content` | `L2Adversarial` |
| `CR-015` | `contract_matrix::cr_015_content_boundary_provenance` | `L2Contract` |
| `CR-016` | `adversarial_cases::cr_016_composition_minimization` | `L2Adversarial` |
| `CR-017` | `adversarial_cases::cr_017_floor_noncompensation` | `L2Adversarial` |
| `CR-017` | `no_authority_surface::cr_017_authority_noninflation` | `L2NoAuthority` |
| `CR-018` | `property_cases::cr_018_facet_distribution_conservation` | `L2Property` |
| `CR-019` | `hold_closure::cr_019_missing_null_hold` | `L2HoldClosure` |
| `CR-019` | `model_cases::cr_019_state_null_na_stale` | `L2Model` |
| `CR-020` | `model_cases::cr_020_checked_accounting` | `L2Model` |
| `CR-020` | `property_cases::cr_020_reconciliation_identity` | `L2Property` |
| `CR-021` | `adversarial_cases::cr_021_burden_shift_rejection` | `L2Adversarial` |
| `CR-021` | `no_authority_surface::cr_021_false_savings_no_authority` | `L2NoAuthority` |
| `CR-022` | `model_cases::cr_022_eco_delivery_adaptive_shape` | `L2Model` |
| `CR-023` | `hold_closure::cr_023_finding_dissent_retention` | `L2HoldClosure` |
| `CR-023` | `source_spine::cr_023_review_independence` | `L2SourceSpine` |
| `CR-024` | `no_authority_surface::cr_024_terminal_no_output_backflow` | `L2NoAuthority` |
| `CR-025` | `hold_closure::cr_025_hold_transpose_propagation` | `L2HoldClosure` |
| `CR-026` | `source_spine::cr_026_invariant_coverage` | `L2SourceSpine` |
| `CR-027` | `property_cases::cr_027_property_evidence_set` | `L2Property` |
| `CR-028` | `model_cases::cr_028_transition_model_evidence` | `L2Model` |
| `CR-029` | `adversarial_cases::cr_029_cross_role_adversarial` | `L2Adversarial` |
| `CR-030` | `contract_matrix::cr_030_per_contract_fixture_matrix` | `L2Contract` |
| `CR-031` | `static_surface::cr_031_parser_surface_absent` | `L1Static` |
| `CR-031` | `adversarial_cases::cr_031_parser_fuzz_authority_absent` | `L2Adversarial` |
| `CR-032` | `model_cases::cr_032_golden_successor_history` | `L2Model` |
| `CR-032` | `property_cases::cr_032_regression_replay` | `L2Property` |
| `CR-033` | `static_surface::cr_033_mode_isolation` | `L1Static` |
| `CR-033` | `test_gate::cr_033_package_isolation` | `L1SupplyChain` |
| `CR-034` | `no_authority_surface::cr_034_generated_no_emission` | `L2NoAuthority` |
| `CR-034` | `source_spine::cr_034_generated_provenance_absence` | `L2SourceSpine` |
| `CR-035` | `static_surface::cr_035_quality_gate_registry` | `L1Static` |
| `CR-035` | `source_spine::cr_035_quality_output_binding` | `L2SourceSpine` |
| `CR-036` | `test_gate::cr_036_dependency_license_advisory` | `L1SupplyChain` |
| `CR-037` | `static_surface::cr_037_resource_bound_registry` | `L1Static` |
| `CR-037` | `adversarial_cases::cr_037_resource_bound_failure` | `L2Adversarial` |
| `CR-038` | `hold_closure::cr_038_waiver_ledger_nonwaiver` | `L2HoldClosure` |
| `CR-039` | `hold_closure::cr_039_evidence_state_history` | `L2HoldClosure` |
| `CR-039` | `source_spine::cr_039_evidence_digest_truth` | `L2SourceSpine` |
| `CR-040` | `source_spine::cr_040_mechanical_trace_contradiction` | `L2SourceSpine` |
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

The unique-identity count remains mechanically derived as
`9 requirements + 19 specifications + 4 design/contract identities + 38 CR
identities + 10 VCL identities + 2 validation identities + 11 actor identities
+ 22 lane identities + 8 separate unknown/TBD identities = 123`. The 38 CR
identities are explicitly `CR-002`–`CR-006` and `CR-008`–`CR-040`;
`CR-001` and `CR-007` are not allocated to this WP. Their 63
edges are derived from each exact `CODE_RIGOR.md` planned-evidence obligation,
not alternated for count. Total forward edges are 148.

### 7.2 Exact CR command/evidence target lists

Each listed assertion is part of the named mode's executed assertion set and is
reported in that mode's `executed_targets` evidence field. A mode may claim no
other CR and may not omit one listed here.

| Evidence mode | Exact CR -> executed assertion list |
|---|---|
| `L1Static` | `CR-005` -> `static_surface::cr_005_call_graph_depth`; `CR-012` -> `static_surface::cr_012_ambient_state_absence`; `CR-014` -> `static_surface::cr_014_consumer_direction`; `CR-031` -> `static_surface::cr_031_parser_surface_absent`; `CR-033` -> `static_surface::cr_033_mode_isolation`; `CR-035` -> `static_surface::cr_035_quality_gate_registry`; `CR-037` -> `static_surface::cr_037_resource_bound_registry` |
| `L1SupplyChain` | `CR-014` -> `test_gate::cr_014_fixed_dependency_graph`; `CR-033` -> `test_gate::cr_033_package_isolation`; `CR-036` -> `test_gate::cr_036_dependency_license_advisory` |
| `L2SourceSpine` | `CR-002` -> `source_spine::cr_002_logical_responsibility`; `CR-011` -> `source_spine::cr_011_digest_reproduction_order`; `CR-023` -> `source_spine::cr_023_review_independence`; `CR-026` -> `source_spine::cr_026_invariant_coverage`; `CR-034` -> `source_spine::cr_034_generated_provenance_absence`; `CR-035` -> `source_spine::cr_035_quality_output_binding`; `CR-039` -> `source_spine::cr_039_evidence_digest_truth`; `CR-040` -> `source_spine::cr_040_mechanical_trace_contradiction` |
| `L2Contract` | `CR-002` -> `contract_matrix::cr_002_logical_contract`; `CR-003` -> `contract_matrix::cr_003_typed_branch_totality`; `CR-009` -> `contract_matrix::cr_009_typed_family_exhaustiveness`; `CR-015` -> `contract_matrix::cr_015_content_boundary_provenance`; `CR-030` -> `contract_matrix::cr_030_per_contract_fixture_matrix` |
| `L2Property` | `CR-004` -> `property_cases::cr_004_finite_bounds_progress`; `CR-010` -> `property_cases::cr_010_universal_admission_bypass`; `CR-011` -> `property_cases::cr_011_order_invariance`; `CR-012` -> `property_cases::cr_012_schedule_equivalence`; `CR-018` -> `property_cases::cr_018_facet_distribution_conservation`; `CR-020` -> `property_cases::cr_020_reconciliation_identity`; `CR-027` -> `property_cases::cr_027_property_evidence_set`; `CR-032` -> `property_cases::cr_032_regression_replay` |
| `L2Model` | `CR-006` -> `model_cases::cr_006_invalid_state`; `CR-009` -> `model_cases::cr_009_typed_state_exhaustiveness`; `CR-011` -> `model_cases::cr_011_replay_identity`; `CR-013` -> `model_cases::cr_013_immutable_successor_acyclic`; `CR-019` -> `model_cases::cr_019_state_null_na_stale`; `CR-020` -> `model_cases::cr_020_checked_accounting`; `CR-022` -> `model_cases::cr_022_eco_delivery_adaptive_shape`; `CR-028` -> `model_cases::cr_028_transition_model_evidence`; `CR-032` -> `model_cases::cr_032_golden_successor_history` |
| `L2Adversarial` | `CR-003` -> `adversarial_cases::cr_003_typed_failure_rejection`; `CR-004` -> `adversarial_cases::cr_004_exhaustion_failure`; `CR-005` -> `adversarial_cases::cr_005_termination_violation`; `CR-006` -> `adversarial_cases::cr_006_hidden_failure_scan`; `CR-008` -> `adversarial_cases::cr_008_default_fallback_rejection`; `CR-015` -> `adversarial_cases::cr_015_prohibited_content`; `CR-016` -> `adversarial_cases::cr_016_composition_minimization`; `CR-017` -> `adversarial_cases::cr_017_floor_noncompensation`; `CR-021` -> `adversarial_cases::cr_021_burden_shift_rejection`; `CR-029` -> `adversarial_cases::cr_029_cross_role_adversarial`; `CR-031` -> `adversarial_cases::cr_031_parser_fuzz_authority_absent`; `CR-037` -> `adversarial_cases::cr_037_resource_bound_failure` |
| `L2HoldClosure` | `CR-008` -> `hold_closure::cr_008_missing_default_hold`; `CR-019` -> `hold_closure::cr_019_missing_null_hold`; `CR-023` -> `hold_closure::cr_023_finding_dissent_retention`; `CR-025` -> `hold_closure::cr_025_hold_transpose_propagation`; `CR-038` -> `hold_closure::cr_038_waiver_ledger_nonwaiver`; `CR-039` -> `hold_closure::cr_039_evidence_state_history` |
| `L2NoAuthority` | `CR-010` -> `no_authority_surface::cr_010_release_exception_no_output`; `CR-017` -> `no_authority_surface::cr_017_authority_noninflation`; `CR-021` -> `no_authority_surface::cr_021_false_savings_no_authority`; `CR-024` -> `no_authority_surface::cr_024_terminal_no_output_backflow`; `CR-034` -> `no_authority_surface::cr_034_generated_no_emission` |

### 7.3 Exact canonical reverse transpose

The reverse representation has one row per consuming edge and repeats every
canonical identity and assertion verbatim. Its mechanically derived edge count
is exactly 148. Per-mode counts are `L1Static`=8, `L1SupplyChain`=3, `L2SourceSpine`=29, `L2Contract`=12, `L2Property`=16, `L2Model`=14, `L2Adversarial`=19, `L2HoldClosure`=19, `L2NoAuthority`=28.

| Evidence mode | Canonical controlled identity | Exact executed target / assertion |
|---|---|---|
| `L1Static` | `CR-005` | `static_surface::cr_005_call_graph_depth` |
| `L1Static` | `CR-012` | `static_surface::cr_012_ambient_state_absence` |
| `L1Static` | `CR-014` | `static_surface::cr_014_consumer_direction` |
| `L1Static` | `CR-031` | `static_surface::cr_031_parser_surface_absent` |
| `L1Static` | `CR-033` | `static_surface::cr_033_mode_isolation` |
| `L1Static` | `CR-035` | `static_surface::cr_035_quality_gate_registry` |
| `L1Static` | `CR-037` | `static_surface::cr_037_resource_bound_registry` |
| `L1Static` | `VCL-09` | `static_surface::trace_vcl_09` |
| `L1SupplyChain` | `CR-014` | `test_gate::cr_014_fixed_dependency_graph` |
| `L1SupplyChain` | `CR-033` | `test_gate::cr_033_package_isolation` |
| `L1SupplyChain` | `CR-036` | `test_gate::cr_036_dependency_license_advisory` |
| `L2SourceSpine` | `BASTION-REQ-TST-001` | `source_spine::trace_bastion_req_tst_001` |
| `L2SourceSpine` | `SPEC-TST-001` | `source_spine::trace_spec_tst_001` |
| `L2SourceSpine` | `SPEC-NF-010` | `source_spine::trace_spec_nf_010` |
| `L2SourceSpine` | `CR-002` | `source_spine::cr_002_logical_responsibility` |
| `L2SourceSpine` | `CR-011` | `source_spine::cr_011_digest_reproduction_order` |
| `L2SourceSpine` | `CR-023` | `source_spine::cr_023_review_independence` |
| `L2SourceSpine` | `CR-026` | `source_spine::cr_026_invariant_coverage` |
| `L2SourceSpine` | `CR-034` | `source_spine::cr_034_generated_provenance_absence` |
| `L2SourceSpine` | `CR-035` | `source_spine::cr_035_quality_output_binding` |
| `L2SourceSpine` | `CR-039` | `source_spine::cr_039_evidence_digest_truth` |
| `L2SourceSpine` | `CR-040` | `source_spine::cr_040_mechanical_trace_contradiction` |
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
| `L2Contract` | `CR-002` | `contract_matrix::cr_002_logical_contract` |
| `L2Contract` | `CR-003` | `contract_matrix::cr_003_typed_branch_totality` |
| `L2Contract` | `CR-009` | `contract_matrix::cr_009_typed_family_exhaustiveness` |
| `L2Contract` | `CR-015` | `contract_matrix::cr_015_content_boundary_provenance` |
| `L2Contract` | `CR-030` | `contract_matrix::cr_030_per_contract_fixture_matrix` |
| `L2Contract` | `VCL-02` | `contract_matrix::trace_vcl_02` |
| `L2Contract` | `ACT-SRC` | `contract_matrix::trace_act_src` |
| `L2Property` | `BASTION-REQ-TST-002` | `property_cases::trace_bastion_req_tst_002` |
| `L2Property` | `SPEC-TST-002` | `property_cases::trace_spec_tst_002` |
| `L2Property` | `SPEC-NF-004` | `property_cases::trace_spec_nf_004` |
| `L2Property` | `SPEC-NF-005` | `property_cases::trace_spec_nf_005` |
| `L2Property` | `SPEC-NF-007` | `property_cases::trace_spec_nf_007` |
| `L2Property` | `CR-004` | `property_cases::cr_004_finite_bounds_progress` |
| `L2Property` | `CR-010` | `property_cases::cr_010_universal_admission_bypass` |
| `L2Property` | `CR-011` | `property_cases::cr_011_order_invariance` |
| `L2Property` | `CR-012` | `property_cases::cr_012_schedule_equivalence` |
| `L2Property` | `CR-018` | `property_cases::cr_018_facet_distribution_conservation` |
| `L2Property` | `CR-020` | `property_cases::cr_020_reconciliation_identity` |
| `L2Property` | `CR-027` | `property_cases::cr_027_property_evidence_set` |
| `L2Property` | `CR-032` | `property_cases::cr_032_regression_replay` |
| `L2Property` | `VCL-04` | `property_cases::trace_vcl_04` |
| `L2Property` | `.roles/panel-reviewer/panel.md` | `property_cases::trace_role_panel_reviewer_panel` |
| `L2Property` | `.roles/editorial/numeracy-checker.md` | `property_cases::trace_role_editorial_numeracy_checker` |
| `L2Model` | `BASTION-REQ-TST-003` | `model_cases::trace_bastion_req_tst_003` |
| `L2Model` | `SPEC-TST-003` | `model_cases::trace_spec_tst_003` |
| `L2Model` | `SPEC-NF-006` | `model_cases::trace_spec_nf_006` |
| `L2Model` | `SPEC-NF-009` | `model_cases::trace_spec_nf_009` |
| `L2Model` | `CR-006` | `model_cases::cr_006_invalid_state` |
| `L2Model` | `CR-009` | `model_cases::cr_009_typed_state_exhaustiveness` |
| `L2Model` | `CR-011` | `model_cases::cr_011_replay_identity` |
| `L2Model` | `CR-013` | `model_cases::cr_013_immutable_successor_acyclic` |
| `L2Model` | `CR-019` | `model_cases::cr_019_state_null_na_stale` |
| `L2Model` | `CR-020` | `model_cases::cr_020_checked_accounting` |
| `L2Model` | `CR-022` | `model_cases::cr_022_eco_delivery_adaptive_shape` |
| `L2Model` | `CR-028` | `model_cases::cr_028_transition_model_evidence` |
| `L2Model` | `CR-032` | `model_cases::cr_032_golden_successor_history` |
| `L2Model` | `VCL-03` | `model_cases::trace_vcl_03` |
| `L2Adversarial` | `BASTION-REQ-TST-006` | `adversarial_cases::trace_bastion_req_tst_006` |
| `L2Adversarial` | `BASTION-REQ-REL-002` | `adversarial_cases::trace_bastion_req_rel_002` |
| `L2Adversarial` | `SPEC-TST-006` | `adversarial_cases::trace_spec_tst_006` |
| `L2Adversarial` | `SPEC-REL-002` | `adversarial_cases::trace_spec_rel_002` |
| `L2Adversarial` | `SPEC-NF-001` | `adversarial_cases::trace_spec_nf_001` |
| `L2Adversarial` | `CR-003` | `adversarial_cases::cr_003_typed_failure_rejection` |
| `L2Adversarial` | `CR-004` | `adversarial_cases::cr_004_exhaustion_failure` |
| `L2Adversarial` | `CR-005` | `adversarial_cases::cr_005_termination_violation` |
| `L2Adversarial` | `CR-006` | `adversarial_cases::cr_006_hidden_failure_scan` |
| `L2Adversarial` | `CR-008` | `adversarial_cases::cr_008_default_fallback_rejection` |
| `L2Adversarial` | `CR-015` | `adversarial_cases::cr_015_prohibited_content` |
| `L2Adversarial` | `CR-016` | `adversarial_cases::cr_016_composition_minimization` |
| `L2Adversarial` | `CR-017` | `adversarial_cases::cr_017_floor_noncompensation` |
| `L2Adversarial` | `CR-021` | `adversarial_cases::cr_021_burden_shift_rejection` |
| `L2Adversarial` | `CR-029` | `adversarial_cases::cr_029_cross_role_adversarial` |
| `L2Adversarial` | `CR-031` | `adversarial_cases::cr_031_parser_fuzz_authority_absent` |
| `L2Adversarial` | `CR-037` | `adversarial_cases::cr_037_resource_bound_failure` |
| `L2Adversarial` | `VCL-06` | `adversarial_cases::trace_vcl_06` |
| `L2Adversarial` | `.roles/assurance/classification-operational-security.md` | `adversarial_cases::trace_role_assurance_classification_operational_security` |
| `L2HoldClosure` | `BASTION-REQ-TST-005` | `hold_closure::trace_bastion_req_tst_005` |
| `L2HoldClosure` | `SPEC-TST-005` | `hold_closure::trace_spec_tst_005` |
| `L2HoldClosure` | `CR-008` | `hold_closure::cr_008_missing_default_hold` |
| `L2HoldClosure` | `CR-019` | `hold_closure::cr_019_missing_null_hold` |
| `L2HoldClosure` | `CR-023` | `hold_closure::cr_023_finding_dissent_retention` |
| `L2HoldClosure` | `CR-025` | `hold_closure::cr_025_hold_transpose_propagation` |
| `L2HoldClosure` | `CR-038` | `hold_closure::cr_038_waiver_ledger_nonwaiver` |
| `L2HoldClosure` | `CR-039` | `hold_closure::cr_039_evidence_state_history` |
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
| `L2NoAuthority` | `CR-010` | `no_authority_surface::cr_010_release_exception_no_output` |
| `L2NoAuthority` | `CR-017` | `no_authority_surface::cr_017_authority_noninflation` |
| `L2NoAuthority` | `CR-021` | `no_authority_surface::cr_021_false_savings_no_authority` |
| `L2NoAuthority` | `CR-024` | `no_authority_surface::cr_024_terminal_no_output_backflow` |
| `L2NoAuthority` | `CR-034` | `no_authority_surface::cr_034_generated_no_emission` |
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

`source_spine` compares the exact 123 unique identities and 148
forward edges against these exact 148 reverse edges after bytewise
sorting. It rejects an absent/orphan row, unlisted extra, alias, range,
shorthand, paired cell, duplicate edge, wrong multiplicity, controlled-source
digest mismatch, target/assertion mismatch, mode mismatch, spelling change, or
non-transpose. `L1Static` and `L1SupplyChain` retain only obligations they
actually inspect; all behavioral, contract, property, model, adversarial,
hold, authority, and trace obligations execute in their allocated L2 modes.

## 8. Closed evidence, set, review, and role custody

Evidence is retained only under `EVID-WP-TST-001` at the exact create-new
paths in section 3. No producer deletes, truncates, overwrites, renames,
quarantines, or reuses a path. Duplicate JSON keys are rejected while decoding;
a missing or extra key, wrong type/cardinality/order/enum/null posture, invalid
UTF-8, BOM, CR, trailing whitespace, path mismatch, or digest mismatch is
invalid and non-promotable.

### 8.1 Canonical encoding and closed primitive types

Every record is one UTF-8, no-BOM JSON object on one line with no insignificant
whitespace and one final LF. Objects use only the stated keys in the stated
order. Arrays use only the stated order. Integers are unsigned base-10 JSON
numbers with no leading zero, decimal point, exponent, sign, or negative zero.
Booleans and JSON null occur only where explicitly required. Strings use
shortest JSON escaping and contain printable ASCII bytes `0x20..0x7e` only.
Unless a literal, enum, regex, or smaller bound is stated, every string is
`1..128` bytes. These named types are exact and normative wherever cited:

| Type | Exact domain |
|---|---|
| `DIGEST` | string matching `^[0-9a-f]{64}$` |
| `GIT_ID` | string matching `^[0-9a-f]{40}$` |
| `VERSION` | integer `1..9999`; `NNNN` is its zero-padded four-digit spelling |
| `MODE` | one of the 16 exact MODE strings in section 6 |
| `SAFE_ID` | string matching `^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*$`, `1..128` bytes |
| `ASSERTION` | string matching `^[a-z][a-z0-9_]*::[a-z][a-z0-9_]*$`, `3..128` bytes |
| `REL_PATH` | string `1..240` bytes, forward slashes, no empty/`.`/`..` segment, no leading slash, drive prefix, backslash, colon, URI scheme, percent escape, NUL, or symlink resolution; bytewise equal to an allowlisted section 3 path |
| `UTC` | string matching `^20[0-9]{2}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]\.[0-9]{3}Z$`, a valid Gregorian instant |
| `LANE` | one exact lane string at the index table in section 8.5 |
| `REVIEWER_ID` | string matching `^REV-[A-Z0-9][A-Z0-9-]{0,59}$`; unique natural person or separately authorized agent identity across the 22 current slots |
| `OUTPUT_POINTER` | exactly one of the three literal pointers in section 8.3 |

Every array forbids duplicate complete elements; arrays identified as sorted
are strictly increasing by unsigned UTF-8 byte comparison of the named key.
Every equality is byte equality after canonical decoding. A referenced digest
must recompute from the named bytes; merely matching `DIGEST` syntax is not
sufficient. No unstated coercion, default, additional property, alternate
encoding, or nullable value exists.

### 8.2 Closed `test-gate-evidence.v3` mode schema

The exact ordered top-level keys are:
`schema,evidence_id,mode,evidence_version,evidence_path,wp_id,
wp_artifact_digest,acceptance_binding,entry_binding,implementation_binding,
logical_predecessor_commit,context_rev_binding,artifact_digests,
trace_manifest_digest,executed_targets,fixture_bindings,command_identity,
exact_argv,tool_versions,environment_digest,resource_bounds,
determinism_controls,expected,actual,observed_outputs,rollback_plan,
reproduction_plan,failure_records,counterexamples,required_review_lanes,
reviewer_decisions,findings,dissent,conflicts,status,invalidation_triggers,
predecessor_evidence,history,evidence_digest`.

| Ordered field | Exact rule |
|---|---|
| `schema` | string literal `test-gate-evidence.v3` |
| `evidence_id` | string exactly `EVID-WP-TST-001-<mode>-vNNNN` |
| `mode` | `MODE` |
| `evidence_version` | `VERSION` |
| `evidence_path` | string exactly `context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/runs/<mode>/<evidence_id>.json` |
| `wp_id` | string literal `WP-TST-001` |
| `wp_artifact_digest` | `DIGEST` of independently accepted R6 bytes |
| `acceptance_binding` | closed `AcceptanceBinding` below |
| `entry_binding` | closed `EntryBinding` below |
| `implementation_binding` | closed `ImplementationBinding` below |
| `logical_predecessor_commit` | literal `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| `context_rev_binding` | closed `ContextRevBinding` below |
| `artifact_digests` | exactly 18 closed `ArtifactDigest` objects, strictly sorted and unique by `path`, one for every section 3 root manifest, lockfile, runner, package, test, support, fixture, and fixture-manifest input; no omission or other path |
| `trace_manifest_digest` | `DIGEST` of the accepted exact 123-identity/148-edge manifest |
| `executed_targets` | `0..148` closed `ExecutedTarget` objects, strictly sorted and unique by tuple `(controlled_id,assertion)`; exactly the section 7 edges assigned to `mode`, and empty exactly when its section 7 target list is empty |
| `fixture_bindings` | exactly four closed `FixtureBinding` objects in ascending `fixture_id` order; exactly the current rows in section 5 |
| `command_identity` | exact section 6 `CMD-*` identity selected by `mode` |
| `exact_argv` | array `1..16` of strings `1..128` bytes, in execution order, byte-for-byte the selected section 6 argv; duplicates allowed only where section 6 literally repeats an argument |
| `tool_versions` | `1..32` closed `ToolVersion` objects strictly sorted and unique by `tool` |
| `environment_digest` | `DIGEST` of the bytewise-name-sorted sanitized `<name><TAB><value><LF>` sequence; names match `^[A-Z][A-Z0-9_]{0,63}$`, values are `0..128` printable-ASCII bytes, and duplicate names reject |
| `resource_bounds` | closed `ResourceBounds` below |
| `determinism_controls` | closed `DeterminismControls` below |
| `expected` | closed `ExpectedResult` below |
| `actual` | closed `ActualResult` below |
| `observed_outputs` | closed `ObservedOutputs` in section 8.3 |
| `rollback_plan` | closed `PlanWrapper<RollbackPlan>` in section 8.4 |
| `reproduction_plan` | closed `PlanWrapper<ReproductionPlan>` in section 8.4 |
| `failure_records` | `0..128` closed `FailureRecord` objects strictly sorted and unique by `id` |
| `counterexamples` | `0..128` closed `Counterexample` objects strictly sorted and unique by `id` |
| `required_review_lanes` | the exact 22 strings in section 8.5 index order |
| `reviewer_decisions` | exactly 22 slots in that same order; each null or one closed `DecisionRecord<mode,index>` |
| `findings` | the exact strictly `id`-sorted projection of every non-null current decision's `findings`; `0..2816` unique `Finding` objects |
| `dissent` | the exact strictly `id`-sorted projection of every non-null current decision's `dissent`; `0..2816` unique `Dissent` objects |
| `conflicts` | the exact strictly `id`-sorted projection of every non-null current decision's `conflicts`; `0..2816` unique `Conflict` objects |
| `status` | derived enum `failed|stale|conflicted|passed` under the precedence rule below |
| `invalidation_triggers` | exactly the strictly bytewise-sorted unique non-empty array of the literals `artifact_digest_change,binding_change,command_change,environment_change,fixture_change,review_change,schema_change,trace_change`; no other value |
| `predecessor_evidence` | null iff version 1; otherwise one closed `EvidenceBinding` naming version `evidence_version-1` of the same mode |
| `history` | exactly `evidence_version-1` closed `EvidenceBinding` objects in ascending consecutive version order, beginning at 1; last equals `predecessor_evidence`; empty iff version 1 |
| `evidence_digest` | `DIGEST`, last; SHA-256 of the canonical complete object with only its preceding comma and this key/value omitted |

The following nested objects are independently closed. Key order is the order
shown; all members are non-null unless stated otherwise:

| Type | Ordered members and exact rules |
|---|---|
| `AcceptanceBinding` | `commit:GIT_ID,pulse_digest:DIGEST`; commit is the committed R6 acceptance pulse and that pulse hashes R6 and earlier inputs, never its own commit |
| `EntryBinding` | `commit:GIT_ID,pulse_digest:DIGEST,tree_digest:GIT_ID`; commit is the direct governance child of acceptance and its pulse binds acceptance, never its own commit |
| `ImplementationBinding` | `commit:GIT_ID,tree_digest:GIT_ID,first_parent:GIT_ID,delta_digest:DIGEST,delta_paths:array`; `first_parent=entry_binding.commit`; `commit` is its non-merge direct child; `delta_paths` has `1..18` distinct `REL_PATH` values strictly bytewise sorted and equals the complete Git changed-path set; `delta_digest` hashes `<path><TAB><blob-sha256><LF>` for those paths in that order |
| `ContextRevBinding` | `exit_commit:GIT_ID,implementation_digest:DIGEST,evidence_set_digest:DIGEST,evidence_tree_digest:GIT_ID,unchanged_result_digest:DIGEST`; values are exactly `ab227cc06f15299b594cfe2be99915bd93c4c081`,`c5c2df1178568cd49b5d721cd01cba7cce3371e049528e07bad30d6b3324ea72`,`b95beff569794125018f2fde3d4d3317ed32278dfcfb1fc22a7d25cf51226bd9`,`d554c8c0c3d534aa96924f085a4dc007b25e3a3c`,`f0a15398cc87614cc904cbaa28459ef65ebc267ed70349e46f86f743ebd708c6`; the last hashes exact UTF-8 `rev_unchanged=true<LF>` and proves context only |
| `ArtifactDigest` | `path:REL_PATH,sha256:DIGEST`; digest hashes exact file bytes |
| `ExecutedTarget` | `controlled_id:SAFE_ID-or-LANE,assertion:ASSERTION`; pair must be one exact section 7 edge; lane strings are admitted only where section 7 uses them |
| `FixtureBinding` | `fixture_id:SAFE_ID,version:VERSION,source_id:SAFE_ID,source_digest:DIGEST,custody_id:SAFE_ID,custody_digest:DIGEST,input_digest:DIGEST,supersession_state:enum(current,superseded)`; values equal one current section 5 row, so all four are `current` in an executable set |
| `ToolVersion` | `tool:string,version:string,digest:DIGEST`; tool matches `^[a-z][a-z0-9_-]{0,31}$`; version matches `^[0-9A-Za-z][0-9A-Za-z.+_-]{0,63}$` |
| `ResourceBounds` | `wall_seconds:60,process_tree_bytes:1073741824,combined_stream_bytes:10485760`; all JSON integers |
| `DeterminismControls` | `order:"bytewise",seed:"disabled",clock:"disabled",locale:"disabled",retry:"disabled"` |
| `ExpectedResult` | `exit:0,result:"passed",posture:"promotable",reason:"expected-outcome"` |
| `ActualResult` | `exit:integer 0..255,result:enum(passed,failed),posture:enum(promotable,non-promotable),reason:enum(expected-outcome,unexpected-exit,assertion-failure,bound-exceeded,binding-mismatch,conflict),start_utc:UTC,end_utc:UTC,duration_ms:integer 0..60000`; end is not earlier than start and duration equals their millisecond difference |
| `EvidenceBinding` | `evidence_id:string,evidence_path:string,evidence_version:VERSION,evidence_digest:DIGEST`; ID/path use the same mode and bound version formulas and digest hashes that immutable predecessor |

`actual.result="passed"` iff exit is 0, posture is `promotable`, reason is
`expected-outcome`, `failure_records`, `counterexamples`, and `conflicts` are
empty, and the structured result reports zero failed and held assertions.
Otherwise result is `failed`, posture is `non-promotable`, and reason identifies
the highest-precedence cause: `bound-exceeded`, `binding-mismatch`, `conflict`,
`unexpected-exit`, then `assertion-failure`. Top-level `status` is `stale` on a
binding/digest/history mismatch; else `conflicted` when `conflicts` is non-empty,
any current decision is conflicted, any open dissent exists, or any open defer
exists; else `failed` when actual failed, failures/counterexamples exist, an
assurance is failed, or a critical/major finding is open; else `passed`. No
latest exit candidate may be stale, conflicted, or failed. An older immutable
record remains byte-identical and is known to be superseded only from a later
record's history; its stored status is never rewritten.

### 8.3 Closed observed-output and result fragments

`ObservedOutputs` has exactly `stdout,stderr,structured`. `stdout` and `stderr`
are closed `ByteStream` objects with ordered keys
`encoding,content_base64,decoded_byte_count,decoded_sha256`: encoding is literal
`base64`; content is canonical padded RFC 4648 using `A-Z a-z 0-9 + /` and
terminal `=` only, decodes without error, and is `0..13981016` characters;
decoded count is integer `0..10485760` equal to its byte length; digest is
`DIGEST` of decoded bytes. Counts sum to at most `10485760`. Empty bytes use
empty content and SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

`structured` is the closed object `encoding,content,byte_count,sha256`.
Encoding is literal `canonical-json`. Across modes and sets, content has one and
only one literal `StructuredResult` schema with ordered keys
`schema,scope,command_identity,exit,result,posture,reason,counts`. Schema is
literal `test-gate-structured-result.v1`; scope is a `MODE` for a mode record or
literal `SET` for a set; command equals the enclosing command; exit/result/
posture/reason equal its closed actual/result values; and `counts` is exactly
`attempted,passed,failed,held`, each integer `0..148`, with attempted equal to
passed plus failed plus held. For a mode, attempted equals its number of
executed targets and held is zero for a passed result. For a set, attempted is
16 and counts the selected modes; held is zero for a full result. Byte count is
integer `2..1048576` equal to canonical content bytes without LF; SHA-256 hashes
those bytes.

The only `OUTPUT_POINTER` values are literal
`/observed_outputs/stdout/content_base64`,
`/observed_outputs/stderr/content_base64`, and
`/observed_outputs/structured/content`. Resolution hashes decoded stream bytes
or canonical structured content bytes. Reject every other spelling, including
empty/root/self/enclosing/ancestor/future/external/absolute/URI/symlink/unknown,
percent-encoded, `~0`/`~1` escaped, relative, fragment, and array-index forms.
No pointer addresses a plan, decision, digest, or enclosing object. No output
contains raw rejected fixture bytes, enough content to reconstruct them, or an
unbounded echo.

### 8.4 Closed plan and diagnostic records

Each plan wrapper has exactly `content,digest`; digest is `DIGEST` of canonical
content bytes without LF. `RollbackPlan` content has exactly
`plan_id,steps,bounds,expected_state`: plan ID is literal
`PLAN-WP-TST-001-ROLLBACK`; steps is exactly
`["remove-tst-package","restore-entry-cargo-toml","restore-entry-cargo-lock",
"verify-rev-unchanged","retain-evidence"]`; bounds is exactly the closed object
`max_steps:5,max_wall_seconds:60,max_changed_paths:18`; expected state is exactly
the closed object `entry_tree:GIT_ID,rev_unchanged:true,evidence_retained:true`
where entry tree equals `entry_binding.tree_digest`.

`ReproductionPlan` content has exactly
`plan_id,command_identity,exact_argv,input_digests,controls,expected_result`.
Plan ID is exactly `PLAN-WP-TST-001-REPRO-<mode>`; command and argv equal the
top-level values; `input_digests` is exactly 18 closed `ArtifactDigest` objects
equal to top-level `artifact_digests`; controls equal top-level
`DeterminismControls`; expected result equals top-level `ExpectedResult`.
Plan strings never contain an external/absolute path, URI, symlink, secret, raw
fixture byte, or reconstructive content.

The following diagnostic objects are closed; keys are exactly ordered as shown:

| Type | Ordered members and exact rules |
|---|---|
| `FailureRecord` | `id,code,assertion,output_pointer,disposition,digest`; id exactly `FAIL-<mode>-vNNNN-NNN` with ordinal `001..128`; code enum `unexpected-exit|assertion-failure|bound-exceeded|binding-mismatch|conflict`; assertion is an `ASSERTION` executed by this mode or literal `supervisor::preflight`; pointer is `OUTPUT_POINTER`; disposition enum `open|retained|remediated`; digest is last and hashes this object with only preceding comma plus digest member omitted |
| `Counterexample` | `id,assertion,input_digest,reproduction_plan_digest,output_pointer,disposition,digest`; ID exactly `CEX-<mode>-vNNNN-NNN`; assertion as above; input digest is `DIGEST` of a bounded non-reconstructive canonical synthetic descriptor; reproduction digest equals top-level reproduction-plan digest; pointer is `OUTPUT_POINTER`; disposition enum `open|retained|remediated`; digest uses the same omission rule |
| `Finding` | `id,severity,claim_digest,evidence_pointer,owner,destination,closure,disposition`; ID exactly `FND-<scope>-Lii-vNNNN-NNN`; scope is mode or `SET`, lane index and decision version equal the enclosing decision, ordinal `001..128`; severity enum `critical|major|minor|note`; claim digest is `DIGEST`; pointer is `OUTPUT_POINTER` resolving only inside the enclosing mode/set observed outputs; owner is `REVIEWER_ID`; destination is one `LANE`; closure enum `open|remediated|not-applicable`; disposition enum `hold|accept|defer` |
| `Dissent` | `id,lane,claim_digest,record_digest,disposition`; ID exactly `DIS-<scope>-Lii-vNNNN-NNN`; lane equals enclosing decision lane; both digests are `DIGEST`, with record digest binding the cited prior evidence or decision, never the enclosing decision; disposition enum `open|retained|resolved` |
| `Conflict` | `id,lane,left_digest,right_digest,owner,disposition`; ID exactly `CON-<scope>-Lii-vNNNN-NNN`; lane equals enclosing decision lane; digests are distinct `DIGEST` values; owner is `REVIEWER_ID`; disposition enum `open|retained|resolved` |

Within each diagnostic/decision array of cardinality N, ordinals are exactly
`001..N` without a gap; IDs are unique across their entire containing top-level
projection. An open critical/major finding, dissent, conflict, or diagnostic
makes promotion fail.
For a finding, `open` permits only disposition `hold|defer`, `remediated`
requires `accept`, and `not-applicable` requires severity `note` and disposition
`accept`. Dissent/conflict disposition `resolved` is the only non-open state;
`retained` remains open and blocking. A diagnostic disposition `remediated`
retains history but cannot make the command's immutable failed actual pass.

All nested digest dependencies are acyclic: output digests hash only their
content; plan digests hash only plan content; diagnostic digests omit only
themselves; decision digests omit only themselves; mode/set digests omit only
themselves. `claim_digest`, `record_digest`, `left_digest`, `right_digest`, and
`input_digest` may bind only a pre-existing controlled artifact, an embedded
output-fragment digest, or an immutable predecessor evidence/decision digest.
They may never equal or depend on the enclosing decision, mode, or set digest,
nor any future commit, pulse, path, record, or digest.

### 8.5 Review records, exact lanes, and successor transition

`required_review_lanes` is exactly the bytewise-sorted set of: the eight
Parliament file identities; `.roles/panel-reviewer/panel.md`; `Role review
steward`; the three Editorial file identities; the seven Stakeholder file
identities; and the two Assurance file identities already enumerated in section
7. `reviewer_decisions` has exactly 22 slots; null means that canonical lane has
not yet decided. The only valid decision pointer for lane index `i` is plain
RFC 6901 `/reviewer_decisions/i`, where `i` matches
`^(0|[1-9]|1[0-9]|2[01])$` with no leading zero and resolves to the lane at the
same index in `required_review_lanes`.

| Index | Exact canonical lane identity |
|---:|---|
| `0` | `.roles/assurance/civilian-control-law-safety-readiness.md` |
| `1` | `.roles/assurance/classification-operational-security.md` |
| `2` | `.roles/editorial/citation-auditor.md` |
| `3` | `.roles/editorial/numeracy-checker.md` |
| `4` | `.roles/editorial/scope-keeper.md` |
| `5` | `.roles/panel-reviewer/panel.md` |
| `6` | `.roles/parliament/acquisition-industrial-base.md` |
| `7` | `.roles/parliament/alliance-interoperability.md` |
| `8` | `.roles/parliament/civilian-strategy-force-planner.md` |
| `9` | `.roles/parliament/defense-comptroller.md` |
| `10` | `.roles/parliament/independent-test-oversight.md` |
| `11` | `.roles/parliament/logistics-sustainment.md` |
| `12` | `.roles/parliament/operational-readiness.md` |
| `13` | `.roles/parliament/service-member-family.md` |
| `14` | `.roles/stakeholders/ally-partner.md` |
| `15` | `.roles/stakeholders/depot-logistics-workforce.md` |
| `16` | `.roles/stakeholders/installation-community.md` |
| `17` | `.roles/stakeholders/mission-user.md` |
| `18` | `.roles/stakeholders/prime-small-supplier.md` |
| `19` | `.roles/stakeholders/service-member-family.md` |
| `20` | `.roles/stakeholders/taxpayer-oversight.md` |
| `21` | `Role review steward` |

A non-null slot is one closed object with exact ordered keys
`decision_id,decision_version,predecessor_decision_id,
predecessor_decision_digest,predecessor_decision_version,lane,status,
reviewer_id,independence,assurance,findings,defer,dissent,conflicts,closure,
decision_record_digest`. For scope `<scope>` equal to enclosing mode or `SET`
and index `ii` zero-padded to two digits, decision ID is exactly
`DEC-WP-TST-001-<scope>-Lii-vNNNN`, where `NNNN` is `decision_version`.
Decision IDs and digests are unique across the complete lane history.

Decision version is `VERSION` and equals 1 for a null-slot predecessor;
otherwise it equals prior same-lane decision version plus 1. On version 1 the
predecessor fields are exactly `null,null,0`; otherwise predecessor ID/digest/
version are non-null and equal the immediately prior same-scope, same-index
decision. Lane equals the indexed lane. Reviewer ID is `REVIEWER_ID`; it differs
from every finding/conflict owner assigned to adjudicate that decision and may
occupy only one current lane unless an explicit conflict is recorded.
Independence is `independent|conflicted`. Assurance is `pass|fail|
not-applicable`: indexes 0 and 1 require pass or fail, all other indexes require
not-applicable. Findings, dissent, and conflicts are respectively `0..128`
strictly ID-sorted unique closed objects from section 8.4, with matching scope,
index, version, lane, and owner constraints.

`defer` is null unless status is `defer`; then it is exactly the closed object
`owner,destination,closure_condition,hold_behavior`. Owner is `REVIEWER_ID`;
destination is `LANE`; closure condition is a `1..128` printable-ASCII string
matching `^[A-Za-z0-9][A-Za-z0-9 ._:-]{0,127}$`; hold behavior is literal
`block-promotion`. Status is `pass|finding|defer`; closure is
`open|remediated|not-applicable`. Coherence is exact: `pass` requires
independent, null defer, no open finding/dissent/conflict, the assurance value
required above, and closure `not-applicable` for a first pass or `remediated`
when its bound predecessor was finding/defer; `finding` requires at least one
open finding, null defer, and closure `open`; `defer` requires non-null defer and
closure `open`; conflicted independence requires status defer and at least one
open conflict. Failed assurance requires status finding and at least one open
critical or major finding. No finding/defer can disappear from custody: a later
remediated pass binds it; the prior decision and its finding digests remain
immutable in predecessor evidence, while any closure objects in the new
decision use new formula-bound IDs and cite the prior claim digests.

`decision_record_digest` is `DIGEST`, last, and hashes canonical decision bytes
with only the preceding comma and that member omitted. It does not hash its
enclosing record or pointer, so the preimage is acyclic.

Version-1 command execution has 22 null slots. A one-lane mode successor opens
only the next unused evidence version and changes exactly one slot. Every
non-derived field is byte-identical to the immediate predecessor, specifically
`schema,mode,wp_id,wp_artifact_digest,acceptance_binding,entry_binding,
implementation_binding,logical_predecessor_commit,context_rev_binding,
artifact_digests,trace_manifest_digest,executed_targets,fixture_bindings,
command_identity,exact_argv,tool_versions,environment_digest,resource_bounds,
determinism_controls,expected,actual,observed_outputs,rollback_plan,
reproduction_plan,failure_records,counterexamples,required_review_lanes,
invalidation_triggers`; the other 21 slots are also byte-identical.

The complete and only synchronized derived delta is: increment
`evidence_version` by 1; regenerate `evidence_id` and `evidence_path`; bind the
immediate predecessor in `predecessor_evidence`; append exactly that binding to
`history`; replace exactly the indexed decision slot with its formula-bound
next decision; recompute `findings`, `dissent`, and `conflicts` as exact sorted
projections of all 22 current slots; recompute `status` by section 8.2; and
recompute `evidence_digest`. No other byte may change. The changed decision's
three arrays remove their prior projected contributions and add their new
contributions; the 21 retained decisions' contributions remain byte-identical.
This reconciliation is exact, not a merge, patch, or reviewer-selected summary.

Only the latest successor with all 22 non-null current lanes may be
`fully_reviewed` or contribute to exit. Every lane is independent, both
assurance lanes are `pass`, all predecessors/digests/versions verify, and zero
current critical/major finding, open defer, open dissent conflict, or evidence
conflict is mandatory.

### 8.6 Independently closed `test-gate-evidence-set.v2` schema

A set uses section 8.1 canonical encoding and these exact ordered keys:
`schema,set_id,set_version,set_path,wp_id,wp_artifact_digest,
acceptance_binding,entry_binding,implementation_binding,mode_records,
aggregate_digest,observed_outputs,required_review_lanes,reviewer_decisions,
findings,dissent,conflicts,status,review_completeness,rollback_plan,reproduction_plan,
predecessor_set,history,invalidation_triggers,set_digest`.

`schema` is literal `test-gate-evidence-set.v2`; set version is `VERSION`; ID is
exactly `EVID-WP-TST-001-SET-vNNNN`; path is exactly
`context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/sets/<set_id>.json`;
WP ID is literal `WP-TST-001`; WP digest and the three binding objects use the
fully closed definitions and equalities in section 8.2. `mode_records` is
exactly 16 closed objects with ordered keys
`mode,evidence_id,evidence_version,evidence_path,evidence_digest`, in section 6
MODE order; each field uses the mode formulas/types in section 8.2, each digest
recomputes, and all selected records have identical WP/acceptance/entry/
implementation bindings. `aggregate_digest` is `DIGEST` over each selected
`<evidence_path><TAB><evidence_digest><LF>` in strict bytewise path order.

`observed_outputs` is a closed section 8.3 `ObservedOutputs`: stdout/stderr use
the identical bounded `ByteStream`; structured content uses the identical one
literal `StructuredResult` schema with scope `SET`, command literal
`CMD-TST-EVIDENCE-SET`, attempted 16, and passed/failed/held counts of the
selected modes' immutable command `actual.result` values, not their later review
status. A full candidate has exit 0, passed 16, failed 0, held 0, result passed,
posture promotable, and reason expected-outcome. Byte count and SHA-256 use the
same exact section 8.3 rules.

Required lanes are the exact 22-index array. Decisions are exactly 22 null or
closed `DecisionRecord<SET,index>` values under every section 8.5 constraint.
Top-level findings, dissent, and conflicts are `0..2816` unique objects and are
the exact strictly ID-sorted projections of those current set decisions; set
finding evidence pointers are `OUTPUT_POINTER` values resolving only within the
set's embedded observed outputs. Status is enum
`partial|conflicted|failed|full`: `conflicted` iff an open conflict/dissent or
conflicted reviewer exists; else `failed` iff an open critical/major finding,
failed assurance, or selected non-passed mode exists; else `full` iff all 22 set
decisions and all 22 decisions in every selected mode are non-null, current,
predecessor-complete passes and every selected mode is passed; otherwise
`partial`. `review_completeness` is `partial|full` and equals `full` iff status
is full. Only full supports exit.

The set rollback wrapper is exactly the section 8.4 rollback wrapper. The set
reproduction wrapper uses the same exact `content,digest` order and digest
preimage, but its closed content is exactly
`plan_id,command_identity,exact_argv,input_digests,controls,expected_result`:
plan ID literal `PLAN-WP-TST-001-REPRO-SET`; command identity literal
`CMD-TST-EVIDENCE-SET`; argv is exactly
`["pwsh","-NoLogo","-NoProfile","-NonInteractive","-File","tools/test_gate.ps1","-AssembleSet"]`;
input digests are exactly two objects `name,sha256` in this order,
with names `wp-artifact` and `implementation-delta` and digests equal to
`wp_artifact_digest` and `implementation_binding.delta_digest`; controls are
exactly section 8.2 `DeterminismControls`; expected result is exactly section
8.2 `ExpectedResult`. No external pointer target exists.
`predecessor_set` is null iff version 1; otherwise it is the closed
object `set_id,set_path,set_version,set_digest`, with same set formulas, version
equal to current minus 1, and digest recomputed. `history` has exactly
`set_version-1` such objects in consecutive ascending order from 1; last equals
predecessor. `invalidation_triggers` is exactly the same eight-literal array as
mode records. `set_digest` is `DIGEST`, last, hashing the complete canonical set
with only its preceding comma and member omitted.

A one-lane set successor changes exactly one decision slot. Every non-derived
field is byte-identical to its immediate predecessor: `schema,wp_id,
wp_artifact_digest,acceptance_binding,entry_binding,implementation_binding,
observed_outputs,required_review_lanes,rollback_plan,reproduction_plan,
invalidation_triggers`; the other 21 slots are byte-identical.
The complete and only derived delta is: set version plus 1; regenerated set ID
and path; immediate `predecessor_set`; history append of exactly that binding;
replace all 16 `mode_records` entries with their just-created one-lane mode
successors for that same lane and no other mode successors, then recompute
`aggregate_digest`; place the formula-bound next set decision in the same lane
slot; exactly project findings, dissent, and conflicts from that changed
decision while retaining the other 21 contributions; recompute status and
review completeness; and recompute set digest. No other byte changes.

Review changes first create the 16 same-lane mode successors and then the
corresponding set one-lane successor; each set decision binds its own prior set
decision and its finding claims bind the selected mode successor digest. No
mode/set/decision preimage includes its own digest, enclosing digest, future
commit, or future pulse. Neither history is mutated, deleted, overwritten,
quarantined, or hidden.

## 9. Entry, stop, exit, and authority

Acceptance of this candidate, if it occurs, authorizes only a later separate
entry decision. The acceptance pulse binds the R6 artifact digest and all prior
governance inputs, but never its own future commit. After it is committed, the
entry pulse binds that acceptance commit and pulse digest, but never its own
future commit. After entry is committed, evidence binds the resulting entry
commit, entry-pulse digest, and entry tree. A clean isolated worktree starts at
that entry commit, and the one atomic implementation commit is its first-parent,
non-merge direct child. The exact allowlist, unchanged REV bytes, and no
unrelated change are mandatory. WS remains
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
