# BASTION Code Rigor Baseline

## 1. Status and controlled inputs

Repo: BASTION

Assignment: `ASG-BASTION-CODE-RIGOR-001`

State: **review-ready high-assurance pre-code baseline; not a fixed point**.

Planned implementation language: Rust for the fixed product boundaries. No
workspace, crate, source, manifest, toolchain, command, dependency, feature,
schema, parser, generator, runtime, storage, transport, deployment, fixture,
work package, quantitative method, or resource value is selected or created.

| Controlled input | SHA-256 |
|---|---|
| `docs/vtrace/MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| `docs/vtrace/CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| `docs/vtrace/REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| `docs/vtrace/SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| `docs/vtrace/ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| `docs/vtrace/PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| `docs/vtrace/INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| Accepted `docs/vtrace/CHANGE_CONTROL.md` | `147b67601e8e992332f47432a188d2c62d9b2e8f757d30ca4141dc94421c668b` |
| Fixed-point `docs/vtrace/DESIGN.md` | `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` |
| DESIGN fixed-point record `pulse-07-design.md` | `87bb645269d315534a321c19092f1ede757a7a432fd3544c1ca074921ce56a2d` |

Any content change invalidates review of this artifact. This document constrains
later work; it authorizes none.

## 2. Scope and exact aliases

The following aliases are exact and add no member:

- `DES-ALL` = `{DES-SOURCE-001, DES-AUTH-001, DES-RDY-001, DES-ACQ-001,
  DES-LOG-001, DES-ALLY-001, DES-DST-001, DES-ECO-001, DES-ADP-001,
  DES-TEST-001, DES-TRACE-001, DES-DEL-001, DES-HND-001, DES-REL-001}`.
- `DES-ACTIVE` = `DES-ALL` minus `DES-REL-001`, exactly 13 decisions.
- `PB-PRODUCT` = `{PB-CST-001, PB-AUT-001, PB-DOM-001, PB-PTH-001,
  PB-REV-001, PB-HND-001, PB-RUN-001}`.
- `PB-SUPPORT` = `{PB-WS-001, PB-DOC-001, PB-TST-001, PB-FIX-001,
  PB-GEN-001}`.
- `PB-ALL` = `PB-PRODUCT` union `PB-SUPPORT`, exactly 12 boundaries.
- `CONTRACT-ALL` = `{CONTRACT-SOURCE-001, CONTRACT-AUTH-001,
  CONTRACT-RDY-001, CONTRACT-ACQ-001, CONTRACT-LOG-001,
  CONTRACT-ALLY-001, CONTRACT-DST-001, CONTRACT-ECO-001,
  CONTRACT-TEST-001, CONTRACT-DEL-001, CONTRACT-HND-001,
  CONTRACT-REL-001, CONTRACT-TRACE-001}`.
- `CONTRACT-ACTIVE` = `CONTRACT-ALL` minus `CONTRACT-REL-001`, exactly 12
  contracts. `IF-TERM-001` remains a branch of `CONTRACT-TEST-001`, never a
  fourteenth contract.
- `HOLD-ALL` = `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001,
  TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001,
  TBD-TST-001, TBD-DEL-001, TBD-HND-001, TBD-REL-001}`.
- `INV-ALL` = `{INV-01, INV-02, INV-03, INV-04, INV-05, INV-06, INV-07,
  INV-08, INV-09, INV-10, INV-11, INV-12, INV-13, INV-14, INV-15, INV-16,
  INV-17, INV-18, INV-19, INV-20, INV-21, INV-22, INV-23, INV-24, INV-25}`,
  the local references to DESIGN section 8 invariants in their fixed order.

Numeric ranges are inclusive. `N/A` means no allocation. Public, aggregate,
unclassified, non-operational evidence is the maximum content boundary.
Classified, controlled, targeting, operational-planning, exploitable
vulnerability, or person-level service content is prohibited. Taxlane retains
exclusive external admission. `REL` remains no-output.

## 3. Normative coding constraints

| CR | Mandatory constraint | Fail-closed consequence | Deferred item |
|---|---|---|---|
| `CR-001` | No code may exist before an accepted exact work package and stage authority. | Reject implementation or implementation-shaped evidence. | Work package and code remain absent. |
| `CR-002` | Each hand-authored function or method has one semantic responsibility and at most 60 logical lines. | Block review until decomposed or an eligible process waiver is accepted. | Counting method remains planned. |
| `CR-003` | Every branch is total, traceable to one fixed rule, and has one explicit success or typed failure. | Reject fallthrough, mixed ownership, or unenumerated paths. | Representation remains deferred. |
| `CR-004` | Every iteration is bounded by an accepted input/resource bound and proves progress. | Reject unbounded or non-progressing iteration. | Bound values remain open. |
| `CR-005` | Recursion is prohibited unless a later independent termination, depth, and resource proof is accepted. | Reject recursive execution without the exact proof. | No recursion is selected. |
| `CR-006` | Product behavior has no hidden panic, abort, unchecked unwrap/expect, placeholder, or unproved-unreachable path. | Convert every reachable failure into an explicit safe typed result. | No code exists. |
| `CR-007` | Unsafe, foreign, unchecked, or ambient privileged surfaces are forbidden unless separately authorized and proven. | Reject the surface and any dependent promotion. | No such surface is selected. |
| `CR-008` | Wildcard/default semantic conversion may not map missing, unknown, stale, held, rejected, or prohibited content favorably. | Fail unknowns closed and preserve the exact blocker. | Encoding remains absent. |
| `CR-009` | Artifact, admission, review, compatibility, lifecycle, and supersession meanings remain distinct typed families. | Reject cross-family coercion and unknown transitions. | Type representation remains deferred. |
| `CR-010` | Every promotable output, including SOURCE transformations and AUTH, is frozen and SOURCE-admitted for its exact bond before TEST and consumption. | Stop on missing, unsafe, stale, mismatched, or changed bonds. | Admission method remains held. |
| `CR-011` | Semantic identity, digest inputs, ordering, comparison, derivation, and predecessor bonds are deterministic and reproducible. | Reject incidental-order or ambient-state dependence. | Canonical encoding remains deferred. |
| `CR-012` | Ambient time, randomness, locale, environment, concurrency, and scheduling cannot alter semantics; concurrency requires sequential equivalence. | Reject nondeterministic or schedule-dependent results. | Runtime model remains deferred. |
| `CR-013` | Accepted artifacts are immutable; correction creates a distinct successor and strictly increasing economic/generation order. | Reject in-place mutation, same-version feedback, retry, or cycle. | Identity representation remains deferred. |
| `CR-014` | Only fixed boundary and contract edges exist; co-location creates no semantic edge and support boundaries never become product truth. | Reject an invented consumer, reverse edge, or authority transfer. | Dependency representation remains deferred. |
| `CR-015` | Only complete public-aggregate, unclassified, non-operational or marked synthetic custody may pass. | Reject prohibited, unowned, incomplete, or operationally revealing content. | Corpus and aggregation methods remain held. |
| `CR-016` | Security composition is evaluated on every exact output/context; failure custody is minimum and non-reconstructive. | Hold or reject unsafe composition without retaining an exploitation recipe. | Security method remains held. |
| `CR-017` | Civilian authority and law, safety/readiness/resilience, surge/recovery, alliance, and assurance gates are conjunctive. | Reject unsafe or unauthorized promotion; no benefit compensates. | Applicable methods remain held. |
| `CR-018` | RDY, ACQ, LOG, ALLY, DST, stakeholder lenses, facets, distributions, tails, and dissent remain separately owned and inspectable. | Reject composite replacement, silent conversion, or erased burden. | Domain methods remain held. |
| `CR-019` | Missing, result, null, held, rejected, reviewed N/A, pass, finding, defer, incompatible, stale, lifecycle, and supersession meanings never collapse. | Reject false zero/pass/N/A/supersession mappings. | Encodings remain deferred. |
| `CR-020` | Arithmetic is checked; units, price bases, horizons, fiscal fields, parties, ledgers, distributions, uncertainty, residuals, and overlap reconcile explicitly. | Reject overflow, silent conversion, false netting, or unreconciled totals. | Accounting method remains held. |
| `CR-021` | Six economic pathways, gross-to-net status, non-cash outcomes, peer limitations, lifecycle and transition costs, ownership, and burdens remain non-additive until accepted reconciliation. | Reject gross opportunity as savings, peer conversion, or burden shifting. | Quantitative/economic methods remain held. |
| `CR-022` | Preliminary `ECO[n]` requires reviewed `DEL[n]`, then final `ECO[n+1]`, then `ADP[n+1]`; later material feedback starts a new sequence. | Reject skipped delivery, paper realization, same-version edge, or in-place finalization. | Cadence and realization methods remain held. |
| `CR-023` | Independent TEST binds the exact frozen digest/context, checks conflict, preserves findings/dissent, and never edits or self-approves. | Hold absent/conflicted/evidence-free review and return findings only to the accountable producer. | TEST method remains held. |
| `CR-024` | HND emits no pack while held; `IF-TERM-001` is minimal, non-product, one-way, and finite; `REL` emits nothing; Taxlane remains external. | Reject receipt recursion/backflow, product-bearing receipt, release, or inferred external state. | HND, TEST, and release holds remain open. |
| `CR-025` | All 13 exact hold pairs propagate through the fixed direct/transitive graph with no closure by default or waiver. | Hold every affected branch. | All 13 pairs remain open. |
| `CR-026` | Each of 25 invariants has a traced effective proof mechanism at every applicable boundary. | Block acceptance for an uncovered or assumed invariant. | Proof evidence remains planned. |
| `CR-027` | Property evidence covers determinism, admission, separation, conservation, holds, successors, floors, no-release, and terminal finiteness. | Block on a missing property class. | Tests remain absent. |
| `CR-028` | State/model evidence covers all 18 fixed transitions, invalid transitions, stale/supersession separation, ECO/DEL/ADP order, and receipt invalidation. | Block on an untested transition or cycle. | Tests remain absent. |
| `CR-029` | Adversarial evidence covers all role lenses, prohibited content, dangerous composition, authority inflation, gate bypass, false savings, burden erasure, and receipt abuse. | Block on an absent adversarial class. | Fixtures remain absent. |
| `CR-030` | Every one of 13 contracts has positive, null/N/A where permitted, missing, stale, mismatch, unauthorized-consumer, and hold cases. | Block a contract lacking exact negative coverage. | Contract tests remain absent. |
| `CR-031` | Any later untrusted parser/decoder boundary requires bounded input, total errors, malformed/oversize/truncation cases, safe corpora, and fuzzing if authorized. | Reject the parser without clean bounded evidence. | No parser or fuzz mechanism is selected. |
| `CR-032` | Regression evidence binds golden semantics, predecessor history, contract identity, and unchanged-bundle behavior rather than incidental bytes. | Reject silent golden updates or history loss. | Golden artifacts remain absent. |
| `CR-033` | Product, test, documentation, fixture, and generated modes prove the fixed dependency direction and isolation. | Reject support-to-product or product-to-generated authority inversion. | Build configuration remains absent. |
| `CR-034` | Generated material is reproducible, provenance-bound, immutable, non-authoritative, and never hand edited. | Reject untraceable output or reverse dependency. | No generator exists. |
| `CR-035` | Later compiler, formatting, lint, test, documentation, and static checks must be clean and digest-bound. | Block until clean or lawfully resolved without touching a non-waivable class. | Tools and configuration remain absent. |
| `CR-036` | Direct/transitive dependencies, features, native/build surfaces, licenses, advisories, maintenance, and reproducibility require review. | Reject an unreviewed or unacceptable dependency surface. | No dependency set exists. |
| `CR-037` | Time, memory, input, output, iteration, cardinality, and degradation bounds require accepted values and boundary/exhaustion evidence. | Hold implementation until values and failure behavior are accepted. | All resource values remain open. |
| `CR-038` | Every proposed waiver is exact, expiring, independently reviewed, evidence-bonded, and barred from non-waivable content. | Reject incomplete, stale, or prohibited waivers. | Zero waivers accepted. |
| `CR-039` | Evidence state is typed as planned, implemented, executed, reviewed, accepted, rejected, expired, or superseded and bound to exact digests. | Reject planned-as-executed or stale evidence claims. | All evidence is planned. |
| `CR-040` | Code, contracts, trace, documentation, evidence, holds, and authority statements remain mechanically consistent. | Block any contradiction, orphan, widened consumer, or false stage claim. | Implementation remains absent. |

## 4. Exact state and finite graph obligations

Every producer generation evaluates once in order: freeze exact identity and
bond; SOURCE custody/security admission; independent TEST; then only the fixed
authorized consumer. The first failure terminates that branch with its exact
typed posture. No later favorable gate compensates.

The fixed outcome families are:

- artifact: `result`, `null`, `held`, `rejected`, permitted reasoned reviewed
  `not_applicable`;
- admission: `pass`, `hold`, `reject`, `stale`, permitted reviewed
  `not_applicable`;
- review: `pass`, `finding`, `defer`, blocking `hold`;
- compatibility: `compatible`, `held`, `incompatible`, `stale`;
- lifecycle, only for adaptive or trigger-driven records: `preserve`, `revise`,
  `hold`, `retire`, `replace`; and
- supersession: an immutable predecessor/successor relationship, never a state
  in the families above.

The same-generation control graph is exactly:

```text
formed -> frozen -> SOURCE exact-output decision -> TEST exact-bond decision
       -> fixed semantic consumer or terminal hold/reject

preliminary ECO[n] -> SOURCE -> TEST -> DEL[n] -> SOURCE -> TEST
 -> final ECO[n+1] -> SOURCE -> TEST -> ADP[n+1] -> SOURCE -> TEST
 -> fixed consumer

frozen HND -> SOURCE -> ordinary TEST -> IF-TERM-001 minimal receipt
 -> external handoff gate -> Taxlane boundary

REL -> no output
```

Every within-generation edge is one-way. A finding or material change creates a
new immutable generation; later economic feedback creates only preliminary
`ECO[m]` where `m > n+1`. There is no `DEL[n] -> ECO[n]`, receipt-to-HND,
receipt-to-SOURCE, receipt-to-TEST, receipt-to-REL, release, generated-to-product,
same-version, retry, revisit, or backflow edge. A changed HND bundle invalidates
the receipt and starts a new generation.

## 5. Exact waiver classes

| Class | Non-waivable content |
|---|---|
| `NW-SEC` | Unclassified/public-aggregate boundary, prohibited content, composition security, minimization, and non-reconstructive failure custody. |
| `NW-CIV` | Civilian control, lawful authority, mission bounds, and prohibition on operational/force/procurement decisions. |
| `NW-SAFE` | Personnel safety, readiness, resilience, surge, recovery, alliance obligations, and every hard floor. |
| `NW-DST` | Stakeholder lenses, distributions, tails, concentrated effects, burdens, rights, and non-compensation. |
| `NW-AUTH` | Stage, implementation, evidence, official-use, budget, allocation, rate, Taxlane, and external-action authority. |
| `NW-SEM` | DES/contract meaning, typed posture families, null/N/A/missing distinctions, accounting separation, and exact consumers. |
| `NW-GRAPH` | SOURCE-before-TEST order, fixed boundary direction, immutable successors, ECO/DEL/ADP order, support isolation, and no backflow/cycle. |
| `NW-HOLD` | Any member of `HOLD-ALL`, paired `SPEC-UNK-*`, acceptance condition, or exact direct/transitive propagation. |
| `NW-EVID` | Evidence-state truth, digest binding, review independence, finding/dissent retention, and no planned-as-executed claim. |
| `NW-HND` | HND no-emission, unchanged-bundle rule, minimal terminal receipt, terminal finiteness, and external Taxlane exclusivity. |
| `NW-REL` | REL no-output, no consumer, no documentation-as-release, and no public-release authority. |
| `W-REVIEW` | Only reviewability/tooling process outside every `NW-*` class may be proposed; none is accepted. |

## 6. Authoritative CR allocation matrix

Each row is planned, not executed. Exact aliases are defined in section 2.

| CR | Exact DES allocation | Exact PB allocation | Exact contract allocation | Planned evidence class |
|---|---|---|---|---|
| `CR-001` | `DES-TEST-001`, `DES-TRACE-001` | `PB-WS-001`, `PB-DOC-001` | `N/A` | Stage/WP and forbidden-code inspection |
| `CR-002` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Logical-line and responsibility review |
| `CR-003` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Total branch/typed-failure trace |
| `CR-004` | `DES-ACTIVE` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ACTIVE` | Bound/progress and exhaustion evidence |
| `CR-005` | `DES-ACTIVE` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ACTIVE` | Call graph and termination/depth review |
| `CR-006` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Hidden-failure scan and invalid-state cases |
| `CR-007` | `DES-SOURCE-001`, `DES-TEST-001`, `DES-TRACE-001`, `DES-HND-001`, `DES-REL-001` | `PB-PRODUCT`, `PB-WS-001` | `CONTRACT-ALL` | Privileged/unchecked surface inspection |
| `CR-008` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Default/fallback scan and negative cases |
| `CR-009` | `DES-ALL` | `PB-PRODUCT` | `CONTRACT-ALL` | Typed-family exhaustiveness |
| `CR-010` | `DES-ACTIVE` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ACTIVE` | Universal admission/bypass properties; REL no-output and minimal non-product `IF-TERM-001` remain the only exceptions |
| `CR-011` | `DES-SOURCE-001`, `DES-ECO-001`, `DES-ADP-001`, `DES-TEST-001`, `DES-TRACE-001`, `DES-DEL-001`, `DES-HND-001` | `PB-PRODUCT`, `PB-GEN-001`, `PB-TST-001` | `CONTRACT-SOURCE-001`, `CONTRACT-ECO-001`, `CONTRACT-TEST-001`, `CONTRACT-DEL-001`, `CONTRACT-HND-001`, `CONTRACT-TRACE-001` | Reproduction/order/digest evidence |
| `CR-012` | `DES-ACTIVE` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ACTIVE` | Ambient-state and schedule equivalence |
| `CR-013` | `DES-SOURCE-001`, `DES-ECO-001`, `DES-ADP-001`, `DES-TEST-001`, `DES-TRACE-001`, `DES-DEL-001`, `DES-HND-001` | `PB-CST-001`, `PB-PTH-001`, `PB-REV-001`, `PB-HND-001`, `PB-RUN-001` | `CONTRACT-SOURCE-001`, `CONTRACT-ECO-001`, `CONTRACT-TEST-001`, `CONTRACT-DEL-001`, `CONTRACT-HND-001`, `CONTRACT-TRACE-001` | Immutability/successor/acyclic properties |
| `CR-014` | `DES-ALL` | `PB-ALL` | `CONTRACT-ALL` | Fixed dependency and consumer inspection |
| `CR-015` | `DES-SOURCE-001`, `DES-AUTH-001`, `DES-RDY-001`, `DES-ACQ-001`, `DES-LOG-001`, `DES-ALLY-001`, `DES-DST-001`, `DES-HND-001`, `DES-REL-001` | `PB-CST-001`, `PB-AUT-001`, `PB-DOM-001`, `PB-HND-001`, `PB-DOC-001`, `PB-TST-001` | `CONTRACT-SOURCE-001`, `CONTRACT-AUTH-001`, `CONTRACT-RDY-001`, `CONTRACT-ACQ-001`, `CONTRACT-LOG-001`, `CONTRACT-ALLY-001`, `CONTRACT-DST-001`, `CONTRACT-HND-001`, `CONTRACT-REL-001` | Content-boundary/provenance cases |
| `CR-016` | `DES-SOURCE-001`, `DES-LOG-001`, `DES-HND-001`, `DES-REL-001` | `PB-CST-001`, `PB-DOM-001`, `PB-HND-001`, `PB-TST-001` | `CONTRACT-SOURCE-001`, `CONTRACT-LOG-001`, `CONTRACT-HND-001`, `CONTRACT-REL-001` | Composition/minimization adversarial cases |
| `CR-017` | `DES-AUTH-001`, `DES-RDY-001`, `DES-LOG-001`, `DES-ALLY-001`, `DES-DST-001`, `DES-ECO-001`, `DES-ADP-001`, `DES-DEL-001`, `DES-HND-001` | `PB-AUT-001`, `PB-DOM-001`, `PB-PTH-001`, `PB-HND-001`, `PB-TST-001` | `CONTRACT-AUTH-001`, `CONTRACT-RDY-001`, `CONTRACT-LOG-001`, `CONTRACT-ALLY-001`, `CONTRACT-DST-001`, `CONTRACT-ECO-001`, `CONTRACT-DEL-001`, `CONTRACT-HND-001` | Authority/floor/non-compensation cases |
| `CR-018` | `DES-RDY-001`, `DES-ACQ-001`, `DES-LOG-001`, `DES-ALLY-001`, `DES-DST-001` | `PB-DOM-001`, `PB-TST-001` | `CONTRACT-RDY-001`, `CONTRACT-ACQ-001`, `CONTRACT-LOG-001`, `CONTRACT-ALLY-001`, `CONTRACT-DST-001` | Domain/facet/distribution conservation |
| `CR-019` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | State/null/N/A/stale exhaustive cases |
| `CR-020` | `DES-ALLY-001`, `DES-DST-001`, `DES-ECO-001`, `DES-ADP-001`, `DES-DEL-001`, `DES-HND-001` | `PB-DOM-001`, `PB-PTH-001`, `PB-HND-001`, `PB-TST-001` | `CONTRACT-ALLY-001`, `CONTRACT-DST-001`, `CONTRACT-ECO-001`, `CONTRACT-DEL-001`, `CONTRACT-HND-001` | Checked accounting/reconciliation cases |
| `CR-021` | `DES-ACQ-001`, `DES-LOG-001`, `DES-ALLY-001`, `DES-DST-001`, `DES-ECO-001`, `DES-ADP-001`, `DES-DEL-001`, `DES-HND-001` | `PB-DOM-001`, `PB-PTH-001`, `PB-HND-001`, `PB-TST-001` | `CONTRACT-ACQ-001`, `CONTRACT-LOG-001`, `CONTRACT-ALLY-001`, `CONTRACT-DST-001`, `CONTRACT-ECO-001`, `CONTRACT-DEL-001`, `CONTRACT-HND-001` | Path/peer/burden/false-savings cases |
| `CR-022` | `DES-ECO-001`, `DES-ADP-001`, `DES-DEL-001`, `DES-HND-001` | `PB-PTH-001`, `PB-HND-001`, `PB-TST-001` | `CONTRACT-ECO-001`, `CONTRACT-DEL-001`, `CONTRACT-HND-001` | ECO/DEL/ADP model evidence |
| `CR-023` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Independence/finding/dissent cases |
| `CR-024` | `DES-TEST-001`, `DES-TRACE-001`, `DES-HND-001`, `DES-REL-001` | `PB-REV-001`, `PB-HND-001`, `PB-DOC-001`, `PB-TST-001` | `CONTRACT-TEST-001`, `CONTRACT-HND-001`, `CONTRACT-REL-001`, `CONTRACT-TRACE-001` | Terminal/no-output/backflow cases |
| `CR-025` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Hold transpose/propagation properties |
| `CR-026` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Invariant coverage report |
| `CR-027` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Property evidence set |
| `CR-028` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Transition/model evidence set |
| `CR-029` | `DES-ALL` | `PB-ALL` | `CONTRACT-ALL` | Cross-role adversarial suite |
| `CR-030` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Per-contract fixture matrix |
| `CR-031` | `DES-SOURCE-001`, `DES-TEST-001`, `DES-TRACE-001`, `DES-HND-001`, `DES-REL-001` | `PB-CST-001`, `PB-REV-001`, `PB-HND-001`, `PB-TST-001`, `PB-FIX-001`, `PB-GEN-001` | `CONTRACT-SOURCE-001`, `CONTRACT-TEST-001`, `CONTRACT-HND-001`, `CONTRACT-REL-001`, `CONTRACT-TRACE-001` | Parser/fuzz-if-authorized plan |
| `CR-032` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001`, `PB-GEN-001` | `CONTRACT-ALL` | Golden/regression/successor evidence |
| `CR-033` | `DES-TEST-001`, `DES-TRACE-001`, `DES-REL-001` | `PB-ALL` | `CONTRACT-ALL` | Mode/isolation inspection |
| `CR-034` | `DES-SOURCE-001`, `DES-TRACE-001`, `DES-REL-001` | `PB-CST-001`, `PB-REV-001`, `PB-DOC-001`, `PB-TST-001`, `PB-FIX-001`, `PB-GEN-001` | `CONTRACT-SOURCE-001`, `CONTRACT-REL-001`, `CONTRACT-TRACE-001` | Generated provenance/reproduction checks |
| `CR-035` | `DES-TEST-001`, `DES-TRACE-001` | `PB-WS-001`, `PB-PRODUCT`, `PB-TST-001` | `N/A` | Planned quality-gate outputs |
| `CR-036` | `DES-TRACE-001` | `PB-WS-001`, `PB-PRODUCT`, `PB-TST-001`, `PB-GEN-001` | `N/A` | Dependency/license/advisory audit |
| `CR-037` | `DES-ALL` | `PB-PRODUCT`, `PB-TST-001` | `CONTRACT-ALL` | Resource-bound register and tests |
| `CR-038` | `DES-TEST-001`, `DES-TRACE-001`, `DES-REL-001` | `PB-REV-001`, `PB-DOC-001`, `PB-TST-001`, `PB-WS-001` | `CONTRACT-TEST-001`, `CONTRACT-REL-001`, `CONTRACT-TRACE-001` | Waiver-ledger inspection |
| `CR-039` | `DES-TEST-001`, `DES-TRACE-001`, `DES-REL-001` | `PB-REV-001`, `PB-DOC-001`, `PB-TST-001` | `CONTRACT-TEST-001`, `CONTRACT-REL-001`, `CONTRACT-TRACE-001` | Evidence-state/digest audit |
| `CR-040` | `DES-ALL` | `PB-ALL` | `CONTRACT-ALL` | Mechanical trace/contradiction review |

The table above and the completion table below form one authoritative matrix
split on the unique `CR-*` key. `Fixed-DES-owner`, `Fixed-CONTRACT-owner`, and
`Fixed-HOLD-owner` are exact frozen lookups in DESIGN, INTERFACES, and
REQUIREMENTS; they add no role.

| CR | Exact invariant allocation | Exact hold allocation | Accountable owner / concurrence | Planned verification | Expected result | Waiver class |
|---|---|---|---|---|---|---|
| `CR-001` | `INV-20`, `INV-22` | `HOLD-ALL` | Maintainer / Review, Scope | `CR-EVID-001` stage/WP inspection | No implementation exists or is implied. | `NW-AUTH`, `NW-HOLD` |
| `CR-002` | `INV-04`, `INV-20` | `TBD-TST-001` | Maintainer / Review | `CR-EVID-001` unit review | Each unit satisfies the fixed review trigger. | `W-REVIEW`; none accepted |
| `CR-003` | `INV-02`, `INV-04`, `INV-13`, `INV-15`, `INV-20` | `HOLD-ALL` | Maintainer / Review, `Fixed-DES-owner` | `CR-EVID-001` branch trace | Every branch is total and owner-bound. | `NW-SEM`, `NW-HOLD` |
| `CR-004` | `INV-09`, `INV-11`, `INV-12`, `INV-25` | `TBD-TST-001` | Maintainer / Review, Methodology | `CR-EVID-001` bound tests | Every iteration terminates or fails safely. | `NW-SEM` |
| `CR-005` | `INV-09`, `INV-11`, `INV-12`, `INV-25` | `TBD-TST-001` | Maintainer / Review, Methodology | `CR-EVID-001` call/decrease proof | No unproved recursion exists. | `W-REVIEW`; none accepted |
| `CR-006` | `INV-13`, `INV-20` | `HOLD-ALL` | Maintainer / Review | `CR-EVID-002` failure scan | No hidden termination path exists. | `NW-SEM`, `NW-HOLD` |
| `CR-007` | `INV-02`, `INV-03`, `INV-04`, `INV-06`, `INV-20`, `INV-22` | `TBD-SEC-001` | Maintainer / Security, Scope | `CR-EVID-002` surface inspection | Zero unauthorized privileged surfaces. | `NW-SEC`, `NW-AUTH` |
| `CR-008` | `INV-13`, `INV-14`, `INV-15`, `INV-20` | `HOLD-ALL` | Review / Maintainer, Methodology | `CR-EVID-002` default scan | Unknowns never map favorably. | `NW-SEM`, `NW-HOLD` |
| `CR-009` | `INV-13`, `INV-14`, `INV-23`, `INV-25` | `HOLD-ALL` | Review / Maintainer, both Assurance | `CR-EVID-002` typed-state enumeration | State families and relationships remain distinct. | `NW-SEM`, `NW-SEC`, `NW-SAFE` |
| `CR-010` | `INV-02`, `INV-03`, `INV-04`, `INV-20` | `TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001` | Source / Security, Review | `CR-EVID-002` gate-bypass suite | Every product follows SOURCE then TEST. | `NW-SEC`, `NW-GRAPH`, `NW-HOLD` |
| `CR-011` | `INV-03`, `INV-07`, `INV-10`, `INV-12`, `INV-23`, `INV-24` | `TBD-SRC-001`, `TBD-TST-001` | Source / Citation, Review | `CR-EVID-003` reproduction tests | Equal semantic inputs reproduce bonds/order. | `NW-SEM`, `NW-EVID` |
| `CR-012` | `INV-02`, `INV-03`, `INV-07`, `INV-10`, `INV-12` | `TBD-TST-001` | Maintainer / Review, Methodology | `CR-EVID-003` schedule evidence | Semantics are ambient-state independent. | `NW-SEM` |
| `CR-013` | `INV-07`, `INV-09`, `INV-10`, `INV-11`, `INV-12`, `INV-24`, `INV-25` | `TBD-ECO-001`, `TBD-DEL-001`, `TBD-HND-001` | Maintainer / Review, Source, Delivery | `CR-EVID-003` graph/successor model | Generations and versions strictly advance. | `NW-GRAPH`, `NW-SEM` |
| `CR-014` | `INV-01`, `INV-04`, `INV-05`, `INV-06`, `INV-08`, `INV-19`, `INV-21`, `INV-22` | `TBD-TST-001` | Maintainer / Review, Scope | `CR-EVID-003` dependency inspection | Only fixed edges and 13 contracts exist. | `NW-GRAPH`, `NW-AUTH` |
| `CR-015` | `INV-02`, `INV-03`, `INV-08`, `INV-20`, `INV-22` | `TBD-SEC-001`, `TBD-SRC-001`, `TBD-REL-001` | Source / Security, Citation, Scope | `CR-EVID-004` boundary cases | Only safe admitted aggregate content passes. | `NW-SEC`, `NW-HOLD` |
| `CR-016` | `INV-02`, `INV-03`, `INV-16`, `INV-20`, `INV-22` | `TBD-SEC-001` | Security / Source, Scope | `CR-EVID-004` composition cases | Unsafe composition fails with minimum custody. | `NW-SEC` |
| `CR-017` | `INV-08`, `INV-15`, `INV-16`, `INV-17`, `INV-22` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001` | Civilian/Safety / domain owners, Security | `CR-EVID-004` bypass/non-compensation | All applicable floors pass independently. | `NW-CIV`, `NW-SAFE`, `NW-DST` |
| `CR-018` | `INV-13`, `INV-14`, `INV-16`, `INV-17`, `INV-21` | `TBD-RDY-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001` | `Fixed-DES-owner` / Review, both Assurance | `CR-EVID-004` conservation properties | Domains/facets/lenses remain distinct. | `NW-SEM`, `NW-DST`, `NW-HOLD` |
| `CR-019` | `INV-13`, `INV-14`, `INV-23`, `INV-25` | `HOLD-ALL` | Review / Methodology, `Fixed-DES-owner` | `CR-EVID-004` posture tests | Missing/null/N/A/stale/lifecycle never collapse. | `NW-SEM`, `NW-HOLD` |
| `CR-020` | `INV-17`, `INV-18` | `TBD-QNT-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-DEL-001` | Finance / Numeracy, domain owners | `CR-EVID-004` accounting properties | Checked ledgers/dimensions reconcile. | `NW-SEM`, `NW-DST`, `NW-HOLD` |
| `CR-021` | `INV-16`, `INV-17`, `INV-18`, `INV-22` | `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-DEL-001` | Finance / Numeracy, Delivery, both Assurance | `CR-EVID-004` pathway/peer/burden cases | No false savings, authority, or burden erasure. | `NW-AUTH`, `NW-SAFE`, `NW-DST` |
| `CR-022` | `INV-07`, `INV-09`, `INV-11`, `INV-12`, `INV-18`, `INV-24` | `TBD-ECO-001`, `TBD-DEL-001`, `TBD-HND-001` | Delivery / Finance, Review | `CR-EVID-004` successor model | Full ECO/DEL/ADP sequence is mandatory. | `NW-GRAPH`, `NW-SAFE`, `NW-HOLD` |
| `CR-023` | `INV-04`, `INV-19`, `INV-20` | `TBD-TST-001` | Review / Test, Maintainer, both Assurance | `CR-EVID-004` independence cases | No self-review or evidence-free pass. | `NW-EVID`, `NW-SEC`, `NW-SAFE` |
| `CR-024` | `INV-05`, `INV-06`, `INV-07`, `INV-08`, `INV-20`, `INV-22` | `TBD-SEC-001`, `TBD-TST-001`, `TBD-HND-001`, `TBD-REL-001` | Handoff / Security, Review, Scope | `CR-EVID-004` terminal/no-output tests | HND/TERM terminates; REL emits nothing. | `NW-HND`, `NW-REL`, `NW-GRAPH`, `NW-AUTH` |
| `CR-025` | `INV-13`, `INV-15`, `INV-20` | `HOLD-ALL` | Review / `Fixed-HOLD-owner`, Maintainer | `CR-EVID-004` hold transpose | All 13 pairs block exact branches. | `NW-HOLD`, `NW-EVID` |
| `CR-026` | `INV-ALL` | `HOLD-ALL` | Review / Maintainer, `Fixed-DES-owner` | `CR-EVID-005` invariant report | All 25 invariants have proof mechanisms. | `NW-SEM`, `NW-HOLD`, `NW-EVID` |
| `CR-027` | `INV-02`–`INV-04`, `INV-07`, `INV-09`–`INV-18`, `INV-20`, `INV-23`–`INV-25` | `HOLD-ALL` | Methodology / Review, both Assurance | `CR-EVID-005` property suite | Generated safe cases preserve all properties. | `NW-SEM`, `NW-HOLD`, `NW-EVID` |
| `CR-028` | `INV-07`, `INV-09`–`INV-12`, `INV-19`, `INV-23`–`INV-25` | `HOLD-ALL` | Review / Maintainer, Methodology | `CR-EVID-005` transition/model suite | All 18 rows and forbidden edges hold. | `NW-GRAPH`, `NW-SEM`, `NW-EVID` |
| `CR-029` | `INV-ALL` | `HOLD-ALL` | Test / all role lenses, both Assurance | `CR-EVID-005` adversarial suite | Every fixed abuse class fails closed. | Applicable `NW-*`; no waiver |
| `CR-030` | `INV-ALL` | `HOLD-ALL` | Test / `Fixed-CONTRACT-owner`, Review | `CR-EVID-005` contract matrix | All 13 contracts have complete coverage. | `NW-SEM`, `NW-HOLD`, `NW-EVID` |
| `CR-031` | `INV-02`, `INV-03`, `INV-05`–`INV-08`, `INV-13`, `INV-14`, `INV-20`, `INV-22`, `INV-23` | `TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001`, `TBD-REL-001` | Source / Security, Test | `CR-EVID-006` parser/fuzz evidence if applicable | Untrusted boundaries are bounded and safe. | `NW-SEC`, `NW-EVID` |
| `CR-032` | `INV-02`, `INV-03`, `INV-07`, `INV-09`–`INV-14`, `INV-19`, `INV-20`, `INV-23`–`INV-25` | `HOLD-ALL` | Maintainer / Review, Citation | `CR-EVID-006` regression suite | Semantics/history reproduce without silent update. | `NW-SEM`, `NW-EVID` |
| `CR-033` | `INV-01`, `INV-04`–`INV-06`, `INV-08`, `INV-19`–`INV-22` | `TBD-TST-001` | Maintainer / Review, Scope | `CR-EVID-006` mode/dependency inspection | Support never becomes product truth. | `NW-GRAPH`, `NW-AUTH` |
| `CR-034` | `INV-02`, `INV-03`, `INV-05`, `INV-20`, `INV-21` | `TBD-SRC-001`, `TBD-REL-001` | Maintainer / Source, Citation, Review | `CR-EVID-006` regeneration inspection | Generated output is derivative and isolated. | `NW-GRAPH`, `NW-EVID`, `NW-REL` |
| `CR-035` | `INV-04`, `INV-20` | `TBD-TST-001` | Maintainer / Review | `CR-EVID-007` quality outputs | Every selected gate is clean; currently absent. | `W-REVIEW`, `NW-EVID` |
| `CR-036` | `INV-04`, `INV-20`, `INV-21` | `TBD-TST-001` | Maintainer / Review, Security | `CR-EVID-008` dependency audit | Dependency surface is reviewed; none exists. | `NW-SEC`, `NW-EVID` |
| `CR-037` | `INV-02`, `INV-03`, `INV-09`, `INV-13`, `INV-15`–`INV-18`, `INV-20`, `INV-22` | `HOLD-ALL` | Maintainer / Review, Methodology, Numeracy | `CR-EVID-009` resource register | Accepted bounds hold at exhaustion. | Applicable `NW-*`; values not waivable |
| `CR-038` | `INV-06`, `INV-08`, `INV-15`, `INV-16`, `INV-19`, `INV-20`, `INV-22` | `TBD-TST-001` | Review / Maintainer, both Assurance | `CR-EVID-010` waiver ledger | Zero prohibited or stale waivers. | All `NW-*`; none accepted |
| `CR-039` | `INV-03`, `INV-06`, `INV-07`, `INV-19`, `INV-20`, `INV-23`, `INV-24` | `TBD-TST-001` | Review / Citation, Maintainer | `CR-EVID-010` evidence audit | Evidence state and digest claims are exact. | `NW-EVID` |
| `CR-040` | `INV-ALL` | `HOLD-ALL` | Maintainer / Review, Scope, both Assurance | `CR-EVID-010` trace audit | No contradiction, orphan, or authority leak. | Applicable `NW-*`; no waiver |

## 7. Complete DES transpose

| Design decision | Exact controlling CR set |
|---|---|
| `DES-SOURCE-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-015, CR-016, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-034, CR-037, CR-040}` |
| `DES-AUTH-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-RDY-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-018, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-ACQ-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-018, CR-019, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-LOG-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-016, CR-017, CR-018, CR-019, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-ALLY-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-018, CR-019, CR-020, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-DST-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-018, CR-019, CR-020, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-ECO-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-ADP-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-TEST-001` | `{CR-001, CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-035, CR-037, CR-038, CR-039, CR-040}` |
| `DES-TRACE-001` | `{CR-001, CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-035, CR-036, CR-037, CR-038, CR-039, CR-040}` |
| `DES-DEL-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `DES-HND-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-015, CR-016, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-037, CR-040}` |
| `DES-REL-001` | `{CR-002, CR-003, CR-006, CR-007, CR-008, CR-009, CR-014, CR-015, CR-016, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-037, CR-038, CR-039, CR-040}` |

## 8. Complete package-boundary transpose

| Boundary | Exact controlling CR set |
|---|---|
| `PB-WS-001` | `{CR-001, CR-007, CR-014, CR-029, CR-033, CR-035, CR-036, CR-038, CR-040}` |
| `PB-CST-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-015, CR-016, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-035, CR-036, CR-037, CR-040}` |
| `PB-AUT-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-014, CR-015, CR-017, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-035, CR-036, CR-037, CR-040}` |
| `PB-DOM-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-014, CR-015, CR-016, CR-017, CR-018, CR-019, CR-020, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-035, CR-036, CR-037, CR-040}` |
| `PB-PTH-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-035, CR-036, CR-037, CR-040}` |
| `PB-REV-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-035, CR-036, CR-037, CR-038, CR-039, CR-040}` |
| `PB-HND-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-015, CR-016, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-035, CR-036, CR-037, CR-040}` |
| `PB-RUN-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-035, CR-036, CR-037, CR-040}` |
| `PB-DOC-001` | `{CR-001, CR-014, CR-015, CR-024, CR-029, CR-033, CR-034, CR-038, CR-039, CR-040}` |
| `PB-TST-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-008, CR-010, CR-011, CR-012, CR-014, CR-015, CR-016, CR-017, CR-018, CR-019, CR-020, CR-021, CR-022, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-035, CR-036, CR-037, CR-038, CR-039, CR-040}` |
| `PB-FIX-001` | `{CR-014, CR-029, CR-031, CR-033, CR-034, CR-040}` |
| `PB-GEN-001` | `{CR-011, CR-014, CR-029, CR-031, CR-032, CR-033, CR-034, CR-036, CR-040}` |

## 9. Complete contract transpose

| Contract | Exact controlling CR set |
|---|---|
| `CONTRACT-SOURCE-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-015, CR-016, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-037, CR-040}` |
| `CONTRACT-AUTH-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-RDY-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-018, CR-019, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-ACQ-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-018, CR-019, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-LOG-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-016, CR-017, CR-018, CR-019, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-ALLY-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-018, CR-019, CR-020, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-DST-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-012, CR-014, CR-015, CR-017, CR-018, CR-019, CR-020, CR-021, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-ECO-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-TEST-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-037, CR-038, CR-039, CR-040}` |
| `CONTRACT-DEL-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-HND-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-015, CR-016, CR-017, CR-019, CR-020, CR-021, CR-022, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-037, CR-040}` |
| `CONTRACT-REL-001` | `{CR-002, CR-003, CR-006, CR-007, CR-008, CR-009, CR-014, CR-015, CR-016, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-037, CR-038, CR-039, CR-040}` |
| `CONTRACT-TRACE-001` | `{CR-002, CR-003, CR-004, CR-005, CR-006, CR-007, CR-008, CR-009, CR-010, CR-011, CR-012, CR-013, CR-014, CR-019, CR-023, CR-024, CR-025, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-037, CR-038, CR-039, CR-040}` |

## 10. Complete invariant transpose

`INV-01` through `INV-25` refer to DESIGN invariants in their fixed order.

| Invariant | Exact controlling CR set |
|---|---|
| `INV-01` | `{CR-014, CR-026, CR-029, CR-030, CR-033, CR-040}` |
| `INV-02` | `{CR-003, CR-007, CR-010, CR-012, CR-015, CR-016, CR-026, CR-027, CR-029, CR-030, CR-031, CR-032, CR-034, CR-037, CR-040}` |
| `INV-03` | `{CR-007, CR-010, CR-011, CR-012, CR-015, CR-016, CR-026, CR-027, CR-029, CR-030, CR-031, CR-032, CR-034, CR-037, CR-039, CR-040}` |
| `INV-04` | `{CR-002, CR-003, CR-007, CR-010, CR-014, CR-023, CR-026, CR-027, CR-029, CR-030, CR-033, CR-035, CR-036, CR-040}` |
| `INV-05` | `{CR-014, CR-024, CR-026, CR-029, CR-030, CR-031, CR-033, CR-034, CR-040}` |
| `INV-06` | `{CR-007, CR-014, CR-024, CR-026, CR-029, CR-030, CR-031, CR-033, CR-038, CR-039, CR-040}` |
| `INV-07` | `{CR-011, CR-012, CR-013, CR-022, CR-024, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-039, CR-040}` |
| `INV-08` | `{CR-014, CR-015, CR-017, CR-024, CR-026, CR-029, CR-030, CR-031, CR-033, CR-038, CR-040}` |
| `INV-09` | `{CR-004, CR-005, CR-013, CR-022, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-037, CR-040}` |
| `INV-10` | `{CR-011, CR-012, CR-013, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-040}` |
| `INV-11` | `{CR-004, CR-005, CR-013, CR-022, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-040}` |
| `INV-12` | `{CR-004, CR-005, CR-011, CR-012, CR-013, CR-022, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-040}` |
| `INV-13` | `{CR-003, CR-006, CR-008, CR-009, CR-018, CR-019, CR-025, CR-026, CR-027, CR-029, CR-030, CR-031, CR-032, CR-037, CR-040}` |
| `INV-14` | `{CR-008, CR-009, CR-018, CR-019, CR-026, CR-027, CR-029, CR-030, CR-031, CR-032, CR-040}` |
| `INV-15` | `{CR-003, CR-008, CR-017, CR-025, CR-026, CR-027, CR-029, CR-030, CR-037, CR-038, CR-040}` |
| `INV-16` | `{CR-016, CR-017, CR-018, CR-021, CR-026, CR-027, CR-029, CR-030, CR-037, CR-038, CR-040}` |
| `INV-17` | `{CR-017, CR-018, CR-020, CR-021, CR-026, CR-027, CR-029, CR-030, CR-037, CR-040}` |
| `INV-18` | `{CR-020, CR-021, CR-022, CR-026, CR-027, CR-029, CR-030, CR-037, CR-040}` |
| `INV-19` | `{CR-014, CR-023, CR-026, CR-028, CR-029, CR-030, CR-032, CR-033, CR-038, CR-039, CR-040}` |
| `INV-20` | `{CR-001, CR-002, CR-003, CR-006, CR-007, CR-008, CR-010, CR-015, CR-016, CR-023, CR-024, CR-025, CR-026, CR-027, CR-029, CR-030, CR-031, CR-032, CR-033, CR-034, CR-035, CR-036, CR-037, CR-038, CR-039, CR-040}` |
| `INV-21` | `{CR-014, CR-018, CR-026, CR-029, CR-030, CR-033, CR-034, CR-036, CR-040}` |
| `INV-22` | `{CR-001, CR-007, CR-014, CR-015, CR-016, CR-017, CR-021, CR-024, CR-026, CR-029, CR-030, CR-031, CR-033, CR-037, CR-038, CR-040}` |
| `INV-23` | `{CR-009, CR-011, CR-019, CR-026, CR-027, CR-028, CR-029, CR-030, CR-031, CR-032, CR-039, CR-040}` |
| `INV-24` | `{CR-011, CR-013, CR-022, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-039, CR-040}` |
| `INV-25` | `{CR-004, CR-005, CR-009, CR-013, CR-019, CR-026, CR-027, CR-028, CR-029, CR-030, CR-032, CR-040}` |

## 11. All 18 transition proof obligations

The transition numbers below follow DESIGN section 9 row order and do not
rename or alter a posture.

| Transition | Fixed event | Exact controlling CR set |
|---|---|---|
| `TR-01` | Unformed inputs become available or remain held. | `{CR-003, CR-008, CR-019, CR-025, CR-028, CR-030}` |
| `TR-02` | Candidate freezes its exact semantic bond. | `{CR-003, CR-011, CR-013, CR-028, CR-030}` |
| `TR-03` | Frozen candidate receives exact-bond SOURCE pass. | `{CR-010, CR-028, CR-030}` |
| `TR-04` | SOURCE hold/reject terminates the affected branch. | `{CR-008, CR-010, CR-019, CR-025, CR-028, CR-030}` |
| `TR-05` | SOURCE identity/digest/context mismatch blocks promotion. | `{CR-009, CR-010, CR-019, CR-023, CR-028, CR-030}` |
| `TR-06` | Admitted candidate receives independent TEST pass. | `{CR-010, CR-023, CR-028, CR-030}` |
| `TR-07` | TEST finding/conflict/absence/blocking dissent holds and returns control. | `{CR-023, CR-028, CR-030, CR-039}` |
| `TR-08` | Admission/compatibility stale blocks the exact bond only. | `{CR-009, CR-013, CR-019, CR-028, CR-032}` |
| `TR-09` | Material change creates a new successor and repeats controls. | `{CR-011, CR-013, CR-028, CR-032}` |
| `TR-10` | Accepted successor retains historical supersession bonds. | `{CR-009, CR-011, CR-013, CR-028, CR-032}` |
| `TR-11` | Adaptive/trigger record receives exactly one lifecycle disposition. | `{CR-009, CR-013, CR-028, CR-030}` |
| `TR-12` | Preliminary `ECO[n]` plus accepted `DEL[n]` may form final `ECO[n+1]`. | `{CR-022, CR-028, CR-030}` |
| `TR-13` | Accepted final `ECO[n+1]` may form `ADP[n+1]`. | `{CR-022, CR-028, CR-030}` |
| `TR-14` | Later material feedback starts a later preliminary successor. | `{CR-013, CR-022, CR-028, CR-032}` |
| `TR-15` | Frozen HND plus SOURCE and TEST may receive one minimal receipt. | `{CR-010, CR-024, CR-028, CR-030}` |
| `TR-16` | External gate consumes the receipt and terminates at Taxlane boundary. | `{CR-024, CR-028, CR-030}` |
| `TR-17` | Any bundle change invalidates the receipt and restarts controls. | `{CR-013, CR-024, CR-028, CR-032}` |
| `TR-18` | No-release record remains no-release and emits nothing. | `{CR-024, CR-028, CR-030}` |

## 12. Controlled holds preserved

No method, value, threshold, schema, acceptance, evidence, or closure is
supplied. Each pair remains open with its fixed direct/transitive DESIGN graph.

| Exact TBD / specification pair | Code-rigor effect | Planned evidence before affected code could pass |
|---|---|---|
| `TBD-SEC-001` / `SPEC-UNK-SEC-001` | Keep affected admission, composition, retention, visualization, handoff, and release held; prohibit unsafe or reconstructive defaults. | Accepted security/aggregation rules and direct/compositional adversarial evidence. |
| `TBD-RDY-001` / `SPEC-UNK-RDY-001` | Keep affected readiness, candidate, savings, delivery, and handoff branches held. | Accepted readiness system, floors, promise, and non-degradation evidence. |
| `TBD-SRC-001` / `SPEC-UNK-SRC-001` | Keep unrepresentable custody, version, re-admission, and review behavior held. | Accepted source/custody representation and round-trip/re-admission evidence. |
| `TBD-QNT-001` / `SPEC-UNK-QNT-001` | Keep affected projection, peer, horizon, cross-scenario total, and fiscal handoff held. | Accepted quantitative methods, bounds, uncertainty, and reproduction evidence. |
| `TBD-ACQ-001` / `SPEC-UNK-ACQ-001` | Keep acquisition, commonality, capacity, competition, schedule, transition, and savings branches held. | Accepted industrial-base semantics and facet/capacity evidence. |
| `TBD-LOG-001` / `SPEC-UNK-LOG-001` | Keep sustainment, readiness, lifecycle, repair-tail, degraded-recovery, and savings branches held. | Accepted logistics/sustainment semantics and conservation/tail evidence. |
| `TBD-ALLY-001` / `SPEC-UNK-ALLY-001` | Keep joint, interoperability, sovereignty, partner-capacity, burden, and fiscal claims held. | Accepted alliance/interoperability rules and separated-ledger evidence. |
| `TBD-DST-001` / `SPEC-UNK-DST-001` | Keep affected efficiency, savings, readiness, distribution, burden, tail, and handoff claims held. | Accepted distribution/incidence methods and all stakeholder assurance evidence. |
| `TBD-ECO-001` / `SPEC-UNK-ECO-001` | Keep monetization, realizable-savings, receipt, net-pressure, and Taxlane handoff claims held. | Accepted economic/accounting semantics and checked gross-to-net/overlap evidence. |
| `TBD-TST-001` / `SPEC-UNK-TST-001` | Preserve the literal product-evidence and downstream promotion hold under accepted `CHG-BA-TST-001`; CODE_RIGOR planning closes neither. | Separately accepted planning-only VERIFICATION method and later executed independent evidence. |
| `TBD-DEL-001` / `SPEC-UNK-DEL-001` | Retain research-hypothesis posture and block realizable savings, implementation, delivery promotion, and handoff. | Accepted delivery method, owner/capacity/milestone/stop/rollback and realization evidence. |
| `TBD-HND-001` / `SPEC-UNK-HND-001` | Enforce deterministic no-pack emission and infer no Taxlane admission. | Future shared mapping acceptance plus round-trip, golden-semantic, security, and no-backflow evidence. |
| `TBD-REL-001` / `SPEC-UNK-REL-001` | Enforce no public release and no output consumer. | New release authority and separately accepted release validation; neither exists. |

## 13. Exceptions and waivers

A proposed waiver must contain a stable identity/status; exact CR, artifact and
digest; DES/PB/contract/invariant/hold scope; rationale; risk and blast radius;
compensating controls; accountable owner and independent concurrence; decision,
expiry, revisit and invalidation conditions; and exact evidence bonds. A
proposal is not acceptance.

Accepted waivers: **none**.

No waiver may touch an `NW-*` class, close a hold, introduce prohibited content,
weaken civilian control or a safety/readiness/distribution floor, change a
contract consumer, reverse a graph edge, skip SOURCE or TEST, alter ECO/DEL/ADP
order, emit HND, widen or recurse `IF-TERM-001`, create REL output, infer
Taxlane state, claim execution, authorize implementation, or authorize release.

## 14. Planned verification and evidence posture

All destinations are required for later separately authorized work. None is
implemented, executed, reviewed, or accepted.

| Evidence ID | Constraint coverage | Planned evidence | Current result |
|---|---|---|---|
| `CR-EVID-001` | `CR-001`–`CR-005` | Stage/WP gate, unit review, branch map, iteration bounds, recursion/termination proof | planned; absent |
| `CR-EVID-002` | `CR-006`–`CR-010` | Hidden-failure, privileged-surface, default, typed-state, and SOURCE-before-TEST bypass evidence | planned; absent |
| `CR-EVID-003` | `CR-011`–`CR-014` | Reproduction, ambient-state, immutable-successor, finite graph, and dependency-direction evidence | planned; absent |
| `CR-EVID-004` | `CR-015`–`CR-025` | Content/security, civilian/floor, domain/distribution, accounting/pathway, delivery, review, terminal, and hold suites | planned; absent |
| `CR-EVID-005` | `CR-026`–`CR-030` | 25-invariant report, properties, all 18 transitions, adversarial cases, and 13-contract matrix | planned; absent |
| `CR-EVID-006` | `CR-031`–`CR-034` | Parser/fuzz-if-authorized, golden/regression, mode isolation, and generated-output evidence | planned; absent |
| `CR-EVID-007` | `CR-035` | Compiler, format, lint, test, documentation, and static-check evidence | planned; tools/configuration absent |
| `CR-EVID-008` | `CR-036` | Dependency, feature, build/native surface, license, advisory, maintenance, and reproduction audit | planned; no dependency set exists |
| `CR-EVID-009` | `CR-037` | Accepted resource-bound register and boundary/exhaustion/degradation evidence | planned; values intentionally absent |
| `CR-EVID-010` | `CR-038`–`CR-040` | Waiver ledger, evidence-state/digest audit, and mechanical contract/invariant/trace review | planned; no waivers or code evidence |

The later Rust gate must treat every selected quality, test, documentation,
static, dependency, license, advisory, resource, and waiver condition as
blocking until satisfied or lawfully resolved. No tool, command, version,
configuration, dependency, threshold, or audit product is selected here.

## 15. Role synthesis and review routing

This is author synthesis, not a role pass.

- Classification & Operational Security challenges every content,
  composition, custody, failure-detail, fixture, HND, and release surface.
- Civilian Control, Law, Safety & Readiness independently challenges authority,
  safety/readiness/resilience floors, delivery, terminal behavior, and every
  safety-sensitive promotion.
- Independent Test & Oversight and the Role review steward challenge exact
  digest review, falsification, conflict, findings, dissent, evidence truth,
  convergence, terminal finiteness, and reviewer independence.
- Citation, Numeracy, and Scope challenge provenance, dimensions, uncertainty,
  accounting, trace, public-aggregate/unclassified limits, and authority.
- The eight parliament roles challenge mission/readiness, acquisition and
  industrial base, logistics/sustainment, fiscal accounting, service-member and
  family burdens, interoperability, delivery, and adverse evidence.
- All seven stakeholder lenses remain mandatory in property and adversarial
  evidence; no majority can erase a concentrated effect or hard floor.
- The Methodology Panel challenges totality, falsifiability, determinism,
  boundedness, null/N/A integrity, successor order, and reproduction without
  importing classified expertise.

Reviewers remain sidecar controls and never become semantic consumers or
producers. Both assurance gates must pass every applicable safety-sensitive
promotion and are non-waivable.

## 16. Review readiness and disposition

This artifact contains 40 unique normative constraints; exact forward and
reverse allocations for 14 DES decisions, 12 package boundaries, 13 contracts,
and all 25 invariants; explicit coverage of all 18 transitions; all 13 exact
open hold pairs; 12 waiver classes; and 10 planned evidence destinations.

Disposition: **review-ready high-assurance CODE_RIGOR author baseline; not a
fixed point**. Independent digest-bound code-rigor, parliament, stakeholder,
editorial, methodology, and both assurance reviews remain required.

All evidence is planned, absent, and unexecuted. All 13 holds remain open and
conjunctive. No code, Cargo/workspace state, work package, dependency, schema,
API, command, tool selection, method value, executed evidence, HND emission,
Taxlane action, official action, procurement, budget, allocation, rate,
deployment, or public release is authorized or created.
