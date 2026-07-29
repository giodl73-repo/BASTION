# BASTION Verification Plan

## 1. Status and authority

Status: `proposed; planning fixed-point candidate; no executed evidence`.

Assignment: `ASG-BASTION-VV-PLANNING-001`.

This document defines future verification methods, evidence identities, and
acceptance rules for the review-ready BASTION bootstrap-planning successor. It does not select a
representation, workspace, toolchain, dependency, concrete command, corpus,
parser, generator, deployment, or operational method. It records no run,
result, pass, accepted work package, hold closure, HND emission, Taxlane action,
release, official use, procurement, allocation, rate, or implementation
authority.

Accepted planning fixed points for this document and companion
`VALIDATION.md` may make later acceptance review of `WP-WS-001`, `WP-REV-001`,
and `WP-TST-001` scaffolding plans eligible. They do not accept those WPs or
make any semantic producer WP eligible.

## 2. Controlled baseline

| Controlled artifact | SHA-256 |
|---|---|
| Fixed `MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| Fixed `CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| Fixed `REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| Fixed `SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| Fixed `ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| Fixed `PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| Fixed `INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| Review-ready `CHANGE_CONTROL.md` / prospective `CHG-BA-TST-BOOT-002` | `ed259afd24fc39ea87d8b21c4b5a535a2b369c5558e9dd6e2f25b1e070deaa3c` |
| Fixed-point `DESIGN.md` | `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` |
| Fixed-point `CODE_RIGOR.md` | `3ce31b808f038291b79bf348547ed43029ca1b0a797520fbb72546d3964801d9` |
| Review-ready successor `IMPLEMENTATION_PLAN.md` | `e1ce5cfa603e491d21d7a71859c610316e62fc49caba1c0687c59b8b8baa15c7` |
| Review-ready successor `WORK_PACKAGES.md` | `5fa703f2f93c5748da5f4cb800cbffe0b153747f006d6f116c577b6f90b412e3` |
| Superseding implementation-planning record `pulse-09-implementation-planning.md` | `deded3e452026688a172faf860e5fbd82491bdc8a587b27165bf8aafe927a5d3` |

The controlled verification universe is exactly 298 planning identities: 98
requirements; 121 specifications comprising 98 functional, 10
non-functional, and 13 controlled unknown identities; 14 DESIGN decisions; 13
contracts; 12 package boundaries; and 40 code-rigor constraints. The 20-WP
register and all 13 paired holds are additional exact planning-control sets;
they are not added to the 298 count.

## 3. Exact aliases and method catalog

Ranges are inclusive. These aliases are exact and mechanically expandable.

- `REQ-ALL` is the exact 98-requirement union in section 7.1.
- `SPEC-F-ALL` is the exact 98-functional-specification union in section 7.1.
- `SPEC-NF-ALL` = `{SPEC-NF-001, SPEC-NF-002, SPEC-NF-003, SPEC-NF-004,
  SPEC-NF-005, SPEC-NF-006, SPEC-NF-007, SPEC-NF-008, SPEC-NF-009,
  SPEC-NF-010}`.
- `SPEC-UNK-ALL` is the exact 13-member set in section 7.3.
- `SPEC-ALL` = `SPEC-F-ALL` + `SPEC-NF-ALL` + `SPEC-UNK-ALL`, exactly 121.
- `DES-ALL` and `CONTRACT-ALL` are the exact 14- and 13-member sets in
  section 7.4.
- `PB-ALL` is the exact 12-member package-boundary set in section 7.6.
- `CR-ALL` = `{CR-001, ..., CR-040}` as exactly partitioned in section 7.5.
- `WP-ALL` is the exact 20-member register in section 8.
- `HOLD-ALL` is the exact 13 paired-hold set in section 7.3.

| Method | Planned use | Required future evidence |
|---|---|---|
| `VM-INS-001` | Digest-bound inspection and exact trace transpose | Input/output digests, inspector identity, exact set comparison, orphan/duplicate/contradiction result |
| `VM-CON-001` | Positive, negative, unauthorized-consumer, and failure contract cases | Case identity, contract/version, fixture digest, expected/actual typed posture, retained failure |
| `VM-PROP-001` | Invariants, checked accounting, distributions, floors, and deterministic properties | Generated-case bounds, seed if used, minimized counterexample, invariant decision |
| `VM-MOD-001` | Finite state/DAG and successor model checking | State/transition set, explored bound, terminal states, invalid-edge and cycle results |
| `VM-ADV-001` | Safe adversarial and prohibited-content rejection | Synthetic/public-aggregate fixture, attack class, safe-failure result, retained reason without dangerous detail |
| `VM-REP-001` | Exact and semantic reproduction | Environment manifest, command resolution, inputs, order/seed/time controls, output and evidence digests |
| `VM-RES-001` | Finite resource-bound verification | Accepted bound identity, measured peak/termination record, failure-on-excess result |
| `VM-SUP-001` | Support isolation, generation, documentation, dependency/license/advisory posture | Dependency graph, custody/regeneration trace, no-reverse-edge and truthful-state result |
| `VM-ASSUR-001` | Independent parliament/domain/formal assurance | Reviewer, lane, input digest, disposition, findings, dissent, expiry and supersession state |
| `VM-INT-001` | Final source spine, all contracts/invariants/holds, HND/TERM/REL, and forbidden edges | Final accepted predecessor digests and complete L2 evidence bundle |

Every required method is future work. `planned`, `unavailable`, `absent`,
`conflicted`, `invalid`, and `accepted` are distinct states. A plan is never an
evidence result.

## 4. Evidence tiers and record schema

### 4.1 Exact evidence tiers

| Tier | Meaning | Minimum future contents | Promotion effect now |
|---|---|---|---|
| `EVT-P` | Planning/custody record | Planned method, owner, destination, expected result, command slot, fixture class, bound and assurance need | None; not product evidence |
| `EVT-L0` | Exact changed-surface evidence | Resolved L0 commands, changed boundary, focused positive/negative cases, exact commit and output digests | None until independently accepted |
| `EVT-L1` | Whole-workspace evidence | Resolved L1 commands, full quality/regression/static/supply-chain results and environment digest | None until independently accepted |
| `EVT-L2` | Integration and adversarial evidence | Resolved L2 commands, source spine, contract/property/model/adversarial/hold/HND/TERM/REL evidence | None until independently accepted |
| `EVT-A` | Independent convergence and assurance | Required lane decisions, findings/dissent, conflict closure, exact evidence-bundle digest, zero-major decision | None until stage controller accepts |

Every implementation WP needs `EVT-L0`; its work-package row fixes required
`EVT-L1` and `EVT-L2`. No tier may be inferred from a higher tier, copied from
another digest, or replaced by documentation. `EVT-P` is the only tier created
by this plan.

### 4.2 Evidence record

Each future `EVID-*` record must bind all of:

1. evidence identity, tier, status, producer WP and exact commit;
2. requirement/specification/DES/contract/CR/hold and VCL coverage;
3. resolved command identity, exact tool/version/configuration and environment
   manifest digest;
4. public-aggregate or clearly synthetic fixture identity, source/custody,
   purpose, input digest and supersession state;
5. finite resource bound, deterministic order, seed/clock/locale controls where
   applicable;
6. expected typed posture, actual typed posture, exit state, output/evidence
   digest and retained failure/counterexample;
7. author, independent reviewer, parliament/domain/formal assurance decisions,
   findings, dissent, conflicts and disposition; and
8. predecessor digests, invalidation triggers, rollback/reproduction pointer,
   history and successor identity.

A missing field is `held`, never favorable. Evidence is immutable after
review; correction creates a superseding record and retains the prior record.

## 5. `TBD-TST-001` / `SPEC-UNK-TST-001` destination plan

This section is the exact planning destination permitted by
`CHG-BA-TST-001`. It proposes the method; it does not execute or accept it.
Both paired identities remain open.

### 5.1 Exact planned verification identities

| Identity | Planned question | Required future evidence |
|---|---|---|
| `VER-TST-001` | Are all required evidence tiers present and non-collapsed? | Tier-completeness transpose for every accepted WP and required VCL area |
| `VER-TST-002` | Can evidence be reproduced at its exact digest? | Exact command/environment/input replay and canonical output comparison |
| `VER-TST-003` | Are authorship, test, review, and assurance independent and are conflicts retained? | Role-separation record, conflict register, dissent and independent rerun |
| `VER-TST-004` | Are every observed failure and finding classified and dispositioned without loss? | Finding-completeness transpose, severity/disposition record and retained counterexamples |
| `VER-TST-005` | Do the exact positive, negative, null, degraded, adversarial, lifecycle and boundary fixtures converge? | `FIX-CONV-ALL` result matrix with no missing case or favorable default |
| `VER-TST-006` | Does the exact bundle meet the zero-unresolved-critical/major gate with every required assurance present? | Digest-bound convergence decision and all mandatory lane dispositions |
| `VER-VTR-001` | Does every controlled source, WP, method, evidence item, finding, hold and consumer transpose exactly? | Forward/reverse set equality, zero orphan/duplicate and final digest map |
| `VER-VTR-002` | Are evidence states, history, conflicts, invalidations and supersessions truthful? | Immutable evidence-state ledger and historical reproduction audit |

`VER-TST-001` through `006` and `VER-VTR-001` through `002` are planning
identities, not executed results.

### 5.2 Reproduction rules

- Exact reproduction requires the same accepted commit, dependency/config
  digest, resolved command, environment class, input/fixture digests, order,
  seed, clock/locale controls and resource bounds to produce the same canonical
  typed result and evidence digest.
- Cross-environment reproduction requires the same canonical semantics and
  ordering. Any permitted representation difference must be prospectively
  accepted, normalized by an exact rule, and retained in the record; no
  tolerance is implied here.
- Nondeterminism, missing input, timeout, resource exhaustion, stale fixture,
  unrecorded transformation, hand edit, digest mismatch, or unverifiable
  environment fails closed and creates a finding.
- Reproduction uses only identified public aggregate or clearly marked
  synthetic fixtures that cannot reconstruct a person or expose sensitive
  operational detail.
- At least one independent replay is required for each evidence bundle before
  convergence. A self-replay is useful diagnostic material but not independent
  evidence.

### 5.3 Conflict, severity, and disposition schema

Conflicting evidence is never averaged, majority-voted, or silently replaced.
An `EVID-CONFLICT-*` record binds both evidence digests, methods, environments,
reviewers, affected sources/WPs/holds, highest plausible severity, owner and
resolution trigger. The affected gate stays `held` until an independent rerun
and exact resolution record supersede the conflict.

| Severity | Meaning | Gate effect |
|---|---|---|
| `critical` | Prohibited content, authority escape, unsafe/security breach, person reconstruction, HND/REL output, graph backflow, or evidence falsification | Immediate fail; non-waivable |
| `major` | Required contract/invariant/hold/VCL/assurance missing or contradicted; non-reproducible semantic result; unbounded or cyclic behavior | Fail convergence; correction and independent recheck required |
| `minor` | Bounded defect that does not change protected semantics or required coverage | Remains open with owner and closure evidence; cannot be hidden |
| `editorial` | Presentation defect with no semantic or trace effect | Correct with editorial evidence |
| `observation` | Non-actionable retained note or future hypothesis | No pass credit; retain with rationale |

Exact dispositions are `pass`, `finding`, `defer`, `invalid`, and
`superseded`. Only an independent reviewer may record `pass`; `finding` remains
open; `defer` names a fixed later destination and cannot satisfy a required
gate; `invalid` preserves unusable evidence and reason; `superseded` preserves
history and successor. No author self-passes.

The convergence gate requires zero unresolved `critical` or `major` findings,
no open evidence conflict, complete required tiers and fixtures, exact
forward/reverse equality, all required assurance decisions, and an accepted
bundle digest. Minor/editorial items require explicit disposition and cannot
mask a protected-floor defect. This zero-major rule is planned, not executed.

### 5.4 Exact convergence fixtures

`FIX-CONV-ALL` is exactly:

| Fixture identity | Planned safe case |
|---|---|
| `FIX-CONV-001` | Positive public-aggregate exact-output path |
| `FIX-CONV-002` | Missing/null/reviewed-N/A with no default |
| `FIX-CONV-003` | Malformed, incomplete, duplicate and out-of-range input |
| `FIX-CONV-004` | Stale, version-mismatched, superseded and digest-conflicted input |
| `FIX-CONV-005` | Unauthorized consumer, forbidden dependency and SOURCE/TEST bypass |
| `FIX-CONV-006` | Prohibited/classified/controlled/targeting/operational/person-level marker rejection without retaining dangerous content |
| `FIX-CONV-007` | Unsafe composition, re-identification path and retained-failure minimization |
| `FIX-CONV-008` | Checked arithmetic, overlap, residual, distribution, floor and uncertainty boundary |
| `FIX-CONV-009` | Finite preliminary `ECO[n] -> DEL[n] -> final ECO[n+1] -> ADP[n+1]` plus invalid same-version cycle |
| `FIX-CONV-010` | HND held/no pack, finite non-product TERM receipt, invalid receipt reuse and no backflow |
| `FIX-CONV-011` | REL no-output/no-consumer and false release/official-authority claim rejection |
| `FIX-CONV-012` | Generated/documentation reproduction, stale evidence, rollback and immutable history |

Every fixture is `planned; absent; unexecuted`. Its future concrete content,
bounds and expected values require an accepted WP and independent safe-corpus
review.

### 5.5 Hold status

`TBD-TST-001` and `SPEC-UNK-TST-001` remain open until accepted future
evidence positively and negatively proves all eight `VER-*` identities at an
exact digest and the Independent Test & Oversight Officer plus required
assurance lanes accept the zero-major decision. Acceptance of this plan,
scaffolding, fixtures-as-definitions, or command identities closes neither
hold.

## 6. L0/L1/L2 command registry

No concrete command, executable target, tool, version, workspace or dependency
is selected. Symbolic command identities are future registry keys only.

| Level | Exact symbolic command slots | Required purpose | Current state |
|---|---|---|---|
| `L0` | `CMD-L0-FORMAT`, `CMD-L0-CHECK`, `CMD-L0-FOCUSED-TEST` | Changed-surface format/type/compile and focused positive/negative behavior | `planned; unavailable; unexecuted` |
| `L1` | `CMD-L1-WORKSPACE-CHECK`, `CMD-L1-LINT`, `CMD-L1-TEST`, `CMD-L1-DOC`, `CMD-L1-STATIC`, `CMD-L1-SUPPLY-CHAIN` | Whole-workspace quality, regression, documentation, static, dependency/feature/license/advisory evidence | `planned; unavailable; unexecuted` |
| `L2` | `CMD-L2-SOURCE-SPINE`, `CMD-L2-CONTRACT-MATRIX`, `CMD-L2-PROPERTY`, `CMD-L2-MODEL`, `CMD-L2-ADVERSARIAL`, `CMD-L2-HOLD-CLOSURE`, `CMD-L2-NO-EMISSION` | Final SOURCE/TEST, contracts, properties, finite graph, adversarial, exact holds and HND/TERM/REL evidence | `planned; unavailable; unexecuted` |

`WP-VV-001` product execution is reasoned `N/A`; its planning closeout uses
digest, inventory, transpose, contradiction and independent-review inspection.
Every future command resolution requires an accepted WP revision and must bind
tool/version, scope, inputs, expected result, finite bound and evidence path.

## 7. Exact controlled-source verification allocation

### 7.1 Requirements and functional specifications

Each row allocates an exact requirement family and its same-numbered functional
SPEC family. Rows are disjoint within each class.

| Exact family pair | Count REQ / SPEC-F | Primary methods | Producer / final evidence |
|---|---:|---|---|
| `BASTION-REQ-SCP-001`–`005`, `BASTION-REQ-SCP-009`–`010`, `BASTION-REQ-SRC-001`–`008`; `SPEC-SCP-001`–`005`, `SPEC-SCP-009`–`010`, `SPEC-SRC-001`–`008` | 15 / 15 | `VM-INS-001`, `VM-CON-001`, `VM-ADV-001`, `VM-REP-001` | `WP-CST-001` / `WP-INT-001` |
| `BASTION-REQ-SCP-006`–`008`; `SPEC-SCP-006`–`008` | 3 / 3 | `VM-CON-001`, `VM-ADV-001`, `VM-ASSUR-001` | `WP-AUT-001` / `WP-INT-001` |
| `BASTION-REQ-RDY-001`–`007`; `SPEC-RDY-001`–`007` | 7 / 7 | `VM-CON-001`, `VM-PROP-001`, `VM-MOD-001`, `VM-ASSUR-001` | `WP-RDY-001` / `WP-INT-001` |
| `BASTION-REQ-ACQ-001`–`008`; `SPEC-ACQ-001`–`008` | 8 / 8 | `VM-CON-001`, `VM-PROP-001`, `VM-ADV-001`, `VM-ASSUR-001` | `WP-ACQ-001` / `WP-INT-001` |
| `BASTION-REQ-LOG-001`–`008`; `SPEC-LOG-001`–`008` | 8 / 8 | `VM-CON-001`, `VM-PROP-001`, `VM-MOD-001`, `VM-RES-001` | `WP-LOG-001` / `WP-INT-001` |
| `BASTION-REQ-ALLY-001`–`006`; `SPEC-ALLY-001`–`006` | 6 / 6 | `VM-CON-001`, `VM-PROP-001`, `VM-ADV-001`, `VM-ASSUR-001` | `WP-ALLY-001` / `WP-INT-001` |
| `BASTION-REQ-DST-001`–`005`; `SPEC-DST-001`–`005` | 5 / 5 | `VM-PROP-001`, `VM-ADV-001`, `VM-ASSUR-001` | `WP-DST-001` / `WP-INT-001` |
| `BASTION-REQ-ECO-001`–`011`, `014`–`016`, `018`–`020`; `SPEC-ECO-001`–`011`, `014`–`016`, `018`–`020` | 17 / 17 | `VM-PROP-001`, `VM-MOD-001`, `VM-REP-001`, `VM-ASSUR-001` | `WP-ECO-PRELIM-001` / `WP-INT-001` |
| `BASTION-REQ-ECO-012`–`013`, `017`; `SPEC-ECO-012`–`013`, `017` | 3 / 3 | `VM-MOD-001`, `VM-PROP-001`, `VM-REP-001` | `WP-ADP-001` / `WP-INT-001` |
| `BASTION-REQ-TST-001`–`006`; `SPEC-TST-001`–`006` | 6 / 6 | `VER-TST-001`–`006`, `VM-ASSUR-001` | `WP-REV-001` and producer-owned deltas / `WP-INT-001` |
| `BASTION-REQ-VTR-001`–`003`; `SPEC-VTR-001`–`003` | 3 / 3 | `VER-VTR-001`–`002`, `VM-INS-001`, `VM-REP-001` | `WP-REV-001` / `WP-INT-001` |
| `BASTION-REQ-DEL-001`–`007`; `SPEC-DEL-001`–`007` | 7 / 7 | `VM-CON-001`, `VM-MOD-001`, `VM-PROP-001`, `VM-ASSUR-001` | `WP-DEL-001` / `WP-INT-001` |
| `BASTION-REQ-HND-001`–`007`; `SPEC-HND-001`–`007` | 7 / 7 | `VM-CON-001`, `VM-MOD-001`, `VM-ADV-001`, `VM-INT-001` | `WP-HND-001` / `WP-INT-001` |
| `BASTION-REQ-REL-001`–`003`; `SPEC-REL-001`–`003` | 3 / 3 | `VM-INS-001`, `VM-ADV-001`, `VM-INT-001` | `WP-DOC-001` / `WP-INT-001`; no output |
| **Exact totals** | **98 / 98** | **zero orphan** | **all pending** |

### 7.2 Non-functional specifications

| Identity | Required methods | Principal coverage |
|---|---|---|
| `SPEC-NF-001` | `VM-ADV-001`, `VM-RES-001`, `VM-INT-001` | bounded total behavior and safe failure |
| `SPEC-NF-002` | `VM-ADV-001`, `VM-ASSUR-001`, `VM-INT-001` | authority/security separation |
| `SPEC-NF-003` | `VM-PROP-001`, `VM-MOD-001`, `VM-INT-001` | readiness/floor/path behavior |
| `SPEC-NF-004` | `VM-PROP-001`, `VM-ADV-001`, `VM-INT-001` | lifecycle/tails/distributions |
| `SPEC-NF-005` | `VM-PROP-001`, `VM-ASSUR-001`, `VM-INT-001` | incidence, uncertainty and non-compensation |
| `SPEC-NF-006` | `VM-PROP-001`, `VM-MOD-001`, `VM-INT-001` | checked accounting and successor behavior |
| `SPEC-NF-007` | `VM-REP-001`, `VM-RES-001`, `VM-INT-001` | determinism and finite bounds |
| `SPEC-NF-008` | `VM-SUP-001`, `VM-REP-001`, `VM-INT-001` | generation/support custody |
| `SPEC-NF-009` | `VM-INS-001`, `VM-ADV-001`, `VM-INT-001` | immutable history, rejection and no emission |
| `SPEC-NF-010` | `VM-INS-001`, `VM-SUP-001`, `VM-ASSUR-001` | trace truth and review state |

### 7.3 Controlled unknowns and holds

Every row is `planned; hold open`. Verification may prove a fail-closed or
no-output posture without supplying the missing favorable meaning.

| Exact held pair | Planned verification | Current result |
|---|---|---|
| `SPEC-UNK-SEC-001` / `TBD-SEC-001` | prohibited/composition rejection, minimization and retained safe reason | absent; open |
| `SPEC-UNK-RDY-001` / `TBD-RDY-001` | readiness/safety/floor positive and negative cases | absent; open |
| `SPEC-UNK-SRC-001` / `TBD-SRC-001` | SOURCE admission, transformation and exact-output cases | absent; open |
| `SPEC-UNK-QNT-001` / `TBD-QNT-001` | checked units, overlap, uncertainty, residual and bounds | absent; open |
| `SPEC-UNK-ACQ-001` / `TBD-ACQ-001` | acquisition/capacity/concentration/commonality facets | absent; open |
| `SPEC-UNK-LOG-001` / `TBD-LOG-001` | custody/condition/tail/degraded recovery cases | absent; open |
| `SPEC-UNK-ALLY-001` / `TBD-ALLY-001` | sovereignty/compatibility/separated burden cases | absent; open |
| `SPEC-UNK-DST-001` / `TBD-DST-001` | all seven stakeholder lenses and distribution/tail cases | absent; open |
| `SPEC-UNK-ECO-001` / `TBD-ECO-001` | six pathways, accounting and predecessor-linked envelope | absent; open |
| `SPEC-UNK-TST-001` / `TBD-TST-001` | all eight section 5 `VER-*` identities | absent; open |
| `SPEC-UNK-DEL-001` / `TBD-DEL-001` | owner/capacity/milestone/stop/rollback/realization | absent; open |
| `SPEC-UNK-HND-001` / `TBD-HND-001` | current held/no-emission and finite TERM proof only | absent; open; future emission blocked |
| `SPEC-UNK-REL-001` / `TBD-REL-001` | REL no-output/no-consumer and false-release rejection | absent; open; future release blocked |

### 7.4 DESIGN decisions and contracts

| Domain | Exact DESIGN / contract set | Required verification |
|---|---|---|
| Source | `DES-SOURCE-001`; `CONTRACT-SOURCE-001` | `VM-INS-001`, `VM-CON-001`, `VM-ADV-001`, `VM-REP-001` |
| Authority | `DES-AUTH-001`; `CONTRACT-AUTH-001` | `VM-CON-001`, `VM-ADV-001`, `VM-ASSUR-001` |
| Readiness | `DES-RDY-001`; `CONTRACT-RDY-001` | `VM-CON-001`, `VM-PROP-001`, `VM-ASSUR-001` |
| Acquisition | `DES-ACQ-001`; `CONTRACT-ACQ-001` | `VM-CON-001`, `VM-PROP-001`, `VM-ADV-001` |
| Logistics | `DES-LOG-001`; `CONTRACT-LOG-001` | `VM-CON-001`, `VM-MOD-001`, `VM-RES-001` |
| Alliance | `DES-ALLY-001`; `CONTRACT-ALLY-001` | `VM-CON-001`, `VM-PROP-001`, `VM-ASSUR-001` |
| Distribution | `DES-DST-001`; `CONTRACT-DST-001` | `VM-PROP-001`, `VM-ADV-001`, `VM-ASSUR-001` |
| Economics | `DES-ECO-001`; `CONTRACT-ECO-001` | `VM-PROP-001`, `VM-MOD-001`, `VM-REP-001` |
| Adaptive successor | `DES-ADP-001`; `CONTRACT-ECO-001` shared, not a new contract | `VM-MOD-001`, `VM-PROP-001`, `VM-REP-001` |
| Delivery | `DES-DEL-001`; `CONTRACT-DEL-001` | `VM-CON-001`, `VM-MOD-001`, `VM-ASSUR-001` |
| Test | `DES-TEST-001`; `CONTRACT-TEST-001` | `VER-TST-001`–`006`, `VM-ASSUR-001` |
| Trace | `DES-TRACE-001`; `CONTRACT-TRACE-001` | `VER-VTR-001`–`002`, `VM-INS-001` |
| Handoff | `DES-HND-001`; `CONTRACT-HND-001` | `VM-CON-001`, `VM-MOD-001`, `VM-ADV-001`, `VM-INT-001` |
| Release | `DES-REL-001`; `CONTRACT-REL-001` | `VM-INS-001`, `VM-ADV-001`, `VM-INT-001`; no output |

The table covers exactly 14 unique DESIGN decisions and 13 unique contracts;
`CONTRACT-ECO-001` is deliberately shared and counted once.

### 7.5 Code-rigor constraints

The rows are an exact disjoint partition of `CR-001` through `CR-040`.

| Exact CR set | Required methods |
|---|---|
| `CR-001` | `VM-INS-001`, `VM-SUP-001` |
| `CR-002` | `VM-CON-001`, `VM-PROP-001` |
| `CR-003`–`CR-006` | `VM-CON-001`, `VM-ADV-001`, `VM-INT-001` |
| `CR-007` | `VM-INS-001`, `VM-MOD-001`, `VM-SUP-001` |
| `CR-008`, `CR-009` | `VM-PROP-001`, `VM-REP-001` |
| `CR-010` | `VM-CON-001`, `VM-INS-001`, `VM-INT-001` |
| `CR-011`–`CR-013` | `VM-MOD-001`, `VM-RES-001`, `VM-SUP-001` |
| `CR-014` | `VM-INS-001`, `VM-ASSUR-001` |
| `CR-015` | `VM-ADV-001`, `VM-ASSUR-001` |
| `CR-016` | `VM-ADV-001`, `VM-INS-001` |
| `CR-017` | `VM-PROP-001`, `VM-ASSUR-001` |
| `CR-018` | `VM-PROP-001`, `VM-INT-001` |
| `CR-019` | `VM-REP-001`, `VM-INS-001` |
| `CR-020`, `CR-021` | `VM-PROP-001`, `VM-MOD-001` |
| `CR-022` | `VM-MOD-001`, `VM-INT-001` |
| `CR-023` | `VER-TST-001`–`006`, `VER-VTR-001`–`002` |
| `CR-024` | `VM-ADV-001`, `VM-INT-001` |
| `CR-025`–`CR-030` | `VM-CON-001`, `VM-PROP-001`, `VM-ADV-001` |
| `CR-031` | `VM-ADV-001`; parser/fuzz branch only if separately authorized |
| `CR-032` | `VM-RES-001`, `VM-REP-001`, `VM-SUP-001` |
| `CR-033` | `VM-INS-001`, `VM-ASSUR-001` |
| `CR-034` | `VM-SUP-001`, `VM-REP-001` |
| `CR-035` | `VM-INS-001`, `VM-CON-001` |
| `CR-036` | `VM-INS-001`, `VM-SUP-001`, `VM-INT-001` |
| `CR-037` | `VM-CON-001`, `VM-PROP-001`, `VM-MOD-001` |
| `CR-038` | `VM-INS-001`, `VER-VTR-001`–`002` |
| `CR-039` | `VM-ASSUR-001`, `VER-VTR-001`–`002` |
| `CR-040` | all applicable methods and final `VM-INT-001` |

### 7.6 Package boundaries

| Exact package boundary | Primary verification suite | Required methods / evidence owner |
|---|---|---|
| `PB-WS-001` | `VS-PB-001` | `VM-INS-001`, `VM-SUP-001`; `WP-WS-001` then `WP-INT-001` |
| `PB-CST-001` | `VS-PB-001` | `VM-CON-001`, `VM-ADV-001`, `VM-REP-001`; `WP-CST-001` then `WP-INT-001` |
| `PB-AUT-001` | `VS-PB-001` | `VM-CON-001`, `VM-ADV-001`, `VM-ASSUR-001`; `WP-AUT-001` then `WP-INT-001` |
| `PB-DOM-001` | `VS-PB-001` | `VM-CON-001`, `VM-PROP-001`, `VM-ASSUR-001`; domain WPs then `WP-INT-001` |
| `PB-PTH-001` | `VS-PB-001` | `VM-PROP-001`, `VM-MOD-001`, `VM-ASSUR-001`; pathway WPs then `WP-INT-001` |
| `PB-REV-001` | `VS-PB-001` | `VM-INS-001`, `VM-REP-001`, `VM-ASSUR-001`; `WP-REV-001` then `WP-INT-001` |
| `PB-HND-001` | `VS-PB-001` | `VM-CON-001`, `VM-ADV-001`, `VM-INT-001`; `WP-HND-001` then `WP-INT-001` |
| `PB-RUN-001` | `VS-PB-001` | `VM-MOD-001`, `VM-SUP-001`, `VM-INT-001`; `WP-RUN-001` then `WP-INT-001` |
| `PB-DOC-001` | `VS-PB-001` | `VM-INS-001`, `VM-SUP-001`; `WP-DOC-001` then `WP-INT-001` |
| `PB-TST-001` | `VS-PB-001` | `VM-CON-001`, `VM-ADV-001`, `VM-ASSUR-001`; `WP-TST-001` then `WP-INT-001` |
| `PB-FIX-001` | `VS-PB-001` | `VM-INS-001`, `VM-ADV-001`, `VM-REP-001`; `WP-TST-001` then `WP-INT-001` |
| `PB-GEN-001` | `VS-PB-001` | `VM-SUP-001`, `VM-REP-001`; `WP-GEN-001` then `WP-INT-001` |

These 12 rows are exact and disjoint. Each boundary also inherits its exact
touch pairs and dependency-direction checks from the fixed implementation
plan; a boundary row does not create package membership or an implementation.

### 7.7 Exact controlled-source forward and reverse transpose

The forward allocation is:

| Exact source set | Count | Exact primary suite |
|---|---:|---|
| `REQ-ALL` | 98 | `VS-REQ-001` |
| `SPEC-F-ALL` | 98 | `VS-SPEC-F-001` |
| `SPEC-NF-ALL` | 10 | `VS-SPEC-NF-001` |
| `SPEC-UNK-ALL` | 13 | `VS-SPEC-UNK-001` |
| `DES-ALL` | 14 | `VS-DES-001` |
| `CONTRACT-ALL` | 13 | `VS-CONTRACT-001` |
| `PB-ALL` | 12 | `VS-PB-001` |
| `CR-ALL` | 40 | `VS-CR-001` |
| **Exact total** | **298** | **298 source-to-suite pairs; zero orphan** |

The independent reverse allocation is:

| Exact primary suite | Exact source set | Count |
|---|---|---:|
| `VS-REQ-001` | `REQ-ALL` | 98 |
| `VS-SPEC-F-001` | `SPEC-F-ALL` | 98 |
| `VS-SPEC-NF-001` | `SPEC-NF-ALL` | 10 |
| `VS-SPEC-UNK-001` | `SPEC-UNK-ALL` | 13 |
| `VS-DES-001` | `DES-ALL` | 14 |
| `VS-CONTRACT-001` | `CONTRACT-ALL` | 13 |
| `VS-PB-001` | `PB-ALL` | 12 |
| `VS-CR-001` | `CR-ALL` | 40 |
| **Exact total** | **298** | **298 suite-to-source pairs; zero orphan** |

The two tables are authoritative exact transposes. Cross-suite strengthening
does not change the one primary suite allocated to each controlled identity.

## 8. Exact 20-WP verification assignment

Every row covers `VCL-ALL`, exact source allocations in section 7, the eight
`PAR-ALL` lanes, fixed domain owners, and both independent formal assurance
gates. `L0/L1/L2` means required future evidence; `N/A-P` is planning-only.

| WP | Planned levels | Primary verification / evidence destination | Current gate |
|---|---|---|---|
| `WP-VV-001` | `N/A-P` | 298-source and all-set transpose, contradiction, registry and independent plan review | discovery only |
| `WP-WS-001` | L0, L1; reasoned L2 fixed-edge proof | empty membership, policy, no-semantics and rollback evidence | unaccepted; blocked |
| `WP-REV-001` | L0, L1, bootstrap L2 | evidence-state, independence, conflict, dissent and immutable-history scaffold only | unaccepted; blocked |
| `WP-TST-001` | L0, L1, bootstrap L2 | safe harness isolation, fixture custody and command registry scaffold only | unaccepted; blocked |
| `WP-CST-001` | L0, L1, L2 | SOURCE/security/custody/re-admission plus producer sidecars | unaccepted; blocked |
| `WP-AUT-001` | L0, L1, L2 | civilian mission/authority and SOURCE re-admission plus sidecars | unaccepted; blocked |
| `WP-ACQ-001` | L0, L1, L2 | acquisition/capacity/concentration/commonality plus sidecars | unaccepted; blocked |
| `WP-LOG-001` | L0, L1, L2 | custody/lifecycle/tails/degraded recovery plus sidecars | unaccepted; blocked |
| `WP-ALLY-001` | L0, L1, L2 | sovereignty/compatibility/partner/separated burden plus sidecars | unaccepted; blocked |
| `WP-RDY-001` | L0, L1, L2 | readiness/safety/resilience/surge/recovery floors plus sidecars | unaccepted; blocked |
| `WP-DST-001` | L0, L1, L2 | seven lenses/distributions/burdens/tails plus sidecars | unaccepted; blocked |
| `WP-ECO-PRELIM-001` | L0, L1, L2 | preliminary six-pathway checked envelope plus sidecars | unaccepted; blocked |
| `WP-DEL-001` | L0, L1, L2 | delivery/stop/rollback/realization plus sidecars | unaccepted; blocked |
| `WP-ECO-FINAL-001` | L0, L1, L2 | predecessor-linked final envelope plus sidecars | unaccepted; blocked |
| `WP-ADP-001` | L0, L1, L2 | immutable successor/lifecycle/later-feedback restart plus sidecars | unaccepted; blocked |
| `WP-HND-001` | L0, L1, mandatory L2 | held/no pack, finite TERM, no backflow/Taxlane/release authority plus sidecars | unaccepted; blocked; open holds are proof inputs only |
| `WP-RUN-001` | L0, L1, L2 | fixed ordering, typed failure and no semantic ownership plus sidecars | unaccepted; blocked |
| `WP-GEN-001` | L0, L1, L2 | deterministic regeneration, safe fixture custody and no reverse edge | unaccepted; blocked |
| `WP-DOC-001` | L0/L1 documentation slots, L2 trace audit | exact trace/evidence truth and REL no output | unaccepted; blocked |
| `WP-INT-001` | L0, L1, all L2 mandatory | final accepted digests; 13 contracts, 25 invariants, 18 transitions, 13 holds, source spine and forbidden edges | unaccepted; blocked |

### 8.1 Acyclic scaffold acceptance and entry exception

Under prospective `CHG-BA-TST-BOOT-002`, the only pre-producer path is exact
and acyclic. The change and this plan are still candidates, so no step is
currently eligible:

1. Both V&V plans first receive independent fixed-point decisions. That event
   accepts neither a WP nor evidence.
2. A separate acceptance decision may then accept `WP-WS-001`; only after that
   decision may the empty no-semantics workspace scaffold enter and execute its
   own accepted checks.
3. Only after accepted WS exit evidence exists may separate decisions accept
   `WP-REV-001` and `WP-TST-001` for pure bootstrap entry. For those bootstrap
   decisions only, `TBD-TST-001` / `SPEC-UNK-TST-001` is an exact proof input:
   the scaffolds must demonstrate independent review/harness isolation,
   evidence-state custody, safe fixture custody, conflicts, dissent, and
   fail-closed absence without presupposing the missing test result.
4. Entry, execution, and exit remain distinct. Accepting the plans, WS, REV,
   TST, a fixture definition, or a command binding does not close the TST held
   pair or create executed evidence. Closure requires executed independent
   bootstrap fixtures, digest-bound evidence for all exact section 5
   identities, required assurance, and an accepted zero-major decision.
5. No semantic producer may be accepted or enter until that closure and all
   of its other exact blocker conditions exist. Bootstrap evidence can prove
   only the scaffolds; it can never pass, validate, or supply semantics for a
   producer.

## 9. Universal and finite-graph verification

- Every producer generation must prove `freeze -> SOURCE exact-output
  decision -> independent TEST decision -> exact consumer or terminal
  hold/reject`. A transformation, AUTH output, generated derivative or
  re-admission follows the same gate.
- Preliminary `ECO[n]` precedes `DEL[n]`; only reviewed `DEL[n]` may contribute
  to final `ECO[n+1]`; only final `ECO[n+1]` may create `ADP[n+1]`. Later
  feedback starts a new generation. Same-version mutation, retry, recursive
  review and back edges fail.
- HND's current result is held/no pack. TERM is finite, minimal, non-product and
  one-way; a bundle change invalidates its receipt. Receipt reuse, product
  content, Taxlane mutation, recursion or return edge fails.
- REL has no output and no consumer. A release artifact, admission, approval,
  official-use implication or hidden output is a critical finding.
- Support boundaries cannot supply semantics or reverse dependencies. GEN and
  DOC precede final INT, whose evidence must bind their final digests.

## 10. Prohibited-content adversarial plan

Future adversarial cases use safe markers and metadata, never the dangerous
content itself. Required rejection classes are: classified or controlled
information markers; targeting or operational-planning requests; exploitable
vulnerability detail; person-level, medical, personnel or service data;
re-identification by aggregate composition; unauthorized source/transformation;
civilian-authority inflation; safety/readiness floor suppression; burden or
distribution erasure; false accounting, savings or realization; HND pack,
Taxlane action, TERM backflow, REL output, release or official-use claims.

Each case must fail closed, retain a minimized non-sensitive reason, emit no
product, and prove that logs/evidence do not preserve prohibited payloads.
No offensive technique, target, unit, location, vulnerability or operational
procedure may appear in a fixture or result.

## 11. Decision and reopen rules

A future verification result may pass only at an accepted implementation
digest when all required tiers, exact source/WP/VCL transposes, commands,
fixtures, bounds, contracts, properties, models, adversarial cases, assurance
lanes and reproduction records are present with zero unresolved critical or
major findings. `pass_with_risk` cannot waive security, civilian control,
law/safety/readiness, distribution, authority, semantics, graph, hold,
evidence, HND/TERM or release protections.

Any changed controlled identity, plan, method, command resolution, fixture,
bound, reviewer, assurance decision, predecessor, implementation or evidence
digest invalidates affected results and requires a retained successor and
independent recheck.

## 12. Planning disposition

This verification plan is review-ready as a governance-only fixed-point
candidate. All method and command slots are planned and unexecuted. All 13
holds remain open; `TBD-TST-001` and `SPEC-UNK-TST-001` are not closed. No WP
is accepted. Implementation readiness remains blocked, and no code, HND
emission, Taxlane action, release, official use or public action is authorized.
