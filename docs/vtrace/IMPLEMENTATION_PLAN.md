# BASTION Implementation Plan

## 1. Status and authority

Status: `active; WP-WS-001 complete; WP-REV-001 and WP-TST-001 acceptance-ready`.

This plan converts the fixed BASTION mission-through-code-rigor chain into
bounded future Rust work. It authorizes no code, workspace, package,
dependency, parser, corpus acquisition, classified or controlled content,
targeting, operational planning, exploitable vulnerability content, person
record, mission/force/procurement decision, budget, allocation, rate, Taxlane
mutation, HND emission, release, official use, or public claim.

The companion `WORK_PACKAGES.md` is an allocation and gate register, not an
accepted work package. Prospective `CHG-BA-TST-BOOT-002` changes only the
future pure-scaffold eligibility described in sections 5 and 11.
Implementation remains blocked until accepted
digest-bound `VERIFICATION.md` and `VALIDATION.md` fixed points exist, every
applicable hold is closed by its fixed owner except where an open hold is the
explicit input to a no-emission proof, and the exact WP, representation,
resource bounds, safe fixtures, toolchain, dependencies, commands, rollback,
and assurance decisions are independently accepted.

## 2. Frozen baseline

| Input | SHA-256 |
|---|---|
| `MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| `CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| `REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| `SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| `ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| `PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| `INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| `CHANGE_CONTROL.md` / prospective `CHG-BA-TST-BOOT-002` | `ed259afd24fc39ea87d8b21c4b5a535a2b369c5558e9dd6e2f25b1e070deaa3c` |
| `DESIGN.md` | `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` |
| `CODE_RIGOR.md` | `3ce31b808f038291b79bf348547ed43029ca1b0a797520fbb72546d3964801d9` |
| CODE_RIGOR fixed-point record `pulse-08-code-rigor.md` | `501c1f23136b2939d94647204536f0b9d49902b97987533d5968f2c768e1eee6` |

Controlled source universe: 98 requirements; 121 specification identities
(98 functional, 10 nonfunctional, 13 held `SPEC-UNK-*`); 14 design decisions;
13 contracts; 12 package boundaries; and 40 code-rigor constraints, exactly
298 controlled identities. This plan changes none.

## 3. Exact planning aliases

- `WP-DOMAIN` = `{WP-RDY-001, WP-ACQ-001, WP-LOG-001, WP-ALLY-001,
  WP-DST-001}`.
- `WP-PATH` = `{WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001,
  WP-ADP-001}`.
- `WP-PRODUCT` = `{WP-CST-001, WP-AUT-001}` + `WP-DOMAIN` + `WP-PATH` +
  `{WP-HND-001, WP-RUN-001}`, exactly 13 semantic producers.
- `WP-SUPPORT` = `{WP-WS-001, WP-REV-001, WP-TST-001, WP-GEN-001,
  WP-DOC-001}`.
- `WP-SIDECAR-PRODUCERS` = `WP-PRODUCT` + `{WP-GEN-001, WP-DOC-001}`,
  exactly 15 WPs. Each member owns its later exact `PB-REV-001` and
  `PB-TST-001` sidecar delta; neither bootstrap WP owns that delta.
- `WP-IMPLEMENTATION` = `WP-PRODUCT` + `WP-SUPPORT` + `{WP-INT-001}`,
  exactly 19 WPs.
- `WP-TST-BLOCKED` = `WP-PRODUCT` + `{WP-GEN-001, WP-DOC-001,
  WP-INT-001}`, exactly 16 WPs. The open TST pair blocks this set, is a
  proof input only for pure `{WP-REV-001, WP-TST-001}` bootstrap, and has no
  relationship to `WP-WS-001`.
- `WP-ALL` = `{WP-VV-001}` + `WP-IMPLEMENTATION`, exactly 20 WPs.
- `CTRL-HND-EMIT-001` is a deferred control, not a WP. It requires a future
  new assignment and newly accepted exact WP after all 12 applicable
  non-release holds and the shared external mapping close. `TBD-REL-001` never
  becomes an HND-emission prerequisite; it remains a separate release-blocking
  hold. The control grants no present work permission.

Within `WP-IMPLEMENTATION`, `WP-WS-001` is complete, `WP-REV-001` and
`WP-TST-001` are acceptance-ready, and every semantic producer remains
`proposed; entry_blocked`. `WP-VV-001` planning is complete.

## 4. Smallest coherent slices

| WP | Fixed boundary/touch | Coherent planned result | Current disposition |
|---|---|---|---|
| `WP-VV-001` | VTRACE planning only | Accepted VERIFICATION and VALIDATION plans with exact method, command, evidence, assurance, and decision registries | `discovery`; planning only |
| `WP-WS-001` | `PB-WS-001` | Empty policy-bearing workspace skeleton with no semantics | `complete` |
| `WP-CST-001` | `PB-CST-001` + `PB-REV-001` / `PB-TST-001` sidecars | Public-aggregate source/security admission, immutable custody, safe rejection, and exact-output re-admission | `block` |
| `WP-AUT-001` | `PB-AUT-001` + `PB-REV-001` / `PB-TST-001` sidecars | Bounded civilian mission/authority abstraction re-admitted through SOURCE | `block` |
| `WP-RDY-001` | RDY slice of `PB-DOM-001` + `PB-REV-001` / `PB-TST-001` sidecars | Separate readiness, safety, resilience, surge, recovery, distributions, and floors | `block` |
| `WP-ACQ-001` | ACQ slice of `PB-DOM-001` + `PB-REV-001` / `PB-TST-001` sidecars | Acquisition, competition, capacity, concentration, qualification, workforce, transition, and commonality facets | `block` |
| `WP-LOG-001` | LOG slice of `PB-DOM-001` + `PB-REV-001` / `PB-TST-001` sidecars | Stock, condition, custody, maintenance, repair tails, workload, lifecycle, and degraded recovery | `block` |
| `WP-ALLY-001` | ALLY slice of `PB-DOM-001` + `PB-REV-001` / `PB-TST-001` sidecars | Commitments, sovereignty, compatibility, standards, partner capacity, logistics, and separated burden ledgers | `block` |
| `WP-DST-001` | DST slice of `PB-DOM-001` + `PB-REV-001` / `PB-TST-001` sidecars | Seven stakeholder lenses, distributions, incidence, concentrated effects, and tails | `block` |
| `WP-ECO-PRELIM-001` | preliminary ECO slice of `PB-PTH-001` + `PB-REV-001` / `PB-TST-001` sidecars | Six non-additive pathways and a frozen preliminary economic envelope | `block` |
| `WP-DEL-001` | DEL slice of `PB-PTH-001` + `PB-REV-001` / `PB-TST-001` sidecars | Mandatory delivery owner, capacity, milestones, observations, stop/rollback, transition, and realization posture | `block` |
| `WP-ECO-FINAL-001` | final ECO slice of `PB-PTH-001` + `PB-REV-001` / `PB-TST-001` sidecars | Predecessor-linked final economic envelope bound to reviewed DEL | `block` |
| `WP-ADP-001` | ADP slice of `PB-PTH-001` + `PB-REV-001` / `PB-TST-001` sidecars | Immutable adaptive successor and later-feedback request | `block` |
| `WP-REV-001` | `PB-REV-001` + `PB-WS-001` membership integration | Bootstrap-only independent review/evidence-state substrate; no producer pass | `acceptance-ready` |
| `WP-HND-001` | `PB-HND-001` + `PB-REV-001` / `PB-TST-001` sidecars | Current deterministic held/no-pack-emission proof only | `block` |
| `WP-RUN-001` | `PB-RUN-001`, `PB-HND-001` + `PB-REV-001` / `PB-TST-001` sidecars | Fixed-order orchestration with no semantic ownership or new interface | `block` |
| `WP-TST-001` | `PB-TST-001`, assigned `PB-FIX-001`, and `PB-WS-001` membership integration | Bootstrap-only isolated safe fixture/property/model/adversarial harness | `acceptance-ready` |
| `WP-GEN-001` | `PB-GEN-001`, `PB-FIX-001` + `PB-REV-001` / `PB-TST-001` sidecars | Reproducible derivative output custody and fixture inventory; no reverse edge | `block` |
| `WP-DOC-001` | `PB-DOC-001` + `PB-REV-001` / `PB-TST-001` sidecars | Contract/trace documentation and REL no-output proof synchronized to accepted digests | `block` |
| `WP-INT-001` | final fixed 12-boundary graph | Post-GEN/DOC source spine, support isolation, contract/invariant/hold transpose, forbidden-edge, HND no-emission, TERM finiteness, and REL no-output proof | `block` |

Multiple semantic slices in one physical boundary are sequential WPs, not new
packages or dependency edges. The first WP that creates each Rust package also
touches `PB-WS-001` solely to register that member; the exact forward/reverse
transpose therefore contains 71 pairs. This membership integration creates no
semantic or runtime edge and remains in the same accepted WP as the new member.
Splitting preliminary and final ECO prevents a package-level ECO/DEL cycle
while preserving their single fixed contract.

## 5. Stable bootstrap, producer, and integration DAG

The proposed topological order is:

1. `WP-VV-001` completes only when the successor VERIFICATION and VALIDATION
   plans receive independent fixed-point decisions; no WP is accepted by that
   event.
2. A separate decision may then accept and enter `WP-WS-001`. It has no TST
   blocker or proof-input relationship and must exit with accepted empty-
   scaffold evidence before the next step.
3. Separate decisions may then accept and enter bootstrap `WP-REV-001` and
   `WP-TST-001`. The open TST pair is a fail-closed proof input for these two
   WPs only. They create empty control scaffolding and assigned fixture custody
   only, have no CST or other product prerequisite, do not pre-build any
   producer's semantic review, fixture, or evidence delta, and cannot claim
   the TST hold closed until executed independent fixtures and accepted
   digest-bound evidence satisfy its exact closure.
4. `WP-CST-001`, including its producer-owned REV/TST sidecar deltas.
5. `WP-AUT-001`.
6. `WP-ACQ-001` after its exact gates; it deterministically creates and
   registers `PB-DOM-001`. Only then may `WP-LOG-001` and `WP-ALLY-001` enter
   independently after their remaining exact gates.
7. `WP-RDY-001` after AUTH and LOG; `WP-DST-001` after ACQ, LOG, and ALLY.
8. `WP-ECO-PRELIM-001` after RDY, ACQ, LOG, ALLY, and DST.
9. `WP-DEL-001` after accepted preliminary ECO.
10. `WP-ECO-FINAL-001` after accepted DEL; it creates only `ECO[n+1]` from
    preliminary `ECO[n]` and reviewed `DEL[n]`.
11. `WP-ADP-001` after accepted final ECO.
12. `WP-HND-001` as held/no-emission proof while `TBD-HND-001` remains open.
13. `WP-RUN-001` against all current product digests and HND no-emission.
14. `WP-GEN-001` and `WP-DOC-001` from accepted product/runner digests.
15. `WP-INT-001` last, against final GEN and DOC digests.

Every producer owns its exact REV/TST sidecar delta: contract cases, safe
fixtures, expected results, review record, dissent, evidence identities, and
producer/output digest. Bootstrap completion cannot satisfy a producer exit,
and later producers cannot mutate an earlier producer's evidence. A producer
depends on the empty bootstrap scaffolds but its sidecar deltas are atomic
parts of that producer WP, not later REV/TST WPs or back edges.

No semantic producer may be accepted or enter while the TST pair is open.
Even after its future closure, all other blocker, WP-acceptance, representation,
command, fixture, assurance, and evidence conditions remain independent and
mandatory.

The per-generation order remains:

`formed -> frozen -> SOURCE exact-output decision -> independent TEST decision
-> exact authorized consumer or terminal hold/reject`.

The path order remains:

`preliminary ECO[n] -> DEL[n] -> final ECO[n+1] -> ADP[n+1]`.

Later material feedback creates a new preliminary generation with a greater
identity/version and repeats the DAG. No same-version edge, retry, revisit,
receipt return, product-to-bootstrap back edge, or recursive review exists.

HND terminates through ordinary SOURCE/TEST and the minimal non-product
`IF-TERM-001` receipt at the external Taxlane boundary. A bundle change
invalidates the receipt and creates a new generation. REL emits nothing.

`CTRL-HND-EMIT-001` remains outside the current DAG. Activating it requires a
new controlled planning successor, new assignment, all 12 applicable
non-release holds and mapping decisions closed, and a separately accepted WP;
`TBD-REL-001` never becomes an HND-emission prerequisite and continues to
block the separate future release chain. No current digest or no-emission
result transfers emission authority.

## 6. Source spine and safe fixtures

The planned spine is:

`identified public aggregate or marked synthetic -> CST security/SOURCE
admission -> AUT -> {ACQ, LOG, ALLY} -> {RDY, DST} -> preliminary ECO -> DEL
-> final ECO -> ADP -> HND held/no emission -> RUN -> derivative GEN/DOC ->
final INT`.

Each producer output is frozen, re-admitted through SOURCE, independently TEST
reviewed, and only then routed to the exact fixed consumer. Fixtures may contain
only identified public aggregate or clearly marked synthetic content that
cannot reconstruct a person or expose sensitive operational meaning. Every
fixture has source/version/custody, purpose, expected typed posture, reason,
digest, and supersession behavior. Negative fixtures cover prohibited content,
dangerous composition, authority inflation, missing/null/N/A, stale/mismatch,
floor failure, burden erasure, arithmetic, cycle, receipt recursion, HND
emission, REL output, and release claims. Fixtures and golden outputs are never
product truth.

## 7. Ten verification-closure areas

Every WP must disposition all ten areas; required evidence and reasoned N/A
branches are fixed only by accepted VERIFICATION and VALIDATION plans.

- `VCL-01`: exact source identity, allocation, trace, custody, and digest.
- `VCL-02`: positive/negative contract and unauthorized-consumer behavior.
- `VCL-03`: typed state, transition, finite DAG, successor, and invalid edge.
- `VCL-04`: invariant/property and deterministic reproduction coverage.
- `VCL-05`: exact 13-hold propagation, missing/null/N/A, and no default.
- `VCL-06`: prohibited-content, composition security, minimization, and safe
  failure custody.
- `VCL-07`: civilian authority, law, safety/readiness, stakeholder,
  distribution, burden, and non-compensation.
- `VCL-08`: checked accounting, pathways, delivery, realization, HND/TERM,
  Taxlane, REL, and no-authority boundaries.
- `VCL-09`: quality, dependency, support isolation, generated custody, and
  accepted resource bounds.
- `VCL-10`: evidence-state truth, independent review/dissent, validation,
  rollback, compatibility, and historical reproduction.

`VCL-ALL` is exactly `{VCL-01, VCL-02, VCL-03, VCL-04, VCL-05, VCL-06,
VCL-07, VCL-08, VCL-09, VCL-10}`. Each WP row in `WORK_PACKAGES.md`
dispositions `VCL-ALL`; this does not claim evidence exists.

## 8. Eight parliament lanes, domain concurrence, and assurance

The eight adversarial parliament lanes are exact:

- `PAR-CIV`: Civilian Strategy & Force Planner;
- `PAR-RDY`: Operational Readiness Officer;
- `PAR-ACQ`: Acquisition & Industrial-Base Lead;
- `PAR-LOG`: Logistics & Sustainment Lead;
- `PAR-FIN`: Defense Comptroller;
- `PAR-PPL`: Service-Member & Family Advocate;
- `PAR-TST`: Independent Test & Oversight Officer; and
- `PAR-ALLY`: Alliance & Interoperability Strategist.

`PAR-ALL` is the exact set of those eight lanes. Every semantic WP requires
`PAR-ALL`, the exact fixed domain owner/concurrences, Classification &
Operational Security, and Civilian Control/Law/Safety/Readiness. They are
independent, pending, non-compensating, and digest-bound. `WP-WS-001` may use
reasoned N/A only while it remains empty coordination with no semantic member;
any semantic change reclassifies every lane as required. No author self-passes.

## 9. Verification levels and command posture

L0 is changed-surface syntax/type/focused behavior; L1 is full-workspace
quality, documentation, static, dependency/license/advisory, and regression;
L2 is source-spine, all-contract, invariant/property, state/model,
adversarial, hold, HND/TERM, REL, and final integration evidence.

The accepted empty workspace and pinned toolchain now exist. `WP-WS-001`
commands and evidence are complete; command slots for every later WP remain
unbound until that exact WP is accepted. `WP-VV-001` product execution was N/A
and its planning checks are complete. A future WP cannot enter with a
placeholder or unavailable command.

## 10. Branch, change, rollback, and integration discipline

- One clean child-repository worktree and branch may be created only after an
  exact WP is accepted. A commit contains one WP and its producer-owned
  sidecar/evidence delta; no TRACKER pointer change is mixed in.
- Child commit/push precedes a separately authorized TRACKER pointer update.
  Neither action implies release.
- Any changed controlled identity, hold, owner, consumer, method, command,
  fixture, bound, assurance decision, or predecessor digest invalidates
  affected evidence and reopens upstream planning.
- Rollback is a reviewed revert of the exact WP commit with failed evidence and
  successor history retained. History and evidence are never erased.
- Integration accepts only topologically eligible exact predecessor digests.
  `WP-INT-001` is last and cannot bind a stale pre-GEN/DOC state.

## 11. Common entry and exit gates

Every implementation WP requires before entry:

1. accepted digest-bound VERIFICATION and VALIDATION fixed points;
2. a newly assigned and independently accepted exact WP revision;
3. every hold in the WP's exact section 3.3.2 blocker set closed by its fixed
   owner; only holds in that row's disjoint proof-input set may remain open,
   solely for the stated fail-closed or no-output proof;
4. exact representation, toolchain/dependencies, finite resource bounds,
   concrete commands, safe fixtures, expected results, evidence destinations,
   compatibility and rollback fixed;
5. all exact semantic owners, applicable `PAR-ALL` lanes, and both assurance
   decisions current and bound to the same digest; and
6. a clean isolated branch/worktree with no unrelated or TRACKER change.

Every implementation WP exits only when its exact scope and producer-owned
sidecar delta are complete; required L0/L1/L2 and all ten VCL areas close with
present digest-bound evidence; all applicable parliament/domain/assurance
lanes pass; independent review has no unresolved major/critical finding; trace
transposes are zero-orphan; rollback is proven; and authority remains bounded.

`WP-WS-001` satisfied its exact entry and exit criteria. `WP-REV-001` and
`WP-TST-001` may now seek separate exact acceptance. No semantic hold is closed
and no waiver is accepted.

The section 3.3.2 transpose supplies the sole prospective bootstrap exception:
WS has no TST relationship; REV and TST may carry the open TST pair only as a
proof input in their pure bootstrap scopes after accepted WS exit. This does
not waive criteria 1, 2, 4, 5, or 6, does not waive any other blocker, and does
not provide exit evidence in advance.

## 12. Readiness conclusion

The bootstrap-successor decomposition contains completed planning and WS
baselines, two acceptance-ready bootstrap WPs, and 16 blocked later WPs. Future HND
emission is not a WP and remains deferred behind new authority and all 12
applicable non-release holds. `TBD-REL-001` remains separately release-blocking
and is never an HND-emission prerequisite.

Semantic implementation readiness is `blocked`. All 13 holds remain open; all
methods, commands, evidence, tools, bounds, and fixtures remain absent. HND
emits nothing, TERM is finite and non-product, REL emits nothing, Taxlane is
external, and BASTION gains no operational, force, procurement, budget,
allocation, rate, release, or official authority.
