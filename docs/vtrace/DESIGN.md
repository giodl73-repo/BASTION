# BASTION Detailed Conceptual Design

## Status and controlled inputs

Repo: BASTION

Assignment: `ASG-BASTION-DESIGN-001`

Design state: **review-ready encoding-neutral conceptual design; not a fixed
point**.

This design allocates the fixed requirements and specifications to the fixed
architecture, package boundaries, and thirteen interface contracts. It makes
semantic processing, state, failure, review, and handoff behavior explicit. It
does not select a language, crate, package, schema, API, serialization, file
format, database, transport, command, dependency, runtime, deployment, test
framework, quantitative threshold, operational method, legal interpretation,
policy choice, or release mechanism. Planned boundaries remain plans, not
implementation authorization.

| Controlled input | SHA-256 |
|---|---|
| `docs/vtrace/MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| `docs/vtrace/CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| `docs/vtrace/REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| `docs/vtrace/SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| `docs/vtrace/ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| `docs/vtrace/PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| `docs/vtrace/INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |

Controlled governance companion:
`docs/vtrace/CHANGE_CONTROL.md` / `CHG-BA-TST-001`, SHA-256
`147b67601e8e992332f47432a188d2c62d9b2e8f757d30ca4141dc94421c668b`.
This binding tailors stage terms only. It changes no requirement,
specification, component, package, contract, hold, domain procedure, consumer,
or authority semantic in this DESIGN. A change to that digest invalidates this
governance binding and requires a controlled successor review.

## Scope

The design covers public, aggregate, unclassified, non-operational research
from source custody through a finite, externally gated Taxlane handoff. It
preserves nulls, uncertainty, dissent, stakeholder incidence, delivery
evidence, and all non-waivable floors. It excludes classified or controlled
information, person-level service data, sensitive operational data, targeting,
operational planning, exploitable vulnerability content, official decisions,
procurement, force employment, budget or rate setting, implementation, and
public release.

All product or material outputs, including SOURCE transformations and AUTH
outputs, must be re-admitted through `CONTRACT-SOURCE-001` before semantic
consumption. `CONTRACT-REL-001` emits no output. The sole non-product exception
is the minimal `IF-TERM-001` governance receipt defined within
`CONTRACT-TEST-001`; it is not a fourteenth contract, does not enter HND, and
cannot carry product content.

## Design decision summary

Ranges below are inclusive and allocate every one of the 98 fixed requirements
and its one-to-one specification. Evidence names are planned verification
classes only; no fixture or test is implemented or executed.

| Decision ID | Semantic decision | Requirement / specification allocation | Component / package / contract | Rationale and rejected alternative | Planned evidence |
|---|---|---|---|---|---|
| `DES-SOURCE-001` | Make custody, claim, derivation, safe composition, security admission, supersession, and exact-output re-admission one fail-closed gate. SOURCE transformations re-enter the gate. | `BASTION-REQ-SCP-001..005`, `BASTION-REQ-SCP-009..010`, `BASTION-REQ-SRC-001..008`; `SPEC-SCP-001..005`, `SPEC-SCP-009..010`, `SPEC-SRC-001..008` | `ARC-SRC-001` / `PB-CST-001` / `CONTRACT-SOURCE-001` | Prevents self-exemption and context laundering; rejects trust-by-origin or admission inherited across transforms. | Custody, prohibited-content, composition, stale-context, supersession, and self-exemption fixtures. |
| `DES-AUTH-001` | Produce an immutable, bounded civilian authority and public mission abstraction, then re-admit its output through SOURCE. | `BASTION-REQ-SCP-006..008`; `SPEC-SCP-006..008` | `ARC-AUTH-001` / `PB-AUT-001` / `CONTRACT-AUTH-001` | Prevents analysis from manufacturing authority; rejects mutable or silently broadened manifests. | Ambiguity, expiry, prohibited-decision, silent-broadening, and SOURCE-bypass fixtures. |
| `DES-RDY-001` | Keep readiness, safety, resilience, surge, recovery, distributions, and floors distinct; produce result, null, held, or independently reviewed N/A without favorable defaulting. | `BASTION-REQ-RDY-001..007`; `SPEC-RDY-001..007` | `ARC-RDY-001` / `PB-DOM-001` / `CONTRACT-RDY-001` | Spending and inventory are not employable readiness; rejects a composite readiness score or economics waiver. | Facet completeness, denominator, degraded-path, floor, tail, null, and N/A fixtures. |
| `DES-ACQ-001` | Evaluate acquisition, capacity, competition, concentration, qualification, workforce, transition, learning, and six commonality facets separately. | `BASTION-REQ-ACQ-001..008`; `SPEC-ACQ-001..008` | `ARC-ACQ-001` / `PB-DOM-001` / `CONTRACT-ACQ-001` | Benefits and brittle concentration can coexist; rejects purchase-price or commonality-only optimization. | Facet, capacity, small-supplier, transition, concentration, schedule, null, and N/A fixtures. |
| `DES-LOG-001` | Preserve stock, condition, custody, maintenance, repair distributions, workload, lifecycle, degraded recovery, and readiness relationships. | `BASTION-REQ-LOG-001..008`; `SPEC-LOG-001..008` | `ARC-LOG-001` / `PB-DOM-001` / `CONTRACT-LOG-001` | Prevents purchase-price-only or average-repair reasoning; rejects unsafe operational detail and uncensored tails. | Boundary reconciliation, censoring, tail, workload, availability, degraded-recovery, and safety fixtures. |
| `DES-ALLY-001` | Keep commitments, sovereignty/control, compatibility, standards, partner capacity, logistics, and separated burden ledgers explicit. | `BASTION-REQ-ALLY-001..006`; `SPEC-ALLY-001..006` | `ARC-ALLY-001` / `PB-DOM-001` / `CONTRACT-ALLY-001` | Domestic savings cannot silently become partner burden; rejects blended ledgers and inferred partner posture. | Authority, compatibility, sovereignty, partner-capacity, separated-ledger, burden, and null fixtures. |
| `DES-DST-001` | Evaluate each stakeholder burden, concentrated effect, distribution, incidence, and tail separately across accepted horizons. | `BASTION-REQ-DST-001..005`; `SPEC-DST-001..005` | `ARC-DST-001` / `PB-DOM-001` / `CONTRACT-DST-001` | Totals and averages can erase harm; rejects composite welfare or unowned burden shifts. | Stakeholder-lens, baseline, denominator, horizon, tail, burden-shift, null, and N/A fixtures. |
| `DES-ECO-001` | Maintain six economic pathways, federal fiscal meanings, gross-to-net logic, uncertainty, timing, overlap, peer limits, ownership, and preliminary/final predecessor bonds. | `BASTION-REQ-ECO-001..011`, `BASTION-REQ-ECO-014..016`, `BASTION-REQ-ECO-018..020`; `SPEC-ECO-001..011`, `SPEC-ECO-014..016`, `SPEC-ECO-018..020` | `ARC-ECO-001` / `PB-PTH-001` / `CONTRACT-ECO-001` | Prevents gross opportunity from becoming booked savings and prevents non-additive totals; rejects one composite savings number. | Path separation, units, horizon, downside, peer, overlap, fiscal-field, false-savings, and version-bond fixtures. |
| `DES-ADP-001` | Produce an adaptive successor only from a final economic envelope and reviewed delivery bond; later feedback starts a new successor cycle. | `BASTION-REQ-ECO-012..013`, `BASTION-REQ-ECO-017`; `SPEC-ECO-012..013`, `SPEC-ECO-017` | `ARC-ADP-001` / `PB-PTH-001` / `CONTRACT-ECO-001` | Adaptation is part of controlled economics, not a new public contract; rejects in-place mutation and same-version cycles. | Predecessor, trigger, lifecycle, immutable-successor, changed-feedback, and cycle fixtures. |
| `DES-TEST-001` | Independently inspect frozen exact digests, preserve findings and dissent, return ordinary findings to the accountable producer, and keep terminal receipt behavior finite. | `BASTION-REQ-TST-001..006`; `SPEC-TST-001..006` | `ARC-REV-001` / `PB-REV-001` / `CONTRACT-TEST-001` | Review cannot be advocacy or self-approval; rejects findings routed to unrelated components and recursive terminal review. | Independence, digest, falsification, finding retention, dissent, convergence, and terminal mutation fixtures. |
| `DES-TRACE-001` | Keep complete source-to-evidence and stage-state links and fail advancement on orphan, stale, missing, or premature links. | `BASTION-REQ-VTR-001..003`; `SPEC-VTR-001..003` | `ARC-REV-001` / `PB-REV-001` / `CONTRACT-TRACE-001` | Prevents ceremonial trace tables; rejects implementation inferred from planned evidence. | Orphan, stale-link, coverage, stage-state, finding-ledger, and premature-work fixtures. |
| `DES-DEL-001` | Make a delivery record mandatory for every preliminary economic envelope and preserve owner, milestones, observations, stops, rollback, transition, and realization evidence. | `BASTION-REQ-DEL-001..007`; `SPEC-DEL-001..007` | `ARC-DEL-001` / `PB-PTH-001` / `CONTRACT-DEL-001` | Analysis is not realizable savings without delivery proof; rejects optional delivery and paper savings. | Owner, resource, cadence, observed-baseline, stop, rollback, burden, realization, and successor fixtures. |
| `DES-HND-001` | Assemble only an exact frozen, source-admitted, independently reviewed candidate bundle, while all current handoff unknowns keep pack emission held. | `BASTION-REQ-HND-001..007`; `SPEC-HND-001..007` | `ARC-HND-001` / `PB-HND-001` / `CONTRACT-HND-001` | Makes Taxlane the exclusive external admission boundary; rejects inferred admission, direct product release, or receipt return into HND. | Completeness, identity, separated-ledger, overlap, security, rejection, unchanged-bundle, and no-pack fixtures. |
| `DES-REL-001` | Record the closed no-release posture and emit no artifact. | `BASTION-REQ-REL-001..003`; `SPEC-REL-001..003` | `ARC-REL-001` / `PB-DOC-001` / `CONTRACT-REL-001` | The repository has no release authority; rejects treating documentation or a terminal handoff as publication. | No-output, no-consumer, unauthorized-release, mosaicing, and context-retention fixtures. |

Allocation audit: **98 requirements, 98 one-to-one specifications, 13
components, and all 13 fixed contracts** are allocated exactly once at the
primary decision level. `ARC-ADP-001` deliberately uses the fixed
`CONTRACT-ECO-001`; `IF-TERM-001` deliberately remains a branch of
`CONTRACT-TEST-001`.

## Physical boundary allocation

This table covers all twelve planned boundaries without creating physical
packages.

| Boundary | Conceptual design responsibility |
|---|---|
| `PB-WS-001` | Future workspace governance only; owns no domain semantics. |
| `PB-CST-001` | Custody, provenance, claim, derivation, and SOURCE admission concepts. |
| `PB-AUT-001` | Civilian authority and public mission-abstraction concepts. |
| `PB-DOM-001` | RDY, ACQ, LOG, ALLY, and DST domains remain logically separate despite co-location. |
| `PB-PTH-001` | ECO, ADP, and DEL sequence with explicit direction and no same-version cycle. |
| `PB-REV-001` | Independent TEST, trace, finding, convergence, and terminal-governance concepts. |
| `PB-HND-001` | Candidate assembly and frozen-bundle identity; no Taxlane admission authority. |
| `PB-RUN-001` | Future orchestration only; owns no policy or semantic rule. |
| `PB-DOC-001` | No-release posture and explanatory material; `REL` emits nothing. |
| `PB-TST-001` | Future verification support only; planned evidence is not product evidence. |
| `PB-FIX-001` | Future non-sensitive fixtures only; no fixture currently exists. |
| `PB-GEN-001` | Future generated sink only; generated material cannot become authoritative input. |

The ten fixed non-functional specifications are allocated below. Their
cross-cutting application is not narrowed by the primary component named in
the fixed architecture.

| Specification | Primary component / package / contract allocation | Design enforcement |
|---|---|---|
| `SPEC-NF-001` | `ARC-SRC-001` / `PB-CST-001` / `CONTRACT-SOURCE-001` | Prohibited content cannot be ingested, retained, derived, or emitted; compositional safety remains held. |
| `SPEC-NF-002` | `ARC-AUTH-001` / `PB-AUT-001` / `CONTRACT-AUTH-001` | No transition broadens civilian authority or silently changes mission or risk. |
| `SPEC-NF-003` | `ARC-RDY-001` / `PB-DOM-001` / `CONTRACT-RDY-001` | A missing or failed readiness/safety floor blocks candidate, savings, delivery, and handoff promotion. |
| `SPEC-NF-004` | `ARC-RDY-001` / `PB-DOM-001` / `CONTRACT-RDY-001`; constrains LOG and DST | Central values cannot replace distributions, repair tails, degraded cases, or concentrated effects. |
| `SPEC-NF-005` | `ARC-ECO-001` / `PB-PTH-001` / `CONTRACT-ECO-001` | Units, horizons, price bases, account measures, parties, and overlap reconcile before combination. |
| `SPEC-NF-006` | `ARC-ECO-001` / `PB-PTH-001` / `CONTRACT-ECO-001` | Pathways and non-cash outcomes remain non-additive and are not automatically converted. |
| `SPEC-NF-007` | `ARC-REV-001` / `PB-REV-001` / `CONTRACT-TEST-001`, `CONTRACT-TRACE-001`; constrains all target components | Missing is never zero; N/A requires the fixed rationale, alternative boundary where required, and independent review. |
| `SPEC-NF-008` | `ARC-REV-001` / `PB-REV-001` / `CONTRACT-TEST-001`, `CONTRACT-TRACE-001` | Provenance, logical identity, ordering, and supersession are deterministic and historical. |
| `SPEC-NF-009` | `ARC-REV-001` / `PB-REV-001` / `CONTRACT-TEST-001`, `CONTRACT-TRACE-001` | Admission, review, and handoff reject a stale digest or context. |
| `SPEC-NF-010` | `ARC-AUTH-001` / `PB-AUT-001` / `CONTRACT-AUTH-001`; constrains all contracts | Successful review cannot create operational, procurement, budget, Taxlane, allocation, rate, implementation, official-use, or release authority. |

The allocation therefore covers **108 fixed specification items**: 98
one-to-one requirement specifications and 10 non-functional specifications.

## Algorithms and logic

### Procedure A — universal product admission

For any candidate product or material output `X`:

1. Freeze the semantic identity, version, exact digest, context, producer,
   inputs, derivation, audience, and applicable expiry posture of `X`.
2. Reject or hold before semantic use if prohibited content, missing custody,
   missing authority, unsafe composition, stale context, absent owner,
   unresolved controlling hold, or unsupported posture is present.
3. Submit the exact frozen `X` to `CONTRACT-SOURCE-001`, even when `X` was
   produced by SOURCE itself or by AUTH.
4. Bind the SOURCE decision to the exact identity, digest, context, joins,
   derivation, audience, and expiry. A decision for any other bond is unusable.
5. On SOURCE `hold`, `reject`, or admission `stale`, stop the affected branch
   and invalidate bound downstream promotion. Preserve only permitted
   non-reconstructive governance metadata.
6. On SOURCE `pass`, submit the same frozen, exact-output-admitted `X` to
   independent TEST before any authorized semantic consumer receives it.
7. On TEST `finding`, `defer`, or blocking `hold`, stop promotion and return the
   control disposition to the exact accountable producer or promotion
   controller. Any product correction is a new successor and restarts step 1.
8. Only after TEST `pass` may the exact unchanged `X` be routed to the exact
   authorized semantic consumers fixed by its contract.

Steps 1 through 8 apply to every promotable product or material output,
including SOURCE transformations, AUTH, each domain result, preliminary and
final ECO, ADP, DEL, ordinary TEST-authored product/material reports, TRACE,
and an HND candidate. A TEST control disposition is gate metadata, not a
promotable semantic product. The only exceptions are the unchanged minimal
non-product `IF-TERM-001` receipt and `REL`, which emits no output.

Semantic equality means equality of every fixed bond required by the contract,
not merely byte equality. Any later encoding must define canonical comparison
without weakening these bonds.

### Procedure B — domain evaluation and composition

1. Resolve exact SOURCE-admitted inputs and the applicable SOURCE-admitted AUTH
   successor.
2. Evaluate RDY, ACQ, LOG, ALLY, and DST in their own owner domains. Co-location
   in `PB-DOM-001` grants no direct dependency or default.
3. For every required facet or horizon, produce a supported result, explicit
   null, held result, rejection, or reasoned independently reviewed N/A only
   where the fixed contract permits it. Missing never becomes zero, false,
   empty, pass, or N/A.
4. Preserve each assurance gate, stakeholder lens, distribution, tail,
   uncertainty, limitation, and dissent separately. No aggregate may waive a
   floor or erase a concentrated effect.
5. Freeze each domain output, obtain SOURCE exact-output admission for that
   frozen bond, then obtain independent TEST pass for the same unchanged bond.
6. Only after both gates pass, route the unchanged output through its fixed
   authorized semantic consumers.
7. Compose ECO only from compatible exact versions. Compatibility `held`,
   `incompatible`, or `stale`, or an open controlling hold, stops that branch
   rather than substituting a favorable value.

### Procedure C — independent review

1. Freeze the candidate identity, digest, context, inputs, producer, applicable
   assurance decisions, and planned claim set.
2. Establish reviewer independence and absence of a controlling unresolved
   conflict under the future accepted TEST method.
3. Attempt the applicable positive, negative, boundary, stale, null, N/A,
   security, numeracy, stakeholder, and trace checks.
4. Record each finding and dissent against the exact candidate bond. Absence of
   executable planned evidence is not a pass.
5. Return an ordinary finding to the exact accountable producer or promotion
   controller. A correction creates a new successor and restarts Procedure A.
6. Permit advancement only when the fixed convergence rule is met; do not infer
   convergence from this design's author checks.

### Procedure D — preliminary economics, mandatory delivery, and adaptation

For an accepted preliminary economic envelope `ECO[n]`:

1. Freeze preliminary `ECO[n]`, obtain SOURCE exact-output admission, then
   obtain independent TEST pass for the same unchanged bond. It cannot be
   edited in place or treated as final.
2. Only after those gates pass, authorize the mandatory `DEL[n]` semantic
   consumer and create `DEL[n]` bound to `ECO[n]`, its exact input versions,
   accountable delivery owner, floors, observations, milestones, stop and
   rollback posture, transition posture, and realization evidence posture.
3. Freeze `DEL[n]`, obtain SOURCE exact-output admission, and then obtain
   independent TEST pass for the same unchanged bond. A hold or finding stops
   promotion and returns to the exact accountable producer.
4. Only after accepted `DEL[n]`, create predecessor-linked final `ECO[n+1]`
   bound to both `ECO[n]` and reviewed `DEL[n]`. Freeze that final output,
   obtain SOURCE exact-output admission, and then obtain independent TEST pass
   before any authorized semantic consumer receives it.
5. Only after accepted final `ECO[n+1]`, create and freeze `ADP[n+1]`, obtain
   SOURCE exact-output admission, and then obtain independent TEST pass for the
   same unchanged adaptive output before any authorized semantic consumer use.
6. If later delivery or adaptive feedback changes a material fact, freeze and
   SOURCE-admit that feedback, independently TEST the same unchanged feedback,
   create a new preliminary `ECO[m]` where `m > n+1`, and repeat the full
   gated successor sequence.

No cadence, threshold, fiscal value, horizon, or realization method is chosen
here. There is no direct `DEL[n] -> ECO[n]` edge, no skipped delivery step, and
no in-place finalization.

### Procedure E — finite HND terminal gate

1. Do not emit a HND pack while `TBD-HND-001` or any other applicable hold is
   open. Under the current baseline the procedure therefore stops here.
2. If a future accepted baseline closes every applicable hold, assemble a
   candidate from exact accepted versions only. Freeze its identity, digest,
   context, provenance, separated ledgers, overlap posture, gates, delivery
   evidence, limitations, uncertainty, dissent, and non-authority statement.
3. Re-admit that exact frozen HND candidate through SOURCE and independently
   review it through ordinary TEST.
4. On a finding, return to the exact accountable producer. Any product,
   material, context, identity, or digest change restarts bundle freeze,
   SOURCE admission, and TEST.
5. On successful terminal review, `IF-TERM-001` may record only the unchanged
   admitted bundle identity/posture, reviewer identity, decision, date, and
   dissent. It contains no product content, derivation, visualization, or
   composition recipe.
6. The receipt is consumed only by the external handoff gate. It is not an HND
   input, is not SOURCE-re-admitted, and is not independently re-reviewed.
7. The unchanged exact admitted bundle advances directly to the external
   Taxlane boundary. Taxlane alone may decide its own admission. BASTION does
   not infer acceptance, budget, allocation, rate, or official-use authority.

This branch terminates. Feeding the receipt into HND, SOURCE, TEST, or a new
product is invalid. Any product change restarts at step 2 and cannot inherit the
old receipt.

### Procedure F — release and generated material

`CONTRACT-REL-001` records no-release posture and emits no artifact to any
consumer. Documentation is not release. Generated artifacts, if later
authorized, are sinks derived from accepted sources; they cannot become
authoritative semantic inputs merely because they are generated.

### Universal gate-bypass verification allocation

Every row is a planned negative fixture. For the named producer, attempt to
deliver an otherwise well-formed product/material output to an authorized
semantic consumer while omitting SOURCE, omitting independent TEST, changing
the bond between either gate and consumption, or reusing a gate decision from
another bond. Every attempt must fail closed. The fixtures remain absent and
unexecuted.

| Producer decision | Exact bypass surface |
|---|---|
| `DES-SOURCE-001` | SOURCE transformation output, including self-exemption. |
| `DES-AUTH-001` | AUTH output before any domain or review consumer. |
| `DES-RDY-001` | RDY result before ECO or DEL consumption. |
| `DES-ACQ-001` | ACQ result before DST, ECO, or DEL consumption. |
| `DES-LOG-001` | LOG result before RDY, DST, ECO, or DEL consumption. |
| `DES-ALLY-001` | ALLY result before DST, ECO, or DEL consumption. |
| `DES-DST-001` | DST result before ECO or DEL consumption. |
| `DES-ECO-001` | Preliminary or final ECO before DEL, ADP, or HND consumption. |
| `DES-ADP-001` | Adaptive product/material output before any authorized consumer. |
| `DES-TEST-001` | Ordinary TEST-authored product/material report before its authorized consumer; TEST control dispositions are not product bypasses. |
| `DES-TRACE-001` | Promotable TRACE product/material output before its authorized governance consumer. |
| `DES-DEL-001` | DEL output before final ECO, ADP, or HND consumption. |
| `DES-HND-001` | Frozen HND candidate before the external handoff gate; the unchanged minimal non-product terminal receipt remains exempt. |

`DES-REL-001` has no bypass fixture because `REL` emits no output. That is the
only no-output case, not an alternative promotion path.

## Dependency graph and finite-termination proof

The conceptual graph distinguishes a product edge from a control edge. A
product edge carries a frozen accepted semantic value to an exact fixed
consumer. A SOURCE or TEST control edge evaluates that exact value but does not
make the gate its semantic producer. Consequently, mandatory re-admission does
not create permission for SOURCE or TEST to feed a value back into its producer.

For every promotable product/material artifact generation, the well-founded
control order is:

`formed -> frozen -> SOURCE exact-output decision -> independent TEST decision
-> authorized semantic consumer promotion or terminal hold/reject`.

Each arrow is single-directional for that exact identity/version/digest/context
bond. A finding or material change cannot move backward; it creates a distinct
successor generation, whose controls start again. Thus retries form an ordered
sequence of immutable generations, not a directed cycle within one artifact.

The economic path has the stricter version and gate order:

`freeze preliminary ECO[n] -> SOURCE -> TEST -> authorized DEL[n] formation ->
freeze DEL[n] -> SOURCE -> TEST -> final ECO[n+1] formation -> freeze final
ECO[n+1] -> SOURCE -> TEST -> authorized ADP[n+1] formation -> freeze ADP[n+1]
-> SOURCE -> TEST -> authorized consumer`.

No `DEL[n] -> ECO[n]` edge exists. Later feedback can create only a preliminary
`ECO[m]` with `m > n+1`, so every feedback traversal strictly increases the
version order. A finite review attempt therefore ends in promotion, hold, or
rejection; continuing work requires an explicitly new successor.

The handoff path is finite:

`frozen HND bundle -> SOURCE -> ordinary TEST -> IF-TERM-001 decision/receipt
-> external handoff gate -> Taxlane boundary`.

There is no receipt edge to HND, SOURCE, TEST, REL, or any product producer.
The receipt carries no product and the unchanged bundle exits BASTION. A
changed bundle invalidates the receipt and begins a new generation rather than
creating a return edge. `REL` emits no output; generated-material boundaries
are sinks. These ordering rules prove acyclicity per artifact generation and finite
termination of each gate attempt without claiming that a future program has a
bounded number of independently authorized successor generations.

## Invariants

1. Exactly thirteen fixed `CONTRACT-*` identities exist; `IF-TERM-001` is not a
   contract.
2. Every product or material output is SOURCE-re-admitted before semantic use,
   including SRC transformations and AUTH outputs.
3. A SOURCE admission is valid only for its exact semantic bond and context.
4. No product producer self-exempts from independent controls.
5. `REL` emits no output and has no consumer.
6. The terminal receipt is non-product, minimal, finite, and never an HND
   semantic input.
7. Any terminal-bundle product change invalidates the receipt and restarts
   freeze, SOURCE admission, and TEST.
8. Taxlane is the exclusive external admission authority.
9. Every preliminary `ECO[n]` has mandatory `DEL[n]` before final
   `ECO[n+1]` and `ADP[n+1]`.
10. No accepted artifact is edited in place; correction creates a successor.
11. No same-version DEL-to-ECO cycle exists.
12. Later material feedback repeats the full successor sequence.
13. Missing is never zero, false, empty, pass, or N/A.
14. N/A is used only when permitted, reasoned, and independently reviewed.
15. All controlling holds are conjunctive; favorable economics, schedule, or
    majority agreement cannot waive one.
16. Civilian authority, safety, readiness, resilience, surge, recovery, and
    alliance obligations are non-waivable floors.
17. Stakeholder results, concentrated effects, distributions, tails, and
    dissent remain separately inspectable.
18. Gross opportunity is not realizable savings; six pathways and federal
    fiscal meanings remain distinct and overlap-controlled.
19. Every ordinary TEST finding returns to the exact accountable producer or
    promotion controller for the reviewed digest.
20. Planned evidence is never represented as executed evidence or acceptance.
21. Co-location grants no semantic consumer edge, conversion, default, or
    authority.
22. No artifact creates force, procurement, operational, budget, allocation,
    rate, implementation, official-use, or public-release authority.
23. Admission or compatibility `stale` blocks use of the exact bond; it is not
    a supersession or lifecycle disposition.
24. A material change creates a new successor identity while predecessor,
    successor, and supersession references remain historical and inspectable.
25. Only adaptive or trigger-driven records use the fixed lifecycle
    dispositions `preserve`, `revise`, `hold`, `retire`, or `replace`.

## State transitions

These are conceptual postures, not enum, schema, or storage declarations.
Unsupported or unknown transitions fail closed.

| Current posture | Event | Next posture / action |
|---|---|---|
| Unformed | Required identity, inputs, owner, and context become available | Candidate may be formed; otherwise remain held. |
| Candidate | Freeze exact semantic bond | Frozen candidate; mutation creates a different candidate. |
| Frozen candidate | SOURCE pass for exact bond | Admitted for only the fixed next consumer/review. |
| Frozen candidate | SOURCE `hold` or `reject` | Remain held or rejected under that exact admission posture; invalidate affected downstream bonds. |
| Frozen candidate | SOURCE identity, digest, or context mismatch | Do not promote; record the applicable fixed admission posture without inferring supersession or lifecycle. |
| Admitted candidate | Independent TEST pass and convergence satisfied | Reviewed candidate eligible for its fixed promotion. |
| Admitted candidate | TEST finding, conflict, absent evidence, or dissent requiring disposition | Held; finding returns to accountable producer. |
| SOURCE admission or compatibility `stale` | Exact admitted context/digest or predecessor/consumer context is no longer current | Block use of that exact bond; do not infer a lifecycle disposition or rewrite history. |
| Accepted artifact | Material input, context, authority, method, owner, or digest changes | Create a new successor identity and restart freeze, SOURCE, and TEST; retain predecessor and successor references. |
| Accepted predecessor with an accepted successor | Successor becomes the controlled current version | Retain the supersession reference; supersession is a historical relationship, not admission `stale` or a lifecycle posture. |
| Adaptive or trigger-driven record | Its fixed lifecycle rule is evaluated | Record exactly one permitted lifecycle disposition: `preserve`, `revise`, `hold`, `retire`, or `replace`; do not use admission/compatibility `stale` as that disposition. |
| Preliminary `ECO[n]` | Mandatory `DEL[n]` accepted | Eligible to form final successor `ECO[n+1]`. |
| Final `ECO[n+1]` | Exact final output accepted | Eligible to form `ADP[n+1]` and, with all other gates, HND input. |
| Accepted delivery/adaptive record | Later material feedback | Freeze feedback; form later preliminary successor and repeat cycle. |
| Frozen HND candidate | SOURCE and ordinary TEST pass | Exact bundle may receive one minimal terminal decision/receipt. |
| Terminally accepted unchanged bundle | External handoff gate consumes receipt | Advance unchanged bundle to Taxlane boundary; terminate BASTION branch. |
| Terminally accepted bundle | Any product/material/context/identity/digest change | Receipt invalid; restart bundle freeze, SOURCE, and TEST. |
| No-release record | Any ordinary repository action | Remain no-release; emit nothing. |

Artifact outcomes remain the fixed semantic alternatives `result`, `null`,
`held`, `rejected`, and permitted reasoned independently reviewed
`not_applicable`. Admission postures remain `pass`, `hold`, `reject`, `stale`,
and permitted reviewed `not_applicable`; review postures remain `pass`,
`finding`, `defer`, or blocking `hold`. Compatibility postures remain
`compatible`, `held`, `incompatible`, or `stale`. Lifecycle postures remain
`preserve`, `revise`, `hold`, `retire`, or `replace` only for adaptive or
trigger-driven records. Supersession remains a retained relationship between
immutable predecessor and successor identities, not a posture in either
family.

## Edge cases

| Case | Deterministic disposition |
|---|---|
| Public inputs compose into a dangerous inference | Hold/reject the exact composition; do not retain reconstructive failure detail. |
| SOURCE transforms an admitted source | Treat the transform as a new output and re-admit it. |
| AUTH is source-backed but its composed output is not re-admitted | Hold all dependent branches. |
| Required facet or horizon has no evidence | Record an explicit null/hold; never substitute zero or omit silently. |
| A proposed N/A lacks rationale or independent review | Hold as missing, not N/A. |
| Two domain versions are individually accepted but incompatible together | Hold composition and identify the exact incompatibility. |
| A readiness floor conflicts with favorable economics | Floor controls; preserve the economic observation without promotion. |
| A total hides a concentrated stakeholder burden | Keep the total and burden separate; block promotion until disposition. |
| Commonality saves purchase cost but increases concentration risk | Preserve both facets; do not net one away. |
| Average repair time improves while the tail worsens | Preserve the distribution/tail and prevent average-only promotion. |
| Domestic savings shift burden to an ally | Preserve separated ledgers and hold the claim pending accepted treatment. |
| Peer data are not comparable | Record limitation/null/hold; do not normalize with an invented method. |
| Gross opportunity overlaps another pathway | Keep pathways separate and hold aggregation until accepted overlap rules apply. |
| Preliminary ECO lacks DEL | Stop; it cannot become final, adaptive, realizable, or handoff-ready. |
| DEL has observations but no accepted owner, stop, or rollback posture | Hold delivery and downstream economics. |
| Later feedback arrives after a final ECO | Start a later preliminary successor; do not edit or append to the accepted final. |
| Review targets a stale digest | Mark the applicable admission or compatibility bond `stale`, block its use, and review the exact current successor candidate. |
| Reviewer conflict cannot be resolved | Hold; do not treat review absence as concurrence. |
| HND candidate changes after successful terminal review | Invalidate receipt and repeat freeze, SOURCE admission, and TEST. |
| Terminal receipt contains product text or a derivation | It is invalid and loses the exemption; restart as changed product material. |
| Terminal receipt is routed back to HND or TEST | Reject the recursive edge; the receipt belongs only to the external gate. |
| Taxlane rejects the unchanged bundle | Preserve Taxlane's external disposition; infer no BASTION acceptance or product mutation. |
| A document is mistaken for release authority | `REL` remains no-output; no release occurs. |
| A generated artifact is proposed as authoritative input | Reject that authority inversion unless a future fixed SOURCE process admits the underlying product. |
| Any of the thirteen controlled unknowns remains applicable | Apply its exact inherited hold; choose no default. |

## Rejected alternatives

- A single optimizing score: rejected because floors, fiscal meanings,
  uncertainty, distributions, and concentrated harms must remain separable.
- Direct gross-savings export: rejected because delivery, realization, costs,
  timing, overlap, and fiscal ownership are mandatory.
- Optional DEL or same-version ECO/DEL iteration: rejected because the fixed
  successor order is mandatory and acyclic.
- SOURCE trust inherited across transformation: rejected because composition,
  audience, granularity, context, and derivation can change safety.
- Reviewer self-approval or evidence-free pass: rejected because independent,
  digest-bound falsification and finding retention are required.
- A fourteenth terminal contract or recursive receipt review: rejected because
  the receipt is the finite governance branch of `CONTRACT-TEST-001`.
- Direct BASTION-to-budget, rate, allocation, implementation, or release
  authority: rejected as outside mission and authority.
- One physical boundary per semantic contract: rejected because fixed package
  co-location is permitted while logical ownership and dependency remain
  explicit.
- A shared semantic core before repeated invariants are proven: rejected; only
  identical non-domain control metadata may later qualify under fixed review.
- Selecting Rust, crates, schemas, dependencies, runtimes, or fixtures now:
  rejected because design fixed point, verification, code rigor, and accepted
  work-package gates have not occurred.

## Migration and rollout posture

There is no implementation or encoded data to migrate. Advancement is a
controlled evidence sequence, not an operational rollout:

1. Independently review this exact DESIGN digest against all fixed inputs,
   requirements, specifications, components, boundaries, contracts, holds, and
   role concerns.
2. Remediate findings by successor artifact; do not edit an accepted design in
   place or call author review convergence.
3. Independently review and accept `CHG-BA-TST-001` at its exact bound digest
   together with this exact DESIGN, with no unresolved critical or major
   finding. Until that pair is accepted, the planning-only exception is
   unavailable. The change preserves the literal TST hold while distinguishing
   governance-only artifact fixed points from product-evidence convergence and
   downstream promotion.
4. Only after that digest-bound pair acceptance, a bounded planning-only
   VERIFICATION artifact may be authored before the later administrative
   recording of the DESIGN fixed point. It may propose falsifiable evidence
   and validation methods but is not executed evidence, acceptance,
   convergence, or downstream-stage passage.
5. Record a governance-only DESIGN fixed point only after independent DESIGN
   and change-control convergence. That fixed point does not accept a
   Verification plan or close `TBD-TST-001` / `SPEC-UNK-TST-001`.
6. Separately review any future Verification plan and its exact held methods.
   Product-evidence verification, validation, implementation, work packages,
   readiness, handoff, and every later promotion remain held until the TST
   method and every other applicable gate are independently accepted.
7. Establish code-rigor controls and an accepted bounded work package before
   any implementation choice or code.
8. For any future representation change, document compatibility, semantic
   equivalence, re-admission, invalidation, and migration behavior before use.

No step above authorizes implementation, deployment, procurement, official
use, Taxlane admission, rate setting, public release, or an external action.

## Code rigor hooks

These hooks constrain a future `CODE_RIGOR.md`; they do not select tools or
implementation:

| Hook | Future rigor obligation |
|---|---|
| Total state handling | Make every permitted posture and invalid transition explicit and fail unknowns closed. |
| Immutable identity | Prevent accepted identity/version/digest/context bonds from in-place mutation. |
| Determinism | Define canonical semantic comparison and reproducible derivation independent of incidental representation. |
| Dimensional integrity | Prevent incompatible units, price bases, horizons, fiscal fields, distributions, and uncertainty from silent combination. |
| Graph safety | Prove allowed producer/consumer direction, absence of same-version cycles, and finite terminal behavior. |
| Admission completeness | Demonstrate every product producer, including SRC and AUTH, returns through SOURCE. |
| Review independence | Enforce exact-digest review, conflict controls, finding retention, dissent, and no evidence-free pass. |
| Hold propagation | Prove each applicable controlled unknown blocks exactly its inherited branches with no hidden default. |
| Data minimization | Prevent prohibited content and reconstructive failure detail from persistence or output. |
| Accounting separation | Preserve six pathways, fiscal meanings, ownership, overlap, costs, timing, and gross-to-net status. |
| Stakeholder preservation | Keep lenses, distributions, tails, burdens, and non-waivable floors separately inspectable. |
| Successor discipline | Enforce preliminary ECO, mandatory DEL, final ECO successor, ADP successor, and later-feedback restart. |
| Terminal finiteness | Permit only the minimal non-product receipt, block recursive consumers, and invalidate on product change. |
| Release closure | Prove REL has no output path and documentation cannot imply release. |
| Trace completeness | Reject orphaned or stale requirement, specification, decision, contract, verification, validation, or evidence links. |
| Generated-output isolation | Keep generated material non-authoritative and reproducibly derived from admitted inputs. |

## Role synthesis

These are author design checks, not role passes or external approvals.

| Role | Design consequence |
|---|---|
| Civilian Control, Law, Safety & Readiness | Authority and safety/readiness floors are separate, conjunctive, and non-waivable. |
| Classification & Operational Security | Every exact composition is re-admitted; prohibited and reconstructive content fails closed. |
| Citation Auditor | Source, claim, derivation, limitation, uncertainty, and supersession bonds remain inspectable. |
| Numeracy Checker | Units, horizons, price bases, distributions, uncertainty, transition costs, and overlap cannot silently combine. |
| Scope Keeper | No operational planning, official decision, implementation, rate, allocation, or release authority is created. |
| BASTION Methodology Panel | Design is falsifiable across public finance, acquisition, logistics, readiness, and civil-military/legal lenses. |
| Acquisition & Industrial-Base Lead | Commonality benefits stay separate from capacity, competition, workforce, transition, and concentration risks. |
| Alliance & Interoperability Strategist | Compatibility, sovereignty, partner capacity, and burden ledgers remain explicit. |
| Civilian Strategy & Force Planner | Public mission abstractions cannot justify inherited structure or become operational planning. |
| Defense Comptroller | Gross opportunity cannot become booked savings; federal fiscal semantics and delivery evidence control promotion. |
| Independent Test & Oversight Officer | Frozen digest, independence, negative evidence, dissent, and finding disposition control advancement. |
| Logistics & Sustainment Lead | Lifecycle, stock, maintenance, repair distributions, workload, and degraded recovery remain explicit. |
| Operational Readiness Officer | Spending or inventory totals never substitute for integrated employable readiness. |
| Service-Member & Family Advocate | Personnel and family burdens cannot be hidden inside efficiency or averages. |
| Ally & Partner | Sovereign constraints, common logistics, standards, commitments, and predictable burden distribution remain visible. |
| Depot & Logistics Workforce | Facilities, skills, workload, safety, surge, spares, and realistic repair schedules remain testable. |
| Installation Community | Local employment, housing, utilities, environment, services, transition, and burden effects remain a separate lens. |
| Mission User | Capability must remain reliable, integrated, supportable, and non-operationally described. |
| Prime & Small Supplier | Competition, cash flow, qualification, capacity, workforce, IP, and demand resilience remain separate. |
| Service Member & Family | Safety, tempo, training, retention, housing, health, moves, and caregiving remain protected lenses. |
| Taxpayer & Oversight Body | Mission linkage, auditability, affordability, delivery, uncertainty, nulls, and failures remain transparent. |

## Controlled unresolved values and exact holds

All thirteen records remain open. The wording below preserves the fixed hold
behavior; this design supplies no value, threshold, method, closure, or
implementation default. A direct set is the exact primary DES allocation of
the fixed direct requirement-to-TBD trace. A transitive-only set is the exact
additional DES allocation reached by an affected product branch under this
design's consumer and mandatory gate graph. `none` means no additional DES,
not waiver of the fixed hold behavior.

| TBD / specification hold | Exact direct DES set | Exact transitive-only DES set | Owner / destination | Exact TBD hold behavior | Exact specification hold behavior |
|---|---|---|---|---|---|
| `TBD-SEC-001` / `SPEC-UNK-SEC-001` | `{DES-SOURCE-001, DES-LOG-001, DES-HND-001, DES-REL-001}` | `{DES-AUTH-001, DES-RDY-001, DES-ACQ-001, DES-ALLY-001, DES-DST-001, DES-ECO-001, DES-ADP-001, DES-TEST-001, DES-TRACE-001, DES-DEL-001}` | Security and aggregation steward / Security specification and verification | Hold admission, derivation, visualization, handoff, and release for affected fields or combinations. | Hold affected admission, retention, derivation, emission, visualization, handoff, release. |
| `TBD-RDY-001` / `SPEC-UNK-RDY-001` | `{DES-AUTH-001, DES-RDY-001, DES-DEL-001, DES-HND-001}` | `{DES-ACQ-001, DES-LOG-001, DES-ALLY-001, DES-DST-001, DES-ECO-001, DES-ADP-001, DES-TEST-001, DES-TRACE-001}` | Readiness-system analyst / Promise specification and verification | Hold the affected readiness result, candidate, savings claim, and handoff. | Hold readiness, candidate, savings, handoff. |
| `TBD-SRC-001` / `SPEC-UNK-SRC-001` | `{DES-SOURCE-001, DES-TEST-001, DES-TRACE-001}` | `{DES-AUTH-001, DES-RDY-001, DES-ACQ-001, DES-LOG-001, DES-ALLY-001, DES-DST-001, DES-ECO-001, DES-ADP-001, DES-DEL-001, DES-HND-001}` | Public-evidence steward / Corpus and interface specifications | Hold any artifact whose custody cannot be represented without loss. | Hold unrepresentable custody/version/review behavior. |
| `TBD-QNT-001` / `SPEC-UNK-QNT-001` | `{DES-RDY-001, DES-ECO-001, DES-HND-001}` | `{DES-ADP-001, DES-TEST-001, DES-TRACE-001, DES-DEL-001}` | Defense resource analyst / Quantitative methods specification | Hold affected projection, peer comparison, cross-scenario total, and fiscal handoff. | Hold projections, peers, horizons, totals, handoffs. |
| `TBD-ACQ-001` / `SPEC-UNK-ACQ-001` | `{DES-ACQ-001, DES-DST-001, DES-DEL-001}` | `{DES-ECO-001, DES-ADP-001, DES-TEST-001, DES-TRACE-001, DES-HND-001}` | Acquisition and industrial-base analyst / Industrial-base specification | Hold affected acquisition, commonality, capacity, schedule, or savings result. | Hold acquisition/commonality/capacity/schedule/savings. |
| `TBD-LOG-001` / `SPEC-UNK-LOG-001` | `{DES-LOG-001, DES-RDY-001, DES-DEL-001}` | `{DES-DST-001, DES-ECO-001, DES-ADP-001, DES-TEST-001, DES-TRACE-001, DES-HND-001}` | Logistics and sustainment analyst / Readiness and sustainment specification | Hold affected sustainment, readiness, lifecycle, and savings result. | Hold sustainment/readiness/lifecycle/savings. |
| `TBD-ALLY-001` / `SPEC-UNK-ALLY-001` | `{DES-ALLY-001, DES-DST-001, DES-DEL-001}` | `{DES-ECO-001, DES-ADP-001, DES-TEST-001, DES-TRACE-001, DES-HND-001}` | Alliance and interoperability analyst / Interoperability specification and interface | Hold joint, interoperability, commitment, burden, and fiscal claims. | Hold joint/interoperability/burden/fiscal claims. |
| `TBD-DST-001` / `SPEC-UNK-DST-001` | `{DES-DST-001, DES-ACQ-001, DES-ALLY-001, DES-ECO-001, DES-DEL-001, DES-HND-001}` | `{DES-ADP-001, DES-TEST-001, DES-TRACE-001}` | Personnel, family, workforce, and community analyst / Distribution specification | Hold affected efficiency, savings, readiness, and handoff claims. | Hold efficiency/savings/readiness/distribution/handoff. |
| `TBD-ECO-001` / `SPEC-UNK-ECO-001` | `{DES-ECO-001, DES-ADP-001, DES-HND-001}` | `{DES-TEST-001, DES-TRACE-001, DES-DEL-001}` | Defense Comptroller / Economics specification and shared accounting interface | Hold monetization, realizable-savings, receipt, net-pressure, and Taxlane handoff claims. | Hold monetization/savings/receipt/net-pressure/handoff. |
| `TBD-TST-001` / `SPEC-UNK-TST-001` | `{DES-TEST-001, DES-TRACE-001}` | `none within the controlled DESIGN semantic graph; stage-term application is governed by CHG-BA-TST-001 without closing either hold` | Independent Test and Oversight Officer / Verification plan | Hold fixed point and every downstream stage. | Hold fixed point and downstream stage. |
| `TBD-DEL-001` / `SPEC-UNK-DEL-001` | `{DES-ECO-001, DES-ADP-001, DES-DEL-001, DES-HND-001}` | `{DES-TEST-001, DES-TRACE-001}` | Delivery owner / Delivery specification and future accepted work package | Retain research-hypothesis posture; block realizable savings, implementation, and handoff. | Retain research hypothesis; block savings/implementation/handoff. |
| `TBD-HND-001` / `SPEC-UNK-HND-001` | `{DES-ECO-001, DES-HND-001}` | `{DES-ADP-001, DES-TEST-001, DES-TRACE-001, DES-DEL-001}` | Taxlane adapter steward / Interface specification after shared Taxlane review | Hold every adapter package; infer no Taxlane admission. | Hold adapter package; infer no Taxlane admission. |
| `TBD-REL-001` / `SPEC-UNK-REL-001` | `{DES-SOURCE-001, DES-REL-001}` | `none; the fixed no-release branch has no product consumer` | Scope Keeper / Release-specific requirements and validation under new authority | No public release. | No public release. |

### Inverse DES-to-hold equality

Each inverse set below equals the transpose of the direct union
transitive-only sets above. No hold is added through a phrase such as “all
downstream,” and no hold is omitted by a package or contract co-location.

| Design decision | Exact controlling hold set |
|---|---|
| `DES-SOURCE-001` | `{TBD-SEC-001, TBD-SRC-001, TBD-REL-001}` |
| `DES-AUTH-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001}` |
| `DES-RDY-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-LOG-001}` |
| `DES-ACQ-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-ACQ-001, TBD-DST-001}` |
| `DES-LOG-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-LOG-001}` |
| `DES-ALLY-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-ALLY-001, TBD-DST-001}` |
| `DES-DST-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001}` |
| `DES-ECO-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001, TBD-DEL-001, TBD-HND-001}` |
| `DES-ADP-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001, TBD-DEL-001, TBD-HND-001}` |
| `DES-TEST-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001, TBD-TST-001, TBD-DEL-001, TBD-HND-001}` |
| `DES-TRACE-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001, TBD-TST-001, TBD-DEL-001, TBD-HND-001}` |
| `DES-DEL-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001, TBD-DEL-001, TBD-HND-001}` |
| `DES-HND-001` | `{TBD-SEC-001, TBD-RDY-001, TBD-SRC-001, TBD-QNT-001, TBD-ACQ-001, TBD-LOG-001, TBD-ALLY-001, TBD-DST-001, TBD-ECO-001, TBD-DEL-001, TBD-HND-001}` |
| `DES-REL-001` | `{TBD-SEC-001, TBD-REL-001}` |

The equality is set equality over the fourteen fixed `DES-*` identities. The
TBD and SPEC-UNK names remain paired one-to-one, so the same inverse applies to
the corresponding thirteen `SPEC-UNK-*` identities.

## Governed resolution — `BA-DES-M03`

`BA-DES-M03` has an explicit bounded author resolution in
`CHG-BA-TST-001`, bound above by exact digest. The fixed upstream wording,
owner, destination, closure condition, dependencies, and hold behavior remain
unchanged; earlier left-side fixed points remain immutable history.

Prospectively after independent acceptance of the exact change/design digests,
the controlled tailoring applies “fixed point and every downstream stage” to
product-evidence convergence, verification/validation claims, implementation,
work packages, delivery/readiness, handoff, Taxlane, release, and official or
operational promotion. Governance-only fixed points over frozen planning
artifacts do not claim those outcomes. A bounded planning-only VERIFICATION
artifact may therefore be authored before the DESIGN fixed point to propose
the exact method whose fixed destination is the Verification plan.

Every evidence tier, reproduction criterion, reviewer-conflict rule,
severity/disposition schema, convergence method, and fixture remains open with
no favorable default. `TBD-TST-001` and `SPEC-UNK-TST-001` remain conjunctive
promotion gates. A DESIGN fixed point grants no Verification-plan acceptance,
verification, validation, implementation, work-package, readiness, HND,
Taxlane, release, or official authority.

The required `IF-TERM-001` / HND impact check found no change: the receipt,
consumer topology, finite terminal branch, no-pack posture, and exclusive
external Taxlane boundary remain exactly as designed.

Author disposition: **`BA-DES-M03` remediated; pending independent
digest-bound DESIGN/change-control convergence**. This DESIGN is eligible for
that fixed-point review but is not itself a fixed point.

## Review readiness

The author allocation covers 98 requirements/specifications, 13 components,
12 package boundaries, 13 contracts, 10 non-functional constraints, 13 open
holds, and 21 role lenses. All evidence remains planned and unexecuted. This
artifact is ready for independent design and change-control review and is
eligible for a governance-only fixed-point decision. It is not a fixed point;
author synthesis and `CHG-BA-TST-001` do not satisfy, close, or silently
reinterpret `TBD-TST-001` or `SPEC-UNK-TST-001`.
