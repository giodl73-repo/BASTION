# BASTION Interface Control Baseline

## Status and controlled inputs

Repo: BASTION

Assignment: `ASG-BASTION-INTERFACES-001`

Interface state: **review-ready encoding-neutral baseline; not a fixed point**.

Controlled fixed `PACKAGE_BOUNDARIES.md` input SHA-256:
`43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695`

Controlled fixed `ARCHITECTURE.md` input SHA-256:
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`

This baseline controls the semantic content, producer/consumer direction,
postures, failure behavior, version bonds, ownership, compatibility, and
planned evidence for exactly the thirteen fixed `CONTRACT-*` identities. It
does not create an implementation interface or close an inherited unknown.

Concrete Rust types, traits, functions, crate features, JSON or other schema
names, serialization, file formats, transport, CLI commands, environment
variables, configuration, storage, algorithms, quantitative thresholds,
dependency libraries or versions, runtime, process topology, deployment, and
retention technology remain deferred or held. No path, command, fixture, test,
generator, corpus, package, or API exists merely because its semantic purpose
is named below.

## Interface inventory

Every row names one existing controlled contract and no additional public
surface. `Version 1` is the semantic control version of this planned baseline,
not an encoding or compatibility claim about an implemented artifact.

| Contract ID | Version and status | Physical custodian / logical producer | Exact direct semantic consumers | Named blockers |
|---|---|---|---|---|
| `CONTRACT-SOURCE-001` | Version 1; planned; encoding absent; held | `PB-CST-001` / `ARC-SRC-001` | `ARC-AUTH-001`, `ARC-RDY-001`, `ARC-ACQ-001`, `ARC-LOG-001`, `ARC-ALLY-001`, `ARC-DST-001`, `ARC-ECO-001`, `ARC-ADP-001`, `ARC-DEL-001`, `ARC-REV-001`, and `ARC-HND-001`; `ARC-SRC-001` transformations re-enter their own gate | `SPEC-UNK-SEC-001`; `SPEC-UNK-SRC-001`; release-context branches also retain `SPEC-UNK-REL-001` |
| `CONTRACT-AUTH-001` | Version 1; planned; encoding absent; held | `PB-AUT-001` / `ARC-AUTH-001` | After `CONTRACT-SOURCE-001` re-admission: `ARC-RDY-001`, `ARC-ACQ-001`, `ARC-LOG-001`, `ARC-ALLY-001`, `ARC-DST-001`, `ARC-ECO-001`, `ARC-ADP-001`, `ARC-DEL-001`, `ARC-REV-001`, and `ARC-HND-001` as applicable | `SPEC-UNK-RDY-001`; representation and re-admission retain `SPEC-UNK-SRC-001` and `SPEC-UNK-SEC-001` |
| `CONTRACT-RDY-001` | Version 1; planned; encoding absent; held | `PB-DOM-001` / `ARC-RDY-001` | Direct values: `ARC-ECO-001`, `ARC-DEL-001`. `ARC-REV-001` inspects through `CONTRACT-TEST-001`; `ARC-ADP-001` and `ARC-HND-001` receive no direct RDY value and require the later final ECO plus delivery bond | `SPEC-UNK-RDY-001`; `SPEC-UNK-LOG-001`; quantitative downside branches retain `SPEC-UNK-QNT-001` |
| `CONTRACT-ACQ-001` | Version 1; planned; encoding absent; held | `PB-DOM-001` / `ARC-ACQ-001` | `ARC-DST-001`, `ARC-ECO-001`, and `ARC-DEL-001`; review occurs through `CONTRACT-TEST-001`, not by widening this consumer set | `SPEC-UNK-ACQ-001`; distribution branches retain `SPEC-UNK-DST-001` |
| `CONTRACT-LOG-001` | Version 1; planned; encoding absent; held | `PB-DOM-001` / `ARC-LOG-001` | `ARC-RDY-001`, `ARC-DST-001`, `ARC-ECO-001`, and `ARC-DEL-001`; review occurs through `CONTRACT-TEST-001` | `SPEC-UNK-LOG-001`; unsafe-detail branches retain `SPEC-UNK-SEC-001` |
| `CONTRACT-ALLY-001` | Version 1; planned; encoding absent; held | `PB-DOM-001` / `ARC-ALLY-001` | `ARC-DST-001`, `ARC-ECO-001`, and `ARC-DEL-001`; review occurs through `CONTRACT-TEST-001` | `SPEC-UNK-ALLY-001`; distribution branches retain `SPEC-UNK-DST-001` |
| `CONTRACT-DST-001` | Version 1; planned; encoding absent; held | `PB-DOM-001` / `ARC-DST-001` | Direct values: `ARC-ECO-001`, `ARC-DEL-001`. `ARC-ADP-001` and `ARC-HND-001` receive no direct DST value and require the later final ECO plus delivery bond | `SPEC-UNK-DST-001`; applicable inputs retain `SPEC-UNK-ACQ-001` and `SPEC-UNK-ALLY-001` |
| `CONTRACT-ECO-001` | Version 1; planned; encoding absent; held | `PB-PTH-001` / `ARC-ECO-001` | Preliminary `ECO[n]`: `ARC-DEL-001[n]`; final `ECO[n+1]`: `ARC-ADP-001[n+1]` and `ARC-HND-001`. `ARC-REV-001` inspects through `CONTRACT-TEST-001`. No same-version DEL-to-ECO consumer edge exists | `SPEC-UNK-QNT-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001`; overlap/handoff branches retain `SPEC-UNK-HND-001` |
| `CONTRACT-TEST-001` | Version 1; planned; encoding absent; held | `PB-REV-001` / `ARC-REV-001` | Ordinary TEST findings: the exact accountable producer or promotion controller for the reviewed digest. Successful terminal decision/receipt: the external handoff gate only, where it advances the unchanged exact admitted HND bundle directly to the external Taxlane handoff boundary; it is not an `ARC-HND-001` semantic input or consumer edge. Stage governance consumes convergence posture only | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` |
| `CONTRACT-DEL-001` | Version 1; planned; encoding absent; held | `PB-PTH-001` / `ARC-DEL-001` | Originating sequence: `ARC-SRC-001`, `ARC-REV-001`; reviewed `DEL[n]`: `ARC-ECO-001` only to form predecessor-linked final `ECO[n+1]`; `ARC-ADP-001` and `ARC-HND-001` only with that final envelope | `SPEC-UNK-RDY-001`; `SPEC-UNK-ACQ-001`; `SPEC-UNK-LOG-001`; `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-DEL-001` |
| `CONTRACT-HND-001` | Version 1; planned; encoding absent; held; no pack emission | `PB-HND-001` / `ARC-HND-001` | External Taxlane admission boundary only. SOURCE admission and TEST inspect the frozen bundle as sidecar controls; neither is a widened HND semantic consumer, and the terminal receipt does not return to HND | `SPEC-UNK-SEC-001`; `SPEC-UNK-RDY-001`; `SPEC-UNK-QNT-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001`; `SPEC-UNK-HND-001` |
| `CONTRACT-REL-001` | Version 1; current closed posture; output absent; held | `PB-DOC-001` / `ARC-REL-001` | none; it records no-release posture and has no artifact, handoff, runner, generated, Taxlane, or public-release consumer | `SPEC-UNK-SEC-001`; `SPEC-UNK-REL-001` |
| `CONTRACT-TRACE-001` | Version 1; planned; encoding absent; held | `PB-REV-001` / `ARC-REV-001` | BASTION maintainer, role review steward, and the exact stage controller whose advancement is being decided; never a domain-value consumer | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` |

Inventory check: **13 rows, 13 unique fixed contract IDs, zero additional
contract IDs, zero missing fixed contracts**. Physical co-location in
`PB-DOM-001` or `PB-PTH-001` grants no consumer, conversion, default, or
authority absent from the exact rows above.

`IF-TERM-001` below names the terminal governance branch of
`CONTRACT-TEST-001`; it is not a fourteenth contract or public surface. It
distinguishes an ordinary TEST finding returned to an accountable producer
from a successful terminal decision/receipt that gates the unchanged admitted
HND bundle at the external handoff boundary.

## Common encoding-neutral control model

### Permitted common envelope

A later representation may share only non-domain control metadata whose
meaning is identical across all participating contracts:

- controlled contract identity and semantic version;
- artifact identity, artifact version, exact digest, and context identity;
- producer identity, accountable custodian, and owner identity;
- exact input identities and digests, provenance references, and derivation
  identity without copying prohibited content;
- predecessor, successor, and supersession references where applicable;
- typed artifact, gate, review, and compatibility postures;
- concurrence and independent-review identities and their digest bonds;
- accepted security re-admission identity, context, and expiry posture;
- safe error or invalidation category and affected dependent identities; and
- creation, decision, and supersession time basis without selecting a concrete
  clock, precision, storage, or encoding.

This common envelope owns no readiness, acquisition, logistics, alliance,
distribution, fiscal, delivery, handoff, release, or defense meaning. Domain
payload groups remain owned by their fixed producers. A shared implementation
is ineligible until at least three independent domains prove the same
non-domain type or invariant, a fixed contract precedes extraction, ownership
and dependency direction are accepted, and every moved rule is reviewed.

### Typed posture families

The following are semantic alternatives, not enum or field declarations:

- artifact outcome: `result`, `null`, `held`, `rejected`, or reasoned,
  independently reviewed `not_applicable` only where the controlled contract
  permits it;
- security/admission gate: `pass`, `hold`, `reject`, `stale`, or
  `not_applicable` only when independently accepted;
- role review: `pass`, `finding`, or `defer`, with blocking `hold` when a
  required role, assurance gate, owner, closure, or digest bond is absent;
- lifecycle: `preserve`, `revise`, `hold`, `retire`, or `replace` only for
  adaptive or trigger-driven records; and
- compatibility: `compatible`, `held`, `incompatible`, or `stale` against an
  exact predecessor and consumer context.

Missing is never zero, false, empty, accepted, or N/A. An unsupported posture,
unknown discriminant, missing required group, prohibited group, stale digest,
or invalid transition fails closed. A representation must preserve unknown
postures rather than map them to a favorable existing posture.

### Universal preconditions and invalidation

Before any product semantic consumer use, every product/material input and
composed product output has:

1. an exact identity/version/digest/context bond;
2. accepted public, aggregate, unclassified, non-operational source custody;
3. a fresh direct-and-compositional security posture for the exact output,
   including changed joins, granularity, derivation, visualization,
   composition, audience, release context, and expiry;
4. an applicable fresh authority, owner, concurrence, floor, numeracy,
   compatibility, and independent-review posture; and
5. no open controlling unknown for the attempted promotion.

Every derived product or material output returns through
`CONTRACT-SOURCE-001`, including `ARC-SRC-001` transformations, AUTH
manifests, domain results, ECO/ADP/DEL, ordinary review product outputs, and
HND candidate bundles. The minimal non-product `IF-TERM-001` governance
receipt is expressly exempt from SOURCE re-admission and independent
re-review: it contains only unchanged admitted bundle identity/posture,
reviewer/decision/date/dissent and creates no product consumer edge. It does
not enter or modify `ARC-HND-001`. `CONTRACT-REL-001` produces no output. A
changed identity, digest, context, source, join, granularity,
derivation, audience, authority, owner, concurrence, floor, method, unit,
horizon, price basis, overlap, predecessor, review, delivery posture, handoff
mapping, or release posture invalidates the affected acceptance and every
bound downstream branch. Correction creates a successor; no accepted artifact
is edited in place.

### Universal safety and authority rules

- No contract may ingest, retain, derive, or emit classified information,
  CUI, person-level service data, sensitive operational data, targeting,
  operational-planning, or exploitable-vulnerability content.
- A safe rejection or `IF-TERM-001` terminal receipt contains only minimum
  non-reconstructive governance metadata. It contains no rejected value, cell,
  join, operational detail, digest of rejected content, product value,
  derivation, visualization, or composition recipe. The exact digest of an
  admitted artifact or bundle appears only where its fixed review or handoff
  bond requires it and never carries rejected content.
- The terminal receipt may contain only the unchanged admitted bundle
  identity/posture, reviewer identity, decision, date, and dissent. It is a
  non-product governance receipt, not an HND payload or semantic input. Any
  copied or new product/material content, or any bundle identity, context, or
  digest change, invalidates the receipt and restarts bundle freeze, SOURCE
  admission, and independent TEST; the exemption cannot be inherited.
- Classification & Operational Security and Civilian Control, Law, Safety &
  Readiness are separate conjunctive assurance gates wherever applicable.
  Passing one, favorable economics, schedule pressure, or majority agreement
  cannot waive the other.
- Civilian authority, personnel safety, readiness, resilience, surge,
  recovery, and alliance obligations are non-waivable floors. Economics,
  review, delivery, handoff, and a later encoding cannot redefine them.
- All applicable stakeholder and harm results remain separate. Averages,
  composites, or fiscal totals cannot erase concentrated or tail harm.
- No contract creates force, procurement, operational, budget, allocation,
  rate, Taxlane-admission, official-use, implementation, or release authority.

## Detailed interface records

### `CONTRACT-SOURCE-001` — source, claim, and security custody

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-CST-001` / `ARC-SRC-001`; exact consumers are AUTH, RDY, ACQ, LOG, ALLY, DST, ECO, ADP, DEL, REV, and HND, with SRC transformations re-entering the gate; `SPEC-UNK-SEC-001`, `SPEC-UNK-SRC-001`, and release-context `SPEC-UNK-REL-001`. |
| Purpose and preconditions | Admit or reject public aggregate source material; preserve stable source/claim/derivation custody; issue fresh exact-output security posture for every producer, including SRC transformations and AUTH. Preconditions are identifiable public custody, known context and composition, accountable owner, and applicable security concurrence. |
| Required semantic payload groups | Source and source-version identity; publisher/custody/access/vintage/reuse/scope/aggregation/unit/denominator/cadence/revision/exclusion/limitation posture; claim class; exact derivation inputs/method/assumptions/units/uncertainty/output identity; artifact digest/context; direct and compositional security decision; safe hold/rejection reason; supersession and expiry/recheck posture. |
| Conditional groups | Visualization, audience, release-context, join, granularity, aggregation/redaction, and repeated-release context only when present; each changes the security context and triggers fresh review. |
| Prohibited groups | Prohibited content or reconstructive failure detail; implicit source replacement; unknown-to-zero conversion; security self-exemption for a custody transformation; release approval or domain conclusion. |
| Identity and version rules | Source identity is stable and non-reused. Revision creates a source successor. Every output-security posture binds the exact output digest, input identities, joins, derivation, context, audience, and expiry. A posture for one digest/context cannot be inherited by another. |
| Errors and invalidation | Unsafe, missing, stale, mismatched, irreconcilable, expired, or unknown posture yields hold/reject. Changed context invalidates prior posture and all dependent artifacts. Only safe non-reconstructive audit metadata may remain after rejection. |
| Owner, concurrence, and assurance | Public-evidence steward accountable; Security and aggregation steward and Classification & Operational Security concur where applicable; Citation Auditor and Scope Keeper inspect custody and boundary; affected semantic owners review use. No custodian self-approves its transformation. |
| Compatibility and breaking triggers | Compatible only when identities, claim/posture meanings, rejection strength, context bonds, and supersession remain interpretable without weakening a prior rejection. Any source, claim class, context trigger, safe-receipt content, expiry, security decision, producer, consumer, or ownership change is breaking and reopens upstream control. |
| Planned fixtures | Accepted public aggregate source; each prohibited-content boundary; incomplete custody; claim-class exhaustiveness; provenance-class separation; dangerous composition; context/expiry change; stale digest; safe non-reconstructive rejection; SRC-transformation self-exemption; per-producer re-admission bypass. All planned, absent, and unexecuted. |

### `CONTRACT-AUTH-001` — mission and civilian authority

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-AUT-001` / `ARC-AUTH-001`; after SOURCE re-admission, exact applicable consumers are RDY, ACQ, LOG, ALLY, DST, ECO, ADP, DEL, REV, and HND; `SPEC-UNK-RDY-001`, `SPEC-UNK-SRC-001`, and `SPEC-UNK-SEC-001`. |
| Purpose and preconditions | Carry an immutable public mission abstraction and lawful civilian authority posture without choosing a mission or decision. Inputs must be admitted public authority sources and the resulting AUTH output must pass fresh `CONTRACT-SOURCE-001` security re-admission before any dependent use. |
| Required semantic payload groups | Mission-abstraction identity/version; public authority source; jurisdiction; accountable decision owner; non-delegable decisions; effective period; analytic boundary; exclusions; ambiguity/hold posture; Scope Keeper concurrence; Civilian Control, Law, Safety & Readiness assurance; exact output digest/context and fresh source/security posture. |
| Conditional groups | Scenario assumptions and narrowed successor authority only when explicitly bounded and source-backed. A narrowing retains the predecessor and rationale. |
| Prohibited groups | Force structure/employment, targets, tactics, procurement, budget, allocation, rate, operational method, official recommendation, broadened authority by default, or delegated non-delegable decision. |
| Identity and version rules | Authority change creates an immutable successor. A successor may narrow authority but cannot silently broaden it. Dependent artifacts bind the exact AUTH identity/version/digest/security posture. |
| Errors and invalidation | Missing, ambiguous, stale, unsafe, expired, mismatched, or broadened authority holds every dependent branch. Any authority/source/context change invalidates dependent artifacts and requires a new AUTH version and security posture. |
| Owner, concurrence, and assurance | Civilian mission and authority steward accountable; Scope Keeper and Civilian Control, Law, Safety & Readiness required; Classification & Operational Security independently accepts the exact AUTH output composition. |
| Compatibility and breaking triggers | Compatible only when mission, jurisdiction, owner, period, non-delegable decisions, exclusions, and no-authority semantics are preserved or narrowed. Any broadened authority, changed consumer, owner, concurrence, context bond, or prohibited-decision boundary is breaking and may require SPEC/ARCH reopening. |
| Planned fixtures | Complete authority manifest; missing/ambiguous authority; silent broadening; non-delegable decision transfer; prohibited output; stale period; AUTH security-bypass; dangerous AUTH composition; changed-context invalidation. All planned, absent, and unexecuted. |

### `CONTRACT-RDY-001` — readiness, safety, resilience, and floors

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-DOM-001` / `ARC-RDY-001`; direct consumers are ECO and DEL, with REV inspection through TEST and ADP/HND receiving only the later final ECO plus delivery bond; `SPEC-UNK-RDY-001`, `SPEC-UNK-LOG-001`, and quantitative downside `SPEC-UNK-QNT-001`. |
| Purpose and preconditions | Preserve public aggregate readiness and safety promises as separate, reviewable facets and hard floors. Inputs require admitted source and authority postures plus compatible domain evidence. |
| Required semantic payload groups | Promise identity; authority/system boundary; measure/denominator/period/horizon; evidence rule; separate staffing, training, personnel-safety, availability, integration, maintenance, supply, repair, resilience, surge, mobilization, and recovery postures; distribution/tail and degraded-mode treatment; floor and hold rule; reconciliation across definitions/boundaries/periods/vintages; bounded downside and failure evidence. |
| Conditional groups | Reviewed `not_applicable` with reason and alternative boundary where a facet genuinely does not apply; quantitative range/probability only under accepted method. |
| Prohibited groups | Spending, inventory, planned value, or average as readiness proof; composite replacement; missing facet; operationally useful detail; fiscal or delivery authority. |
| Identity and version rules | Each promise/facet/result has stable identity and version. Changed authority, definition, denominator, boundary, horizon, floor, evidence rule, or degraded case creates a successor and invalidates affected ECO/DEL/review bonds. |
| Errors and invalidation | Missing, unreconciled, proxy-only, unsafe, failed-floor, absent-tail, or incompatible evidence yields null/hold/reject as controlled. A failed floor blocks fiscal, delivery, adaptation, review promotion, and handoff. |
| Owner, concurrence, and assurance | Readiness-system analyst accountable; Operational Readiness Officer; Civilian Control, Law, Safety & Readiness; Classification & Operational Security; Logistics & Sustainment; Mission User; Service-Member/Family and affected harm lenses participate. |
| Compatibility and breaking triggers | Compatible only when facet identity, denominator, horizon, distribution/tail, degraded treatment, and floor meaning remain explicit. Any facet merge, proxy acceptance, floor weakening, consumer widening, or safe-abstraction change is breaking. |
| Planned fixtures | Complete eleven-facet promise; omitted facet; failed/missing floor; proxy-only and average-only cases; incompatible series; bounded degraded case; missing downside; reviewed N/A; unsafe detail; downstream floor-bypass. All planned, absent, and unexecuted. |

### `CONTRACT-ACQ-001` — acquisition, industrial base, and commonality

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-DOM-001` / `ARC-ACQ-001`; exact consumers are DST, ECO, and DEL, with independent review through TEST; `SPEC-UNK-ACQ-001` and distribution `SPEC-UNK-DST-001`. |
| Purpose and preconditions | Carry safe aggregate acquisition/industrial-base evidence while preserving delivery capacity, supplier incidence, lifecycle risk, and six distinct commonality facets. Inputs require admitted source/AUTH and accepted aggregation. |
| Required semantic payload groups | Bounded program/asset-class and supplier-segment identities; requirements stability; cycle time; competition; qualification; production/repair capacity; concentration; technical-data/IP constraints; workforce/facilities; cash flow; demand stability; learning; transition; surge; separate shared-support value, unique-system need, concentration effect, transition effect, interoperability effect, and common-mode/unique-system failure risk; each facet's evidence/method/units/uncertainty/posture; lifecycle and supplier-incidence effects. |
| Conditional groups | Reviewed N/A for an inapplicable facet; quantitative capacity or concentration only under accepted methods and safe aggregation. |
| Prohibited groups | Composite commonality score; supplier-sensitive or exploitable detail; vendor assertion or gross obligation as delivery proof; supplier exit, brittle concentration, unpriced qualification, delayed capability, or shifted burden as efficiency; procurement selection. |
| Identity and version rules | Program/asset class, supplier segment, lifecycle stage, facet, evidence, and result identities remain distinct. Changed aggregation, supplier boundary, evidence tier, facet definition, or lifecycle basis creates a successor. |
| Errors and invalidation | Missing facet/evidence, unsafe detail, vendor-only support, incomplete competition, incompatible boundary, or unpriced lifecycle effect holds affected capacity, schedule, candidate, fiscal, distribution, and delivery branches. |
| Owner, concurrence, and assurance | Acquisition and industrial-base analyst accountable; Acquisition & Industrial-Base Lead, supplier/workforce/community lenses, Independent Test & Oversight, Classification & Operational Security, Civilian Control/Law/Safety/Readiness, Citation, Numeracy, and Scope as applicable. |
| Compatibility and breaking triggers | Compatible only when all six facets, supplier partitions, units, lifecycle boundaries, evidence posture, and nulls remain separate. Facet collapse, changed safe aggregation, owner/consumer change, or lifecycle/category conversion is breaking. |
| Planned fixtures | Complete six-facet result; one omitted facet each; composite-score rejection; unsafe supplier granularity; vendor-only evidence; incomplete competition; supplier-exit false efficiency; lifecycle-cost omission; reviewed N/A. All planned, absent, and unexecuted. |

### `CONTRACT-LOG-001` — logistics, inventory, repair, and sustainment

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-DOM-001` / `ARC-LOG-001`; exact consumers are RDY, DST, ECO, and DEL, with independent review through TEST; `SPEC-UNK-LOG-001` and unsafe-detail `SPEC-UNK-SEC-001`. |
| Purpose and preconditions | Preserve lifecycle-support custody, inventory boundaries, repair distributions/tails, degraded recovery, and security-safe logistics meaning. Inputs require admitted source/AUTH and compatible lifecycle boundaries. |
| Required semantic payload groups | Support-boundary identity; inventory stocks/condition states/ownership or custody/period/units/inclusions/exclusions/stock-policy basis/reconciliation; repair population/denominator/start-stop events/censoring/period/central measure/tails/uncertainty/degraded treatment; separate inventory and repair evidence/postures; technical data, workforce, facilities, spares, distribution, energy, maintenance, repair, upgrades, disposal, transition; deferred maintenance, obsolescence, cannibalization, queues, surge, recovery, supplier/workforce constraints, and lifecycle effects. |
| Conditional groups | Reviewed N/A for genuinely inapplicable inventory or repair facets; safely broadened abstraction after a separately admitted transformation. |
| Prohibited groups | Operationally useful bottleneck detail; average-only repair result; implicit stock boundary; missing-support-to-zero cost; purchase-price-only comparison; unsafe map/detail retained for convenience. |
| Identity and version rules | Stock boundary, repair distribution, workload/lifecycle basis, evidence version, and security context remain bound. Changed stock policy, population, censoring, period, aggregation, or degraded case creates a successor. |
| Errors and invalidation | Missing/incompatible inventory or repair evidence, unsafe detail, absent tail/censoring, irreconcilable lifecycle boundary, or security rejection holds/rejects dependent RDY/DST/ECO/DEL results; missing cost remains null/held. |
| Owner, concurrence, and assurance | Logistics and sustainment analyst accountable; Logistics & Sustainment Lead, Operational Readiness, Depot & Logistics Workforce, supplier, alliance, Mission User, both assurance roles, Citation, Numeracy, and Independent Test as applicable. |
| Compatibility and breaking triggers | Compatible only when stock, custody, units, time, start/stop, censoring, tails, degraded treatment, lifecycle, and security meaning remain explicit. Any average substitution, stock/repair basis change, owner/consumer widening, or unsafe detail change is breaking. |
| Planned fixtures | Complete inventory boundary; missing stock field; complete repair distribution; average-only, missing-tail, and censoring failures; incompatible boundary; security-rejected map/detail; zero-cost inference; degraded recovery and lifecycle omission. All planned, absent, and unexecuted. |

### `CONTRACT-ALLY-001` — alliance, interoperability, and sovereignty

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-DOM-001` / `ARC-ALLY-001`; exact consumers are DST, ECO, and DEL, with independent review through TEST; `SPEC-UNK-ALLY-001` and distribution `SPEC-UNK-DST-001`. |
| Purpose and preconditions | Preserve public commitments, interoperability, common logistics, sovereignty/control, partner assumptions, and party-separated incidence without operational or intent inference. Inputs require admitted public authority and safe evidence. |
| Required semantic payload groups | Commitment/standard/compatibility identities; public authority source; common logistics; partner capacity assumption; sovereign/export/control boundary; transition dependency; separate U.S., partner, shared, and externalized cost/benefit/burden/receipt/risk partitions; normal and degraded support results; uncertainty, null, and dissent. |
| Conditional groups | Reviewed N/A where a commitment, standard, or party partition genuinely does not apply; partner assumptions only when explicitly labelled and bounded. |
| Prohibited groups | Partner operational posture or intent inference; implied consent; cross-party netting; unilateral gain erasing interoperability or burden; unsafe partner detail. |
| Identity and version rules | Commitment, authority, standard, partner, compatibility, control, scenario, and party partition identities are stable and versioned. Any source, commitment, standard, control, or assumption change creates a successor. |
| Errors and invalidation | Missing/unsafe authority, obligation, partner evidence, transition cost, sovereign constraint, incidence, degraded result, uncertainty, or dissent holds the joint/burden/fiscal branch while preserving domestic results separately. |
| Owner, concurrence, and assurance | Alliance and interoperability analyst accountable; Alliance & Interoperability Strategist, Ally & Partner, Civilian Control/Law/Safety/Readiness, Classification & Operational Security, logistics, distribution, Defense Comptroller, Citation, Scope, and Independent Test as applicable. |
| Compatibility and breaking triggers | Compatible only when domestic/partner/shared/externalized partitions and sovereignty/control remain distinct. Any party netting, changed commitment/standard/control, unsafe inference, owner/consumer change, or degraded-path removal is breaking. |
| Planned fixtures | Complete party-separated posture; cross-party netting rejection; missing partner evidence; normal/degraded pair; omitted degraded case; unsafe intent inference; conflicting domestic/partner results; uncertainty/dissent omission. All planned, absent, and unexecuted. |

### `CONTRACT-DST-001` — stakeholder distribution and tails

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-DOM-001` / `ARC-DST-001`; direct consumers are ECO and DEL, with ADP/HND receiving only the later final ECO plus delivery bond; `SPEC-UNK-DST-001` plus applicable `SPEC-UNK-ACQ-001` and `SPEC-UNK-ALLY-001`. |
| Purpose and preconditions | Expose who bears benefit, harm, cost, delay, transition, risk, and tail effects without ranking human worth or collapsing stakeholders. Inputs are accepted domain results at safe aggregation. |
| Required semantic payload groups | Separate service-member/family, mission-user, civilian/depot/logistics-workforce, prime/small-supplier, installation-community, taxpayer/oversight, and ally/partner result identities; affected aggregate; denominator; period/horizon; baseline; uncertainty; burden category; central and tail/concentrated results; evidence posture; safety, tempo, staffing, retention, skills, moves, housing, health, caregiving, local services, environment, employment transition, supplier cash flow, and burden-shift outcomes where applicable; conflict, null, and dissent. |
| Conditional groups | Reasoned reviewed N/A for a genuinely inapplicable stakeholder or harm facet; safely broadened groups under accepted security posture. |
| Prohibited groups | Person-level records; composite priority/readiness/savings or human-worth score; averages replacing tails; cross-stakeholder netting; hidden cost/risk/burden shift. |
| Identity and version rules | Stakeholder lens, affected aggregate, denominator, time basis, burden category, evidence, and result version remain bound. Boundary, denominator, tail method, or evidence change creates a successor. |
| Errors and invalidation | Omitted lens/facet, unsafe aggregation, incompatible denominator, missing tail, hidden burden, cross-party netting, or unreviewed N/A holds efficiency, readiness, fiscal, delivery, adaptation, and handoff claims. |
| Owner, concurrence, and assurance | Personnel/family/workforce/community analyst accountable; all seven stakeholder roles, Service-Member & Family Advocate, Acquisition/Logistics/Alliance owners, Numeracy, Scope, Citation, both assurance roles, and Independent Test as applicable. |
| Compatibility and breaking triggers | Compatible only when stakeholder, denominator, period, tail, burden, null, conflict, dissent, and review meaning remain distinct. Any merge, composite, unsafe granularity, changed harm ownership, or consumer widening is breaking. |
| Planned fixtures | Complete seven-lens matrix; one omitted lens/facet each; average-only and hidden-tail cases; conflicting-result preservation; valid null; cross-lens netting; composite-score rejection; unsafe group; missing required reviewer. All planned, absent, and unexecuted. |

### `CONTRACT-ECO-001` — quantitative, fiscal, pathway, and adaptive envelope

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-PTH-001` / `ARC-ECO-001`; preliminary consumer DEL[n], final consumers ADP[n+1] and HND, with REV inspection through TEST; `SPEC-UNK-QNT-001`, `SPEC-UNK-DST-001`, `SPEC-UNK-ECO-001`, `SPEC-UNK-DEL-001`, and overlap/handoff `SPEC-UNK-HND-001`. |
| Purpose and preconditions | Preserve quantitative bases, six non-interchangeable pathways, separated federal and whole-system ledgers, peer limits, overlap, floors, and the immutable preliminary/final sequence. Inputs require accepted source/AUTH/domain/distribution postures and compatible dimensions. |
| Required semantic payload groups | Envelope/pathway identity and version; cited baseline/counterfactual; direct public cost reduction, process efficiency, avoided future cost/risk, readiness/capacity/resilience gain, lawful receipt effect, and null as separate states; near/medium/long horizon result/null/reviewed-N/A plus alternative boundary where required; units, quantities, price year, uncertainty method/range, downside, applicable probability and financial conventions; gross opportunity, realizable public savings, external benefit, lawful receipts, lifecycle, financing/collection, transition/implementation, risk, timing, overlap, and net pressure separately; budget authority, obligations, outlays, transfers, offsetting receipts, appropriation/account/period/fiscal owner separately; peer comparability/limits; realization authority/owner/capture/path/cost/schedule; floor/distribution results; predecessor, delivery bond, and supersession. |
| Conditional groups | Evidence-supported probability; discount/present-value/inflation/exchange/purchasing-power conventions; reviewed N/A and alternative time boundary; realization values only after all `SPEC-ECO-006` gates and mandatory DEL evidence pass. |
| Prohibited groups | Automatic pathway addition; avoided risk, capacity/readiness gain, external benefit, private/operator revenue, or peer gap converted to booked savings/receipt/target/quota/allocation/rate; implicit unit/price/horizon conversion; duplicate overlap; partial pathway envelope; fabricated owner/schedule/value. |
| Identity and version rules | Freeze preliminary `ECO[n]`; only `DEL[n]` consumes it. After DEL custody, security, and independent review, create immutable predecessor-linked final `ECO[n+1]`; only then may `ADP[n+1]` consume it. Preliminary/final/delivery identities and overlap keys remain stable. Later disposition/observation requests a new preliminary successor and repeats DEL. |
| Errors and invalidation | Missing/incompatible dimension, horizon, price basis, method, owner, floor, distribution, overlap, predecessor, delivery bond, review, or unknown closure yields held/null/rejected as controlled. Same-version DEL-to-ECO, ECO/ADP feedback, stale/missing predecessor, bypass, duplicate effect, false conversion, or in-place mutation rejects the proposed successor and preserves history. |
| Owner, concurrence, and assurance | Defense resource analyst accountable; Defense Comptroller owns federal accounting/realization semantics; Numeracy Checker, Delivery owner, applicable domain semantic owners, Scope Keeper, taxpayer/oversight, affected stakeholders, both assurance roles, and Independent Test participate. No fiscal result overrides a floor. |
| Compatibility and breaking triggers | Compatible only when six pathways, federal measures, party/account partitions, units, price bases, horizons, peer limits, overlap, null/N/A, floors, ownership, and predecessor/successor order remain lossless. Any category merge/conversion, equation/method, peer set, horizon, price basis, account, owner, overlap, cadence, floor, consumer, or version-order change is breaking and may reopen SPEC/ARCH. |
| Planned fixtures | All six pathway states; non-additivity and forbidden conversions; federal-measure matrix; complete/all-or-hold envelope; each missing envelope group; three horizons and reviewed N/A; incompatible basis; peer diagnostic/mandate misuse; overlap duplicates; realization-gate failure; `ECO[n]→DEL[n]→review→ECO[n+1]→ADP[n+1]` acceptance; same-version cycles, stale/missing predecessor, bypass, and mutation rejection. All planned, absent, and unexecuted. |

### `CONTRACT-TEST-001` — independent test and convergence

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-REV-001` / `ARC-REV-001`. Ordinary TEST findings return only to the exact accountable producer or promotion controller for the reviewed digest. A successful `IF-TERM-001` decision/receipt is consumed only by the external handoff gate to advance the unchanged exact admitted HND bundle directly to the external Taxlane handoff boundary; `ARC-HND-001` is not its semantic consumer and the receipt is not an HND input. Stage governance consumes convergence posture only. Blockers are `SPEC-UNK-SRC-001` and `SPEC-UNK-TST-001`. |
| Purpose and preconditions | Bind independent test, findings, dissent, and convergence to one frozen, security-admitted artifact digest without mutating the producer or creating authority. Ordinary findings request producer correction. Separately, `IF-TERM-001` records a finite successful terminal governance decision over an unchanged exact admitted HND bundle so that bundle may advance at the external boundary. Preconditions include a complete review packet, independent reviewer, required role set, and current digest/context. |
| Required semantic payload groups | Review/reviewer/subject identities; frozen subject digest/context and admitted posture; evidence manifest; derivations; gate matrix; negative cases; unresolved questions; reproduction/adverse/failure/uncertainty/dimensional/lifecycle/double-count results; stable finding identity, role, severity, affected claim, evidence, disposition, owner, destination, closure, independence, dissent; required role and assurance decisions. |
| Conditional groups | Successful `IF-TERM-001` receipt containing only the unchanged admitted bundle identity/posture, reviewer identity, terminal decision, date, and dissent. A defer requires named owner, destination, and substantive closure. |
| Prohibited groups | Producer mutation; reviewer self-approval; majority waiver; product/domain values in a finding or terminal receipt; terminal receipt as an `ARC-HND-001` input; inaccessible classified appeal or credentials replacing evidence; SOURCE re-admission or TEST re-review of the unchanged minimal receipt; recursive terminal review; external approval claim. |
| Identity and version rules | Every review binds one exact subject digest/context and role corpus version. Artifact change makes the review stale and requires a successor packet. Ordinary findings return through custody to the accountable producer for a new product version. The successful terminal receipt binds only the unchanged admitted HND bundle and accompanies its direct advance at the external handoff gate; it neither returns to nor changes HND. |
| Errors and invalidation | Stale digest, conflicted reviewer, missing role, failed assurance, incomplete packet/finding/defer, false approval, mismatched terminal bundle, product-bearing receipt, or unresolved critical/major finding blocks promotion. Any product/material/context/digest change invalidates the terminal receipt and restarts bundle freeze, SOURCE admission, and independent TEST. |
| Owner, concurrence, and assurance | Role review steward accountable; Independent Test & Oversight Officer owns applicable reproduction/adverse review; every applicable parliament, stakeholder, editorial, assurance, and methodology role acts independently; both assurance gates required where applicable. |
| Compatibility and breaking triggers | Compatible only when digest binding, evidence state, severity/disposition, owner/closure, independence, role requirements, dissent, and non-mutating behavior remain reproducible. Any role, severity, evidence, independence, convergence, terminal-metadata, consumer, or authority change is breaking. |
| Planned fixtures | Complete frozen packet; ordinary finding returned to its exact accountable producer; stale digest/context; seeded quantitative and qualitative failures; negative evidence retention; incomplete finding/defer; missing role; failed assurance; author/owner self-approval; false approval; terminal bundle mismatch; missing decision; product-bearing terminal receipt; changed material/context/digest restart; unchanged minimal terminal receipt exempt from SOURCE re-admission/re-review; direct unchanged-bundle advance with no `ARC-HND-001` receipt consumption. All planned, absent, and unexecuted. |

### `CONTRACT-DEL-001` — delivery, observation, feedback, and rollback

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-PTH-001` / `ARC-DEL-001`; SRC and REV consume the originating posture, ECO consumes reviewed DEL[n] only to form final ECO[n+1], and ADP/HND consume only with that final envelope; `SPEC-UNK-RDY-001`, `SPEC-UNK-ACQ-001`, `SPEC-UNK-LOG-001`, `SPEC-UNK-ALLY-001`, `SPEC-UNK-DST-001`, and `SPEC-UNK-DEL-001`. |
| Purpose and preconditions | Make delivery posture mandatory for every candidate, pathway, domain-floor, null, and research-hypothesis branch before realizability, final ECO, ADP, final review, or HND. Input preliminary `ECO[n]` and domain postures must be immutable, admitted, and compatible. |
| Required semantic payload groups | Candidate/pathway/domain-floor identity; authority; accountable owner; dependencies/resources; milestones; measures; safety/readiness/alliance floors; observation cadence; stop conditions; evaluation; rollback; realization-evidence plan; held/rejected/delivery-testable posture; exact cited baseline and peer posture; schedule, cost, burden, overlap, safety, readiness, supplier, workforce, community, and alliance deviations separately; trigger, exactly one stop/hold/revise/retire/replace action, evidence, rationale, version, owner, rollback, notification; preliminary ECO predecessor and final ECO successor bond. |
| Conditional groups | Research-hypothesis posture when later authority, ownership, capacity, measurement, or rollback is absent; observed-delivery groups only after later authority exists; reviewed N/A only for genuinely inapplicable realization fields, never for observation/custody ownership. |
| Prohibited groups | Omitted delivery record; same-version DEL-to-ECO or ECO/DEL reciprocity; mutation of preliminary ECO; fabricated owner, authority, schedule, cadence, effect, or zero; delivery readiness from planned evidence; cost-only observation hiding other deviations. |
| Identity and version rules | `DEL[n]` binds immutable preliminary `ECO[n]`. After custody/security/review it may support only explicitly predecessor-linked final `ECO[n+1]`; ADP and HND use the final envelope plus matching delivery bond. Observation feedback requests a later preliminary successor and repeats DEL. |
| Errors and invalidation | Missing authority/owner/resource/capacity/floor/measure/stop/evaluation/rollback/evidence, stale/mismatched predecessor, bypass, in-place mutation, or unreviewed deviation retains held research hypothesis or rejection, blocks final ECO/ADP/HND, and preserves preliminary evidence. |
| Owner, concurrence, and assurance | Delivery owner accountable; all applicable domain owners and the complete deviation/harm lenses; Independent Test; Numeracy; Scope; Classification & Operational Security; Civilian Control, Law, Safety & Readiness. A real-world realization owner is not replaced by the interface custodian. |
| Compatibility and breaking triggers | Compatible only when authority, owner, floors, measures, cadence, triggers/actions, rollback, deviations, notification, and predecessor/successor bonds remain linked and lossless. Any omission, order change, new action, floor weakening, owner/consumer change, or same-version edge is breaking. |
| Planned fixtures | Complete delivery posture; held research hypothesis; missing owner/authority/resource/floor/measure/stop/rollback; all ten seeded deviations; each trigger/action; missing notification; preliminary ECO preservation; cycle, stale/missing/mismatched predecessor, bypass, mutation; accepted ordered sequence; observation successor repeats DEL. All planned, absent, and unexecuted. |

### `CONTRACT-HND-001` — immutable held Taxlane candidate

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held/no emission; `PB-HND-001` / `ARC-HND-001`; exact HND consumer is external Taxlane only. SOURCE and TEST act as admission/review sidecars on the frozen bundle, not semantic consumers, and the terminal receipt never enters HND. Blockers are `SPEC-UNK-SEC-001`, `SPEC-UNK-RDY-001`, `SPEC-UNK-QNT-001`, `SPEC-UNK-DST-001`, `SPEC-UNK-ECO-001`, `SPEC-UNK-DEL-001`, and `SPEC-UNK-HND-001`. |
| Purpose and preconditions | Preserve a future immutable BASTION-side held or rejected `LaneEvidencePack` candidate for external Taxlane consideration without admission or semantic conversion. While `SPEC-UNK-HND-001`/`TBD-HND-001` is open, no pack is emitted. |
| Required semantic payload groups | BASTION/source/artifact/adapter identities and digests; final predecessor-linked ECO; ADP disposition; matching mandatory DEL bond; fresh source/security and applicable pre-terminal gate postures; six distinct pathways; separated fiscal measures/ledgers; nulls; uncertainty/downside; peer limits; distribution/tails; floors; realization owner/cadence/transition costs; overlap keys; provenance; semantic-owner concurrences; residual risk/dissent; held/rejected BASTION posture; external Taxlane ownership marker. The terminal receipt is excluded from this payload. |
| Conditional groups | Privacy/security-reviewed minimum non-reconstructive rejection receipt. After the exact immutable candidate bundle is security-admitted and independently reviewed, a separate finite `IF-TERM-001` governance receipt may gate its external advance; that receipt is neither HND payload nor input. A later shared mapping exists only after both repositories accept it. |
| Prohibited groups | Fabricated value; category conversion; omitted null/risk/dissent/floor/owner; terminal receipt consumed by or copied into HND; inferred Taxlane admission, combination, allocation, rebalance, rate, official use, or publication; Taxlane backflow; generated mirror as HND/Taxlane source; product values in terminal receipt. |
| Identity and version rules | HND forms one immutable bundle from accepted inputs, then binds its exact digest/context to source/security admission and the matching finite independent decision. A successful decision advances that unchanged admitted bundle directly at the external handoff boundary; it does not produce a new HND version or enter HND. Any product/material/context/digest change invalidates the receipt, creates a new bundle, and restarts freeze/SOURCE admission/independent TEST. An optional generated mirror occurs only after production and is never source. |
| Errors and invalidation | A stale, incomplete, unsafe, unreconciled, unowned, double-counted, falsely precise, floor-failing, mapping-incompatible, unconcurred, delivery-held/rejected, or digest-mismatched candidate remains held/rejected and emits no pack. A missing/mismatched terminal decision blocks advance at the external handoff gate without entering or mutating HND. Taxlane silence leaves BASTION held. |
| Owner, concurrence, and assurance | Taxlane adapter steward accountable for mapping/held posture; every source semantic owner concurs on mapped meaning; Defense Comptroller, Numeracy, Citation, Scope, affected stakeholders, Independent Test, and both assurance roles review. Taxlane alone owns external admission. |
| Compatibility and breaking triggers | Compatible only when identities, six pathways, federal and whole-system ledgers, nulls, uncertainty, gates, floors, distribution, delivery, overlap, provenance, dissent, held state, and external ownership round-trip without loss. Any mapping, field-group, digest, gate, owner, overlap, terminal-receipt, external-authority, consumer, or admission change is breaking and requires shared review; authority change may reopen ARCH/SPEC. |
| Planned fixtures | Semantically complete held candidate under hypothetical closed holds; each named rejection state; category-loss/round-trip; missing value no fabrication; stale/context change; delivery hold; bundle digest mismatch; stale posture; missing terminal decision; product-bearing receipt; minimal non-product receipt not consumed by HND; unchanged admitted bundle advances directly at the external gate; changed material/context/digest restarts freeze/SOURCE/TEST; false Taxlane state; generated-mirror backflow. All planned, absent, and unexecuted; no pack is produced now. |

### `CONTRACT-REL-001` — closed no-output release posture

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; current closed/held/no output; `PB-DOC-001` / `ARC-REL-001`; exact consumer set is empty; `SPEC-UNK-SEC-001` and `SPEC-UNK-REL-001`. |
| Purpose and preconditions | Preserve the current closed release boundary. It records governance posture only and has no product producer, payload emission, consumer, or public route. Any future release requires separate authority, requirements, composition/security controls, and fixed point. |
| Required semantic payload groups | No-release posture; controlling authority absence; applicable unknown identities; governance owner and review identity. These remain governance records, not a product output. |
| Conditional groups | None under current authority. Future audience, artifact, composition, provenance, correction/takedown, or communication groups are ineligible until a separately authorized release chain exists. |
| Prohibited groups | Product or public artifact; release approval; audience/transport; route from review/HND/generated/runner to publication; context-stripped communication; privacy/security pass interpreted as release authority. |
| Identity and version rules | The closed posture binds the controlling governance digest. Any proposed release creates a new separately authorized chain; no current artifact can be re-labelled as release-compatible. |
| Errors and invalidation | Any emission, consumer, release route, implied approval, or missing separate authority is rejected and invalidates the proposed release branch. `ARC-REL-001` emits nothing, so it does not enter recursive source re-admission. |
| Owner, concurrence, and assurance | Scope Keeper accountable; Classification & Operational Security and Citation Auditor required for any separately authorized future work; Civilian Control/Law/Safety/Readiness and affected roles as applicable. Current review cannot activate release. |
| Compatibility and breaking triggers | Current compatibility means only preservation of output-free/no-authority posture. Any audience, artifact, composition, context, correction, takedown, consumer, owner, or authority proposal is breaking and reopens the separately authorized upstream chain. |
| Planned fixtures | Unauthorized release; review-to-release, HND-to-release, generated-to-release, and runner-to-release route-around attempts; output-free REL inspection; context-loss and mosaicing only under future separate authority. All planned, absent, and unexecuted. |

### `CONTRACT-TRACE-001` — stage and trace authority

| Control | Encoding-neutral decision |
|---|---|
| Identity, custody, consumers, blockers | Version 1; planned/held; `PB-REV-001` / `ARC-REV-001`; exact consumers are BASTION maintainer, role review steward, and the exact stage controller, never a domain-value consumer; `SPEC-UNK-SRC-001` and `SPEC-UNK-TST-001`. |
| Purpose and preconditions | Preserve parent/child identities, gate decisions, evidence posture, and stage authority so later work cannot retroactively create approval. Inputs are frozen controlled artifacts and independent review decisions. |
| Required semantic payload groups | Stable mission/CONOPS/requirement/specification/architecture/package/interface and later design/work/verification/validation/evidence identities as applicable; parent digest; child digest; owning stage; gate decision; finding/defer/hold posture; evidence pointer posture; reviewer independence; supersession/invalidation relation; next eligible stage and explicit non-authorizations. |
| Conditional groups | Later implementation, verification, validation, work-package, and evidence links only after those controlled artifacts exist. Missing later links remain absent/held, not fabricated. |
| Prohibited groups | Domain values; product behavior; retroactive authority; planned evidence represented as executed; fixed-point decision without required roles/assurance; implementation or release readiness inferred from trace completeness. |
| Identity and version rules | Every trace/gate record binds exact parent and subject digests. A predecessor change makes dependent records stale until explicit reconciliation or regeneration. Supersession never rewrites the prior decision. |
| Errors and invalidation | Orphan, duplicate, stale, mismatched, missing-role, failed-assurance, unowned-defer, false-approval, or incomplete-evidence trace blocks stage advancement. A successor invalidates or requires digest-bound reconciliation of every dependent artifact. |
| Owner, concurrence, and assurance | BASTION maintainer accountable for repository stage truth; Role review steward owns convergence record; applicable independent parliament, stakeholder, editorial, methodology, and both assurance roles decide within their authority. |
| Compatibility and breaking triggers | Compatible only when parent/child identity, gate meaning, evidence posture, independence, invalidation, dissent, and non-authority remain visible. Any ID scheme, parent, gate, evidence, owner, role, accepted-risk, stage, or authority change is breaking. |
| Planned fixtures | Complete trace; orphan/duplicate; stale parent; digest mismatch; missing role/assurance; unowned defer; planned-as-executed evidence; premature stage advance; retroactive authority; successor invalidation/reconciliation. All planned, absent, and unexecuted. |

## Cross-contract ordering invariants

### Source/security and authority

```text
admitted source posture
  -> AUTH semantic output
  -> fresh SOURCE security re-admission for exact AUTH digest/context
  -> dependent domain use

every product/material producer output, including SRC transformations
  -> fresh SOURCE security re-admission
  -> independent review
  -> exact authorized semantic consumers only

minimal non-product IF-TERM-001 governance receipt
  -> exempt from SOURCE re-admission and independent re-review
  -> external handoff gate only; never ARC-HND-001

REL
  -> no output
```

Neither public discoverability nor authority meaning is security acceptance.
Every changed or composed output repeats the gate, and a failed gate
invalidates all dependent branches.

### Preliminary ECO, mandatory delivery, final ECO, and adaptation

```text
immutable preliminary ECO[n]
  -> mandatory DEL[n]
  -> SOURCE custody and exact-context security re-admission
  -> independent TEST review
  -> immutable predecessor-linked final ECO[n+1]
  -> ADP[n+1]
```

No preliminary `ECO[n]` consumes `DEL[n]`; no delivery record mutates its ECO
predecessor; no ADP consumes a preliminary envelope. Later ADP disposition or
DEL observation returns through custody/security/review to request a later
preliminary successor, which repeats mandatory delivery before another final
envelope. Missing or mismatched predecessor, stale posture, bypass, reverse
same-version edge, or in-place mutation fails closed.

### Finite held handoff

```text
final ECO + ADP + matching DEL bond + semantic concurrences
  -> HND immutable BASTION candidate bundle
  -> SOURCE exact bundle admission
  -> independent TEST over unchanged bundle
     -> ordinary finding: accountable producer correction and a new bundle
     -> successful IF-TERM-001: minimal non-product decision/receipt
  -> unchanged exact admitted HND bundle advances directly
  -> external Taxlane handoff boundary (no BASTION admission)
```

The terminal receipt is minimum non-reconstructive governance metadata only:
unchanged admitted bundle identity/posture, reviewer/decision/date/dissent. It
contains no product content, is not an HND input, and is expressly exempt from
SOURCE re-admission and independent re-review, so it creates no recursive
review. Any product/material/context/digest change invalidates the receipt and
restarts bundle freeze, SOURCE admission, and independent TEST. BASTION can
produce only held/rejected state; Taxlane alone may admit, combine, allocate,
rebalance, or test/set rates, and no Taxlane decision flows back as BASTION
evidence or gate authority.

## Compatibility and change control

Compatibility is semantic and consumer-specific. A change is compatible only
when every previously accepted identity, posture, category distinction,
failure, unknown, owner/concurrence, security context, version bond, and
consumer interpretation remains lossless and no new authority or consumer is
introduced. Optionality cannot make a promotion-gating group silently
disappear. A new representation that preserves bytes but changes controlled
meaning is incompatible.

For `IF-TERM-001`, compatibility additionally requires the ordinary-finding
producer return, terminal-receipt external-gate-only consumer, unchanged-bundle
direct advance, minimal non-product fields, SOURCE/TEST recursion exemption,
and change-triggered freeze/admission/review restart to remain exact. Making
`ARC-HND-001` a receipt consumer or input is breaking.

Any contract ID/version, producer, consumer, physical custodian, semantic
payload group, posture, required/conditional/prohibited classification,
identity/digest/context rule, predecessor/successor relation, failure,
invalidation, owner, concurrence, assurance, compatibility rule, or planned
fixture change creates a new `INTERFACES.md` digest and requires independent
interface, affected domain, editorial, methodology, and both assurance
reviews. The author or affected owner cannot self-approve.

A change that alters logical producer/consumer direction, component
responsibility, source/security re-admission, AUTH gating, ECO/DEL/ADP order,
finite terminal review, Taxlane/release authority, package dependency, or a
protected set reopens `ARCHITECTURE.md` or `PACKAGE_BOUNDARIES.md` first. A
change to specification meaning, controlled terminology, accountable owner,
required concurrence, expected result, or a `SPEC-UNK-*` dependency/closure
reopens `SPECIFICATION_BASELINE.md` and its parent requirement where
applicable. A successor invalidates or requires explicit digest-bound
reconciliation of every dependent design, verification, fixture, work package,
implementation, generated artifact, review, handoff, and release record.

No common or shared implementation is authorized here. Extraction remains
subject to the three-independent-domain proof and fixed-contract rule in the
package baseline. Co-location cannot widen consumers or move defense semantics
into a common envelope.

## Planned interface fixture register

Fixtures are semantic plans only. No fixture path, representation, generator,
command, or expected byte sequence is selected or present.

| Fixture family | Contracts covered | Required planned branches | State |
|---|---|---|---|
| Custody/security | SOURCE, AUTH, every product producer, TEST terminal receipt, REL | accepted source; direct prohibited content; dangerous composition; SRC transformation and AUTH bypass; stale/changed context; downstream invalidation; safe receipt; unchanged minimal terminal-governance receipt exemption; product-bearing or changed-material/context/digest receipt restart; REL output refusal | planned; absent; unexecuted |
| Domain preservation | RDY, ACQ, LOG, ALLY, DST | complete, null, reviewed N/A, missing facet, incompatible basis, tail/degraded case, composite/netting rejection, unsafe aggregation, owner/concurrence failure | planned; absent; unexecuted |
| Fiscal and version order | ECO, DEL | six pathways; federal measures; all-or-hold envelope; horizons/peers/overlap; realization failure; ordered predecessor/successor acceptance; cycle, stale, missing predecessor, bypass, mutation, and fabricated-value refusal | planned; absent; unexecuted |
| Independent review and trace | TEST, TRACE | frozen digest; reproduction/adverse failure; role/assurance/self-approval; finding/defer completeness; orphan/stale trace; successor invalidation; planned-as-executed refusal | planned; absent; unexecuted |
| Terminal handoff | HND, SOURCE, TEST | complete held candidate only after hypothetical hold closure; semantic round trip; every named rejection; exact bundle posture; ordinary findings return to accountable producers; minimal terminal receipt is not consumed by HND and receives no SOURCE/TEST recursion; unchanged exact admitted HND bundle advances directly to the external gate; product/material/context/digest change invalidates and restarts; no inferred Taxlane state or generated backflow | planned; absent; unexecuted |
| Closed release | REL, SOURCE, TRACE | output-free current posture; every route-around attempt; separate-authority requirement; no false approval | planned; absent; unexecuted |

No planned fixture is accepted evidence. Future `VERIFICATION.md`, design, and
accepted work packages must define representations, expected results,
commands, and custody without weakening these semantics. Product boundaries
must never depend on `PB-TST-001` or `PB-FIX-001`.

## Protected-set and role preservation

This interface baseline preserves by reference:

- all 98 controlled SPEC allocations and 98 planned VER identities;
- all 13 fixed contract IDs and exact logical producers;
- all 10 nonfunctional constraints;
- all 13 exact `SPEC-UNK-*` controls and their dependent hold sets;
- all 13 logical components and their physical allocations;
- all 21 parliament, stakeholder, editorial, assurance, and methodology role
  files; and
- both independent, conjunctive assurance gates and the prohibition on author
  or owner self-approval.

The thirteen unknowns remain open: `SPEC-UNK-SEC-001`, `SPEC-UNK-RDY-001`,
`SPEC-UNK-SRC-001`, `SPEC-UNK-QNT-001`, `SPEC-UNK-ACQ-001`,
`SPEC-UNK-LOG-001`, `SPEC-UNK-ALLY-001`, `SPEC-UNK-DST-001`,
`SPEC-UNK-ECO-001`, `SPEC-UNK-TST-001`, `SPEC-UNK-DEL-001`,
`SPEC-UNK-HND-001`, and `SPEC-UNK-REL-001`. This stage closes none, selects no
hidden default, and emits no held pack or release artifact.

### Exact role routing

Each of the 21 role files is routed exactly once below. Participation never
transfers semantic ownership or permits self-approval.

| Role file | Interface-control participation |
|---|---|
| `.roles/parliament/civilian-strategy-force-planner.md` | AUTH, RDY, DEL, TEST, and TRACE authority/delivery review |
| `.roles/parliament/operational-readiness.md` | RDY, LOG, DEL, and TEST readiness/degraded-path review |
| `.roles/parliament/acquisition-industrial-base.md` | ACQ, DST, ECO, and DEL supplier/capacity/transition review |
| `.roles/parliament/logistics-sustainment.md` | LOG, RDY, ALLY, and DEL lifecycle/degraded-recovery review |
| `.roles/parliament/defense-comptroller.md` | ECO, DEL, HND, TEST, and TRACE accounting/realization review |
| `.roles/parliament/service-member-family.md` | RDY, DST, and DEL personnel/family floor review |
| `.roles/parliament/independent-test-oversight.md` | TEST and TRACE, plus independent inspection of every producer |
| `.roles/parliament/alliance-interoperability.md` | ALLY, LOG, DST, and DEL interoperability/burden review |
| `.roles/editorial/citation-auditor.md` | SOURCE, TEST, HND, REL, TRACE, and every producer's custody review |
| `.roles/editorial/scope-keeper.md` | SOURCE, AUTH, HND, REL, TEST, and TRACE scope/no-authority review |
| `.roles/editorial/numeracy-checker.md` | ECO, DEL, HND, TEST, and TRACE dimensional/accounting review |
| `.roles/assurance/classification-operational-security.md` | SOURCE re-admission for every producer; HND, TEST, REL, and TRACE security assurance |
| `.roles/assurance/civilian-control-law-safety-readiness.md` | AUTH, RDY, all domain floors, DEL, HND, TEST, and TRACE civilian/safety assurance |
| `.roles/stakeholders/mission-user.md` | RDY, ACQ, LOG, ALLY, DST, and DEL mission-support review |
| `.roles/stakeholders/service-member-family.md` | RDY, DST, and DEL safety/tempo/family-burden review |
| `.roles/stakeholders/depot-logistics-workforce.md` | ACQ, LOG, DST, and DEL workforce/repair-capacity review |
| `.roles/stakeholders/prime-small-supplier.md` | ACQ, DST, and DEL supplier/competition/cash-flow review |
| `.roles/stakeholders/installation-community.md` | DST, ECO, and DEL community/transition review |
| `.roles/stakeholders/ally-partner.md` | ALLY, DST, and DEL sovereignty/interoperability review |
| `.roles/stakeholders/taxpayer-oversight.md` | ECO, DEL, TEST, TRACE, and HND auditability/delivery review |
| `.roles/panel-reviewer/panel.md` | TEST and TRACE methodology review across all thirteen contracts |

Role count: **21 rows, 21 unique repository role paths, zero missing or
duplicate substantive role files**. `.roles/ROLE.md` remains the governing
manifest and is not counted as a substantive role file.

## Deferred implementation decisions

The following remain explicitly unselected: Rust or other language-level
types, traits, functions, ownership/lifetime model, error types, crate
features, dependency versions, API visibility, schema/file/event/config names,
field names, cardinality, byte order, numeric precision, serialization,
transport, CLI, environment/configuration, persistence, storage, retention,
algorithms, quantitative thresholds, performance limits, runtime, deployment,
and Taxlane integration mechanism.

L0, L1, and L2 interface validation commands are unavailable and unexecuted
because no representation, package, fixture, test target, generator, or
implementation exists. This authoring stage performs document inspection only;
it does not claim verification or validation evidence.

## Disposition

Disposition: **review-ready encoding-neutral interface-control baseline with
13 inherited open holds; not a fixed point**.

Independent digest-bound interface, applicable domain-role, stakeholder,
editorial, methodology, and both assurance reviews are required before a
fixed-point decision. `DESIGN.md`, verification planning, work packages,
packages, Cargo/Rust implementation, corpus work, generated artifacts,
Taxlane action, and release remain unauthorized.

No package, crate, module, Rust source, interface representation, schema, API,
file format, CLI, transport, storage, runtime, deployment, fixture, test,
generator, corpus, work package, implementation, Taxlane state, official
action, commit, push, remote mutation, or public release was created or
authorized.
