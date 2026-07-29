# WP-TST-001-R1 — isolated boundary-test and safe-fixture bootstrap

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundaries: `PB-TST-001` and assigned `PB-FIX-001`, plus
configuration-only membership integration in `PB-WS-001`

Predecessors: accepted `WP-WS-001` exit and accepted `WP-REV-001` bootstrap
exit; the latter is a test-only local dependency, not product evidence or
authority

## 1. Controlled baseline and custody

This candidate is based on clean child-repository `main` commit
`ab227cc06f15299b594cfe2be99915bd93c4c081`. A later entry commit must descend
from this commit through only the candidate, independent-review, acceptance,
and entry governance records. Any different ancestry, dirty unrelated path,
predecessor digest, or REV surface holds acceptance and entry.

| Controlled artifact | Exact identity |
|---|---|
| Current BASTION `main` / candidate base | `ab227cc06f15299b594cfe2be99915bd93c4c081` |
| Accepted REV implementation commit | `5c4e96306d3c463a44be7621371759da8bca399b` |
| Accepted REV evidence commit | `3594500d461d5e39e6d44bf721708f3e0735948a` |
| Accepted REV independent-review commit | `a0827675b8918256223975ace0d1c73a9b8eb0b8` |
| Accepted REV evidence-set SHA-256 | `b95beff569794125018f2fde3d4d3317ed32278dfcfb1fc22a7d25cf51226bd9` |
| Accepted REV evidence Git tree | `d554c8c0c3d534aa96924f085a4dc007b25e3a3c` |
| Accepted REV implementation SHA-256 | `c5c2df1178568cd49b5d721cd01cba7cce3371e049528e07bad30d6b3324ea72` |
| REV exit pulse SHA-256 | `f80693db1569c4be2c5e58e1dafa0c3ae7cf2195069934f737e4ba2b9bab7309` |
| `PACKAGE_BOUNDARIES.md` | `04f3923bf09f14b57cd02006dcb5e17abb4cac8981fced4c4749f0d446a71ca6` |
| `IMPLEMENTATION_PLAN.md` | `d8db35026e02796bc2bba5e034aa4981394ae1d9a4574943c82f4705bd64d9aa` |
| `WORK_PACKAGES.md` | `596761dd75e2d9b0721c7ecabb8086ee58682513b5a2c7d2a94ab1f7a79ea129` |
| `VERIFICATION.md` | `e25d1999a72575bd8e6c920cd450f50146754e08262e06127f64ff5c732fe080` |
| `VALIDATION.md` | `325be52b0ba258f97415709c040880f4282452d4ea3d609855dacd31a2bb1016` |
| `CHANGE_CONTROL.md` | `13cbfc8990ae42a5719f3945fe9dd7a46076def0db9be1f12ce4316c2dc57190` |
| Current workspace manifest SHA-256 | `6defae120ce4a75ca73e17b3186cfc76ebe37d634c0927f38763eb4d1010e82f` |
| Current lockfile SHA-256 | `003f80cd529c8768c1913a511e7d754b137569f1a19553b43e9bff660370e3e2` |
| Accepted `bastion-review` manifest SHA-256 | `a879b19a53d12770fea6e5024525d9d5a5e516c23f81de3353d89ad154167a4e` |

The accepted REV exit is unchanged and non-authoritative. This candidate may
exercise its public review/control surface from a test-only package, but may
not alter REV, manufacture a producer packet, claim a producer pass, or treat
`PassRecommended` as acceptance.

## 2. Objective and hard boundary

The smallest coherent result is an independently owned, deterministic Rust
integration-test package and an inert synthetic fixture-custody scaffold. It
proves only that:

1. test and fixture dependencies point downward and never enter a product
   target;
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

The root files may change only to add the test package as the second workspace
member and the corresponding local lock entry. The later evidence-retention
commit may add exactly one JSON file for each of the 16 modes in section 6 at:

```text
context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/<Mode>.json
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
- exactly one development dependency:
  `bastion-review = { path = "../bastion-review" }`; and
- exactly eight explicit integration-test targets named `source_spine`,
  `contract_matrix`, `property_cases`, `model_cases`, `adversarial_cases`,
  `hold_closure`, `no_authority_surface`, and `static_surface`, each mapped to
  its same-named allowlisted file.

The root member list becomes the canonical bytewise ordering
`["crates/bastion-boundary-tests", "crates/bastion-review"]`. The only Cargo
dependency edge added is test-only `PB-TST-001 -> PB-REV-001`. No product
target may depend on `PB-TST-001` or `PB-FIX-001`; fixtures are compile-time
test inputs only through `include_bytes!`, never runtime, product, build-script,
or generated inputs.

Test support uses only `std` and the accepted public `bastion-review` surface.
It performs no runtime filesystem, network, environment, process, thread,
clock, locale, retry, randomness, or recursive operation. It exposes no public
library surface and contains no product value or decision. Explicit
`unwrap`, `expect`, `panic!`, `todo!`, and `unimplemented!` are forbidden;
ordinary Rust test assertion macros remain the test verdict mechanism.

## 5. Exact fixture-custody scaffold

`manifest.tsv` is UTF-8 without BOM, LF-only, ASCII, and contains one header
followed by exactly four rows in ascending fixture-ID order. Its exact columns
are:

```text
fixture_id<TAB>version<TAB>path<TAB>sha256<TAB>class<TAB>source_posture<TAB>purpose_id<TAB>expected_posture<TAB>proof_input_hold
```

The four rows are:

| Fixture ID | File | Class | Purpose ID | Expected posture | Proof-input hold |
|---|---|---|---|---|---|
| `FIX-TST-BOOT-001` | `cases/valid.fixture` | `valid-custody` | `PURPOSE-TST-CUSTODY-001` | `accepted-for-harness-only` | `TBD-TST-001` |
| `FIX-TST-BOOT-002` | `cases/absent.fixture` | `explicit-absence` | `PURPOSE-TST-ABSENCE-001` | `held` | `TBD-SRC-001` |
| `FIX-TST-BOOT-003` | `cases/stale.fixture` | `stale-binding` | `PURPOSE-TST-STALE-001` | `rejected` | `TBD-TST-001` |
| `FIX-TST-BOOT-004` | `cases/deny-marker.fixture` | `safe-denial-marker` | `PURPOSE-TST-DENY-001` | `rejected-safe` | `TBD-SEC-001` |

Each fixture file is UTF-8 without BOM, LF-only ASCII with exactly six
single-valued `key=value` rows: `fixture_id`, `version`, `class`,
`source_posture`, `purpose_id`, and `token`. Keys occur in that order and only
once. `source_posture` is exactly `synthetic-inert`. Tokens use only ASCII
upper-case letters, digits, underscore, colon, and hyphen and convey no
real-world content. The manifest SHA-256 is lower-case hex over exact fixture
bytes. The test harness never interprets a token as evidence, semantics, or
authority.

Bounds are hard failures before favorable evaluation:

- manifest: at most 16 KiB and 32 rows;
- each fixture: at most 4 KiB; all fixture bytes together at most 32 KiB;
- fixture/path/purpose/token field: non-empty and at most 128 bytes;
- fixture version: positive `u64`;
- path: exact allowlisted relative path, with no absolute, parent, alternate
  separator, drive, URI, symlink, or normalization behavior; and
- manifest and fixture IDs unique; lower-case 64-hex digest exact; no ignored,
  extra, duplicate, reordered, or silently defaulted field.

The four committed files are inert positive/custody representatives. Negative
variants are derived in bounded test memory and are never retained as unsafe
fixture content. Any expected-posture change requires a new fixture version,
exact predecessor digest, new review, and retained predecessor; a hand edit or
silent golden replacement rejects.

## 6. Exact runner, commands, and execution bounds

`tools/test_gate.ps1` is a non-product verification runner with the same
supervisor/worker separation as the accepted REV runner. Invocation is:

```text
pwsh -NoLogo -NoProfile -NonInteractive -File tools/test_gate.ps1 -Mode <MODE>
```

The supervisor deletes or quarantines stale same-mode evidence before launch,
creates a Windows Job Object, sanitizes the environment to exactly
`APPDATA`, `CARGO_HOME`, `COMSPEC`, `HOME`, `LOCALAPPDATA`, `PATH`, `PATHEXT`,
`PSModulePath`, `RUSTUP_HOME`, `SYSTEMROOT`, `TEMP`, `TMP`, `USERPROFILE`, and
`WINDIR` when non-empty, and publishes pass evidence only after bounded stream
capture, zero worker exit, and all postconditions. Failure writes a fail record
or leaves no pass record. Command stdout and stderr share one streaming budget
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
| `CMD-L1-SUPPLY-CHAIN` | `L1SupplyChain` | `cargo +1.95.0 metadata --locked --offline --no-deps --format-version 1`, then assert the exact two-package graph and one local dev edge |
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
acceptance/runner/root-manifest/lock/REV predecessor digest, exact argv,
sanitized-environment digest, start/end/duration, bounds, per-command exit and
stream hashes/bytes, combined bytes, assertions, executed case target, and
result in canonical `test-gate-evidence.v1` JSON. All 16 modes must pass at
one identical binding. A zero-test target, skipped target, missing field,
mutation during a run, mismatched digest, or output after supervisor failure
is a failure.

## 7. Required bootstrap cases

`source_spine` must prove the bounded bootstrap-only chain
`accepted REV binding -> inert fixture custody -> isolated test verdict ->
non-authoritative evidence record`. Every node and edge is digest-bound; a
missing, reordered, substituted, reverse, producer, HND, Taxlane, or release
edge rejects. The target proves no semantic source or product result exists.

`contract_matrix` must prove the exact four-row manifest, header/field order,
fixture byte digests, exact allowed paths, unique IDs, four expected postures,
compile-time fixture custody, accepted REV test-only edge, and absence of a
product target. It also proves lower/upper accepted field and byte bounds.

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
four fixtures, manifest, local test-only dependency, no public library/binary,
no unsafe/FFI/runtime I/O/ambient state/thread/randomness/recursion, no hidden
path, and no product-to-test/fixture or generated-source edge.

The full L0/L1 runs must also prove formatting, compilation, all tests, docs,
lint with warnings denied, offline supply-chain closure, and exact rollback
surface. Case labels in evidence may state only an assertion or test target
that actually executed.

## 8. Evidence, independent review, and roles

Evidence is retained only under `EVID-WP-TST-001`. Each record is immutable;
correction creates a successor and retains the failed, conflicted, invalid, or
superseded predecessor. Missing, planned, absent, stale, conflicted, held,
failed, rejected, or zero-test evidence cannot count as pass. Independent
reproduction is required before exit consideration.

All decisions bind the same WP, implementation, fixture-manifest, runner,
environment, evidence-set, predecessor, and policy digests. Required lanes are:

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
digests, clean isolated worktree, current REV exit, exact allowlist, one atomic
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

Rollback is one atomic tree change that removes the exact TST package,
fixtures, runner, and live TST evidence; restores the pre-entry root manifest
and lock bytes; and leaves the accepted REV implementation/evidence untouched.
The implementation/evidence commits, failed results, findings, dissent, and
rollback decision remain recoverable in Git history and bound by digest.

Reopen this WP before entry for any changed baseline, representation,
allowlist, dependency direction, fixture inventory/schema/content class,
expected posture, test target, command, runner behavior, resource bound,
evidence schema/destination, reviewer set, stop/exit rule, or rollback. After
entry, any such change requires stop, retained evidence, and a separately
accepted successor; it is not an implementation detail.

Disposition: **exact WP candidate for independent review only; not accepted;
not entered; no implementation or authority**.
