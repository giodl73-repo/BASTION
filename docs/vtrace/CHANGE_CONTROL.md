# BASTION Change Control

## Scope

Repo: BASTION

Assignment: `ASG-BASTION-TST-GOVERNANCE-001`

State: **review-ready governance tailoring; not a fixed point or downstream
authorization**.

This file controls the original stage-term tailoring decision needed to remove
the governance deadlock recorded as `BA-DES-M03` and one prospective bootstrap
successor correction. It does not edit, supersede,
silently reinterpret, or retroactively change any fixed upstream artifact. It
does not close `TBD-TST-001` or `SPEC-UNK-TST-001`, select any held method, or
create verification, validation, implementation, work-package, handoff,
Taxlane, release, budget, rate, procurement, official-use, or operational
authority.

## Controlled inputs

| Fixed or frozen input | SHA-256 |
|---|---|
| `docs/vtrace/MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| `docs/vtrace/CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| `docs/vtrace/REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| `docs/vtrace/SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| `docs/vtrace/ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| `docs/vtrace/PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| `docs/vtrace/INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| Frozen pre-tailoring `docs/vtrace/DESIGN.md` | `268293f8758a64c7dc7d3453c078d26a87cd764da4a8cf63567d9eb046ef6b8d` |

## Change log

| Change ID | Date | Reason | Affected IDs | Decision | Reviewer / lens | Trace updates |
|---|---|---|---|---|---|---|
| `CHG-BA-TST-001` | 2026-07-28 | Resolve the circular stage reading in which the exact TST method is destined for a Verification plan while a literal unscoped reading of “fixed point and every downstream stage” would prohibit authoring that planning artifact. | Exact inventory below, including `BA-DES-M03`, TST/VTR requirements, specifications and planned verification identities, REV allocation, TEST/TRACE contracts, and stage-status/gate text | Controlled prospective stage-term tailoring proposed and review-ready; author remediation complete, independent convergence required | Independent Test & Oversight; Role review steward; BASTION maintainer/stage controller; Scope, Citation, Numeracy, Civilian Control/Law/Safety/Readiness, Classification/Operational Security, and Methodology Panel lenses | Bind successor DESIGN and Pulse 07 to this exact change digest; any later VERIFICATION artifact must cite this change and remain planning-only until separately accepted |
| `CHG-BA-TST-BOOT-002` | 2026-07-29 | Remove the remaining circular implementation-planning reading in which the TST held pair blocks entry to the pure REV/TST scaffolds needed to execute the independent bootstrap fixtures that alone can close it. | `TBD-TST-001`; `SPEC-UNK-TST-001`; `WP-WS-001`; `WP-REV-001`; `WP-TST-001`; exact hold blocker/proof transposes; PLAN/WP and V&V successor digests; Pulses 09/10 | Prospective governance-only bootstrap allocation proposed and review-ready; no current acceptance, entry, execution, evidence, or closure | Same independent test, review, stage-controller, scope, citation, numeracy, assurance, and methodology lenses as `CHG-BA-TST-001` | Bind review-ready PLAN/WP successors, superseding Pulse 09 candidate record, rebound V&V successors, and Pulse 10; preserve all other allocations |
| `CHG-BA-WS-MEM-003` | 2026-07-29 | Reconcile accepted empty `WP-WS-001` exit with deterministic later member creation. | `PB-WS-001`; first-member WPs; PLAN/WP/status ledgers | Assign configuration-only membership integration to the exact first WP for each Rust boundary; require ACQ before LOG/ALLY so ACQ deterministically creates `PB-DOM-001` | Independent test, review, stage-controller, scope, security, civilian-control, and VTRACE lenses | Bind successor PLAN/WP digests; no semantic/runtime edge, hold closure, HND emission, release, or authority |

## Exact impact inventory

| Impact class | Exact identities / artifact text | Controlled effect |
|---|---|---|
| Unknowns | `TBD-TST-001`; `SPEC-UNK-TST-001` | Wording, owner, destination, closure, and hold remain unchanged and open. |
| Requirements | `BASTION-REQ-TST-001..006`; `BASTION-REQ-VTR-001..002` | Stage-term application only; normative product/review behavior unchanged. |
| Specifications | `SPEC-TST-001..006`; `SPEC-VTR-001..002` | Stage-term application only; target behavior and inherited holds unchanged. |
| Planned verification identities | `VER-TST-001..006`; `VER-VTR-001..002` | May be described in a planning-only VERIFICATION artifact; none exists as executed or accepted evidence through this change. |
| Logical / physical allocation | `ARC-REV-001`; `PB-REV-001` | Ownership and package direction unchanged; no edge added. |
| Contracts | `CONTRACT-TEST-001`; `CONTRACT-TRACE-001` | Consumer, posture, blocker, and failure semantics unchanged. |
| Design decisions | `DES-TEST-001`; `DES-TRACE-001` | Exact hold sets and product/review procedures unchanged; only stage eligibility is clarified. |
| Finding and stage text | `BA-DES-M03`; DESIGN migration/readiness text; Pulse 07 finding/stage disposition; future VERIFICATION eligibility/gate text | Replaces deadlock with the prospective planning-only path and records no downstream pass. |
| Terminal impact check only | `IF-TERM-001`; `CONTRACT-HND-001`; `ARC-HND-001`; `PB-HND-001`; `DES-HND-001` | Checked because TEST participates in the terminal gate. No semantic input, consumer, receipt, HND, Taxlane, or graph edge changes; the finite terminal branch remains exact. |

Every one of the 13 inherited `TBD-*` / `SPEC-UNK-*` pairs remains open. No
non-TST hold receives a tailored meaning or exception.

## `CHG-BA-TST-001` decision

### Preserved literal controls

The following upstream statements remain literal, open, and unchanged:

- `TBD-TST-001` governs the exact evidence tiers, reproduction criteria,
  reviewer conflicts, severity/disposition schema, and convergence fixtures;
- its owner remains the Independent Test and Oversight Officer;
- its destination remains the Verification plan;
- its closure condition remains positive and negative fixtures proving digest
  binding, independence, failure retention, finding completeness, assurance
  presence, and zero-major convergence;
- its hold behavior remains “Hold fixed point and every downstream stage”;
- `SPEC-UNK-TST-001` remains the paired unknown and retains “Hold fixed point
  and downstream stage”; and
- all fixed dependencies, owners, consumers, and hold routes in REQUIREMENTS,
  SPECIFICATION, ARCHITECTURE, PACKAGE_BOUNDARIES, and INTERFACES remain
  unchanged.

No sentence above is declared satisfied by this change record.

### Controlled stage-term tailoring

Prospectively, after independent acceptance of the exact CHANGE_CONTROL and
bound DESIGN digests, BASTION stage governance applies the terms in the
preserved TST hold as follows:

1. **Product-evidence scope.** “Fixed point” and “downstream stage” in the TST
   hold apply to product-evidence convergence, verification or validation
   claims, implementation, accepted work packages, delivery/readiness claims,
   handoff, Taxlane use, release, and official or operational use. An open TST
   hold blocks every such promotion.
2. **Governance-only fixed points.** Repo-local fixed points for MISSION,
   CONOPS, REQUIREMENTS, SPECIFICATION, ARCHITECTURE, PACKAGE_BOUNDARIES,
   INTERFACES, DESIGN, and change-control records are governance decisions over
   frozen planning artifacts. They are not product-evidence convergence and do
   not claim that the TST methods, fixtures, verification, or validation have
   passed. Earlier recorded left-side fixed points therefore remain historical
   and are neither revoked nor rewritten.
3. **Planning-only VERIFICATION exception.** A bounded VERIFICATION artifact
   may be authored before the DESIGN fixed point solely to propose the method
   whose fixed destination is the Verification plan. It is a pre-code plan,
   not executed evidence, verification success, validation, convergence,
   readiness, acceptance, or downstream promotion.
4. **No favorable defaults.** Until separately accepted evidence closes the
   exact unknown, evidence tiers, reproduction criteria, reviewer-conflict
   rules, severity/disposition semantics, convergence methods, and fixtures
   remain open. Missing or disputed meaning remains held. No author, reviewer,
   plan, DESIGN fixed point, or change-control record supplies a favorable
   value by implication.
5. **Promotion remains gated.** `TBD-TST-001` and `SPEC-UNK-TST-001` remain
   conjunctive gates on `DES-TEST-001`, `DES-TRACE-001`, the exact dependent
   TST/VTR requirements and specifications, product-evidence convergence, and
   every affected later promotion.
6. **No authority inflation.** A DESIGN fixed point grants no Verification-plan
   acceptance, executed verification, validation claim, implementation or
   work-package authority, delivery/readiness finding, HND pack, Taxlane
   admission, allocation, rebalance, rate, release, procurement, budget,
   official-use, or operational authority.

This tailoring distinguishes stage kinds; it does not alter the upstream
words, narrow their product-evidence protection, or close their method. It is
prospective only: it does not change the meaning or disposition of any earlier
fixed point, finding, pulse, or artifact digest.

## Stage DAG

Each arrow is an eligibility edge, not evidence of passage:

```text
fixed governance-only left side through INTERFACES
  -> frozen DESIGN candidate + CHG-BA-TST-001
      -> independent digest-bound CHG + DESIGN review and acceptance
          -> bounded planning-only VERIFICATION draft may be authored
              -> later governance-only DESIGN fixed point may be recorded
                 if its independent convergence is recorded

bounded planning-only VERIFICATION draft
  + governance-only DESIGN fixed point
  + independent verification-plan review
      -> accepted Verification plan, only if its own gate passes
          -> separately accepted TST method/fixture evidence may close
             TBD-TST-001 / SPEC-UNK-TST-001
              -> product-evidence verification and validation may execute
                  -> later promotion only when every other applicable gate
                     and hold also passes
```

The planning-only exception is unavailable until independent digest-bound
review accepts the exact `CHG-BA-TST-001` and bound DESIGN pair with no
unresolved critical or major finding. The permitted draft may still precede
the later administrative recording of the governance-only DESIGN fixed point;
it cannot precede acceptance of the tailoring pair.

There is no edge from acceptance of the tailoring pair, a planning-only
VERIFICATION draft, or DESIGN fixed point directly to executed evidence, work
package, code, delivery/readiness, HND, Taxlane, release, or official action. A
plan that contains executed-evidence claims is outside this exception and
fails closed.

## Remediation finding dispositions

| Finding | Author disposition |
|---|---|
| `BA-CHG-M01` | Corrected the controlled INTERFACES SHA-256 to the complete 64-character digest and required exact digest validation before governance review. |
| `BA-CHG-M02` | Reordered the stage DAG so independent digest-bound acceptance of the exact CHANGE_CONTROL + DESIGN pair is a strict predecessor of any planning-only VERIFICATION draft; the draft may precede only the later DESIGN fixed-point recording. |

Both dispositions require independent recheck and confer no author acceptance.

## Rationale

The fixed TST unknown names the Verification plan as the place where its exact
method is to be resolved. Permitting a bounded planning artifact is the minimum
governance action that makes that destination reachable without treating a
plan as proof. Separating governance-only artifact convergence from
product-evidence convergence preserves the protective hold, preserves earlier
history, and keeps all product and downstream claims unavailable until the
method and evidence are independently accepted.

## Rejected alternatives

| Alternative | Decision | Reason |
|---|---|---|
| Literal deadlock: prohibit even a planning-only Verification artifact until TST is closed | rejected | TST's fixed closure destination would be unreachable, so the held method could never be proposed for review. |
| Silent reinterpretation inside DESIGN or a future Verification artifact | rejected | It would change stage meaning without a visible, reviewable governance decision and could be mistaken for hold closure. |
| Rewrite REQUIREMENTS, SPECIFICATION, earlier pulses, or other upstream history | rejected | It would destroy digest-bound history and exceed this bounded lease; the fixed upstream wording and prior fixed points remain unchanged. |
| Treat a draft plan, DESIGN fixed point, reviewer pass, or this change as accepted TST evidence | rejected | A planning/governance record cannot satisfy the still-open evidence, method, and fixture closure conditions. |
| General exception for other holds or downstream artifacts | rejected | The tailoring is limited to the named TST planning deadlock and creates no reusable bypass. |

## Reviewer lenses and required disposition

| Reviewer / lens | Required challenge |
|---|---|
| Independent Test and Oversight Officer | Confirm that every held method remains open, no planned fixture is represented as executed, and the future plan can be falsified. |
| Role review steward | Confirm digest binding, independence, finding severity/disposition, dissent, closure, and zero-unresolved-major governance for this change. |
| BASTION maintainer / stage controller | Confirm the stage DAG has no implementation, work-package, evidence, or handoff bypass. |
| Scope Keeper | Confirm this is stage tailoring only and creates no official, policy, rate, allocation, or release claim. |
| Citation Auditor | Confirm every affected ID, literal hold, digest, and historical fixed point remains attributable and unchanged. |
| Numeracy Checker | Confirm no tolerance, threshold, evidence tier, reproduction rule, or convergence measure was selected. |
| Civilian Control, Law, Safety & Readiness | Confirm governance advancement cannot waive authority, safety, readiness, resilience, or other non-waivable floors. |
| Classification & Operational Security | Confirm planning and future fixtures remain public, aggregate, unclassified, non-operational, and non-reconstructive. |
| Methodology Panel | Challenge the deadlock resolution, falsifiability, plan/evidence distinction, and absence of favorable defaults. |

Independent review must return an exact digest-bound disposition. Author
remediation is not independent acceptance.

## Reopen and invalidation triggers

Reopen `CHG-BA-TST-001`, DESIGN, and the affected stage trace if any of the
following occurs:

- an upstream affected artifact, literal TST hold, owner, destination, closure
  condition, dependency, consumer, or fixed-point record changes;
- this change or its bound DESIGN digest changes without successor control;
- the planning-only exception is applied outside a bounded VERIFICATION plan
  or to any other TBD/SPEC-UNK;
- a VERIFICATION draft claims executed evidence, method acceptance,
  verification/validation success, readiness, convergence, or a favorable
  default;
- DESIGN fixed point is used to imply Verification-plan acceptance, work
  package, implementation, HND, Taxlane, release, official, or operational
  authority;
- any open TST method is defaulted, omitted, weakened, or declared closed
  without its exact independent evidence;
- a required independent reviewer, assurance lens, dissent, finding,
  destination, closure, or digest bond is absent; or
- a later VTRACE governance rule conflicts with this tailored stage DAG.

Any trigger fails closed. It does not retroactively rewrite the controlled
history.

## `CHG-BA-TST-BOOT-002` prospective bootstrap successor

### Exact controlled decision

The original change permits planning the TST method but does not by itself
make the method executable. This successor proposes the minimum acyclic
allocation needed for later independent execution:

1. `TBD-TST-001` / `SPEC-UNK-TST-001` remains open and blocks exactly
   `WP-PRODUCT` plus `{WP-GEN-001, WP-DOC-001, WP-INT-001}`: 16 current
   semantic, derivative, documentation, and integration WPs.
2. The same held pair is an exact fail-closed proof input, not a blocker, for
   the pure bootstrap scopes of `{WP-REV-001, WP-TST-001}` only. Those WPs may
   observe absence, exercise accepted safe fixtures, and record independent
   evidence-state/harness behavior; they may not supply favorable domain
   meaning or producer evidence.
3. `WP-WS-001` has neither a TST blocker nor a TST proof-input dependency. It
   may be considered first only after the V&V planning successors reach
   independent fixed points and a separate exact WP acceptance decision is
   made.
4. After accepted WS exit evidence, `WP-REV-001` and `WP-TST-001` may each be
   considered through separate acceptance and entry decisions. Their entry is
   bootstrap-only and cannot pre-accept any producer sidecar.
5. The TST pair closes only after those accepted scaffolds execute the exact
   positive and negative bootstrap fixtures, bind the resulting evidence and
   environment digests, retain failures/conflicts/dissent, receive required
   independent assurance, and converge with zero unresolved critical or major
   finding. Until then every semantic/product WP remains blocked.

The resulting exact current transpose is 134 blocker pairs and 10 proof-input
pairs. It changes only the three TST/WP relationships above: the former TST
blocker pairs for WS, REV, and TST are removed; REV and TST receive TST as a
proof input; WS receives no replacement. All other TST blocker relationships,
all allocations for the other 12 held pairs, the 12-hold deferred HND-emission
control, and the separate REL release block remain unchanged.

### Eligibility DAG and non-authority

Each arrow is future eligibility, never present acceptance or evidence:

```text
independently fixed successor VERIFICATION + VALIDATION plans
  -> separately accept and enter empty WP-WS-001
      -> accepted WS exit evidence
          -> separately accept and enter pure WP-REV-001 / WP-TST-001 bootstrap
              -> execute independent bootstrap fixtures and accept evidence
                  -> possibly close TBD-TST-001 / SPEC-UNK-TST-001
                      -> semantic WP eligibility remains subject to every
                         other exact blocker and a separate WP acceptance
```

This author record does not perform any arrow. It accepts no plan or WP,
creates no workspace/code/fixture/evidence, closes no hold, and authorizes no
producer, HND emission, Taxlane action, release, official use, budget,
allocation, rate, procurement, or operational action.

### Successor review and invalidation

Independent review must verify the exact 134/10 forward and reverse sets, the
unchanged 13 held pairs and non-TST allocations, the 298-source and 20-WP
inventories, and the absence of any producer or authority bypass. A changed
set, implied current acceptance, plan-as-evidence statement, missing fixture
execution, or semantic use of bootstrap output invalidates this successor and
holds the entire path.

## Open changes

| Change ID | Blocking question | Owner | Due / trigger |
|---|---|---|---|
| `CHG-BA-TST-001` | Will independent digest-bound review accept this stage-term tailoring with zero unresolved critical or major findings? | BASTION maintainer and Role review steward, with Independent Test and Oversight concurrence | Before DESIGN fixed-point disposition or any use of the planning-only VERIFICATION exception |
| `CHG-BA-TST-BOOT-002` | Will independent digest-bound review accept the exact 134-blocker/10-proof bootstrap allocation without a producer, evidence, hold, or authority bypass? | BASTION maintainer and Role review steward, with Independent Test and Oversight concurrence | Before recording any successor PLAN/WP or V&V fixed point or considering WS acceptance |

## Disposition

`CHG-BA-TST-001` remains the historical bounded stage-tailoring decision.
`CHG-BA-TST-BOOT-002` is a **review-ready prospective successor candidate**,
subject to independent convergence. It closes no TBD or SPEC-UNK, accepts no
WP, records no evidence, and authorizes no downstream action.
