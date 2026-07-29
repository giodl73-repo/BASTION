# WP-TST-001-R20 — externally-dispatched schema-version corrective amendment

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

Date: 2026-07-29

Primary boundaries: `PB-TST-001` and assigned `PB-FIX-001`, plus
configuration-only membership integration in `PB-WS-001`

Logical WP predecessor: accepted `WP-WS-001` exit only. R1 commit
`62116481b7b3e7d671517b6053c8cc3f20f93fce` and R2 commit
`21c8066445c72358a444c0b506422ec3b9dc63e0` are retained governance history.
R14 was accepted and entered, but its direct implementation child failed
independent implementation review. R15's attempted lineage amendment and
R16's attempted closure are also retained and failed VTRACE review. R15 did
not close its delta projection,
schema versions, positive identity replacement, governance observations, or
failed-attempt disposition. R16 additionally failed its observation domains,
failed-author projection, acceptance ancestry, lineage-intent preimages, audit
identity, and ledger reference. R17 closed those issues but failed to bind the
entry and implementation commit parents, used status-filtered no-extra diffs,
and left set-level dual-bound identity visibility ambiguous. R18 closed those
issues but retained earlier outer schema literals for wire contracts whose
mandatory revision, source enums, and nested exact shapes had changed. R19
versioned those contracts but collapsed distinct R17/R18 frozen schemas,
allowed nested candidate identity to participate in validator selection, left
one ledger reference stale, did not version the expanded failed-amendment
review, and omitted failed R15 from one historical-negative description. R20
supersedes only the failed amendments and authorizes one separately accepted R20 corrective entry and implementation
successor on the same linear lineage. Accepted REV is only a
context co-member: workspace co-membership and Git ancestry are explicitly not
WP-predecessor or dependency relationships.

## 1. Controlled baseline and custody

The R20 candidate commit is the exact commit containing these WP and Pulse 34
bytes. The R20 acceptance commit must have that exact R20 candidate commit as
its sole first parent; the R20 corrective-entry commit must be its later
direct governance successor; and the corrective implementation commit must be
the direct child of that R20 corrective entry. Accepted `WP-WS-001` exit
`cd1f1d75ec312789fed63a265219d8ad9069a17a` remains the sole logical WP
predecessor. Any nonlinear implementation ancestry, dirty unrelated path,
predecessor digest, or Cargo edge holds acceptance and entry.

| Controlled artifact | Exact identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R1 governance commit (not accepted) | `62116481b7b3e7d671517b6053c8cc3f20f93fce` |
| Retained R1 WP SHA-256 | `93ea15ea87b140b7e45ae67db5a4133e24e8f18778db1ce41a891042b1157554` |
| Retained R1 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` / `f52849a3908decbae20724986026ac42c00d7e938d7d193a1c17fd8eb0a9a80e` / `b387aa17a9d50ca510c52552a83bb6d0581cbccd` at R1 commit |
| Retained R2 governance commit (not accepted) | `21c8066445c72358a444c0b506422ec3b9dc63e0` |
| Retained R2 WP SHA-256 / blob | `4ecd246d67bb5d07c94496a9975c99cdc8488295e8e74235be29391b3725e146` / `47687aff86c392b7e30b237de1015b9d304d4fc4` |
| Retained R2 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` / `2a2868748ce53369d68e6978b8d3d02d3a684d7aec98f6f5f0c3d6fea9a2110a` / `88b11c49e7d8ced29e1ebcb40f68bf5dc6b519ad` at R2 commit |
| Retained R3 governance commit (not accepted) | `ae64448e98744668e3b80e3411255503bfbdd4ae` |
| Retained R3 WP SHA-256 / blob | `76f259e3189cbb53be5e88b84dc922a13673ec52572efbe842f55fe85a67c2ae` / `655f38734b4f52ed7ff740fd2117c3cd5916f977` |
| Retained R3 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` / `954e17de5d0833d98f0a44c932476af60c0163b126c50e1be741646ee8d65bc4` / `4730684c910689009d2b81604c021b91862264ae` at R3 commit |
| Retained R4 governance commit (not accepted) | `b919512fb73472149afea5a55d1a022bf6aec8da` |
| Retained R4 WP SHA-256 / blob | `eaff0bd15d34afb533306ab5a4a967cb672149422e14b634ae263fea512f4f70` / `18e616868d9f94b97264e4b744961d85b6442f3d` |
| Retained R4 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` / `42416f3d638d06e4930413f7e3ed5ef211143f6de19ee6f31cf4eb70d3ac434d` / `2d211769e4adfb4d9d0b6171909cdeb947d76492` at R4 commit |
| Retained R5 governance commit (not accepted) | `77e0abb94a427a1f824e4f5659e580b1aae74137` |
| Retained R5 WP SHA-256 / blob | `c618af6d61d05c51fe689a791f7a8bc9f2ed908c4c42e7e48dd07badec2a633d` / `42f8ff4bd0e9350ac269b0a3a137209b1be1f120` |
| Retained R5 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` / `f231cc4684943275771cd06056abad619b8d5d8ea6c2587de52776a9da114382` / `30b7c92da8e21de9e7177779ef6d4e4127f095a4` at R5 commit |
| Retained R6 governance commit (not accepted) | `1e0157aca9e20eb78cf1cd345fa5cc5bfc5729f3` |
| Retained R6 WP SHA-256 / blob | `e155df20adac753a6b92cf2f36205233626c98551b0c7ac8459ff3a975dd0ced` / `34ba2c245c49478e32186fb5f7e4581e4a755847` |
| Retained R6 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-18-wp-tst-001-r6-candidate.md` / `cfc4b2268fd45682d655d3543f1daa06889baea16dad4c3bf370c37a6760fe4b` / `8944d7b91e7abe90fff004433d613465b601b75a` at R6 commit |
| Retained R7 governance commit (not accepted) | `3550c5deece2ec97207fbe8c1b4dda4c44d62a97` |
| Retained R7 WP SHA-256 / blob | `e9b2c5e82a31eb6ee172f35fe06b2db46f3affcebae0f8b5391264cca59644ff` / `89ab301d7e3596fbb32678b84c1356a41b3f2bc0` |
| Retained R7 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-19-wp-tst-001-r7-candidate.md` / `ec12c55fa7a08d3e764c596b25ce5af96daafcef9a0a314eb9e2cded31aeba13` / `97249bc4a20bc7dd4dff4950dd169c78cf25590a` at R7 commit |
| Retained R8 governance commit (not accepted) | `264cff6959c74f4e9430fda3ca9e72b529da318a` |
| Retained R8 WP SHA-256 / blob | `3165853787462ede6a39c154b060accd8d4ab43e83a36ea1373e5d01aaf86de7` / `41a279ad001ef327c8beb2b76d1ac5d2cb84e540` |
| Retained R8 pulse SHA-256 / blob | `6e7344ec728788dd7df8289466a888ae4f545416fc18b09dd5d693578ce135fe` / `860d5da041418e08b79c4e6b33074f1fdc903292` |
| Retained R9 governance commit (not accepted) | `69dc2f86783c3bf35cdc1367b2ed787c5da423a9` |
| Retained R9 WP SHA-256 / blob | `402d4704b5cdc3de593039090478f12ec2ed2f93cf9ce8db222deed6309f82b4` / `1fbe238b42b56b0e3d87590f767e1ce02d89b0cf` |
| Retained R9 pulse SHA-256 / blob | `5e7e69f2b9e22227336b60dbceeb8e3b76ed804bf28f849d11e522788299560a` / `a098cead061bce86ff28b33741d6dfe1fbb081ef` |
| Retained R10 governance commit (not accepted) | `07cdba818ae4dc1120780104995b43143a6bee16` |
| Retained R10 WP SHA-256 / blob | `6149e23aba203bb529d13936f872781f98202094984fac7a9c74e0279b700a1b` / `a6f74dcbc319921a128dda1562b821995b620735` |
| Retained R10 pulse SHA-256 / blob | `e08e93750045e894dc976126d236849a10c37bbbfb2fc6dcc89594a0916b0da8` / `cabee5936027bc005b5bd58287f3939bab5f4e82` |
| Retained R11 governance commit (not accepted) | `2cc8ef35d99a2b49878dce2943b639991df1feff` |
| Retained R11 WP SHA-256 / blob | `b3082d4853c64c7f0f7505112ccb9bb22d504cdf61e21126f8f27a0c6a5e3b9e` / `f2f20f2fda1c73b3cd0924b1bc1a2c06867043e9` |
| Retained R11 pulse SHA-256 / blob | `8bb730b00a59634259d835e2fb82fe4346c035f9bb870bb252e5572d48c17f7c` / `6db09b5a99b328edd53cd03514b80e34049e2a11` |
| Retained R12 WP SHA-256 / blob (not accepted) | `b6654694983513c99730ceb0a900f44a288f26a845db41bf4ec1a7395bb193aa` / `64c051acc6b5724661b7e7181a35deca219d5ac7` |
| Retained R12 pulse SHA-256 / blob | `8723e1cd1759f7ad29a5b0366e310b411e73a37f0971b779c3cb19edee9f2ba6` / `a9cb2cbecefa066a60602a341013443147650ab1` |
| Retained R12 governance commit (not accepted) | `cfb466029d759919c0f8ef5e6ab7a7fe3c1aab3c` |
| Retained R13 WP SHA-256 / blob (not accepted) | `6bebe5ec95924acbc02f90d14869695ea8d7bc19b331c16c7a34a27b0cbca5a9` / `677191e393d275ddb1044ccb7369e667f44b22cb` |
| Retained R13 pulse SHA-256 / blob | `47a1fb382babd7872b7394660bb1d12dd530995f9eab9ff20808b501407d5d9c` / `536b759b15254c8b79143e4319b0922df4885a22` |
| Retained R13 governance commit (not accepted) | `3166b0cf86af45f3fd04454ff1c9734cac37843d` |
| Accepted R14 candidate commit / WP SHA-256 / blob | `0705a2b42228e865c92a2da7ea2bfc82489bf49e` / `0c909cb0aab010d4b936c93ae770ebf98fdabc421b5c4883ba967ef6a5c6955b` / `182b36ffba985c7e8d432bb5a3b18aa0b76a557a` |
| R14 acceptance commit / Pulse 27 SHA-256 | `8bb1140925688f32a4997926567919997e9bf3f9` / `2624c41bc933753f303c845061d177ed79ab9398d3d3d85e5c7a8f78a0709d8f` |
| R14 entry commit / Pulse 28 SHA-256 | `6354f5184b97923571dcd397ac9871167833e86e` / `84bdc53fed341919db253d2799a2d7119a965fce5b505dd28df0fec840f5c035` |
| Retained failed R14 implementation commit / tree | `7e4591838dfffdc8d1fc35f0e97e77133a56490b` / `53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f` |
| Failed implementation disposition | independent audit: four critical and two major findings; no promotable evidence; no exit |
| Retained failed R15 amendment commit / parent | `060ddce2e7c0500d162282e928c624fc5f0e0753` / `7e4591838dfffdc8d1fc35f0e97e77133a56490b` |
| Retained failed R15 WP SHA-256 / blob | `b38dd43763cc705771ef4bf2ff7c838c81b4c16ae58f79a319ac8f92fe95b6cb` / `99ddada77364e46cd82ab9b37a1eeac68c9dfb4a` |
| Retained failed R15 candidate pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-29-wp-tst-001-r15-corrective-lineage-candidate.md` / `f2007ab6a6031fdaafd9cc696fd2dcb9e12b903d995fac396a086a4910da1e7e` / `3ee4d9aa06a0b603410b58f2bb12924873fb061d` |
| Failed R15 disposition | VTRACE finding; not accepted; not entered; no implementation authority; no evidence or exit |
| Retained failed R16 amendment commit / parent | `be689ae15837b6f5ce0c24a89ee66a6b5939aa75` / `060ddce2e7c0500d162282e928c624fc5f0e0753` |
| Retained failed R16 WP SHA-256 / blob | `f276da805c02d8ac17a8eb8c2f3f11bc56b04191bdc4705234dd9118e2b97477` / `b85cedfc6fd0290038ecfeebb25ac61f97166ca2` |
| Retained failed R16 pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-30-wp-tst-001-r16-corrective-lineage-candidate.md` / `d8c709214f4597274d929a45bd0ea0975987c27d0358243c3b31f7c97eee28a2` / `5b31c06a55fc35c383b6690b310b420bdbaaaeba` |
| Failed R16 disposition | VTRACE finding; not accepted; not entered; no implementation authority; no evidence or exit |
| Retained failed R17 amendment commit / parent | `e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e` / `be689ae15837b6f5ce0c24a89ee66a6b5939aa75` |
| Retained failed R17 WP SHA-256 / blob | `21a9260a114477503f8be588a76869a680972226ff9c585414123402f38d73ee` / `84d1ce2f83a4e0ce9804c8b8390006ee8c1b8d32` |
| Retained failed R17 pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-31-wp-tst-001-r17-corrective-lineage-candidate.md` / `4f2f5e7d2b94687bf120e3b566d4d9ba6df9dc19d2c3030ef3739a3a98f89bc6` / `1a7dd03c9462059cd2e4280ddc54009a5ef750a5` |
| Failed R17 disposition | VTRACE finding; not accepted; not entered; no implementation authority; no evidence or exit |
| Retained failed R18 amendment commit / parent | `97716b9165cc4b8f0e6a51010376b0a8e46cc169` / `e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e` |
| Retained failed R18 WP SHA-256 / blob | `785a7ec20b07ba4e52ce6a6ed446c12aa13efc055225f46f82f71d98c6b63a9f` / `99e1aee5250052cbf534db9a4f1a66575caaa256` |
| Retained failed R18 pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-32-wp-tst-001-r18-corrective-lineage-candidate.md` / `67b168c99c106daeeb10b9250bd8441454fd97685778a692fa354fd714da1ddf` / `643d51c127354f4892367090cad5b3a8ce297c67` |
| Failed R18 disposition | VTRACE finding; not accepted; not entered; no implementation authority; no evidence or exit |
| Retained failed R19 amendment commit / parent | `4602ced667aa1188133429c2011d57736d203a72` / `97716b9165cc4b8f0e6a51010376b0a8e46cc169` |
| Retained failed R19 WP SHA-256 / blob | `33763165576e450132a98547f8d23d5746c96c7c1f95b8688ccf8b5a6766b8a4` / `daac24b6eaa19e22979b77f6f2b7fbab215a3507` |
| Retained failed R19 pulse path / SHA-256 / blob | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-33-wp-tst-001-r19-corrective-lineage-candidate.md` / `a5edc0c0f217581fe8d46329eea6c7c7ed967df75e70d54a0b52743c26e4e3fe` / `9126b40d140c6b9c272f3440ff31b3a574f9a7e4` |
| Failed R19 disposition | combined independent-review findings; not accepted; not entered; no implementation authority; no evidence or exit |
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

For each retained R1–R7 pulse row, `git cat-file -e <commit>:<path>` must
succeed, `git rev-parse <commit>:<path>` must equal the listed blob, and
SHA-256 of the exact raw `git cat-file blob <blob>` bytes must equal the listed
SHA. The commit:path pair, not the reused basename or current worktree file,
selects the version. R1–R5 intentionally bind five distinct immutable blobs at
the same Pulse 17 path; R6 and R7 bind their versioned Pulse 18 and 19 paths.
No later pulse bytes, rename, reconstruction, or prose copy satisfies a row.

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
original R14 implementation commit is retained as a failed direct child of the
R14 entry and is never a promotable implementation result or evidence base.
The R15, R16, R17, R18, and R19 amendment commits are retained as failed governance and
authorize nothing. A later R20 acceptance and corrective entry authorize exactly one
corrective implementation commit as the direct non-merge child of the R20
corrective entry. That corrective commit may change only the same 18 paths
above.

Two deltas are mandatory and distinct:

1. `complete_implementation_delta` is the exact projection from original R14
   entry `6354f5184b97923571dcd397ac9871167833e86e` to the corrective
   implementation commit over the 18 literal path arguments above. Its rows
   contain exactly those 18 paths, each `A|M` as fixed by R14, once, in Git's
   unsigned raw-path-byte emission order. Intervening governance paths are excluded only by
   those explicit path arguments and are separately bound below; an unscoped
   diff, inferred pathset, glob, negative pathspec, or post-filter is invalid.
2. `corrective_delta` is the exact projection from the accepted R20
   corrective-entry commit to its direct corrective child over the same 18
   literal path arguments. It contains a nonempty subset of those paths, each
   `A|M`, once, in unsigned raw-path-byte order and no other path.
3. `governance_delta` is the complete unprojected governance path sequence
   from failed implementation through R20 corrective entry. It binds the R15,
   R16, R17, R18, and R19 failed amendments, R20 candidate, future R20 acceptance, and future R20
   corrective-entry commits and proves that none changes an implementation
   path.

The exact common Git prefix `GIT-C` is the literal array
`["git","-C","<absolute-repo-root>","-c","core.autocrlf=false","-c","core.safecrlf=false","-c","core.quotepath=false","-c","core.excludesFile=NUL","-c","core.attributesFile=NUL","-c","diff.renames=true","-c","status.renames=true","-c","diff.renameLimit=0","-c","diff.algorithm=myers","-c","status.relativePaths=false","-c","core.precomposeUnicode=false","-c","core.ignoreCase=false","-c","submodule.recurse=false"]`.
`IMPL-PATHS` expands, without joining or sorting, to 18 separate arguments in
the section 3 code-block order. Every implementation projection uses exactly
those 18 literal path arguments. The expressly named
`corrective_delta_unscoped_raw` command is the sole exception: it has no
pathspec and exists only to prove that the corrective child has no extra path;
it never supplies implementation rows or either manifest. These are the only
normative arrays:

```json
{
  "complete_delta_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","6354f5184b97923571dcd397ac9871167833e86e","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "complete_delta_binary": ["<GIT-C>","diff","--binary","--full-index","--no-ext-diff","--find-renames=100%","6354f5184b97923571dcd397ac9871167833e86e","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "corrective_delta_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","<corrective_entry_commit>","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "corrective_delta_binary": ["<GIT-C>","diff","--binary","--full-index","--no-ext-diff","--find-renames=100%","<corrective_entry_commit>","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "corrective_delta_unscoped_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","<corrective_entry_commit>","<corrective_implementation_commit>"],
  "governance_r15_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","7e4591838dfffdc8d1fc35f0e97e77133a56490b","060ddce2e7c0500d162282e928c624fc5f0e0753","--","docs/vtrace/WP_TST_001.md","context/waves/2026-07-28-bastion-foundation/pulses/pulse-29-wp-tst-001-r15-corrective-lineage-candidate.md"],
  "governance_r16_failed_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","060ddce2e7c0500d162282e928c624fc5f0e0753","be689ae15837b6f5ce0c24a89ee66a6b5939aa75","--","docs/vtrace/WP_TST_001.md","context/waves/2026-07-28-bastion-foundation/pulses/pulse-30-wp-tst-001-r16-corrective-lineage-candidate.md"],
  "governance_r17_failed_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","be689ae15837b6f5ce0c24a89ee66a6b5939aa75","e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e","--","docs/vtrace/WP_TST_001.md","context/waves/2026-07-28-bastion-foundation/pulses/pulse-31-wp-tst-001-r17-corrective-lineage-candidate.md"],
  "governance_r18_failed_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e","97716b9165cc4b8f0e6a51010376b0a8e46cc169","--","docs/vtrace/WP_TST_001.md","context/waves/2026-07-28-bastion-foundation/pulses/pulse-32-wp-tst-001-r18-corrective-lineage-candidate.md"],
  "governance_r19_failed_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","97716b9165cc4b8f0e6a51010376b0a8e46cc169","4602ced667aa1188133429c2011d57736d203a72","--","docs/vtrace/WP_TST_001.md","context/waves/2026-07-28-bastion-foundation/pulses/pulse-33-wp-tst-001-r19-corrective-lineage-candidate.md"],
  "governance_r20_candidate_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","4602ced667aa1188133429c2011d57736d203a72","<r20_candidate_commit>","--","docs/vtrace/WP_TST_001.md","context/waves/2026-07-28-bastion-foundation/pulses/pulse-34-wp-tst-001-r20-corrective-lineage-candidate.md"],
  "governance_r20_acceptance_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","<r20_candidate_commit>","<r20_acceptance_commit>","--","context/waves/2026-07-28-bastion-foundation/pulses/pulse-35-wp-tst-001-r20-acceptance.md"],
  "r20_acceptance_commit_object": ["<GIT-C>","cat-file","commit","<r20_acceptance_commit>"],
  "corrective_entry_commit_object": ["<GIT-C>","cat-file","commit","<corrective_entry_commit>"],
  "corrective_implementation_commit_object": ["<GIT-C>","cat-file","commit","<corrective_implementation_commit>"],
  "governance_r20_entry_raw": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","<r20_acceptance_commit>","<corrective_entry_commit>","--","context/waves/2026-07-28-bastion-foundation/pulses/pulse-36-wp-tst-001-r20-corrective-entry.md"]
}
```

Angle-bracket tokens in that display are specification aliases expanded to the
single already bound value or complete array before execution and retention;
no angle bracket is serialized. Every invocation uses the section 6 sanitized
Git environment and absolute root cwd. Flattening is exact array concatenation:
`complete_delta_raw = GIT-C || ["diff",...,"--"] || IMPL-PATHS`, and likewise
for the other three projected arrays; `corrective_delta_unscoped_raw` is the
literal no-pathspec exception above; governance arrays are `GIT-C` followed by
every displayed remaining string. The retained `raw_argv`/`binary_argv` are the
fully expanded arrays, never the displayed aliases. Their independent argv
digest hashes exact UTF-8
`schema=test-gate-argv.v1<LF>command_id=<command-id><LF>` followed by one
`<zero-based-decimal-index><TAB><UTF-8-byte-count><TAB><argument><LF>` per
expanded argument. This gives a mechanical equality check for every argument,
including the 18 distinct path arguments and forbids joined path lists.

The exact projected status/path inventory is:

```text
M Cargo.lock
M Cargo.toml
A crates/bastion-boundary-tests/Cargo.toml
A crates/bastion-boundary-tests/tests/adversarial_cases.rs
A crates/bastion-boundary-tests/tests/contract_matrix.rs
A crates/bastion-boundary-tests/tests/hold_closure.rs
A crates/bastion-boundary-tests/tests/model_cases.rs
A crates/bastion-boundary-tests/tests/no_authority_surface.rs
A crates/bastion-boundary-tests/tests/property_cases.rs
A crates/bastion-boundary-tests/tests/source_spine.rs
A crates/bastion-boundary-tests/tests/static_surface.rs
A crates/bastion-boundary-tests/tests/support/mod.rs
A fixtures/bootstrap/cases/absent.fixture
A fixtures/bootstrap/cases/deny-marker.fixture
A fixtures/bootstrap/cases/stale.fixture
A fixtures/bootstrap/cases/valid.fixture
A fixtures/bootstrap/manifest.tsv
A tools/test_gate.ps1
```

The complete delta must equal all 18 rows. The corrective delta must equal the
subsequence whose corrective-entry and corrective-child object IDs differ;
that subsequence is nonempty, raw-path-byte ordered, and no caller selects it.
The independently retained `corrective_delta_unscoped_raw` stdout must be
byte-identical to `corrective_delta_raw` stdout. It proves only that this
subsequence is the complete R20-entry-to-child change and contains no extra
path; it never supplies implementation rows or either manifest. The
delta preimage is exact UTF-8
`schema=test-gate-delta.v1<LF>kind=<kind><LF>base=<base><LF>target=<target><LF>`
followed by every canonical raw-diff row as
`<status><TAB><path><TAB><old-oid><TAB><new-oid><LF>` and then
`binary_sha256=<binary-stdout-sha256><LF>`. `path` is canonical padded base64
of raw Git path bytes. `complete` has 18 rows; `corrective` has `1..18` rows.
The SHA-256 of that entire preimage is the delta digest.

The governance allowlist is exact and exhaustive: R15, failed R16, failed R17,
failed R18, failed R19, and the R20 candidate each have the two paths named by their exact arrays;
acceptance has only Pulse 35; corrective entry has only Pulse 36. Each
governance observation must show exactly its stated `M/A`, `M/A`, `M/A`,
`M/A`, `M/A`, `M/A`, `A`, or `A` path inventory, respectively, and the union is
disjoint from the 18 implementation paths. The unscoped raw diff for each
adjacent commit pair is additionally required to be byte-identical to its
scoped raw diff, preventing a hidden excluded governance path.

Each adjacent pair therefore has a second exact unscoped argv formed as
`GIT-C || ["diff","--raw","-z","--no-abbrev","--full-index",
"--no-ext-diff","--find-renames=100%",<base>,<target>]`
with no `--` or path argument. Bases/targets, in order, are exactly
`7e4591838dfffdc8d1fc35f0e97e77133a56490b -> 060ddce2e7c0500d162282e928c624fc5f0e0753`,
`060ddce2e7c0500d162282e928c624fc5f0e0753 -> be689ae15837b6f5ce0c24a89ee66a6b5939aa75`,
`be689ae15837b6f5ce0c24a89ee66a6b5939aa75 -> e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e`,
`e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e -> 97716b9165cc4b8f0e6a51010376b0a8e46cc169`,
`97716b9165cc4b8f0e6a51010376b0a8e46cc169 -> 4602ced667aa1188133429c2011d57736d203a72`,
`4602ced667aa1188133429c2011d57736d203a72 -> <r20_candidate_commit>`,
`<r20_candidate_commit> -> <r20_acceptance_commit>`, and
`<r20_acceptance_commit> -> <corrective_entry_commit>`, expanded to the full
40-lowercase-hex values already bound by the corresponding scoped array. The
retained command IDs are the scoped ID plus suffix `_unscoped`; their argv
digests use the same `test-gate-argv.v1` preimage. Native/portable exit,
complete raw stdout/stderr bytes, counts, and hashes are retained separately;
both exits are zero and scoped/unscoped stdout is byte-identical. Because every
unscoped no-extra command has no diff filter, any unauthorized or unsupported
status—including `T`—appears there, makes the equality fail, and rejects. No
status is hidden by a favorable path/status projection. Unscoped raw parsing is
total over Git's raw status alphabet `A|Cnnn|D|M|Rnnn|T|U|X|B`; `nnn` is exactly
three decimal digits. Every parsed status outside the pair's exact expected
`A|M` inventory is unauthorized and rejects, and every unknown letter, missing
score, malformed arity, or unsupported combined form is a terminal parse
failure rather than an ignored row.

The exact governance rows are respectively `M docs/vtrace/WP_TST_001.md`,
`A .../pulse-29-wp-tst-001-r15-corrective-lineage-candidate.md`;
`M docs/vtrace/WP_TST_001.md`,
`A .../pulse-30-wp-tst-001-r16-corrective-lineage-candidate.md`;
`M docs/vtrace/WP_TST_001.md`,
`A .../pulse-31-wp-tst-001-r17-corrective-lineage-candidate.md`;
`M docs/vtrace/WP_TST_001.md`,
`A .../pulse-32-wp-tst-001-r18-corrective-lineage-candidate.md`;
`M docs/vtrace/WP_TST_001.md`,
`A .../pulse-33-wp-tst-001-r19-corrective-lineage-candidate.md`;
`M docs/vtrace/WP_TST_001.md`,
`A .../pulse-34-wp-tst-001-r20-corrective-lineage-candidate.md`;
`A .../pulse-35-wp-tst-001-r20-acceptance.md`; and
`A .../pulse-36-wp-tst-001-r20-corrective-entry.md`, where each `.../` expands
only to `context/waves/2026-07-28-bastion-foundation/pulses/`. No alias is
retained in a row.

### 3.1 R20 lineage objects and externally selected schema-version closure

R20 replaces, rather than informally extending, every affected
R14/R15/R16/R17/R18/R19 record. Validator selection is not data-directed by a
nested member. Before any nested parse, the supervisor obtains an external
dispatch tuple `(artifact_kind,outer_schema,wp_revision,candidate_commit)`:
artifact kind comes from the already classified closed path/kind grammar;
outer schema and WP revision come from a bounded top-level envelope read; and
candidate commit comes only from the verified governance context (the exact
acceptance parent/current candidate for R20 or the frozen custody row for a
historical revision). It then resolves
`<candidate_commit>:docs/vtrace/WP_TST_001.md` with the sanitized Git object
commands, requires the resolved blob ID and SHA-256 of complete raw blob bytes
to equal the frozen row below, and only then selects the one immutable validator
for the complete tuple. A candidate commit copied from `lineage_binding`, any
other nested member, caller input, or working-tree bytes cannot select or alter
the validator. Tuple mismatch rejects before nested parsing.

Frozen dispatch identities are R17 commit/WP SHA/blob
`e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e` /
`21a9260a114477503f8be588a76869a680972226ff9c585414123402f38d73ee` /
`84d1ce2f83a4e0ce9804c8b8390006ee8c1b8d32`; R18
`97716b9165cc4b8f0e6a51010376b0a8e46cc169` /
`785a7ec20b07ba4e52ce6a6ed446c12aa13efc055225f46f82f71d98c6b63a9f` /
`99e1aee5250052cbf534db9a4f1a66575caaa256`; and R19
`4602ced667aa1188133429c2011d57736d203a72` /
`33763165576e450132a98547f8d23d5746c96c7c1f95b8688ccf8b5a6766b8a4` /
`daac24b6eaa19e22979b77f6f2b7fbab215a3507`. R20 uses the exact committed
candidate selected by its future acceptance-parent proof and the WP digest/blob
committed with Pulse 34. R14/R15/R16 use their section 1 frozen commit/WP
SHA/blob rows and original validators.

| Affected closed artifact | Frozen R17 | Frozen R18 | Frozen R19 | Current R20 |
|---|---|---|---|---|
| Mode evidence | `test-gate-evidence.v12` | `test-gate-evidence.v12` | `test-gate-evidence.v13` | `test-gate-evidence.v13` |
| Set evidence | `test-gate-evidence-set.v11` | `test-gate-evidence-set.v11` | `test-gate-evidence-set.v12` | `test-gate-evidence-set.v12` |
| Ledger | `test-gate-ledger.v3` | `test-gate-ledger.v3` | `test-gate-ledger.v4` | `test-gate-ledger.v4` |
| Publication receipt | `test-gate-publication-receipt.v2` | `test-gate-publication-receipt.v2` | `test-gate-publication-receipt.v3` | `test-gate-publication-receipt.v3` |
| Publication finalization | `test-gate-publication-finalization.v2` | `test-gate-publication-finalization.v2` | `test-gate-publication-finalization.v3` | `test-gate-publication-finalization.v3` |
| Decision | `test-gate-decision.v2` | `test-gate-decision.v2` | `test-gate-decision.v3` | `test-gate-decision.v3` |
| Review authentication | `vtrace-review-auth.v2` | `vtrace-review-auth.v2` | `vtrace-review-auth.v3` | `vtrace-review-auth.v3` |
| Author binding | `vtrace-author-binding.v2` | `vtrace-author-binding.v2` | `vtrace-author-binding.v3` | `vtrace-author-binding.v3` |
| Identity registry | `vtrace-identity-registry.v2` | `vtrace-identity-registry.v2` | `vtrace-identity-registry.v3` | `vtrace-identity-registry.v3` |
| Acceptance/entry author custody | `vtrace-author-custody.v2` | `vtrace-author-custody.v2` | `vtrace-author-custody.v3` | `vtrace-author-custody.v3` |
| Evidence custody preimage | `vtrace-evidence-custody.v1` | `vtrace-evidence-custody.v1` | `vtrace-evidence-custody.v2` | `vtrace-evidence-custody.v2` |
| Corrective-lineage intent | `corrective-lineage-intent.v2` | `corrective-lineage-intent.v3` | `corrective-lineage-intent.v4` | `corrective-lineage-intent.v5` |
| Governance delta | `test-gate-governance-delta.v2` | `test-gate-governance-delta.v3` | `test-gate-governance-delta.v4` | `test-gate-governance-delta.v5` |
| Failed-amendment review | `test-gate-failed-amendment-review.v1` | `test-gate-failed-amendment-review.v1` | `test-gate-failed-amendment-review.v1` | `test-gate-failed-amendment-review.v2` |
| Failed amendment | `test-gate-failed-amendment.v2` | `test-gate-failed-amendment.v3` | `test-gate-failed-amendment.v4` | `test-gate-failed-amendment.v5` |
| Corrective lineage | `test-gate-corrective-lineage.v2` | `test-gate-corrective-lineage.v3` | `test-gate-corrective-lineage.v4` | `test-gate-corrective-lineage.v5` |
| Acceptance binding | `test-gate-acceptance-binding.v3` | `test-gate-acceptance-binding.v4` | `test-gate-acceptance-binding.v5` | `test-gate-acceptance-binding.v5` |
| Corrective-entry binding | `test-gate-corrective-entry-binding.v3` | `test-gate-corrective-entry-binding.v4` | `test-gate-corrective-entry-binding.v5` | `test-gate-corrective-entry-binding.v5` |
| Implementation binding | `test-gate-implementation-binding.v3` | `test-gate-implementation-binding.v4` | `test-gate-implementation-binding.v5` | `test-gate-implementation-binding.v6` |

Historical records validate only under the exact tuple-selected frozen validator
and digest preimages in this matrix; R17 and R18 are deliberately distinct.
R20 retains an R19 schema literal exactly where that artifact's own ordered
members and value algebra are unchanged; the external tuple still selects a
distinct revision-specific nested validator. Versions advance only for the
expanded intent, governance delta, failed review/binding, corrective lineage,
and implementation-binding contracts. The current R20 validator accepts only
the Current R20 column for its externally verified tuple. Cross-revision relabel,
old outer/new nested bytes, new outer/old nested bytes, commit/revision/schema
substitution, or parsing nested candidate identity before external dispatch
rejects. Generic observation, argv, raw delta, failed-attempt,
structured-result, and other types not listed above retain their existing
schema because R20 changes none of their ordered members, enums, nested exact
shapes, or digest preimages.
The following types are closed; listed order is canonical key order and no
additional or omitted member is valid:

| Type/schema | Exact ordered members and rule |
|---|---|
| `GovernanceGitObservation` / `test-gate-governance-git-observation.v1` | `schema,command_id,argv,native_exit_u32,portable_exit,stdout_byte_count,stdout_sha256,stderr_byte_count,stderr_sha256,observation_digest`; argv is one exact scoped/unscoped array above, exits are `0,0`, counts/hash bind complete raw streams, and digest is last. |
| `DeltaBinding` / `test-gate-delta-binding.v1` | `schema,kind,base_commit,target_commit,raw_observation,binary_observation,rows,row_count,delta_digest`; kind is `complete-implementation|corrective-implementation`; observations use the same ordered members/types as `GovernanceGitObservation` but schemas `test-gate-delta-git-observation.v1` and exact raw/binary argv above; rows use `status,path,old_oid,new_oid` in unsigned raw-path-byte Git emission order and exactly parse raw stdout; binary stdout SHA supplies the delta preimage; digest is last and hashes the defined delta preimage, not JSON. |
| `CommitParentObservation` / `test-gate-commit-parent-observation.v1` | `schema,command_id,argv,native_exit_u32,portable_exit,stdout_byte_count,stdout_sha256,stderr_byte_count,stderr_sha256,commit,tree,parent_count,first_parent,parse_status,observation_digest`; argv is exact `GIT-C || ["cat-file","commit",commit]`; exits are `0,0`; stream counts/hashes bind complete raw bytes; commit/tree/parent are lowercase `GIT_ID`; parent count is literal `1`; parse status is `complete`; digest is last and hashes canonical JSON with only itself and its preceding comma omitted. |
| `AcceptanceCommitObservation` / `test-gate-commit-parent-observation.v1` | the acceptance-role use of the one shared `CommitParentObservation` schema above, with no alternate keys or preimage; command ID is `r20_acceptance_commit_object`, commit is R20 acceptance, and first parent is the exact externally selected R20 candidate. Entry and implementation use that identical schema with only their formula-bound command/commit/tree/parent values changed. |
| `GovernanceDeltaBinding` / `test-gate-governance-delta.v5` | `schema,r15_failed_amendment_commit,r15_paths,r15_scoped,r15_unscoped,r16_failed_amendment_commit,r16_paths,r16_scoped,r16_unscoped,r17_failed_amendment_commit,r17_paths,r17_scoped,r17_unscoped,r18_failed_amendment_commit,r18_paths,r18_scoped,r18_unscoped,r19_failed_amendment_commit,r19_paths,r19_scoped,r19_unscoped,r20_candidate_commit,r20_candidate_paths,r20_candidate_scoped,r20_candidate_unscoped,r20_acceptance_commit,r20_acceptance_paths,r20_acceptance_scoped,r20_acceptance_unscoped,corrective_entry_commit,corrective_entry_paths,corrective_entry_scoped,corrective_entry_unscoped,aggregate_digest`; path arrays are exact; each unscoped observation uses no diff filter and must be byte-identical to its scoped peer; aggregate hashes all sixteen observations then commit/path rows in field order; digest is last. |
| `FailedAttemptBinding` / `test-gate-failed-attempt.v1` | `schema,commit,tree_digest,first_parent,implementation_paths,implementation_manifest_digest,audit_id,audit_digest,critical_count,major_count,disposition,promotable,evidence_claimed,exit_claimed,binding_digest`; constants are failed commit/tree/R14 entry, exact 18 paths, audit ID `AUDIT-WP-TST-001-R14-IMPLEMENTATION-001`, counts `4,2`, disposition `failed-retained`, and three booleans `false`; audit digest hashes the canonical six-finding audit record; binding digest is last. |
| `FailedAmendmentBinding` / `test-gate-failed-amendment.v5` | same exact ordered keys/rules as v4; exactly five bindings exist in revision order R15,R16,R17,R18,R19 using section 1 identities, reviews `AUDIT-WP-TST-001-R15-AMENDMENT-001`, `...R16...`, `...R17...`, `...R18...`, `...R19...`, actionable counts `5,6,4,1,5`, disposition `failed-retained`, and three false booleans; binding digest is last. |
| `CorrectiveLineageBinding` / `test-gate-corrective-lineage.v5` | `schema,wp_revision,wp_artifact_digest,r20_candidate_commit,acceptance_binding,corrective_entry_binding,original_r14_entry_commit,original_r14_entry_pulse_digest,failed_attempt,failed_amendments,governance_delta,complete_implementation_delta,corrective_delta,corrective_implementation_commit,corrective_implementation_tree,lineage_digest`; revision `R20`; failed amendments exactly `[R15,R16,R17,R18,R19]`; positive WP/acceptance/entry values are R20 only; all nested observations/digests recompute; lineage digest is last. |
| `AcceptanceBindingV5` / `test-gate-acceptance-binding.v5` | `schema,wp_revision,commit,first_parent,commit_observation,pulse_path,pulse_digest,subject_id,subject_digest,binding_digest`; revision `R20`; observation is exact `AcceptanceCommitObservation`, its commit equals this commit and its sole first parent equals the externally selected `r20_candidate_commit`; path Pulse 35, subject `WP-TST-001-R20-ACCEPTANCE`, subject digest equals R20 WP digest; binding digest is last. |
| `CorrectiveEntryBindingV5` / `test-gate-corrective-entry-binding.v5` | `schema,wp_revision,commit,tree_digest,first_parent,commit_observation,pulse_path,pulse_digest,subject_id,acceptance_commit,acceptance_pulse_digest,binding_digest`; revision `R20`; exact `CommitParentObservation` command ID `corrective_entry_commit_object` binds commit/tree and exactly one parent equal to acceptance; path Pulse 36, subject `WP-TST-001-R20-CORRECTIVE-ENTRY`; binding digest is last. |
| `ImplementationBindingV6` / `test-gate-implementation-binding.v6` | `schema,commit,tree_digest,first_parent,commit_observation,allowed_paths,failed_attempt_binding_digest,failed_amendment_binding_digests,governance_delta_digest,complete_implementation_delta,corrective_delta,corrective_no_extra_observation,observed_preflight,binding_digest`; exact `CommitParentObservation` command ID `corrective_implementation_commit_object` binds commit/tree and exactly one parent equal to corrective entry; failed amendment digests are R15,R16,R17,R18,R19; no-extra observation has no diff filter and must equal corrective raw stdout; binding digest is last. |
| `AuthorBindingV3` / `vtrace-author-binding.v3` | `schema,source_kind,source_ref,source_digest,wp_revision,lineage_digest,author_id,controller_id,binding_digest`; source kind is exactly `wp_r20_candidate|r20_acceptance_pulse|r20_corrective_entry_pulse|failed_r14_implementation|corrective_implementation_commit|mode_evidence|set_evidence`; R14/R15/R16/R17/R18/R19 governance is not a positive registry source; failed kind grants no authority; binding digest is last. |
| `IdentityRegistryV3` / `vtrace-identity-registry.v3` | `schema,bindings,candidate_author_ids,failed_author_ids,registry_digest`; bindings are source-kind/source-ref sorted unique `AuthorBindingV3`; candidate IDs project every author/controller ID having at least one positive nonfailed binding, even when the same ID also occurs in a failed binding; failed IDs project every failed-kind ID for visibility and independence collision checks. Failed bindings confer no authority. An ID is excluded from candidate IDs only when all of its bindings are failed-only; registry digest is last. |

Each commit-object command retains the complete stdout and stderr byte streams
through their exact counts and SHA-256 values. Its parser consumes the raw
`git cat-file commit` header block through the first empty LF line, accepts
exactly one `tree <40-lowercase-hex>` header and exactly one
`parent <40-lowercase-hex>` header, permits only Git-defined continuation lines
for other headers, and rejects CR, truncated/non-UTF-8 header bytes, duplicate
tree, zero/multiple parent, abbreviated/uppercase ID, malformed header, trailing
header substitution, nonzero exit, or nonempty stderr. `tree`, `parent_count`,
and `first_parent` are parsed solely from those retained stdout bytes. The
observation digest preimage is the canonical JSON object in the displayed key
order with only `observation_digest` and its preceding comma omitted. Binding
self-digests then embed the complete observation object, not only its digest.
Consequently acceptance-to-entry and entry-to-corrective-child are each proven
by two exact retained commit objects and equality of the later object's sole
parent to the earlier binding's commit.

The audit object referenced by `FailedAttemptBinding` is one closed
`test-gate-failed-implementation-audit.v1` object with exact ordered keys
`schema,audit_id,subject_commit,subject_tree,reviewed_wp_revision,findings,
critical_count,major_count,disposition,promotable,evidence_accepted,exit_accepted,
audit_digest`. The six `findings` are in the following order and each is the
closed object `id,severity,title,affected_paths,behavior,disposition`:

```json
[
  {"id":"BA-TST-R14-IMPL-C01","severity":"critical","title":"required custody/evidence system absent","affected_paths":["tools/test_gate.ps1"],"behavior":"No required immutable materialization, run ledger, evidence record, publication receipt/finalization, repository watch, or durable evidence-set lifecycle was implemented.","disposition":"open-retained"},
  {"id":"BA-TST-R14-IMPL-C02","severity":"critical","title":"AssembleSet unsafe false-pass","affected_paths":["tools/test_gate.ps1"],"behavior":"AssembleSet checks only that sixteen mode directories exist and then emits a promotable passing SET result without parsing or validating the required records, digests, reviews, receipts, finalizations, aggregate, or authority gates.","disposition":"open-retained"},
  {"id":"BA-TST-R14-IMPL-C03","severity":"critical","title":"mode runner nonfunctional after Cargo success: Measure-Object combined_bytes failure","affected_paths":["tools/test_gate.ps1"],"behavior":"After a successful Cargo/assertion command, Measure-Object -Property combined_bytes is applied to ordered dictionaries that do not expose combined_bytes as a measurable object property, so the worker fails instead of producing its pass envelope.","disposition":"open-retained"},
  {"id":"BA-TST-R14-IMPL-C04","severity":"critical","title":"Job-wide bounded execution contract not implemented","affected_paths":["tools/test_gate.ps1"],"behavior":"The runner does not implement the accepted suspended-launch, completion-port association and signaling, whole-Job timeout/output/memory termination, recovery, ledger, and terminal active-process proof contract.","disposition":"open-retained"},
  {"id":"BA-TST-R14-IMPL-M01","severity":"major","title":"required bootstrap cases are labels over a shared smoke test","affected_paths":["crates/bastion-boundary-tests/tests/support/mod.rs","crates/bastion-boundary-tests/tests/source_spine.rs","crates/bastion-boundary-tests/tests/contract_matrix.rs","crates/bastion-boundary-tests/tests/property_cases.rs","crates/bastion-boundary-tests/tests/model_cases.rs","crates/bastion-boundary-tests/tests/adversarial_cases.rs","crates/bastion-boundary-tests/tests/hold_closure.rs","crates/bastion-boundary-tests/tests/no_authority_surface.rs","crates/bastion-boundary-tests/tests/static_surface.rs"],"behavior":"The required per-identity bootstrap cases dispatch labels through one shared smoke assertion rather than prove the allocated source, contract, property, model, adversarial, hold, no-authority, and static obligations.","disposition":"open-retained"},
  {"id":"BA-TST-R14-IMPL-M02","severity":"major","title":"static and supply-chain checks do not prove their WP claims","affected_paths":["tools/test_gate.ps1","crates/bastion-boundary-tests/tests/support/mod.rs","crates/bastion-boundary-tests/tests/static_surface.rs"],"behavior":"The static and supply-chain checks use shallow token/count/self-comparison tests and do not prove the accepted package graph, consumer direction, isolation, call-graph, ambient-state, parser, license/advisory, or complete allowlist claims.","disposition":"open-retained"}
]
```

Those identities, severities, titles, affected-path arrays, behaviors, order,
and `open-retained` dispositions are the original implementation audit and may
not be paraphrased, collapsed, substituted, or replaced by a review-successor
finding.

The failed attempt's `implementation_manifest_digest` is literal
`c238e0e4d0661afec9f7a9c91b883ffadf886e15349ab08982ba1076f91ee9f7`.
It is SHA-256 over these exact UTF-8 LF bytes, including the final LF and the
literal TAB separators displayed as `<TAB>`:

```text
schema=test-gate-implementation-manifest.v1
commit=7e4591838dfffdc8d1fc35f0e97e77133a56490b
tree=53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f
row_count=18
M<TAB>Cargo.lock<TAB>c7cd560a953c4d2020cba3a82b0b83355a56f6ce
M<TAB>Cargo.toml<TAB>36b5c19594ca17433d6f3c9c0294a6124ab719d7
A<TAB>crates/bastion-boundary-tests/Cargo.toml<TAB>60ee758f95d683aa549d5a08f81662f1087c47ab
A<TAB>crates/bastion-boundary-tests/tests/adversarial_cases.rs<TAB>01902893db3d3dda4da1a17289cf1b40d5b6a490
A<TAB>crates/bastion-boundary-tests/tests/contract_matrix.rs<TAB>6e797efcbe09aeb461b420b4f3e721c8ec40dcb9
A<TAB>crates/bastion-boundary-tests/tests/hold_closure.rs<TAB>571a36342dae04b529a38b1a6c9af0eda9d4c8c8
A<TAB>crates/bastion-boundary-tests/tests/model_cases.rs<TAB>11a64d6a367c04fe2be1743a44a0aaaa7ba15f77
A<TAB>crates/bastion-boundary-tests/tests/no_authority_surface.rs<TAB>fb5954d9a4b072bfde7a047a712f818529addf8f
A<TAB>crates/bastion-boundary-tests/tests/property_cases.rs<TAB>02c99108c07c776b8bffc71d5b8f7cf899a4bbe4
A<TAB>crates/bastion-boundary-tests/tests/source_spine.rs<TAB>ad7beed6473b923ed8852f2741950d9ee8dca5c5
A<TAB>crates/bastion-boundary-tests/tests/static_surface.rs<TAB>de7ca445c0e71a9e82bd41e5b56f7744b1bad004
A<TAB>crates/bastion-boundary-tests/tests/support/mod.rs<TAB>4581442d6e1516f13387c0863cc66042d1e24493
A<TAB>fixtures/bootstrap/cases/absent.fixture<TAB>ae580a147119a119d328bfed9206855f40e6414b
A<TAB>fixtures/bootstrap/cases/deny-marker.fixture<TAB>f30679a94d961fb0dcdb3287dd2c7f66d79e8a21
A<TAB>fixtures/bootstrap/cases/stale.fixture<TAB>db2c2d5fd56bc186be62470841332e1cf9eebfe2
A<TAB>fixtures/bootstrap/cases/valid.fixture<TAB>94e864b85f90a8dbaaf294cd3dec29045c6fa3e4
A<TAB>fixtures/bootstrap/manifest.tsv<TAB>118b4ffa62fb2f249021faaf387657d32acf9951
A<TAB>tools/test_gate.ps1<TAB>9362a4b4f284371f431076d1bd07a07955daddd0
```

The audit constants are subject commit/tree from section 1, reviewed revision
`R14`, counts `4,2`, disposition `failed-retained`, and booleans `false,false,
false`; `audit_digest` is last and uses the canonical JSON omission rule. The
R20 corrective evidence may close the technical conditions only by new results;
it never changes any audit finding disposition or failed-attempt boolean. The
failed commit/tree/audit/binding digests are unequal to the corrective commit/
tree/complete-delta/corrective-delta/lineage/evidence/set digests. Any equality,
substitution, omission, favorable projection, accepted evidence pointer, or
exit inclusion is a `binding-mismatch` and non-promotable.

Each amendment review referenced by `FailedAmendmentBinding` dispatches
independently before its findings are parsed: retained R15/R16/R17/R18 is one
frozen `test-gate-failed-amendment-review.v1` object and newly retained R19 is
one `test-gate-failed-amendment-review.v2` object. Both have exact ordered keys
`schema,review_id,subject_commit,findings,actionable_count,disposition,
accepted,entered,authority_created,review_digest`. `findings` is exactly the
ordered string array `["complete-delta-projection-unclosed",
"affected-schema-versions-unclosed","positive-identity-remains-r14",
"corrective-and-governance-observation-commands-unclosed",
"failed-attempt-disposition-unclosed"]`.
Count is `5`, disposition is `failed-retained`, and all booleans are false.
The R16 review has the exact ordered finding array
`["observed-preflight-domain-confusion","failed-author-positive-binding-erasure",
"acceptance-first-parent-unbound","lineage-intent-preimage-incomplete",
"original-implementation-audit-substituted","ledger-version-reference-stale"]`,
count `6`, disposition `failed-retained`, and all booleans false. Each review
digest is last. The R17 review has exact findings
`["entry-and-implementation-parent-observations-unbound",
"unscoped-no-extra-diff-status-filtered","set-dual-bound-identity-ambiguous",
"commit-parent-observation-schema-unclosed"]`, count `4`, disposition
`failed-retained`, and all booleans false. The R18 review has exact findings
`["affected-outer-schema-versions-not-incremented"]`, count `1`, disposition
`failed-retained`, and all booleans false. The R19 review has exact findings
`["frozen-r17-schema-matrix-collapsed",
"validator-dispatch-nested-candidate-identity",
"ledger-version-reference-stale",
"failed-amendment-review-schema-unversioned",
"failed-r15-negative-custody-omitted"]`, count `5`, disposition
`failed-retained`, and all booleans false. R15/R16/R17/R18 retain their exact
frozen v1 review validators; R20 v2 adds only the exact R19 review ID/subject/
finding array/count and selects it through the external dispatch tuple. Neither review nor any failed WP/pulse may appear
as a positive registry source, executable entry, implementation input, review
pass, or promotion claim.

Future Pulse 35 contains exactly one fenced `vtrace-author-custody.v3` block
with LF rows in this order: `subject=WP-TST-001-R20-ACCEPTANCE`,
`wp_revision=R20`, `wp_digest=<R20-WP-DIGEST>`,
`r20_candidate_commit=<R20-CANDIDATE-COMMIT>`,
`acceptance_first_parent=<R20-CANDIDATE-COMMIT>`,
`lineage_intent_digest=<DIGEST>`, `author_id=REV-TST-ACCEPTANCE-AUTHOR`,
`controller_id=REV-TST-GOVERNANCE-CONTROLLER`. The intent digest hashes exact
UTF-8 bytes consisting of every following literal key/value row in exactly this
order, each terminated by LF including the last:

```text
schema=corrective-lineage-intent.v5
wp_revision=R20
wp_digest=<R20-WP-DIGEST>
original_r14_entry_commit=6354f5184b97923571dcd397ac9871167833e86e
original_r14_entry_pulse_digest=84bdc53fed341919db253d2799a2d7119a965fce5b505dd28df0fec840f5c035
failed_r14_implementation_commit=7e4591838dfffdc8d1fc35f0e97e77133a56490b
failed_r14_implementation_tree=53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f
failed_r14_implementation_manifest_digest=c238e0e4d0661afec9f7a9c91b883ffadf886e15349ab08982ba1076f91ee9f7
failed_r15_amendment_commit=060ddce2e7c0500d162282e928c624fc5f0e0753
failed_r16_amendment_commit=be689ae15837b6f5ce0c24a89ee66a6b5939aa75
failed_r17_amendment_commit=e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e
failed_r18_amendment_commit=97716b9165cc4b8f0e6a51010376b0a8e46cc169
failed_r19_amendment_commit=4602ced667aa1188133429c2011d57736d203a72
r20_candidate_commit=<R20-CANDIDATE-COMMIT>
complete_delta_kind=complete-implementation
corrective_delta_kind=corrective-implementation
```

Angle-bracket values expand to the one already bound lowercase digest/commit;
the brackets are not hashed. Pulse 36 contains exactly one fenced
`vtrace-author-custody.v3` block with rows `subject=WP-TST-001-R20-CORRECTIVE-ENTRY`,
`wp_revision=R20`, `wp_digest=<R20-WP-DIGEST>`,
`acceptance_commit=<R20-ACCEPTANCE-COMMIT>`,
`acceptance_pulse_digest=<PULSE-35-DIGEST>`,
`author_id=REV-TST-ENTRY-AUTHOR`,
`controller_id=REV-TST-GOVERNANCE-CONTROLLER`. Neither block contains or hashes
its own containing future commit. Their parsed fields populate only the v5
acceptance/corrective-entry bindings above.

Every self-digest hashes canonical JSON with its final digest member and the
immediately preceding comma omitted. Nested bindings hash independently before
their digest is used. `failed_attempt.binding_digest`,
all three ordered `failed_amendments[*].binding_digest` values, both delta digests, and
`governance_delta.aggregate_digest` must all differ from `lineage_digest`, every
evidence/set digest, and every decision digest. The failed attempt may appear
only in `failed_attempt`, failed-kind registry custody, predecessor/history, or
diagnostic provenance; it may never equal the current implementation,
execution origin, positive author source, expected/actual pass input, receipt,
finalization, set mode record, or exit tree.

All subsequent references to R14 evidence schemas are replaced by the R20
versions and placements in section 8 below. Every R14 path, command, fixture,
bound, trace, behavior, review lane, exit, rollback, and authority restriction
remains normative without weakening. Evidence remains a separate later commit.

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

The following valid JSON object, not the prose table or shell-token parsing, is
the normative exhaustive Mode-to-internal-argv map. Each value is an ordered
array of `1..2` argv arrays; every inner string is a literal process argument.

```json
{
  "L0Format": [["cargo","+1.95.0","fmt","--all","--","--check"]],
  "L0Check": [["cargo","+1.95.0","check","-p","bastion-boundary-tests","--locked","--offline","--all-targets"]],
  "L0FocusedTest": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline"]],
  "L1WorkspaceCheck": [["cargo","+1.95.0","check","--workspace","--locked","--offline","--all-targets"]],
  "L1Clippy": [["cargo","+1.95.0","clippy","--workspace","--locked","--offline","--all-targets","--","-D","warnings"]],
  "L1Test": [["cargo","+1.95.0","test","--workspace","--locked","--offline"]],
  "L1Doc": [["cargo","+1.95.0","doc","--workspace","--locked","--offline","--no-deps"],["cargo","+1.95.0","test","--workspace","--doc","--locked","--offline"]],
  "L1Static": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","static_surface"],["test_gate","assert-static-surface"]],
  "L1SupplyChain": [["cargo","+1.95.0","metadata","--locked","--offline","--no-deps","--format-version","1"],["test_gate","assert-supply-chain-shape"]],
  "L2SourceSpine": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","source_spine"]],
  "L2Contract": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","contract_matrix"]],
  "L2Property": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","property_cases"]],
  "L2Model": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","model_cases"]],
  "L2Adversarial": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","adversarial_cases"]],
  "L2HoldClosure": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","hold_closure"]],
  "L2NoAuthority": [["cargo","+1.95.0","test","-p","bastion-boundary-tests","--locked","--offline","--test","no_authority_surface"]]
}
```

The exact Cargo tuple is `cargo|1.95.0|version-preimage|
ef832b044f264767454d23858c009cdfe9b0cbf53a642f38ff3ffb3152dbe59c`.
The exact specialized-tool tuples are `rustfmt|1.95.0|version-preimage|
ed8180ee07eb513528102ea51ccee3a917b973a4846554589843dcd526cfdd43`
for `L0Format`, `clippy|1.95.0|version-preimage|
73aac2093a1eede5d9f4758102c86d4bd23fb404ca0892d7b590343cb861a404`
for `L1Clippy`, `test_gate|<runner-sha256>|artifact-bytes|<runner-sha256>`
for every internal runner assertion. `version-preimage` is the exact UTF-8 byte sequence
`tool=<tool><LF>version=1.95.0<LF>` with no BOM or other byte. For `artifact-
bytes`, both version and digest are SHA-256 of the exact raw
`tools/test_gate.ps1` artifact bytes with no conversion. A mode containing a
second `test_gate` assertion step retains the primary Cargo tuple plus the
runner tuple in its `tool_versions` array.

Tuple symbols below expand to the exact tuple/preimage/digest above. The array
is exact per mode and ordered by first internal use:

| Mode | Exact `tool_versions` tuple sequence |
|---|---|
| `L0Format` | `[cargo,rustfmt]` |
| `L0Check` | `[cargo]` |
| `L0FocusedTest` | `[cargo]` |
| `L1WorkspaceCheck` | `[cargo]` |
| `L1Clippy` | `[cargo,clippy]` |
| `L1Test` | `[cargo]` |
| `L1Doc` | `[cargo]` |
| `L1Static` | `[cargo,test_gate]` |
| `L1SupplyChain` | `[cargo,test_gate]` |
| `L2SourceSpine` | `[cargo]` |
| `L2Contract` | `[cargo]` |
| `L2Property` | `[cargo]` |
| `L2Model` | `[cargo]` |
| `L2Adversarial` | `[cargo]` |
| `L2HoldClosure` | `[cargo]` |
| `L2NoAuthority` | `[cargo]` |

R20 retains the exact R14 serial phase plan. Each closed `ExecutionPhase` has ordered
keys `phase_ordinal,phase_id,kind,tool_version_indices,argv,target_first,
target_count`: ordinal is consecutive from 1; kind is `command|targets`;
tool indices select the exact top-level tuples in use order; argv is a literal
argv array for command or a literal expansion template for targets; and the
target range is null/null for command or a contiguous allocation range for
targets. Ranges partition all allocations once in section 7 order.

`phase_id` is exactly `PHASE-<MODE>-NN`, with literal mode spelling and
one-based two-digit ordinal. `tool_version_indices` are zero-based integers
into that mode's `tool_versions`, in domain `0..length-1`, first-use ordered
and unique. `target_first` is a zero-based `allocated_targets` index; ranges
are contiguous, nonoverlapping, and cover `0..length-1`. Command phases use
null/null. A targets-phase `argv` is exactly either the mode-map Cargo argv
followed by `["<ASSERTION>","--","--exact","--test-threads=1","--nocapture"]`
or `["pwsh","-NoLogo","-NoProfile","-NonInteractive","-File",
"tools/test_gate.ps1","-AssertTarget","<ASSERTION>"]`. Expansion replaces
the sole complete `<ASSERTION>` token with the allocation assertion; it never
interpolates or splits a string.

This is the exhaustive per-mode phase sequence. `CARGO` means the literal
mode-map argv above, `CARGO[n]` its indexed argv, `CARGO-T` its literal target
template, `TG-S` literal `pwsh -NoLogo -NoProfile -NonInteractive -File
tools/test_gate.ps1 -AssertPhase assert-static-surface`, and `TG-T` the literal
test-gate template. Each tuple is `ordinal,phase_id,kind,tool indices,argv,
target_first,target_count`.
The uppercase table symbols are specification aliases only and are never
serialized: the canonical object expands each alias to the referenced literal
JSON argv array before hashing or comparison. This expansion is total and
admits no caller-selected value.

| Mode | Exact phase tuples in ordinal order |
|---|---|
| `L0Format` | `1,PHASE-L0Format-01,command,[0,1],CARGO,null,null` |
| `L0Check` | `1,PHASE-L0Check-01,command,[0],CARGO,null,null` |
| `L0FocusedTest` | `1,PHASE-L0FocusedTest-01,command,[0],CARGO,null,null` |
| `L1WorkspaceCheck` | `1,PHASE-L1WorkspaceCheck-01,command,[0],CARGO,null,null` |
| `L1Clippy` | `1,PHASE-L1Clippy-01,command,[0,1],CARGO,null,null` |
| `L1Test` | `1,PHASE-L1Test-01,command,[0],CARGO,null,null` |
| `L1Doc` | `1,PHASE-L1Doc-01,command,[0],CARGO[0],null,null`; `2,PHASE-L1Doc-02,command,[0],CARGO[1],null,null` |
| `L1Static` | `1,PHASE-L1Static-01,targets,[0],CARGO-T,0,8`; `2,PHASE-L1Static-02,command,[1],TG-S,null,null` |
| `L1SupplyChain` | `1,PHASE-L1SupplyChain-01,command,[0],CARGO,null,null`; `2,PHASE-L1SupplyChain-02,targets,[1],TG-T,0,3` |
| `L2SourceSpine` | `1,PHASE-L2SourceSpine-01,targets,[0],CARGO-T,0,29` |
| `L2Contract` | `1,PHASE-L2Contract-01,targets,[0],CARGO-T,0,12` |
| `L2Property` | `1,PHASE-L2Property-01,targets,[0],CARGO-T,0,16` |
| `L2Model` | `1,PHASE-L2Model-01,targets,[0],CARGO-T,0,14` |
| `L2Adversarial` | `1,PHASE-L2Adversarial-01,targets,[0],CARGO-T,0,19` |
| `L2HoldClosure` | `1,PHASE-L2HoldClosure-01,targets,[0],CARGO-T,0,19` |
| `L2NoAuthority` | `1,PHASE-L2NoAuthority-01,targets,[0],CARGO-T,0,28` |

Every ordinary allocated Cargo-test mode has one targets phase whose children
insert exact assertion before `-- --exact --test-threads=1 --nocapture`.
`L1Static` has, in order: its Cargo targets phase, then command phase exact argv
`pwsh -NoLogo -NoProfile -NonInteractive -File tools/test_gate.ps1
-AssertPhase assert-static-surface`. `L1SupplyChain` has, in order: command
phase exact Cargo metadata argv from the JSON map, then a targets phase whose
children are `pwsh -NoLogo -NoProfile -NonInteractive -File
tools/test_gate.ps1 -AssertTarget <assertion>`. Modes with no allocation have
one command phase per exact JSON argv entry, in entry order. No Cargo or
test-gate phase may be merged, omitted, moved, or represented only by a tool
tuple. The flattened actual child argv in phase/target order is top-level
`exact_argv`; every argv's tools are selected by its phase.

The ledger root is exact
`context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/ledger/<mode>/vEEEE/run-aAAAA/`,
where EEEE is immutable `execution_evidence_version`, not mutable review
`evidence_version`, and AAAAA is the next unused positive run attempt
`00001..99999`, allocated by create-new directory. `execution_id` is exact
`EXEC-WP-TST-001-<mode>-vEEEE`; `run_id` is exact
`RUN-WP-TST-001-<mode>-vEEEE-aAAAA`. Every review successor retains the same
execution ID, execution ordinal, execution version, execution origin, and ledger binding
byte-identically; review version advancement never changes a ledger formula.

Every record filename is `rRRRR-<kind>.json`, where RRRR begins `0000` and
increments by one in durable chain order. Kinds are exactly `run-start`,
`phase-NN-start`, `phase-NN-completion|phase-NN-recovery`,
`target-MMM-start`, `target-MMM-completion|target-MMM-recovery`, and final
`run-completion|run-recovery`; NN is two-digit phase ordinal and MMM is
three-digit one-based mode-allocation ordinal. Completion/recovery uses the next RRRR,
not its start's RRRR. No other entry, sequence gap, duplicate, alternate width,
temp remnant, or filename is legal. Bytewise filename order is chain order.

Ledger schema is literal `test-gate-ledger.v4`. Every variant has exact ordered
common prefix `schema,record_kind,wp_id,wp_revision,wp_digest,lineage_binding,
acceptance_commit,corrective_entry_commit,failed_implementation_commit,
implementation_commit,mode,execution_id,
execution_evidence_version,run_attempt,run_id,ordinal,predecessor_digest` and
final key `record_digest`. The digest hashes canonical complete bytes omitting
only that final key and its preceding comma. Predecessor is null only for
`RunStart`; otherwise it hashes the immediately prior durable record. No
variant admits another variant's keys or placeholder null fields.

`wp_revision` is literal `R20`; `wp_digest` is the accepted R20 WP digest;
`lineage_binding` is the complete closed `CorrectiveLineageBinding`; the three
commit fields equal its acceptance, corrective-entry, failed-attempt, and
current corrective-implementation projections. Thus every ledger record binds
the corrective delta and failed disposition before its variant suffix; no R14
positive acceptance/entry field is admitted by v3.

| Variant / exact `record_kind` | Exact ordered suffix before `record_digest` |
|---|---|
| `RunStart` / `run-start` | `start_utc` |
| `PhaseStart(command)` / `phase-NN-start` | `phase_ordinal,phase_id,phase_kind,argv,process_id,process_creation_filetime,job_name,job_configuration_digest,start_utc` |
| `PhaseStart(targets)` / `phase-NN-start` | `phase_ordinal,phase_id,phase_kind,argv,start_utc` |
| `TargetStart` / `target-MMM-start` | `phase_ordinal,phase_id,target_ordinal,controlled_id,assertion,argv,process_id,process_creation_filetime,job_name,job_configuration_digest,start_utc` |
| `TargetCompletion` / `target-MMM-completion` | `phase_ordinal,phase_id,target_ordinal,controlled_id,assertion,argv,paired_start_digest,native_exit_u32,portable_exit,state,reason,stdout_sha256,stderr_sha256,end_utc` |
| `TargetRecovery` / `target-MMM-recovery` | `phase_ordinal,phase_id,target_ordinal,controlled_id,assertion,argv,paired_start_digest,termination_observation,native_exit_u32,portable_exit,state,reason,stdout_sha256,stderr_sha256,recovery_utc` |
| `PhaseCompletion` / `phase-NN-completion` | `phase_ordinal,phase_id,phase_kind,argv,paired_start_digest,native_exit_u32,portable_exit,state,reason,end_utc` |
| `PhaseRecovery` / `phase-NN-recovery` | `phase_ordinal,phase_id,phase_kind,argv,paired_start_digest,termination_observation,native_exit_u32,portable_exit,state,reason,recovery_utc` |
| `RunCompletion` / `run-completion` | `paired_start_digest,native_exit_u32,portable_exit,state,reason,end_utc` |
| `RunRecovery` / `run-recovery` | `paired_start_digest,termination_observation,native_exit_u32,portable_exit,state,reason,recovery_utc` |

Record `ordinal` is uint equal to filename RRRR; run attempt is uint
`1..99999`; phase ordinal is uint `1..2` equal to NN; target ordinal is uint
`1..148` equal to MMM and the one-based index of that mode allocation. Times
are `UTC`; all IDs/digests use section 8 primitives. The enclosing
`ledger_binding` supplies the execution ordinal without changing the retained
`test-gate-ledger.v4` record schema; its execution ID/version/run root bind those exact
records.

All identity and argv values equal their exact phase/allocation. The
`phase_kind` discriminant selects exactly one PhaseStart key set; a command
phase launches its sole child and therefore carries the same non-null process
identity triple required of TargetStart, while a targets phase launches no
child at phase start and admits none of those keys. Each
completion/recovery's `paired_start_digest` names its unique earlier unpaired
start at the same scope; no start is paired twice. Start process ID is uint
`1..4294967295`; creation FILETIME is the lossless uint64 count of 100-ns
intervals returned by `GetProcessTimes`, never a formatted timestamp. Job name
is exact `Local\\BAS-TST-<first-32-lowercase-hex-of-SHA256(run_id)>`; its
configuration digest is defined below. All four values are captured after Job
assignment and before child release. Completion native exit is uint `0..4294967295`;
portable exit is `0..255` and equals native exit when at most 255, otherwise
255. State is `passed|failed|held`; reason is the scope's closed terminal enum.
Target completion stream hashes are `DIGEST`.

Target completion reason is exactly `assertion-passed|assertion-failed|
target-held|unexpected-exit`; target recovery reason is exactly
`timeout|wall-limit|memory-limit|output-limit|bound-kill|crash|
unexpected-exit|supervisor-loss|ledger-corruption` and equals observation
kind. Phase completion reason is exactly `all-children-passed|command-passed|
child-failed|child-held|command-failed`; phase recovery reason is the same
closed recovery enum and equals observation kind. Run completion reason is
exactly `all-phases-passed|phase-failed|phase-held`; run recovery reason is the
same closed recovery enum and equals observation kind. State/reason pairs not
named by these mappings reject.

The supervisor's new-run Job sequence is exact: call
`CreateJobObjectW(null,<job_name>)` and require a new object
(`GetLastError()!=ERROR_ALREADY_EXISTS`); create a private I/O completion port
with `CreateIoCompletionPort(INVALID_HANDLE_VALUE,null,0,1)`, derive the
pointer-width completion key by interpreting the first `sizeof(ULONG_PTR)`
digest bytes of SHA-256 of exact UTF-8 `completion-key=<run_id><LF>` as an
unsigned little-endian integer and bitwise-ORing one, and associate the inactive
Job by `SetInformationJobObject(JobObjectAssociateCompletionPortInformation)`
with that key and port. Then call `SetInformationJobObject` with
`JOBOBJECT_EXTENDED_LIMIT_INFORMATION.BasicLimitInformation.LimitFlags`
containing the mandatory flags
`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE|JOB_OBJECT_LIMIT_JOB_MEMORY`, set
`JobMemoryLimit=1073741824`, and optionally add
`JOB_OBJECT_LIMIT_PROCESS_MEMORY` with `ProcessMemoryLimit=1073741824` as an
additional per-process ceiling. The process flag/field may never replace or
relax the mandatory Job-wide flag/field. Query the extended-limit information
back and require exact flag/value equality. As supplemental detection only,
set `JobObjectNotificationLimitInformation2` with every byte zero except
`LimitFlags=JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH` and
`JobHighMemoryLimit` (the `JobMemoryLimit` union member) `=1073741824`, then
query that same information class and require exact equality. This notification
threshold never replaces, weakens, or proves the hard extended limit. Create
the child suspended;
`AssignProcessToJobObject`; obtain PID and creation FILETIME; durably write the
child start; then `ResumeThread`. `job_configuration_digest` hashes canonical
LF rows `name=<job_name>`, `kill_on_close=true`,
`hard_limit_flags=JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE|JOB_OBJECT_LIMIT_JOB_MEMORY<PROCESS_SUFFIX>`,
`job_memory_bytes=1073741824`,
`process_memory_bytes=<1073741824-or-null>`,
`completion_key_digest=<SHA256-of-the-key-preimage>`,
`notification_limit_flags=JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH`,
`notification_job_memory_bytes=1073741824`, `run_id=<run_id>`, in that order.
Null means the optional process flag is absent and its ignored native field is
zero; non-null means that flag is present and the field equals the value.
`PROCESS_SUFFIX` is exactly empty for null or literal
`|JOB_OBJECT_LIMIT_PROCESS_MEMORY` for the non-null value; the alias token is
expanded before hashing and is never serialized.
Both memory rows equal `resource_bounds`; the mandatory Job row is never null.
The completion-port handle is not inherited. A dedicated supervisor thread
calls `GetQueuedCompletionStatus` from before child resume until the Job's
active-process count is zero, retaining every dequeued message, completion-key
comparison, reported PID value, timeout, Boolean result, and `GetLastError`.
Every API
name, Win32 return/error code, queried limit, and handle close is retained in
the supervisor event stream. The name is durable identity, not proof that an
object still exists.

The executable platform floor is Windows 10 desktop or Windows Server 2016,
where information-class-2 notification structures are supported. An unsupported
information class, failed association/set/query, or unavailable completion-port
API is a prelaunch `binding-mismatch`; there is no polling-only fallback.

Recovery first calls `OpenJobObjectW(QUERY|TERMINATE|SYNCHRONIZE,false,
<job_name>)`. Success requires queried configuration equal to the start and a
Job process list containing the recorded PID whose `GetProcessTimes` creation
FILETIME equals the start. It then calls `TerminateJobObject` with exact native
exit `0xE0000001`, waits for the Job, re-queries until active-process count is
zero, reads the process exit, and closes the handle. If open returns only
`ERROR_FILE_NOT_FOUND`, recovery calls `OpenProcess(QUERY_LIMITED_INFORMATION|
SYNCHRONIZE,false,<pid>)`: absent process is proven already terminated; a
present process must have matching creation FILETIME and be absent from any
reopenable named Job before recovery rejects as `job-identity-lost`—it is never
killed by PID alone. Any other open/query/identity/API result rejects recovery.
Normal supervisor close proves kill-on-close by the queried flag; unexpected
last-handle close may terminate the Job, after which the FILETIME-based absent
proof is the only legal recovery branch.

Recovery equality covers the queryable extended and notification limit
structures. It does not invent or replace the lost supervisor's private
completion port; the original association/key/API proof remains bound by the
durable start configuration and event digest, while recovered live work is
terminated through the named Job handle.

The 1 GiB process-tree bound is therefore the Windows Job aggregate committed-
memory ceiling, never an inferred sum of per-process observations. Every
configuration query must prove the mandatory Job flag and exact
`JobMemoryLimit`; a missing Job flag, zero/different Job limit, or only the
process-memory flag is `binding-mismatch` before launch and `job-identity-lost`
during recovery. A `JOB_OBJECT_MSG_JOB_MEMORY_LIMIT` dequeued with the exact
completion key is the documented hard-limit attempt signal. A supplemental
`JOB_OBJECT_MSG_NOTIFICATION_LIMIT` with that key is usable only when followed
immediately by `QueryInformationJobObject(JobObjectLimitViolationInformation2)`
whose `LimitFlags` and `ViolationLimitFlags` contain
`JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH`, whose `JobHighMemoryLimit=1073741824`, and
whose `JobMemory>=1073741824`. The query is required to rearm that notification.
Neither `JobObjectLimitViolationInformation2` nor its flags are queried or
claimed without the configured notification message. The optional process
message/flag may additionally be reported, but a process-only violation is
`unexpected-exit`, not `memory-limit`.

`TerminationObservation` has exact ordered keys
`kind,worker_started,process_id,process_creation_filetime,job_name,
job_configuration_digest,job_open_result,job_query_digest,memory_limit_proof,
identity_match,job_active_count_before,termination_action,
terminate_job_result,job_active_count_after,native_exit_u32,portable_exit,
stdout_sha256,stderr_sha256,observed_utc,event_digest`. Kind is exactly
`timeout|wall-limit|memory-limit|output-limit|bound-kill|crash|
unexpected-exit|supervisor-loss|ledger-corruption|job-identity-lost`; open
result is `created-handle|reopened-handle|not-found|not-attempted`; action is
`none|terminate-job|already-absent|reject-identity`; terminate result is
`succeeded|not-called|failed`; identity match is bool; counts are uint
`0..4294967295`; native is uint32 or null; portable is `0..255|null`; stream
hashes are `DIGEST|null`; event digest is defined below.

`memory_limit_proof` is null iff kind is not `memory-limit`; otherwise it is a
closed object with exact ordered keys
`signal,completion_key_digest,message_id,reported_process_id,
violation_query_digest,proof_digest`. Signal/message are exactly
`hard-job-memory/JOB_OBJECT_MSG_JOB_MEMORY_LIMIT` or
`notification-memory/JOB_OBJECT_MSG_NOTIFICATION_LIMIT`; completion-key digest
equals the configuration; reported PID is uint `1..4294967295`; violation
query digest is null for the documented hard message and is non-null for the
notification message, hashing the exact successful violation structure above.
Proof digest omits only itself. A wrong-key, process-memory, stale, malformed,
unconfigured, timeout, or API-failure message cannot produce this object.

The exhaustive tuples are: ordinary observed exit/crash = worker true, all
identity/config/query fields non-null, open `created-handle`, action none,
terminate not-called, before/after zero, native/portable non-null, both complete
stream hashes non-null, memory proof null; recovered live Job = worker true, open
`reopened-handle`, identity true, positive before, action terminate-job,
terminate succeeded, zero after, native/portable non-null after wait, and each
stream hash non-null iff its complete pipe was recovered; already absent =
worker true, open not-found, exact recorded-PID absence proof bound in
`job_query_digest`, zero/zero, action
already-absent, terminate not-called, native/portable and streams nullable only
when no complete observation exists; pre-child supervisor/ledger loss = worker
false, all process/Job/config/query/native/portable/stream fields null, open
not-attempted, identity false, zero/zero, action already-absent, terminate
not-called; identity/API rejection = kind job-identity-lost or
ledger-corruption, observed identity fields retained, action reject-identity,
terminate not-called or failed, no recovery record and no promotion. Portable
equals native through the established `<=255 ? native : 255` mapping whenever
native is non-null. Every non-memory branch has null memory proof. A live
completion-port memory terminal is worker true, open `created-handle`, identity
true, before any uint, action terminate-job, terminate succeeded, after zero,
exits non-null after wait, stream hashes non-null iff complete, and carries the
exact non-null proof; the supervisor calls `TerminateJobObject` even when the
pretermination active count raced to zero. Before classifying a simultaneous
process exit it drains all already-queued completion messages with zero-timeout
calls; the first valid retained memory proof wins, otherwise the observed-exit
rule applies. Job query digest is non-null for every created/reopened/
not-found/rejection branch and hashes canonical API result, exact extended-
limit flags, `JobMemoryLimit`, optional `ProcessMemoryLimit`, exact notification
configuration, applicable notification-violation query, accounting/
`PeakJobMemoryUsed`, process-list, and PID-FILETIME rows; unavailable
fields are explicit null only where the exhaustive branch permits them. It is
null only for pre-child not-attempted. No other
state/reason/null tuple is legal.
For every legal recovery record, its native/portable exits and stdout/stderr
hashes are byte-identical projections of its `termination_observation`; state
is `failed` and reason equals observation kind. A null projection where the
observation is non-null, or the reverse, rejects.

`event_digest` is SHA-256 of exact UTF-8 LF rows
`schema=termination-events.v1`, `run_id=<run_id>`,
`paired_start_digest=<Digest>`, followed by every supervisor event as
`sequence=<u64><TAB>api=<closed-api-name><TAB>result=<decimal-u32><TAB>
value=<canonical-value-or-minus><LF>` in observed order, then
`terminal_kind=<kind><LF>`. Closed API names are exactly
`CreateJobObjectW,CreateIoCompletionPort,SetInformationJobObject,
QueryInformationJobObject,GetQueuedCompletionStatus,
CreateProcessW,AssignProcessToJobObject,GetProcessTimes,ResumeThread,
OpenJobObjectW,OpenProcess,TerminateJobObject,WaitForSingleObject,
GetExitCodeProcess,ReadFile,CloseHandle`. The digest includes failed calls and
the event that selected termination; no prose, native struct padding, omitted
event, or alternate order is legal.

For `SetInformationJobObject`/`QueryInformationJobObject`, canonical value is
`class=<AssociateCompletionPort|ExtendedLimit|NotificationLimit2|LimitViolation2>;
fields=<canonical-zero-explicit-field-projection>`. For
`CreateIoCompletionPort` it binds success/error plus the non-inherited handle
identity digest. For `GetQueuedCompletionStatus`, result is Boolean `0|1` and
value is exact
`message=<decimal-DWORD-or-minus>;completion_key_digest=<DIGEST-or-minus>;
key_match=<true|false>;reported_pid=<uint-or-minus>;last_error=<u32>`;
a timeout is result zero, all message/key/PID values minus, and
`last_error=WAIT_TIMEOUT`. Every dequeue, timeout, wrong key, unrelated Job
message, and post-terminal drain call is retained; no polling inference or
synthetic violation row is legal.

Each file is create-new, completely written, `FlushFileBuffers`-flushed,
closed, and followed by a parent-directory handle flush before child launch or
the next record. Exactly one child runs. Normal completion and recovery are
exclusive. A phase completion's predecessor is its last child terminal while
its paired digest binds its own phase start; a run completion's predecessor is
the last phase terminal while its paired digest binds its own run start.
Phase state/exit/reason is the closed aggregation of its ordered child or sole
command terminal: the first child with state not passed or native exit nonzero
wins and supplies both exits/state plus `child-failed|child-held`; otherwise
zero/passed/`all-children-passed`. A command phase maps its sole command to
zero/passed/`command-passed` or its exact nonzero/failed/`command-failed`.
Run aggregation applies the same ordered rule to phases, yielding
`all-phases-passed|phase-failed|phase-held`. Recovery values instead derive
only from the frozen termination observation and never from missing children.

Startup scans bytewise filenames and validates the full chain. The only
recoverable prefixes have unique properly nested open ancestors and exactly
one of four final records: (1) an unpaired target/command-phase start, (2) a
paired target terminal with its phase/run open, (3) a paired phase terminal
with its run open, or (4) run start before any phase. Every earlier sibling is
paired and no unrelated start is open. Case 1 applies the exact named-Job/
FILETIME reopen/query/termination proof and freezes that full observation.
Cases 2–4 freeze the exhaustive pre-child/no-active-worker `supervisor-loss`
tuple above because the prior child terminal is already
durable or no child began. Case 1 target unwind is exactly target recovery ->
phase recovery -> run recovery; case 1 command phase and cases 2 unwind phase
recovery -> run recovery; cases 3–4 write run recovery. Each predecessor names
the immediately prior terminal/recovery, each recovery pairs its own scope's
start, and all levels in one unwind retain the identical observation. Later
scopes derive not-run without fabricated ledger records. Recovery never
launches/retries. Any other incomplete shape, live ambiguity, extra file,
broken chain, or record after a run terminal rejects and requires a new run
attempt.

`target_results` derives only from this ledger. A phase failure before its
target range makes that range and all later ranges not-run. A unique target
start identifies the active target on timeout/crash. Default parallel libtest,
aggregate target invocation, missing exact/serial flag, text-derived outcome,
or an actual argv absent from `execution_phases`, `exact_argv`, or the ledger
rejects.

The runner derives its only repository root before any observation: it resolves
the absolute raw filesystem path of its own `tools/test_gate.ps1`, removes the
literal final `tools/test_gate.ps1`, then requires
`git -C <candidate-root> rev-parse --show-toplevel` to return that same absolute
path after Windows handle-based normalization. Different roots, symlink/junction
redirection, relative cwd, drive-relative form, or a caller cwd rejects. Every
supervisor Git process receives that exact root as cwd and every Git argv
contains `-C <absolute-repo-root>`. No worker executes there: worker cwd and
all executable/config/source inputs come only from the immutable commit-tree
materialization defined below.

The preflight uses only the following literal Git argv arrays, substituting the
already bound lowercase commit/object IDs as single arguments. All invocations
run with exact environment additions `GIT_CONFIG_NOSYSTEM=1`,
`GIT_CONFIG_SYSTEM=NUL`, `GIT_CONFIG_GLOBAL=NUL`, and
`GIT_ATTR_NOSYSTEM=1`, `GIT_OPTIONAL_LOCKS=0`,
`GIT_NO_REPLACE_OBJECTS=1`; the otherwise sanitized environment contains no other
`GIT_*` variable. The config/diff/status/tree/blob semantic invocations fix the
same configuration in the same order. Root/git/common/index discovery and
replace-ref enumeration use their literal arrays without `-c`; they are bound
by the same environment and perform only path resolution or disabled-ref listing.
The local configuration cannot be disabled by Git and therefore is captured
byte-for-byte before and after the observations; named semantic controls are
overridden on the command line and every remaining local setting is exactly
bound by those snapshots. System/global configuration and
attribute files are disabled, and global/local excludes and attributes are
neutralized by the two `NUL` values. Rename detection is enabled,
unlimited, Myers-based, and exact-content-only (`100%`); no Git default may
alter it.

The common semantic-command configuration is additionally exact
`status.relativePaths=false`, `core.precomposeUnicode=false`,
`core.ignoreCase=false`, and `submodule.recurse=false`. The two porcelain
commands use the one literal whole-tree pathspec `--`, `:(top,glob)**` solely
for dirty/untracked/ignored custody. The three ObservedPreflight implementation
commands use `--` followed by all 18 literal `IMPL-PATHS` arguments, and their
base is always the original R14 entry. The separately named
`corrective_delta_unscoped_raw` is the sole no-pathspec exception and supplies
only the no-extra/subset assertion described in section 3. It has no
`--diff-filter`, parses the total raw status alphabet, and fails on every status
other than the exact scoped corrective `A|M` subsequence. No implicit prefix,
cwd-relative default, exclude magic, attribute-selected pathset, caller
pathspec, or normative whole-tree implementation manifest is allowed. All commands are
read-only under optional-lock suppression.

```json
{
  "root_discovery": ["git","-C","<candidate-root>","rev-parse","--show-toplevel"],
  "git_dir": ["git","-C","<absolute-repo-root>","rev-parse","--path-format=absolute","--git-dir"],
  "common_dir": ["git","-C","<absolute-repo-root>","rev-parse","--path-format=absolute","--git-common-dir"],
  "index_path": ["git","-C","<absolute-repo-root>","rev-parse","--path-format=absolute","--git-path","index"],
  "replace_refs_before": ["git","-C","<absolute-repo-root>","for-each-ref","--format=%(objectname)%00%(refname)%00","refs/replace"],
  "local_config_before": ["git","-C","<absolute-repo-root>","-c","core.autocrlf=false","-c","core.safecrlf=false","-c","core.quotepath=false","-c","core.excludesFile=NUL","-c","core.attributesFile=NUL","-c","diff.renames=true","-c","status.renames=true","-c","diff.renameLimit=0","-c","diff.algorithm=myers","-c","status.relativePaths=false","-c","core.precomposeUnicode=false","-c","core.ignoreCase=false","-c","submodule.recurse=false","config","--null","--list","--show-origin","--show-scope"],
  "committed_status": ["<GIT-C>","diff","--raw","-z","--no-abbrev","--full-index","--no-ext-diff","--find-renames=100%","--diff-filter=AMDR","6354f5184b97923571dcd397ac9871167833e86e","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "porcelain_state_before": ["git","-C","<absolute-repo-root>","-c","core.autocrlf=false","-c","core.safecrlf=false","-c","core.quotepath=false","-c","core.excludesFile=NUL","-c","core.attributesFile=NUL","-c","diff.renames=true","-c","status.renames=true","-c","diff.renameLimit=0","-c","diff.algorithm=myers","-c","status.relativePaths=false","-c","core.precomposeUnicode=false","-c","core.ignoreCase=false","-c","submodule.recurse=false","status","--porcelain=v2","-z","--untracked-files=all","--ignored=matching","--find-renames=100%","--",":(top,glob)**"],
  "binary_diff": ["<GIT-C>","diff","--binary","--full-index","--no-ext-diff","--find-renames=100%","6354f5184b97923571dcd397ac9871167833e86e","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "tree_inventory": ["<GIT-C>","ls-tree","-r","-z","--full-tree","<corrective_implementation_commit>","--","<IMPL-PATHS>"],
  "blob_bytes": ["git","-C","<absolute-repo-root>","-c","core.autocrlf=false","-c","core.safecrlf=false","-c","core.quotepath=false","-c","core.excludesFile=NUL","-c","core.attributesFile=NUL","-c","diff.renames=true","-c","status.renames=true","-c","diff.renameLimit=0","-c","diff.algorithm=myers","-c","status.relativePaths=false","-c","core.precomposeUnicode=false","-c","core.ignoreCase=false","-c","submodule.recurse=false","cat-file","blob","<object-id>"],
  "porcelain_state_after": ["git","-C","<absolute-repo-root>","-c","core.autocrlf=false","-c","core.safecrlf=false","-c","core.quotepath=false","-c","core.excludesFile=NUL","-c","core.attributesFile=NUL","-c","diff.renames=true","-c","status.renames=true","-c","diff.renameLimit=0","-c","diff.algorithm=myers","-c","status.relativePaths=false","-c","core.precomposeUnicode=false","-c","core.ignoreCase=false","-c","submodule.recurse=false","status","--porcelain=v2","-z","--untracked-files=all","--ignored=matching","--find-renames=100%","--",":(top,glob)**"],
  "replace_refs_after": ["git","-C","<absolute-repo-root>","for-each-ref","--format=%(objectname)%00%(refname)%00","refs/replace"],
  "local_config_after": ["git","-C","<absolute-repo-root>","-c","core.autocrlf=false","-c","core.safecrlf=false","-c","core.quotepath=false","-c","core.excludesFile=NUL","-c","core.attributesFile=NUL","-c","diff.renames=true","-c","status.renames=true","-c","diff.renameLimit=0","-c","diff.algorithm=myers","-c","status.relativePaths=false","-c","core.precomposeUnicode=false","-c","core.ignoreCase=false","-c","submodule.recurse=false","config","--null","--list","--show-origin","--show-scope"]
}
```

`committed_status` is byte-identical in argv, stdout, stderr, counts, hashes,
parsed rows, and delta digest to the section 3
`complete_implementation_delta.raw_observation`; it is the sole A/M/D/R and
old/target-path source. `binary_diff` is likewise the same complete-delta
binary observation. `tree_inventory` returns exactly the 18 allowlisted target
blobs and no whole-tree row. The unscoped corrective observation is retained in
`implementation_binding.corrective_no_extra_observation` only and cannot feed
`ObservedPreflight.rows`, expected/actual manifests, or artifact bindings.
Its raw `-z` grammar is exactly `:OLDMODE SP NEWMODE SP OLDOID SP NEWOID SP
STATUS NUL PATH NUL` for `A|M|D`, or the same prefix with `STATUS=R100` followed
by `OLD_PATH NUL NEW_PATH NUL`; modes are six octal digits and object IDs are
40 lowercase hex digits. Any other status/score, field arity, delimiter,
object width, or path count rejects. `tree_inventory` accepts only
`MODE SP TYPE SP OBJECT TAB PATH NUL`, with six-octal-digit mode, literal type
`blob`, 40-lowercase-hex object ID, and one raw path token retained as
`RawGitPath`. The local-config and
binary-diff streams are retained raw and not text-parsed; every blob stream is
retained as opaque object bytes.
Both `porcelain_state_before` and `porcelain_state_after` accept only NUL-terminated records with these exact
space-delimited prefixes and arities (the final path is the uninterpreted
remainder and may contain spaces): `1 XY SUB mH mI mW hH hI PATH`; `2 XY SUB mH
mI mW hH hI XNNN PATH`, immediately followed by one `ORIG_PATH` NUL token;
`u XY SUB m1 m2 m3 mW h1 h2 h3 PATH`; `? PATH`; or `! PATH`. `XY` is exactly
two status bytes; every `m*` is six octal digits; every `h*` is exactly 40
lowercase hex digits or 40 zeroes where porcelain permits an absent object;
`SUB` is exactly four bytes; and `XNNN` is `R|C` plus three decimal digits.
Headers, LF termination, missing/extra fields, empty paths, embedded NUL,
additional rename tokens, and every other tag reject. Index `X` supplies
staged, worktree `Y` supplies dirty, `?` supplies untracked, and `!` supplies
ignored paths; conflict `u` is retained as both staged and dirty plus failure.
`binary_diff` stdout bytes supply the
content-diff digest. `tree_inventory` supplies committed path/object bindings;
each blob SHA-256 hashes exact `blob_bytes` stdout. A nonzero Git exit, malformed
NUL record, unexpected status token, stderr-only result, object substitution,
or output after supervisor termination is a typed preflight failure. The runner
records each argv array and SHA-256 of its raw stdout/stderr in the preflight;
no shell string, locale parsing, implicit config, alternate rename heuristic,
working-tree newline conversion, or unspecified Git command is permitted.

Every mode has a 60-second wall limit, an exact 1 GiB Job-wide aggregate
committed-memory limit for the complete process tree, an optional additional
1 GiB per-process ceiling, and
10 MiB combined stdout/stderr limit. The runner itself performs no network
access and all Cargo commands are `--locked --offline`. It binds and verifies
one implementation commit, implementation/test/fixture-manifest/WP/
acceptance/runner/root-manifest/lock/WS predecessor digest, exact argv,
sanitized-environment digest, start/end/duration, bounds, per-command exit and
stream hashes/bytes, combined bytes, assertions, executed case target, and
result in canonical `test-gate-evidence.v13` JSON. All 16 modes must pass at
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

Section 7 is retained byte-identically from R7. Its legacy phrase
`executed_targets evidence field` names the allocation association only; in
the R10 schema that association is represented without ambiguity by immutable
`allocated_targets` plus same-cardinality observed `target_results`.

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
| `RawGitPath` | closed object with exact keys `encoding,content,decoded_byte_count,decoded_sha256`; encoding literal `base64`; content canonical padded RFC 4648 and bounded by the 10 MiB observation stream; count uint64 equals decoded bytes; digest hashes them; decoded bytes are nonempty and contain no NUL, but every other byte including invalid UTF-8, control, TAB, LF, CR, backslash, and colon is retained losslessly; a token that crosses the stream bound is not fabricated as a path and terminates with `stream-bound` while raw stream hash/length remain retained |
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

### 8.2 Closed `test-gate-evidence.v13` mode schema

The exact ordered top-level keys are:
`schema,evidence_id,mode,evidence_version,evidence_path,execution_id,
execution_ordinal,execution_evidence_version,execution_origin,successor_kind,
predecessor_execution,execution_history,wp_id,wp_revision,
wp_artifact_digest,lineage_binding,acceptance_binding,corrective_entry_binding,implementation_binding,
logical_predecessor_commit,context_rev_binding,identity_registry,candidate_author_ids,artifact_digests,
trace_manifest_digest,allocated_targets,execution_phases,target_results,ledger_binding,post_execution_custody,fixture_bindings,command_identity,
exact_argv,tool_versions,environment_digest,resource_bounds,
determinism_controls,expected,actual,observed_outputs,rollback_plan,
reproduction_plan,failure_records,counterexamples,required_review_lanes,
reviewer_decisions,findings,defers,dissent,conflicts,status,invalidation_triggers,
predecessor_evidence,history,evidence_digest`.

| Ordered field | Exact rule |
|---|---|
| `schema` | string literal `test-gate-evidence.v13` |
| `evidence_id` | string exactly `EVID-WP-TST-001-<mode>-vNNNN` |
| `mode` | `MODE` |
| `evidence_version` | `VERSION` |
| `evidence_path` | string exactly `context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/runs/<mode>/<evidence_id>.json` |
| `execution_id` | string exactly `EXEC-WP-TST-001-<mode>-vEEEE` |
| `execution_ordinal` | JSON integer `1..9999`; consecutive across execution origins for this mode under the exact rules below and independent of evidence record version |
| `execution_evidence_version` | `VERSION`; fixed when execution begins and immutable across every review successor |
| `execution_origin` | closed immutable `ExecutionOrigin` below; independently binds this execution and ledger without any review-record digest |
| `successor_kind` | `origin|review|execution`; exact transition rules below |
| `predecessor_execution` | null iff `execution_history=[]`; otherwise the last closed `ExecutionBinding` in that history, retained by reviews of the new execution |
| `execution_history` | exact ascending immutable `ExecutionBinding` array defined below |
| `wp_id` | string literal `WP-TST-001` |
| `wp_revision` | string literal `R20` |
| `wp_artifact_digest` | `DIGEST` of independently accepted R20 bytes |
| `lineage_binding` | complete closed `CorrectiveLineageBinding`; its WP digest equals the preceding field and every nested delta/failed/governance digest recomputes |
| `acceptance_binding` | exact `AcceptanceBindingV5` equal to `lineage_binding.acceptance_binding` |
| `corrective_entry_binding` | exact `CorrectiveEntryBindingV5` equal to `lineage_binding.corrective_entry_binding` |
| `implementation_binding` | closed `ImplementationBindingV6` below and equal to the lineage corrective implementation/deltas/no-extra/commit-parent observations |
| `logical_predecessor_commit` | literal `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| `context_rev_binding` | closed `ContextRevBinding` below |
| `identity_registry` | one closed `IdentityRegistryV3`; its mode-evidence binding matches this mode and review successors retain the registry byte-identically |
| `candidate_author_ids` | exact bytewise-sorted unique projection of `identity_registry.candidate_author_ids`; an ID with any positive binding remains present even if it is also visible in `failed_author_ids`; only failed-only IDs are absent and no failed binding grants authority |
| `artifact_digests` | exactly 18 closed `ArtifactBinding` objects, strictly sorted and unique by `path`, one for every section 3 path; equality/null/reason rules below |
| `trace_manifest_digest` | `DIGEST` of the accepted exact 123-identity/148-edge manifest |
| `allocated_targets` | immutable `0..148` closed `AllocatedTarget` objects in exact section 7 allocation order for `mode`; empty exactly when that mode has no edge |
| `execution_phases` | exact closed section 6 phase plan, `1..2` objects in ordinal order |
| `target_results` | exactly one closed `TargetResult` per allocated target in identical order and identity; states/reasons/pointers and prefix rule below |
| `ledger_binding` | closed object `execution_id,execution_ordinal,execution_evidence_version,run_id,root_path,run_attempt,first_record_digest,last_record_digest,record_count,aggregate_digest`; execution fields equal the top level; ID/root/attempt use exact section 6 formulas; first/last are `DIGEST`; record count is exact uint `2..302`; aggregate hashes `<filename><TAB><record_digest><LF>` in filename order and values bind the complete validated chain |
| `post_execution_custody` | closed post-run repository/root nonmutation object defined below |
| `fixture_bindings` | exactly four closed `FixtureBinding` objects in ascending `fixture_id` order; exactly the current rows in section 5 |
| `command_identity` | exact section 6 `CMD-*` identity selected by `mode` |
| `exact_argv` | exact section 6 flattened phase/child argv array, including every Cargo/test-gate command and per-target child; no joining, omission, shell spelling, aggregate libtest execution, or prose reconstruction |
| `tool_versions` | exactly the selected section 6 per-mode tuple sequence; closed `ToolVersion` objects in first-use order, unique by `tool` |
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
| `defers` | the exact strictly `id`-sorted projection of every non-null current decision's `defer`; `0..2816` unique `Defer` objects |
| `dissent` | the exact strictly `id`-sorted projection of every non-null current decision's `dissent`; `0..2816` unique `Dissent` objects |
| `conflicts` | the exact strictly `id`-sorted projection of every non-null current decision's `conflicts`; `0..2816` unique `Conflict` objects |
| `status` | derived enum `failed|stale|conflicted|passed` under the precedence rule below |
| `invalidation_triggers` | exactly the strictly bytewise-sorted unique non-empty array of the literals `artifact_digest_change,binding_change,command_change,environment_change,fixture_change,review_change,schema_change,trace_change`; no other value |
| `predecessor_evidence` | null iff version 1; otherwise one closed `EvidenceBinding` naming version `evidence_version-1` of the same mode |
| `history` | exactly `evidence_version-1` closed `EvidenceBinding` objects in ascending consecutive version order, beginning at 1; last equals `predecessor_evidence`; empty iff version 1 |
| `evidence_digest` | `DIGEST`, last; SHA-256 of the canonical complete object with only its preceding comma and this key/value omitted |

The following nested objects are independently closed. Key order is the order
shown; all members are non-null unless stated otherwise:

At the execution-origin record, `successor_kind=origin`, predecessor execution
is null, execution history is `[]`, `execution_ordinal=1`, and
`execution_evidence_version` equals
`evidence_version`. A review-only successor increments only
`evidence_version`; the execution version remains the origin value. Ledger
records never contain review `evidence_version`, and review evidence never
derives ledger paths from its mutable version.

`execution_history` has cardinality `0..9998`; its bindings are strictly
increasing and consecutive by `execution_ordinal`, begin at ordinal 1, and
have no omission or duplicate. Their `execution_evidence_version` values are
the exact evidence versions at which those executions originated and need not
be consecutive because intervening review successors consume evidence
versions. Each member binds the latest
immutable evidence record of that completed execution immediately before its
successor execution began. Its last member equals `predecessor_execution`.
The current `execution_ordinal` is exactly history cardinality plus one, and
each history member's ordinal is exactly its one-based array position. Thus an
origin has zero members, each new execution increments the prior ordinal by
exactly one even after any number of reviews, and review successors retain the
ordinal and array byte-identically.

| Type | Ordered members and exact rules |
|---|---|
| `ExecutionOrigin` | exact keys `wp_revision,lineage_digest,execution_id,execution_ordinal,execution_evidence_version,implementation_commit,run_id,ledger_root,ledger_aggregate_digest`; revision/lineage equal R20 top level; values equal the implementation and ledger bindings; ordinal is independent of evidence version; it contains no evidence ID/version/path/digest and is byte-identical in every review successor |
| `ExecutionBinding` | exact keys `wp_revision,lineage_digest,execution_id,execution_ordinal,execution_evidence_version,execution_origin_digest,evidence_id,evidence_version,evidence_path,evidence_digest,run_id,ledger_root,ledger_aggregate_digest`; origin digest hashes canonical `ExecutionOrigin`; evidence values bind the last immutable record of that execution immediately before the next execution begins; ordinal obeys the complete consecutive history rule while the two evidence-version fields retain their exact record meanings; every digest recomputes |
| `AcceptanceBindingV5` | exact closed section 3.1 object; only R20 acceptance is positive and its retained commit observation proves the sole externally selected R20-candidate parent |
| `CorrectiveEntryBindingV5` | exact closed section 3.1 object; only R20 corrective entry is executable entry and its retained commit observation proves the acceptance parent |
| `ImplementationBindingV6` | exact closed section 3.1 object; current commit/tree, retained sole R20-entry parent, exact complete/corrective deltas, unfiltered no-extra observation, governance and failed custody all validate |
| `ContextRevBinding` | `exit_commit:GIT_ID,implementation_digest:DIGEST,evidence_set_digest:DIGEST,evidence_tree_digest:GIT_ID,unchanged_result_digest:DIGEST`; values are exactly `ab227cc06f15299b594cfe2be99915bd93c4c081`,`c5c2df1178568cd49b5d721cd01cba7cce3371e049528e07bad30d6b3324ea72`,`b95beff569794125018f2fde3d4d3317ed32278dfcfb1fc22a7d25cf51226bd9`,`d554c8c0c3d534aa96924f085a4dc007b25e3a3c`,`f0a15398cc87614cc904cbaa28459ef65ebc267ed70349e46f86f743ebd708c6`; the last hashes exact UTF-8 `rev_unchanged=true<LF>` and proves context only |
| `AuthorBindingV3` | exact closed section 3.1 object; positive and failed source kinds, R20 revision, lineage digest, failed-only exclusion, and dual-bound-ID visibility are mandatory |
| `ArtifactBinding` | `path:REL_PATH,expected_sha256:DIGEST|null,actual_sha256:DIGEST|null,reason:null|enum(missing,deleted,renamed,substituted,unreadable)`; when a same-path row exists the digests/reason equal it; total read/Git failure permits both null with unreadable; reason is null iff both digests are non-null and equal |
| `AllocatedTarget` | exact ordered keys `controlled_id,assertion`; pair must be one exact section 7 edge assigned to this mode; lane strings are admitted only where section 7 uses them |
| `TargetResult` | exact ordered keys `controlled_id,assertion,attempt_ordinal,attempt_argv,start_record_digest,completion_record_digest,recovery_record_digest,state,reason,output_pointer`; identity/ordinal/argv equal same-index allocation and section 6 serial expansion; all three record digests are null for not-run; normal attempts have start/completion non-null and recovery null; every exact recovery-kind terminal has start/recovery non-null and completion null, with recovery binding that start; state/reason/pointer obey the exhaustive target table below |
| `FixtureBinding` | `fixture_id:SAFE_ID,version:VERSION,source_id:SAFE_ID,source_digest:DIGEST,custody_id:SAFE_ID,custody_digest:DIGEST,input_digest:DIGEST,supersession_state:enum(current,superseded)`; values equal one current section 5 row, so all four are `current` in an executable set |
| `ToolVersion` | exact ordered keys `tool,version,digest_source,digest_preimage,digest`; tool/version/source/digest equal the selected section 6 tuple; source enum `version-preimage|artifact-bytes`; preimage is the exact printable string with literal `<LF>` tokens for version-preimage or literal `tools/test_gate.ps1:raw-bytes` for artifact-bytes; digest recomputes from the described bytes |
| `ResourceBounds` | exact keys `wall_seconds,job_memory_bytes,process_memory_bytes,combined_stream_bytes`; values are `60,1073741824,null|1073741824,10485760`; the nullable process bound records only the optional additional `JOB_OBJECT_LIMIT_PROCESS_MEMORY`, while the non-null Job bound always maps to `JOB_OBJECT_LIMIT_JOB_MEMORY`/`JobMemoryLimit`; all non-null values are JSON integers |
| `DeterminismControls` | `order:"bytewise",seed:"disabled",clock:"disabled",locale:"disabled",retry:"disabled"` |
| `ExpectedResult` | `native_exit_u32:0,portable_exit:0,result:"passed",posture:"promotable",reason:"expected-outcome"` |
| `ActualResult` | `native_exit_u32:integer 0..4294967295|null,portable_exit:integer 0..255|null,result:enum(passed,failed,not-run),posture:enum(promotable,non-promotable),reason:enum(expected-outcome,preflight-failed,command-not-started,target-held,unexpected-exit,assertion-failure,bound-exceeded,binding-mismatch,conflict),start_utc:UTC,end_utc:UTC,duration_ms:integer 0..60000`; end is not earlier than start and duration equals their millisecond difference; exits are null together iff no internal argv began; otherwise portable equals native when at most 255 and 255 when larger |
| `EvidenceBinding` | exact keys `wp_revision,lineage_digest,evidence_id,evidence_path,evidence_version,evidence_digest`; revision/lineage equal the enclosing R20 record; ID/path use the same mode and bound version formulas and digest hashes that immutable predecessor |

Identity extraction is literal and total. `wp_r20_candidate` hashes committed
`pulse-34-wp-tst-001-r20-corrective-lineage-candidate.md` bytes and extracts
its sole fenced `vtrace-author-custody.v1` block with exact LF rows
`subject=WP-TST-001-R20`,
`author_id=REV-TST-WP-AUTHOR`, `controller_id=REV-TST-GOVERNANCE-CONTROLLER`,
`subject_digest=<wp_artifact_digest>`. `r20_acceptance_pulse` and
`r20_corrective_entry_pulse` similarly use future committed pulses 35 and 36,
subjects `WP-TST-001-R20-ACCEPTANCE` and
`WP-TST-001-R20-CORRECTIVE-ENTRY`, authors
`REV-TST-ACCEPTANCE-AUTHOR` and `REV-TST-ENTRY-AUTHOR`, governance controller,
and subject digests respectively equal to the WP and acceptance-pulse digests.
Their source digests hash complete raw pulse bytes.

`corrective_implementation_commit` hashes raw `git cat-file commit` bytes and extracts
exactly one final ordered trailer pair `Vtrace-Author-Id:
REV-TST-IMPLEMENTATION-AUTHOR` and `Vtrace-Controller-Id:
REV-TST-IMPLEMENTATION-CONTROLLER`. `mode_evidence` and `set_evidence` hash
canonical LF rows `schema=vtrace-evidence-custody.v2`, `scope=<mode-or-SET>`,
`evidence_id=<execution-origin-id>`, `author_id=REV-TST-EVIDENCE-AUTHOR`,
`controller_id=REV-TST-EVIDENCE-CONTROLLER`, and
`wp_revision=R20`, `lineage_digest=<lineage_binding.lineage_digest>`, and
`wp_digest=<wp_artifact_digest>`. `failed_r14_implementation` instead extracts
the failed commit's trailers, always places those IDs in
`identity_registry.failed_author_ids`, and binds the exact
`FailedAttemptBinding`. If either ID also has a positive binding it remains in
`candidate_author_ids` and remains visible to independence checks; the failed
binding itself grants no candidate, reviewer, execution, or exit authority. An
ID having only failed bindings is excluded from `candidate_author_ids`.
Missing/duplicate/reordered source fields,
working-tree substitution, caller identity, digest mismatch, or any author/
controller not projected into `candidate_author_ids` rejects. Thus no candidate
author can be omitted from independence checks.
For a mode, execution-origin ID is the evidence ID whose version equals
`execution_evidence_version`; review successors retain it and an execution
successor replaces only this binding with its new origin. Positive source refs
are respectively the committed R20 candidate, R20 acceptance, R20 corrective
entry, corrective implementation commit, that execution-origin mode evidence
ID, and the initial set evidence ID. The separate failed source ref is the R14
failed commit and is the only failed-kind binding; its IDs may also occur in
the positive projection only through separate positive bindings. Each source
must resolve to the exact immutable bytes just described.

Target-result optionality is exact:

| State | Exact reason | Exact output pointer |
|---|---|---|
| `passed` | null | `/observed_outputs/structured/content` |
| `held` | `expected-hold|dependency-hold` | `/observed_outputs/structured/content` |
| `failed` | `assertion-failed|timeout|wall-limit|memory-limit|output-limit|bound-kill|crash|supervisor-loss|ledger-corruption|job-identity-lost|unexpected-exit` | one `OUTPUT_POINTER` |
| `not_run` | `preflight-failed|command-not-started|after-terminal` | null |

`target_results` always has allocation cardinality; an attempted prefix is
represented by states, never by truncation or omission. The first
`timeout|wall-limit|memory-limit|output-limit|bound-kill|crash|supervisor-loss|ledger-corruption|job-identity-lost|unexpected-exit`
is terminal. If it occurs while a
target is active, that target is failed with the terminal reason; every later
allocation is `not_run/after-terminal/null`. If termination occurs before or
between targets, the completed prefix is retained and every remaining target
is `not_run/after-terminal/null`; if it occurs after the last target, all target
states remain observed and a `FailureRecord` carries the terminal event. A
preflight failure makes every allocation
`not_run/preflight-failed/null`; a command-start failure makes all
`not_run/command-not-started/null`. Nonterminal assertion failure may be
followed by further attempted results only when the worker actually continued.
No result may move from not-run back to attempted after a terminal boundary.

The structured `counts` are derived, never supplied independently:
`passed`, `failed`, `held`, and `not_run` equal their exact state cardinalities;
`attempted=passed+failed+held`; and
`attempted+not_run=allocated_targets.length`. Modes with no allocation use five
zeros. Mismatch, result omission/reorder, identity substitution, skipped middle
target, non-null optional misuse, or favorable state after terminal rejects.
`actual.result` is `passed` iff every allocated target is passed (including the
vacuous empty allocation), preflight passed, every internal argv exited zero,
the phase/ledger chain is complete, post-execution custody passed, the external
receipt/finalization chain and final snapshot/watch commitments verify, and
all postconditions passed. Any failed target derives `failed`; otherwise
any held or not-run target derives `not-run`; `target-held` is used iff argv
ran and no earlier exact failure reason applies. Its reason is the first
applicable terminal/preflight/command/assertion/binding reason in observed order. Review
state cannot rewrite target state, counts, or command actuals.

`ObservedPreflight` has exact ordered keys
`status,git_observations,repository_inputs,generated_paths,rows,missing_paths,extra_paths,dirty_paths,staged_paths,
untracked_paths,ignored_paths,expected_manifest_digest,actual_manifest_digest,
failure_reasons,preflight_digest`. Status is `pass|failure`.

`git_observations` is exactly the closed sequence in section 6 command-ID
order: `root_discovery,git_dir,common_dir,index_path,replace_refs_before,local_config_before,
committed_status,porcelain_state_before`, one
`binary_diff`, one `tree_inventory`, then one `blob_bytes` object for every
tree-inventory object in emission order, then `porcelain_state_after,
replace_refs_after,local_config_after`. Thus no
Git invocation is aggregated away or omitted.
Each has ordered keys
`command_id,argv,native_exit_u32,portable_exit,stdout_byte_count,stdout_sha256,stderr_byte_count,
stderr_sha256,parse_status,reason`; argv equals the literal command after exact
ID substitution; native exit is uint `0..4294967295|null`, portable exit is
`0..255|null`, both are null together before launch, and otherwise portable
equals native when at most 255 or 255 when larger; byte counts are uint64 and hashes
cover raw complete streams; parse status is `complete|terminal`; reason is null
iff native and portable exits are 0 and parsing is complete, otherwise
`command-failed|malformed-output|stream-bound|count-overflow`. Each
`blob_bytes` object retains its substituted object ID in argv and its own raw
stdout/stderr byte counts and hashes. The before/after local-config observations
must have byte-identical stdout/stderr, proving that complete raw local
effective local/worktree/command-scope configuration custody did not change
during observation.

`repository_inputs` is a closed object with exact ordered keys
`repo_root,git_dir,common_dir,index_path,index_before,index_after,
common_inputs_before,common_inputs_after`. The four paths equal the raw
absolute outputs of the discovery commands after handle normalization; git-dir
and common-dir may differ in a linked worktree, and index path is resolved from
git-dir, never assumed under `.git`. Each index binding and each common-input
binding is a closed `FileBinding` with exact keys
`absolute_path,state,byte_count,sha256`: state is `absent|present`; absent
requires zero/null, present requires exact uint64 raw byte count and digest.
Common inputs are exactly `<common-dir>/info/attributes`,
`<common-dir>/info/exclude`, and `<common-dir>/info/grafts` in that order.
Replace-ref observations must be byte-identical; replacement semantics remain
disabled, and pass additionally requires their stdout empty and the grafts
binding absent. Before/after index bindings and common-input arrays
must be byte-identical, as must raw stdout/stderr for the two porcelain and two
local-config observations. This is the before/after nonmutation custody for the
index and worktree view; a change is terminal failure. System/global inputs
remain disabled and all remaining repository-local inputs are bound.

`generated_paths` has exact ordered keys `root,items,total_count,
manifest_digest`. Root is literal
`context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/`; items are
raw-path-byte-sorted unique objects with exact keys
`path,kind,porcelain_tag,sha256,schema,record_digest,predecessor_digest`, kind
`ledger-record|mode-evidence|set-evidence|review-auth|publication-receipt|
publication-finalization` and tag `?|!`. Schema is the parsed literal schema;
record digest recomputes from the complete canonical file under that schema;
predecessor digest is its exact immediate predecessor or null only where the
closed artifact schema permits an origin. A decoded path is admitted only by
one complete anchored grammar:
`ledger/<MODE>/vEEEE/run-aAAAA/rRRRR-<closed-ledger-kind>.json`,
`runs/<MODE>/EVID-WP-TST-001-<MODE>-vNNNN.json`, or
`sets/EVID-WP-TST-001-SET-vNNNN.json`,
`review-auth/Lii/<REVIEWER_ID>.json`,
`receipts/<MODE>/vEEEE/run-aAAAA/publication-receipt.json`, or
`receipts/<MODE>/vEEEE/run-aAAAA/publication-finalization.json`. `ii` is exact
two-digit `00..21`; reviewer ID must satisfy the closed `REVIEWER_ID` grammar
and the file's lane/reviewer fields. Numeric widths, MODEs, record kinds, and
version/run/ordinal relationships are exactly those closed in this WP. SHA-256
hashes exact raw bytes and must agree with canonical record digest rules;

For R20 the kind-to-schema/digest-key map is exact:
`ledger-record -> test-gate-ledger.v4/record_digest`,
`mode-evidence -> test-gate-evidence.v13/evidence_digest`,
`set-evidence -> test-gate-evidence-set.v12/set_digest`,
`review-auth -> vtrace-review-auth.v3/record_digest`,
`publication-receipt -> test-gate-publication-receipt.v3/receipt_digest`, and
`publication-finalization -> test-gate-publication-finalization.v3/finalization_digest`.
No earlier schema version is current, generated-path-valid, selectable by set
assembly, or promotable under R20.

total count equals item count; manifest digest hashes each
canonical complete item plus LF in item order. Before each mode this object
must enumerate every prior retained ledger, mode/set evidence, and review
successor/authentication/receipt/finalization artifact and nothing else, and
each item must validate its own schema, digest, predecessor chain, and formula
binding. Every previously retained terminal run and every promotion candidate
requires its complete ledger plus mode evidence, publication receipt, and
finalization. For the single current run, post-worker/post-evidence/
post-receipt snapshots permit only the exact proper creation prefix named by
`publication_plan`; post-finalization requires the complete quartet. Every non-null decision requires its
one formula-bound review-auth artifact; every referenced set requires its set
file. A malformed path, missing mandatory artifact, unknown file, wrong
hash, stale chain, omitted retained item, or substitution fails preflight.
Generated items are explicitly excluded from `rows`, both 18-path manifest
preimages, and missing/extra classification because those derive only from the
original-R14-entry-to-corrective-implementation
`complete_implementation_delta`; their sole porcelain custody is the
exact untracked/ignored projection below. They can never satisfy an expected
implementation row or conceal a dirty/staged implementation path.

`rows`, `missing_paths`, `extra_paths`, `dirty_paths`, `staged_paths`,
`untracked_paths`, and `ignored_paths` are closed `TotalList` objects with exact
keys `retained_items,total_count,count_state,full_set_digest`.
`retained_items` contains the first `min(total_count,64)` canonical items in
the authoritative order (`missing_paths` can contain at most the 18 allowed
paths). `total_count` is uint64 `0..18446744073709551615`; `count_state` is
`exact|count-overflow`. Exact means total count is the true complete count.
Count-overflow is the terminal variant when incrementing the uint64 maximum
would overflow; total count then remains the maximum sentinel, retained items
remain honest, preflight fails, and reason `count-overflow` is mandatory. There
is no smaller arbitrary ceiling. `full_set_digest` is always `DIGEST` over
`schema=preflight-list.v1<LF>kind=<field><LF>` followed by every canonical item
and `<LF>` in authoritative order; for terminal parsing it additionally ends
`terminal=<reason><LF>`. The streaming digest covers the full observed set,
including every unretained member and the item that triggered count overflow.
Every Git-derived path collection is ordered by unsigned decoded raw-path
bytes, never Unicode, locale, case folding, quoted display, or base64 spelling;
duplicate decoded byte strings reject even if encoded differently.

Each row item is a closed `ObservedDeltaRow` with exact keys
`path,allowlisted_path,classification,git_status,old_path,expected_sha256,actual_sha256,reason`.
Classification is `matched|missing|extra|nonallowlisted|deleted|renamed|
substituted`; `path` is always `RawGitPath`; `allowlisted_path` is a `REL_PATH`
only when decoded bytes are exactly that path's UTF-8 bytes and otherwise null;
Git status is `A|M|D|R|null`; old path is `RawGitPath|null`; expected
and actual hashes are independently `DIGEST|null`; reason is null or the same
non-matched classification. Coherence is exhaustive:

| Classification | Allowlisted / Git / old path | Expected / actual digest | Reason |
|---|---|---|---|
| `matched` | non-null / `A|M` / null | both non-null and equal | null |
| `missing` | non-null / null / null | non-null / null | `missing` |
| `extra` | null / `A|M` / null | null / non-null | `extra` |
| `nonallowlisted` | null / `A|M` / null | null / non-null | `nonallowlisted` |
| `deleted` | non-null / `D` / null | non-null / null | `deleted` |
| `renamed` | nullable / `R` / distinct non-null | expected nullable / non-null | `renamed` |
| `substituted` | non-null / `A|M` / null | both non-null and unequal | `substituted` |

Row authoritative order is the exact 18-path complete-implementation-delta
emission order, followed by
missing allowed paths in allowed-path order. No expected row is fabricated for
an absent observation and no D/R/extra row is discarded. An unlisted target
under `crates/bastion-boundary-tests/`, `fixtures/bootstrap/`, or `tools/` is
`extra`; every other unlisted target is `nonallowlisted`. Rename classification
takes precedence while its target also appears in extra paths when unlisted.
Porcelain authoritative order supplies dirty/staged/untracked/ignored lists,
whose retained items are `RawGitPath` objects;
missing/extra are exact row projections. Duplicate logical paths reject.

Failure reasons are the exact applicable subset of the following enum, emitted
strictly bytewise sorted:
`git-preflight-failed,malformed-git-output,configuration-change,repository-input-change,missing,extra,nonallowlisted,deleted,
renamed,substituted,dirty,staged,untracked,ignored,manifest-mismatch,
stream-bound,count-overflow`, `0..17`. Expected and actual manifest digests are
independently `DIGEST|null`; the expected side projects the exact accepted
18-row section 3 status/path inventory with the corrective target blob OIDs,
while the actual side projects only the freshly executed
`complete_implementation_delta` and its 18 blob reads. Each hashes all canonical
JSON-line bytes `{"path":<canonical-RawGitPath>,"sha256":<DIGEST-or-null>}<LF>`
for its side in the 18-path allowlist order and may be null only when a
Git/read/terminal failure makes that full side unobservable. Equality is
required only for pass. `preflight_digest` is last and omits only itself.
Expected paths are canonical `RawGitPath` encodings of the literal allowlist
UTF-8 bytes; actual paths are the observed raw objects. JSON key order, base64
padding, explicit null, object boundary, and LF make every preimage
unambiguous even when decoded paths contain TAB/LF/CR/control or invalid UTF-8.

Preflight is pass iff every required Git observation is complete/zero/null-reason,
the two local-config streams, two porcelain streams, index bindings, and
repository-input snapshots are byte-identical;
all seven lists have exact count state; rows contain exactly 18 matched allowed
paths with required A/M status; missing/extra/dirty/staged are empty;
untracked and ignored are exactly the `generated_paths` projections selected
by `porcelain_tag`, with no path in both, and contain no other item; both manifest
digests are non-null/equal; and the 18 artifact bindings equal the allowed-row
projection. Every other honest shape is failure, carries every applicable
reason, makes actual `not-run` with both exits null/reason `preflight-failed`, and
requires a `preflight-failed` FailureRecord; it is schema-valid but never
promotable. Negative cases cover partial/missing/extra/nonallowlisted/D/R/
substituted rows, every porcelain class, Git/parse/stream failure,
configuration/repository-input mutation, full-set
digest/count substitution, count-overflow, and copied-allowlist fabrication.
Conversely the not-run preflight variant is valid iff preflight is failure.

After the run ledger reaches completion/recovery and before the final evidence
path exists, the supervisor performs `post_execution_custody`. It has exact
ordered keys `status,execution_material,pre_materialization_snapshot,
post_materialization_snapshot,post_worker_snapshot,
root_observation,git_dir_observation,common_dir_observation,
effective_config_observation,index_binding,porcelain_observation,
replace_refs_observation,common_inputs,implementation_tree_digest,
generated_paths_after,allowed_new_paths,publication_plan,custody_digest`. Every Git observation is the exact section 6
argv/environment with command ID suffixed `_post`, retaining native/portable exits and raw
stdout/stderr counts/digests. Root/git/common outputs, effective config, index,
replace refs, common inputs (including grafts), and implementation tree must be
byte-identical to preflight.

Postrun invocation order and mapping are literal:
`root_discovery_post=root_discovery`, `git_dir_post=git_dir`,
`common_dir_post=common_dir`, `index_path_post=index_path`,
`replace_refs_post=replace_refs_before`,
`local_config_post=local_config_before`,
`porcelain_state_post=porcelain_state_before`,
`tree_inventory_post=tree_inventory`, followed by one
`blob_bytes_post=blob_bytes` per implementation-tree object in emission order.
The right side names the exact complete argv array in section 6; `_post`
changes only retained command ID, never argv. Substitutions remain the same
absolute root and implementation/object IDs. Root/git/common/index discovery
and replace-ref arrays intentionally carry no `-c` entries because they only
resolve paths/list disabled replacement refs; all config-, diff-, status-,
tree-, and blob-semantic arrays carry the complete common `-c` sequence exactly
as printed in section 6. No invocation may inherit or synthesize that sequence.
The post tree inventory plus per-object blob reads recompute the exact
implementation tree binding and must equal both the preflight value and
`implementation_binding.tree_digest`; a merely clean worktree is insufficient.

`generated_paths_after` is a fresh full filesystem/content parse under the
same exact closed schema as preflight `generated_paths`; it never reuses the
preflight list. It must equal the preflight items byte-for-byte plus exactly
the current run's newly durable ledger records in record order. Every prior
path/tag/raw hash/schema/record digest/predecessor chain is recomputed, and the
complete manifest count/digest is recomputed. `allowed_new_paths` is exactly
the raw-byte-sorted projection of only those current-ledger additions;
porcelain may add only those paths or
an emitted directory ancestor whose decoded bytes are an exact prefix ending
`/` of that root. All prior generated entries retain exact tag/hash/schema/
digest/chain. All other pre-existing porcelain entries remain byte-identical and
no other entry may appear, disappear, or change class. Status is pass iff those
equalities hold, execution material and all three execution snapshots validate,
the watch has no prohibited event/overflow, and every post command is
zero/complete. The final evidence
file is create-new published only after this pass, so it is not an exception.
Any root/index/worktree/config/replace/graft/tree drift is retained as
non-promotable `binding-mismatch`; review cannot rewrite it.

`execution_material` is a closed object with exact ordered keys
`source_commit,source_tree,materialization_root,cargo_target_dir,
watch_identity,materialization_invocations,tree_manifest_digest,
worker_cwd,worker_argv,environment_digest,denied_handle_set_digest,
pre_worker_manifest_digest,post_worker_manifest_digest,
watch_prefix_count,watch_prefix_digest,material_digest`. Source commit/tree
equal the implementation binding. Both roots are distinct create-new
directories below resolved external `TEMP`, outside the repository and each
other, with no reparse point or alternate data stream. Worker cwd equals the
materialization root; worker argv is the section 6 exact internal argv; Cargo
receives the exact external target directory. No worktree path is opened by a
worker, Cargo, rustfmt, clippy, test, or assertion process.

Before the first materialization Git read, the bootstrap supervisor opens
read/delete/write-denying handles to every resolved repository input, arms
recursive change watches on the worktree, worktree Git dir, common Git dir,
and the two external roots, and writes a `watch-start` event. `watch_identity`
is a `DIGEST` of canonical root/volume/file-ID rows for those watched handles.
The watch remains armed without a gap through finalization creation and its
verification below. Every event has exact keys
`sequence,source,action,raw_path,byte_count,content_sha256`: sequence is
consecutive uint64; source is `worktree|worktree-git|common-git|
materialization|cargo-target`; action is `create|write|rename-from|rename-to|
delete|overflow|watch-start|watch-stop`; raw path is `RawGitPath|null` only for
start/stop/overflow; byte count is uint64 or null; content digest is `DIGEST|
null` and is required for a stable file after create/write, except the
formula-bound finalization self-event below. Overflow is terminal. Event digest
hashes canonical event JSON plus LF; prefix/full digests use the section 6
Merkle rule and exact counts.

Materialization invocations are the exact fixed Git prefix followed by
`ls-tree -r -z --full-tree <implementation_commit>` and one
`cat-file blob <object-id>` per tree blob in raw-path order. Each invocation
retains the closed native/portable exit and full stream custody. Tree entries
may be only regular nonexecutable/executable blobs or directories; symlink,
submodule, device, reparse point, duplicate raw path, case-fold collision, or
alternate stream rejects. For every blob the supervisor recomputes Git object
ID from exact `blob <decimal-byte-count>\0<raw-bytes>` and requires the bound
object ID, writes create-new bytes, flushes file and parent directory, reopens
read-only with write/delete sharing denied, and rehashes. Manifest digest hashes
canonical `RawGitPath,mode,object-id,byte-count,SHA-256` rows. The materialized
runner then re-execs as supervisor and launches the worker; the original
worktree runner is only a root/materialization bootstrap and supplies no argv,
code, fixture, configuration, outcome, or evidence byte after re-exec.
Pre/post worker manifest digests recompute every materialized input and must
equal the tree manifest. Denied-handle digest binds sorted volume/file-ID/
path/size/content/access/share-mode rows for all repository and materialized
input handles held through finalization verification. Any watch event for an
original repository input or materialized input, even if restored, is
`transient-mutation`; Cargo-target events are expected but cannot name another
root. Material digest omits only itself.

Every snapshot named here—`pre_materialization_snapshot`,
`post_materialization_snapshot`, `post_worker_snapshot`, `post-evidence`,
`post-receipt`, and `post-finalization`—is a closed `CustodySnapshot` with exact keys
`stage,git_observations,repository_inputs,generated_paths,self_projection,
implementation_tree_digest,material_manifest_digest,worktree_digest,
watch_event_count,watch_event_digest,snapshot_digest`. Stage is the matching
literal `pre-materialization|post-materialization|post-worker|post-evidence|
post-receipt|post-finalization`; Git observations repeat
the complete literal postrun mapping; inputs and generated paths are freshly
read/parsed; tree and material digests recompute; worktree digest covers the
complete raw status set; watch count/digest bind the exact prefix. The first is
taken only after handles/watches are active and before any materialization read;
the second after all material files/handles verify and before worker launch;
the third after Job termination/report/ledger closure and before evidence
publication. Repository/tree/generated bindings must be equal across the first
three except exact current-run ledger additions at post-worker. Publication
snapshots then add exactly evidence, receipt, and finalization in prescribed
one-at-a-time order; any other current or transient difference fails. Thus neither a restored mutation nor a
materialization-time mutation can disappear between snapshots.
`self_projection` is null in every stage except post-finalization. There it has
exact keys `path,kind,porcelain_tag,schema,byte_count,self_sha256`, with the
formula-bound finalization values and literal null self SHA. That snapshot's
`generated_paths` contains every required item except the current
finalization; validators merge the self projection only after separately
hashing/verifying the finalization record. No other missing generated item or
self projection is legal.

`publication_plan` has exact ordered keys
`evidence_path,receipt_path,finalization_path,allowed_creation_order,
finalization_self_event,watch_stop_event,plan_digest`. Evidence path is the enclosing create-new
mode path. Receipt and finalization paths are exact
`context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/receipts/
<MODE>/vEEEE/run-aAAAA/publication-receipt.json` and
`.../publication-finalization.json`. Creation order is exactly
`[evidence_path,receipt_path,finalization_path]`. The self event is the exact
future watch event for finalization creation with its known next sequence,
source worktree, action create, raw path, final byte count, and null content
digest; null is required solely to avoid a self-digest cycle. Watch-stop is the
immediately following exact event with next sequence, source worktree, action
watch-stop, null path/count/digest. Plan digest omits only itself.

Publication is exact and create-new throughout. After the passing post-worker
snapshot, the supervisor canonicalizes evidence in memory, writes/flushes/
closes it, flushes its parent, and takes a full `post-evidence` snapshot. It
then writes a canonical `test-gate-publication-receipt.v3` with exact keys
`schema,wp_revision,lineage_binding,execution_id,run_id,evidence_path,evidence_digest,
post_evidence_snapshot,watch_event_count,watch_event_digest,status,
failure_reasons,receipt_digest`; status is `pass|failure`; reasons are the
exact sorted subset of `write-failed|flush-failed|snapshot-failed|
unexpected-path|transient-mutation|watch-overflow`, and pass iff exact `[]`
and all prior equalities hold. The digest omits only itself. After durable receipt creation it takes a full
`post-receipt` snapshot and prepares `test-gate-publication-finalization.v3`
with exact keys `schema,wp_revision,lineage_binding,execution_id,run_id,evidence_path,evidence_digest,
receipt_path,receipt_digest,post_receipt_snapshot,
expected_post_finalization_snapshot_digest,expected_final_watch_event_count,
expected_final_watch_digest,status,failure_reasons,finalization_digest`.
Status/reasons use the identical receipt rule and include every applicable
failure. Its digest omits only itself.
The expected final snapshot is canonical status/input/generated custody with
the finalization represented only by the exact `self_projection`; all other
file hashes are non-null. Expected final watch
fields inside that snapshot bind the prefix through `finalization_self_event`;
the finalization's separate expected final watch digest/count additionally
append exactly `watch_stop_event`.

For both v3 publication records, `wp_revision` is literal `R20` and
`lineage_binding` is byte-identical to the enclosing evidence v13 object. Its
accepted R20 WP, corrective entry, failed attempt, governance delta, complete
delta, corrective delta, and current implementation values validate before any
receipt/finalization digest is computed. V1 publication records cannot bind or
promote an R20 execution.

The supervisor durably create-new writes finalization, observes exactly that
self event, takes the actual `post-finalization` snapshot while every watch and
denied handle remains active, then emits/observes the exact watch-stop event
and closes the watch. It requires snapshot digest equal the precommitted expected digest using
the same canonical self projection; it separately
verifies the finalization SHA-256 against `finalization_digest`. It also
requires actual final watch count/digest equal the precommitted values. No
additional event is permitted between self-event and watch-stop. Validators
and set assembly recompute evidence, receipt, finalization, all four
publication snapshots, and the complete closed watch log; they require all
three files and exact creation order before promotion. Evidence contains only
the acyclic plan, receipt binds evidence, and finalization binds receipt plus
precommitted self-excluding final checks. No artifact is rewritten, no digest
contains itself, and no later mutable completion field exists.

`actual.result="passed"` iff both exits are 0, posture is `promotable`, reason is
`expected-outcome`, `failure_records`, `counterexamples`, and `conflicts` are
empty, preflight status is pass, every allocated target passed, and the exact
publication receipt/finalization chain verifies. Preflight
failure, command-not-started, or an otherwise nonfailed held/not-run allocation
derive `not-run` with the exact exit-pair/reason mapping above. Otherwise result is `failed`, posture
is `non-promotable`, and reason identifies the highest-precedence cause:
`bound-exceeded`, `binding-mismatch`, `conflict`, `unexpected-exit`, then
`assertion-failure`. A valid Job-wide `memory-limit` termination derives
`bound-exceeded` and requires a matching `bound-exceeded` FailureRecord; a
missing/mismatched mandatory Job limit derives `binding-mismatch` and never
masquerades as a measured bound event. Top-level
`status` is `stale` on a
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
`schema,scope,command_identity,native_exit_u32,portable_exit,result,posture,reason,counts`. Schema is
literal `test-gate-structured-result.v2`; scope is a `MODE` for a mode record or
literal `SET` for a set; command equals the enclosing command; exit pair/result/
posture/reason equal its closed actual/result values; and `counts` is exactly
`attempted,passed,failed,held,not_run`, each integer `0..148`, with attempted
equal to passed plus failed plus held and attempted plus not-run equal to the
allocation cardinality. For a mode these are the exact `target_results` state
projections; a preflight not-run variant has both exits null, attempted/passed/
failed/held zero, and not-run equal to allocation cardinality. Held and not-run
are zero for a passed result. For a set, the allocation cardinality is 16 modes
and the five counts project their immutable actuals as specified below. Native
exit is `integer 0..4294967295|null`; portable exit is `0..255|null` under the
lossless mapping above; result is `passed|failed|not-run|held`, posture is
`promotable|non-promotable|held`, and reason uses the enclosing closed mapping.
Byte count is
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
where entry tree equals `corrective_entry_binding.tree_digest`.

`ReproductionPlan` content has exactly
`plan_id,command_identity,exact_argv,input_digests,controls,expected_result`.
Plan ID is exactly `PLAN-WP-TST-001-REPRO-<mode>`; command and argv equal the
top-level values; `input_digests` is exactly 18 closed `ArtifactBinding` objects
equal to top-level `artifact_digests`; controls equal top-level
`DeterminismControls`; expected result equals top-level `ExpectedResult`.
Plan strings never contain an external/absolute path, URI, symlink, secret, raw
fixture byte, or reconstructive content.

The following diagnostic objects are closed; keys are exactly ordered as shown:

| Type | Ordered members and exact rules |
|---|---|
| `FailureRecord` | `id,code,assertion,output_pointer,disposition,digest`; id exactly `FAIL-<mode>-vNNNN-NNN` with ordinal `001..128`; code enum `preflight-failed|unexpected-exit|assertion-failure|bound-exceeded|binding-mismatch|conflict`; assertion is an `ASSERTION` executed by this mode or literal `supervisor::preflight`; pointer is `OUTPUT_POINTER`; disposition enum `open|retained|remediated`; digest is last and hashes this object with only preceding comma plus digest member omitted |
| `Counterexample` | `id,assertion,input_digest,reproduction_plan_digest,output_pointer,disposition,digest`; ID exactly `CEX-<mode>-vNNNN-NNN`; assertion as above; input digest is `DIGEST` of a bounded non-reconstructive canonical synthetic descriptor; reproduction digest equals top-level reproduction-plan digest; pointer is `OUTPUT_POINTER`; disposition enum `open|retained|remediated`; digest uses the same omission rule |
| `Finding` | `id,severity,claim_digest,evidence_pointer,owner,destination,closure,disposition`; ID exactly `FND-<scope>-Lii-vCCCC-NNN`, where `CCCC` is its immutable creation-decision version `1..enclosing decision_version`; lane/scope remain enclosing; severity enum `critical|major|minor|note`; claim is `DIGEST`; pointer is local `OUTPUT_POINTER`; owner is `REVIEWER_ID`; destination is `LANE`; closure enum `open|remediated|not-applicable`; disposition enum `hold|accept|defer` |
| `Defer` | `id,claim_digest,owner,destination,closure_condition,hold_behavior,closure`; ID exactly `DEF-<scope>-Lii-vCCCC-NNN`; claim is `DIGEST`; owner is `REVIEWER_ID`; destination is `LANE`; condition matches `^[A-Za-z0-9][A-Za-z0-9 ._:-]{0,127}$`; hold behavior literal `block-promotion`; closure enum `open|remediated` |
| `Dissent` | `id,lane,claim_digest,record_digest,disposition`; ID exactly `DIS-<scope>-Lii-vCCCC-NNN`; lane equals enclosing; both digests are `DIGEST`, record digest cites prior evidence/decision only; disposition enum `open|retained|resolved` |
| `Conflict` | `id,lane,left_digest,right_digest,owner,disposition`; ID exactly `CON-<scope>-Lii-vCCCC-NNN`; lane equals enclosing; distinct digests are `DIGEST`; owner is `REVIEWER_ID`; disposition enum `open|retained|resolved` |

Within each newly appended type/version batch of cardinality N, ordinals are
exactly `001..N` without a gap. Decision item arrays use ascending creation
version then ordinal; IDs are unique across their containing projection. An
open critical/major finding, defer, dissent, conflict, or diagnostic blocks.
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

A non-null slot is one closed `test-gate-decision.v3` object with exact ordered keys
`schema,wp_revision,lineage_binding_digest,decision_id,decision_version,predecessor_decision_id,
predecessor_decision_digest,predecessor_decision_version,lane,status,
reviewer_id,reviewer_authentication_digest,independence,assurance,findings,defer,dissent,conflicts,closure,
decision_record_digest`. For scope `<scope>` equal to enclosing mode or `SET`
and index `ii` zero-padded to two digits, decision ID is exactly
`DEC-WP-TST-001-<scope>-Lii-vNNNN`, where `NNNN` is `decision_version`.
Decision IDs and digests are unique across the complete lane history.

Decision version is `VERSION` and equals 1 for a null-slot predecessor;
otherwise it equals prior same-lane decision version plus 1. On version 1 the
predecessor fields are exactly `null,null,0`; otherwise predecessor ID/digest/
version are non-null and equal the immediately prior same-scope, same-index
decision. Lane equals the indexed lane. Reviewer ID is `REVIEWER_ID`; it differs
from every finding/defer/conflict owner assigned to adjudicate that decision.
`reviewer_authentication_digest` is the `DIGEST` of that reviewer's immutable
authentication record at exact create-new path
`context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/review-auth/Lii/<reviewer_id>.json`
for the lane's zero-padded index. That closed canonical record has exact keys
`schema,wp_revision,lineage_binding_digest,lane_index,lane,reviewer_id,authorizing_controller_id,
wp_artifact_digest,record_digest`; schema is literal
`vtrace-review-auth.v3`, revision is literal `R20`, lineage digest equals the
enclosing evidence/set lineage, lane fields equal the indexed lane, authorizing
controller is a `REVIEWER_ID` present in `candidate_author_ids`, WP digest
equals the enclosing value, and record digest omits only itself. The top-level
authentication digest equals that verified record digest and is byte-identical
for every occurrence of that reviewer across current and historical records;
no caller-presented identity or free authentication preimage is accepted.
The decision's schema/revision/lineage fields are immutable across same-lane
successors and equal its enclosing evidence v13 or set v12. A v1/v2 decision or
v1/v2 review-auth record, an R14/R15 WP digest, or a failed-attempt author/controller
cannot authenticate an R20 review.
Reviewer identity is an absolute candidate-wide lane bijection: one
`reviewer_id` may occur in mode/set/current/history decisions for one and only
one lane, and every non-null lane has exactly one reviewer ID. Reuse across a
second lane rejects even if a conflict is recorded; the duplicate-reviewer
negative fixture covers mode, set, current, and predecessor substitution.
Independence is `independent|conflicted`, but it is a derived projection rather
than a reviewer assertion: it is `independent` iff `reviewer_id` differs from
every candidate-wide `candidate_author_ids` member, retains
one authenticated identity across its lane history, and occurs in no other
lane; otherwise it is `conflicted` and an open conflict naming the identity
collision is mandatory. A supplied enum inconsistent with those custody facts
rejects. Assurance is `pass|fail|
not-applicable`: indexes 0 and 1 require pass or fail, all other indexes require
not-applicable. Findings, defer, dissent, and conflicts are each `0..128`
ordered unique closed arrays from section 8.4 with matching scope/index/lane.
Their immutable creation version encoded in ID is at most decision version.

Status is `pass|finding|defer`; decision closure is
`open|remediated|not-applicable`. Derivation is exact and precedence ordered:
`defer` iff independence is conflicted or an open defer/dissent/conflict exists;
else `finding` iff an open finding or failed assurance exists; else `pass`.
Closure is open iff status is finding/defer; remediated iff this successor
closed at least one predecessor-open item and no item remains open; otherwise
not-applicable. Assurance pass/fail and open critical/major finding are
conjunctively coherent; failed assurance requires an open critical/major
finding. Conflicted independence requires at least one open conflict. Pass
requires independent, no open item, and assurance pass at indexes 0/1.
For this derivation, dissent/conflict disposition `retained` is open; a finding
is open iff closure is open; and a defer is open iff closure is open.

For each of the four arrays, a successor's first P items correspond one-to-one
to all P predecessor items in identical order. ID, creation version, claim
digest, evidence/record/left/right digest, lane, owner, destination, closure
condition, and hold behavior are byte-identical. Only the state fields may move
under this exhaustive matrix:

| Item | Allowed predecessor -> successor state |
|---|---|
| finding | tuple `(open,hold)` remains `(open,hold)` or becomes `(remediated,accept)`; `(open,defer)` remains `(open,defer)` or becomes `(remediated,accept)`; `(remediated,accept)` is stable; `(not-applicable,accept)` is stable |
| defer | `open -> open|remediated`; `remediated -> remediated` |
| dissent | `open -> retained|resolved`; `retained -> retained|resolved`; `resolved -> resolved` |
| conflict | `open -> retained|resolved`; `retained -> retained|resolved`; `resolved -> resolved` |

No predecessor item may be deleted, reordered, renamed, have its claim changed,
or be represented only in predecessor history. New items are allowed only as a
contiguous append after all predecessor items, must use the current decision
version and consecutive creation-batch ordinals, and start open (`open` plus
hold/defer for a finding). A pass successor may append no item. Thus cardinality
is exactly P+N with N `0..(128-P)`; closure may reconcile old items and append
new ones in the same finding/defer successor, but derived status follows the
remaining open items. Negative cases cover dropped, reordered, duplicate,
renumbered, changed-claim, falsely closed, non-appended, and pass-added items.

`decision_record_digest` is `DIGEST`, last, and hashes canonical decision bytes
with only the preceding comma and that member omitted. It does not hash its
enclosing record or pointer, so the preimage is acyclic.

Version-1 command execution has 22 null slots. A one-lane mode successor opens
only the next unused evidence version and changes exactly one slot. Every
non-derived field is byte-identical to the immediate predecessor, specifically
`schema,mode,execution_id,execution_ordinal,execution_evidence_version,execution_origin,
predecessor_execution,execution_history,wp_id,wp_revision,wp_artifact_digest,lineage_binding,acceptance_binding,corrective_entry_binding,
implementation_binding,logical_predecessor_commit,context_rev_binding,
identity_registry,candidate_author_ids,artifact_digests,trace_manifest_digest,allocated_targets,
execution_phases,target_results,ledger_binding,post_execution_custody,fixture_bindings,
command_identity,exact_argv,tool_versions,environment_digest,resource_bounds,
determinism_controls,expected,actual,observed_outputs,rollback_plan,
reproduction_plan,failure_records,counterexamples,required_review_lanes,
invalidation_triggers`; the other 21 slots are also byte-identical.

The complete and only synchronized derived delta is: increment
`evidence_version` by 1; set `successor_kind=review`; regenerate `evidence_id`
and `evidence_path`; bind the
immediate predecessor in `predecessor_evidence`; append exactly that binding to
`history`; replace exactly the indexed decision slot with its formula-bound
next decision; recompute `findings`, `defers`, `dissent`, and `conflicts` as exact sorted
projections of all 22 current slots; recompute `status` by section 8.2; and
recompute `evidence_digest`. No other byte may change. The changed decision's
four arrays reconcile predecessor items one-to-one and append only permitted
new items; the 21 retained decisions' contributions remain byte-identical.
This reconciliation is exact, not a merge, patch, or reviewer-selected summary.

An execution successor is distinct and is permitted only after the current run
is terminal and its evidence/receipt/finalization validate. It represents an
explicit failed, corrected-supervision, or independently reproduced new run;
it never edits or retries the old ledger. The immediate predecessor record may
be `origin|review|execution`, but the bound prior execution is the execution
origin shared by that record and its review chain. Exactly these stable fields
remain byte-identical:

```text
schema, mode, wp_id, wp_revision, wp_artifact_digest, lineage_binding, acceptance_binding, corrective_entry_binding,
implementation_binding, logical_predecessor_commit, context_rev_binding,
candidate_author_ids, artifact_digests, trace_manifest_digest,
allocated_targets, execution_phases, fixture_bindings, command_identity,
exact_argv, tool_versions, environment_digest, resource_bounds,
determinism_controls, expected, rollback_plan, reproduction_plan,
required_review_lanes, invalidation_triggers
```

The complete execution-transition delta is exact: increment
`evidence_version` by one, set `execution_evidence_version` equal to that
new record version, and increment `execution_ordinal` by exactly one from the
prior execution irrespective of intervening review versions; allocate the next
unused run attempt and new run ID/ledger root;
derive the new execution ID/origin; set `successor_kind=execution`; set
`predecessor_execution` to the exact `ExecutionBinding` of the prior execution's
latest immutable evidence record; append exactly that binding to
`execution_history` (origin has zero, each new execution has one more, in
consecutive execution-ordinal order, while retaining each exact nonconsecutive
origin evidence version); regenerate evidence ID/path; replace only the
mode-evidence `AuthorBinding` with the formula-bound new execution record while
retaining the other six of the seven mode-registry bindings, including the
failed binding as nonauthorizing visible history, and retaining the identical
positive author-ID projection; create fresh
target results, ledger binding, execution material, repository/publication
custody, actual result, observed outputs, failure records, and counterexamples
from the new run; reset all 22 reviewer slots to null and the derived findings,
defers, dissent, and conflicts to `[]`; derive status from the new execution;
bind the immediate prior evidence in `predecessor_evidence`, append it once to
the complete record `history`, and recompute evidence digest. No other byte may
change.

Fresh execution fields never copy an observed byte/digest merely because the
run is a reproduction: equality must be independently re-observed. A corrected
run may correct supervisor operation or external transient state only; any
implementation, expected tuple, plan, fixture, tool, environment, or governance
change requires a new accepted WP/implementation binding, not an execution
successor. Prior evidence, ledgers, receipts, finalizations, review-auth files,
decisions, findings, and both histories remain create-new and reachable.
Negative transitions reject reused run attempt/root, changed stable field,
missing/reset history, carried review decision, copied observation, rewritten
prior artifact, execution ordinal gap/duplicate, execution version not equal to new record version, or review
successor that changes any execution field.

Only the latest successor with all 22 non-null current lanes may be
`fully_reviewed` or contribute to exit. Every lane is independent, both
assurance lanes are `pass`, all predecessors/digests/versions verify, and zero
current critical/major finding, open defer, open dissent conflict, or evidence
conflict is mandatory.

### 8.6 Independently closed `test-gate-evidence-set.v12` schema

A set uses section 8.1 canonical encoding and these exact ordered keys:
`schema,set_id,set_version,set_path,wp_id,wp_revision,wp_artifact_digest,lineage_binding,
acceptance_binding,corrective_entry_binding,implementation_binding,identity_registry,candidate_author_ids,mode_records,
aggregate_digest,observed_outputs,required_review_lanes,reviewer_decisions,
findings,defers,dissent,conflicts,status,review_completeness,rollback_plan,reproduction_plan,
predecessor_set,history,invalidation_triggers,set_digest`.

`schema` is literal `test-gate-evidence-set.v12`; set version is `VERSION`; ID is
exactly `EVID-WP-TST-001-SET-vNNNN`; path is exactly
`context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/sets/<set_id>.json`;
WP ID is literal `WP-TST-001`; `wp_revision` is literal `R20`; WP digest,
lineage, and the three binding objects use the fully closed definitions and
equalities in sections 3.1 and 8.2. Set `identity_registry` is one closed
`IdentityRegistryV3` whose bindings are the exact source-kind/source-ref-sorted
unique union of the 16 selected mode registries: the shared R20 candidate,
acceptance, corrective-entry, failed-attempt, and corrective-implementation
bindings, 16 distinct mode-evidence bindings, and one shared set-evidence
binding, exactly 22 bindings. Set projections apply the same total rule as each
mode: a failed binding is always nonauthorizing; every ID named by a failed
binding remains in `failed_author_ids`; an ID also named by any positive
binding remains simultaneously visible in `candidate_author_ids`; only an ID
whose bindings are all failed is excluded from the candidate projection.
`candidate_author_ids` is the identical author/controller projection in the
set and every selected mode. `mode_records` is
exactly 16 closed objects with ordered keys
`wp_revision,lineage_digest,mode,execution_id,execution_ordinal,execution_evidence_version,
evidence_id,evidence_version,evidence_path,evidence_digest,
receipt_path,receipt_digest,finalization_path,finalization_digest`, in section 6
MODE order; execution fields equal the selected mode record and obey its
ordinal/history rules; each field uses the mode formulas/types in section 8.2, each digest
recomputes, receipt/finalization paths equal that execution's publication
plan, and the complete acyclic publication/final snapshot/watch chain verifies.
All selected records have identical R20 WP/lineage/acceptance/corrective-entry/
implementation/failed-attempt/complete-delta/corrective-delta bindings.
`aggregate_digest` is `DIGEST` over each selected
`<wp-revision><TAB><lineage-digest><TAB><execution-id><TAB><execution-ordinal><TAB><execution-evidence-version><TAB>
<evidence_path><TAB><evidence_digest><TAB><receipt-path><TAB><receipt-digest>
<TAB><finalization-path><TAB><finalization-digest><LF>` in strict bytewise path order.

`observed_outputs` is a closed section 8.3 `ObservedOutputs`: stdout/stderr use
the identical bounded `ByteStream`; structured content uses the identical one
literal `StructuredResult` schema with scope `SET`, command literal
`CMD-TST-EVIDENCE-SET`. Counts are an exact projection of the 16 selected
immutable command actuals: passed counts `passed`; failed
counts `failed`; held is zero because mode `ActualResult` has no held state;
not-run counts `not-run`; and
attempted equals passed plus failed plus held while attempted plus not-run is
16. Review
state never changes those counts. Exit/result/posture/reason are instead the
exact top-level set-state mapping below; byte count and hash use section 8.3.

Required lanes are the exact 22-index array. Decisions are exactly 22 null or
closed `DecisionRecord<SET,index>` values under every section 8.5 constraint.
Top-level findings, defers, dissent, and conflicts are `0..2816` unique objects and are
the exact strictly ID-sorted projections of those current set decisions; set
finding evidence pointers are `OUTPUT_POINTER` values resolving only within the
set's embedded observed outputs. Status derivation uses strict precedence:
`conflicted` iff an open conflict/dissent or conflicted reviewer exists;
otherwise `failed` iff an open critical/major finding/defer, failed assurance,
or selected non-passed mode exists; otherwise `full` iff all 22 set decisions
and all 22 decisions in every selected mode are non-null, current,
predecessor-complete passes; otherwise `partial`.

| Exact set status | Structured native / portable exit | result | posture | reason |
|---|---:|---|---|---|
| `partial` | `3 / 3` | `held` | `held` | `review-incomplete` |
| `conflicted` | `4 / 4` | `held` | `held` | `review-conflicted` |
| `failed` | `5 / 5` | `failed` | `non-promotable` | `set-failed` |
| `full` | `0 / 0` | `passed` | `promotable` | `expected-outcome` |

No alternate tuple is valid, including a pass-shaped partial set or a
review-shaped change to counts. `review_completeness` is `partial|full` and is
full iff status is full. Only full supports exit.

The set rollback wrapper is exactly the section 8.4 rollback wrapper. The set
reproduction wrapper uses the same exact `content,digest` order and digest
preimage, but its closed content is exactly
`plan_id,command_identity,exact_argv,input_digests,controls,expected_result`:
plan ID literal `PLAN-WP-TST-001-REPRO-SET`; command identity literal
`CMD-TST-EVIDENCE-SET`; argv is exactly
`["pwsh","-NoLogo","-NoProfile","-NonInteractive","-File","tools/test_gate.ps1","-AssembleSet"]`;
input digests are exactly two objects `name,sha256` in this order,
with names `wp-artifact` and `implementation-manifest` and digests equal to
`wp_artifact_digest` and the non-null equal
`implementation_binding.observed_preflight.expected_manifest_digest` /
`actual_manifest_digest`; controls are
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
field is byte-identical to its immediate predecessor: `schema,wp_id,wp_revision,
wp_artifact_digest,lineage_binding,acceptance_binding,corrective_entry_binding,implementation_binding,
identity_registry,candidate_author_ids,
required_review_lanes,rollback_plan,reproduction_plan,
invalidation_triggers`; the other 21 slots are byte-identical.
The complete and only derived delta is: set version plus 1; regenerated set ID
and path; immediate `predecessor_set`; history append of exactly that binding;
replace all 16 `mode_records` entries with their just-created one-lane mode
successors for that same lane and no other mode successors, then recompute
`aggregate_digest`; place the formula-bound next set decision in the same lane
slot; exactly project findings, defers, dissent, and conflicts from that changed
decision while retaining the other 21 contributions; recompute status and
review completeness; retain observed stdout/stderr and structured counts byte-
identically while remapping only structured exit/result/posture/reason and
recomputing its byte count/hash under the status table; then recompute set
digest. No other byte changes.

Review changes first create the 16 same-lane mode successors and then the
corresponding set one-lane successor; each set decision binds its own prior set
decision and its finding claims bind the selected mode successor digest. No
mode/set/decision preimage includes its own digest, enclosing digest, future
commit, or future pulse. Neither history is mutated, deleted, overwritten,
quarantined, or hidden.

The dedicated R20 negative suite retains every R14 and failed-R15/R16/R17/R18/R19 case with
exact section 1 commit:path, blob, and raw-byte SHA custody, and
additionally rejects every cross-revision relabel, `(schema,wp_revision)`
mismatch, historical-validator substitution, old outer/new nested combination,
new outer/old nested combination, and any nested parse begun before exact
external artifact-kind/schema/revision/candidate-commit/WP-blob dispatch. It also
additionally rejects a root/merge/multi-parent/malformed/substituted acceptance,
entry, or implementation commit object; nonzero cat-file exit; nonempty stderr;
wrong raw stream count/hash; parsed tree/parent not derived from retained bytes;
acceptance parent other than exact candidate, entry parent other than exact
acceptance, or implementation parent other than exact entry; any unscoped
no-extra observation containing `T` or another unsupported/unauthorized status,
any diff-filter on an unscoped no-extra argv, or scoped/unscoped inequality;
failed binding used as authority, dual-bound ID absent from either candidate or
failed projection, or failed-only ID present in candidate projection. It also rejects a reused/preexisting Job
name; missing or changed kill-on-close/Job-memory configuration; a process-
memory-only configuration; absent, zero, relaxed, or substituted
`JobMemoryLimit`; completion-port creation/association after assignment;
inherited port handle; wrong completion key; missing/mismatched notification
set/query; synthesized, stale, process-only, wrong-key, or unconfigured
message; notification proof without immediate successful violation query;
hard-message proof with a fabricated violation query; false `memory-limit`
without one exact documented signal; PID reuse with a
different creation FILETIME, every Create/Open/Query/Assign/Terminate/Wait/
Close failure, PID-only kill, nonzero active count after termination, truncated
native exit, mismatched portable exit/stream hash/event preimage, and every
unnamed termination null/state/reason tuple. It rejects a generated-path
manifest that omits or misclassifies any ledger/evidence/set/review-auth/
receipt/finalization artifact, trusts a prior manifest rather than rereading
bytes, admits a non-current addition postrun, or breaks a schema/digest/
predecessor bond. It rejects review/execution successor confusion, missing/
duplicated/gapped/review-derived execution ordinals, falsely consecutive
execution evidence versions, carried
review slots into a rerun, reset history, copied observations, reused run root,
or changed stable execution input. It injects transient mutations before and
during materialization, during the worker, between every publication stage,
and immediately around finalization; every restored mutation, watch gap/
overflow, worktree-sourced worker byte, blob-object mismatch, unexpected path,
wrong create order, self-digest attempt, missing receipt/finalization, or final
snapshot/watch mismatch is non-promotable.
It also re-resolves each R1–R7 commit:path pair and rejects any absent pulse,
working-tree substitution, path/version alias, wrong historical blob, raw-byte
SHA mismatch, or attempt to treat the current Pulse 17 bytes as an earlier
revision's bytes.

## 9. Entry, stop, exit, and authority

Acceptance of this candidate, if it occurs, authorizes only a later separate
R20 corrective-entry decision. The acceptance pulse binds the R20 artifact
digest, its exact externally selected R20 candidate first parent and complete commit observation,
accepted R14 custody, the retained failed
implementation, failed R15, R16, R17, R18, and R19 amendments, exact governance
observations, and all prior governance inputs, but never its own future commit. After it is committed, the
corrective-entry pulse binds that acceptance commit and pulse digest, but never
its own future commit. After corrective entry is committed, evidence binds the
R20 WP/acceptance/corrective entry as its only positive governance identity,
and binds the original R14 entry solely as complete-delta base plus the failed
R14/R15/R16/R17/R18/R19 custody. A clean isolated worktree starts at the R20 corrective-entry
commit, and the corrective
implementation commit is its sole-parent, non-merge direct child; exact raw
commit observations prove acceptance-to-entry and entry-to-child. Its complete
implementation delta is still measured from the original R14 entry exactly as
section 3 fixes. The exact allowlist, unchanged REV bytes, and no unrelated
change are mandatory. WS remains
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

Disposition: **corrective lineage amendment candidate for independent review
only; not accepted; not entered; retained failed implementation creates no
evidence or authority**.
