# BASTION Logical Architecture

## Status and controlled input

Repo: BASTION

Architecture state: **review-ready draft; not a fixed point**.

Controlled `SPECIFICATION_BASELINE.md` input SHA-256:
`48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b`

This artifact allocates settled specification responsibilities to conceptual
components and defines their permitted relationships. It is a logical
architecture only. It does not select crates, packages, modules, languages,
APIs, schemas, encodings, algorithms, storage products, deployment topology,
security mechanisms, force structure, procurement, budgets, rates,
operational methods, or sensitive thresholds.

Physical ownership and build boundaries are deferred to
`PACKAGE_BOUNDARIES.md`. Contract fields and encodings are deferred to
`INTERFACES.md`. Neither later artifact may weaken this architecture or create
authority absent from the controlled specification baseline.

## Scope

BASTION is an internal public-evidence research and review system for reasoning
about defense as a mission-readiness network rather than as an equipment,
organization, account, or spending list. The architecture supports safe source
custody, lawful civilian authority, readiness and safety floors, acquisition,
logistics, alliance, distribution, economics, adaptive lifecycle, independent
review, delivery evidence, and a held Taxlane handoff.

The scope remains aggregate, unclassified, non-operational, planning-only, and
non-official. Peer gaps are diagnostic comparators only. They are not savings,
funding targets, force judgments, allocation instructions, or rate
instructions.

## Architecture summary

The architecture uses thirteen domain-aligned conceptual components. Source
and authority controls form the upstream admission boundary. Readiness,
acquisition, logistics, and alliance components preserve distinct domain
semantics. Distribution analysis makes affected parties and tails visible.
Economics keeps six non-additive pathways and fiscal ledgers separate. An
adaptive evaluator manages evidence-bound pathway lifecycle without performing
fiscal rebalance. Independent review is a sidecar gate across every promotable
artifact. Delivery posture is mandatory before realizability, final review,
adaptive disposition, or held handoff; delivery evidence can reopen a reviewed
pathway. Every derived, retained, visualized, composed, or emitted artifact is
digest- and context-bound and returns through source/security re-admission
before downstream use. The held adapter
may preserve a reviewed `LaneEvidencePack` candidate for Taxlane, but only
Taxlane can admit, combine, allocate, rebalance, or test rates. Release remains
closed under a separately authorized future boundary.

All changes occur through explicit new versions and supersession. No component
may silently mutate an upstream record, infer a missing value, convert a null
to zero, waive its own gate, or use successful verification to inflate its
authority.

## Package and language boundaries

All package, crate, module, workspace, language, runtime, process, deployment,
storage and trust-zone boundaries are **deferred to `PACKAGE_BOUNDARIES.md`**.
The component IDs below are logical responsibilities and must not be treated as
instructions to create one package, crate, process or repository per component.
No language or implementation technology is selected by this artifact.

## Logical components and total responsibility allocation

`Primary SPEC allocation` is exhaustive and exclusive: each of the 98
controlled `SPEC-*` rows appears under exactly one accountable logical
component. Other components may consume a result through a named logical
contract but do not acquire its semantic authority.

| Component ID | Logical responsibility | Primary SPEC allocation | Count | Direct REQ relationship |
|---|---|---:|---:|---|
| `ARC-SRC-001` | Public-source admission, safe rejection, claim identity, provenance, derivation, and digest/context-bound security re-admission of every derived or changed output, including its own transformations | `SPEC-SCP-001..005`, `SPEC-SCP-009..010`, `SPEC-SRC-001..008` | 15 | One-to-one to the same-suffix `BASTION-REQ-SCP-*` and `BASTION-REQ-SRC-*` rows. |
| `ARC-AUTH-001` | Mission abstraction, civilian authority, scope and no-authority-inflation controls whose derived manifest must pass source/security re-admission before dependent use | `SPEC-SCP-006..008` | 3 | One-to-one to `BASTION-REQ-SCP-006..008`. |
| `ARC-RDY-001` | Public aggregate readiness, safety, resilience, surge, recovery and degraded-support promises | `SPEC-RDY-001..007` | 7 | One-to-one to `BASTION-REQ-RDY-001..007`. |
| `ARC-ACQ-001` | Acquisition, industrial-base capacity, six-part commonality and supplier-risk analysis at safe abstraction | `SPEC-ACQ-001..008` | 8 | One-to-one to `BASTION-REQ-ACQ-001..008`. |
| `ARC-LOG-001` | Inventory, repair distributions, sustainment custody and lifecycle support at safe abstraction | `SPEC-LOG-001..008` | 8 | One-to-one to `BASTION-REQ-LOG-001..008`. |
| `ARC-ALLY-001` | Public alliance commitments, interoperability, sovereignty/control and partner-incidence separation | `SPEC-ALLY-001..006` | 6 | One-to-one to `BASTION-REQ-ALLY-001..006`. |
| `ARC-DST-001` | Personnel, family, workforce, supplier, partner and community distribution, concentration and tail analysis | `SPEC-DST-001..005` | 5 | One-to-one to `BASTION-REQ-DST-001..005`. |
| `ARC-ECO-001` | Quantitative basis, peers, six separate pathways, fiscal ledgers, overlap and all-or-hold envelope | `SPEC-ECO-001..011`, `SPEC-ECO-014..016`, `SPEC-ECO-018..020` | 17 | One-to-one to the enumerated same-suffix `BASTION-REQ-ECO-*` rows. |
| `ARC-ADP-001` | Delivery-gated pathway disposition, evidence-bound re-evaluation and retained lifecycle history without same-version mutation | `SPEC-ECO-012..013`, `SPEC-ECO-017` | 3 | One-to-one to `BASTION-REQ-ECO-012..013` and `BASTION-REQ-ECO-017`. |
| `ARC-REV-001` | Frozen-packet independent test, findings, dissent, convergence, trace and stage authority | `SPEC-TST-001..006`, `SPEC-VTR-001..003` | 9 | One-to-one to `BASTION-REQ-TST-001..006` and `BASTION-REQ-VTR-001..003`. |
| `ARC-DEL-001` | Mandatory candidate/pathway/domain-floor delivery posture, research-hypothesis custody, observations, deviations, triggers, stop/hold/revise/retire/replace actions and rollback evidence | `SPEC-DEL-001..007` | 7 | One-to-one to `BASTION-REQ-DEL-001..007`. |
| `ARC-HND-001` | Semantic preservation and fail-closed construction of a held `LaneEvidencePack` candidate | `SPEC-HND-001..007` | 7 | One-to-one to `BASTION-REQ-HND-001..007`. |
| `ARC-REL-001` | Closed release boundary and custody of future release unknowns | `SPEC-REL-001..003` | 3 | One-to-one to `BASTION-REQ-REL-001..003`. |
| **Total** |  |  | **98** | **All 98 REQ→SPEC relationships retained.** |

## Logical contract allocation

These are references to the thirteen versioned logical contracts already
settled in `SPECIFICATION_BASELINE.md`; they are not APIs or schemas.

| Contract ID | Accountable component | Required concurrence or consumers | Architecture behavior |
|---|---|---|---|
| `CONTRACT-SOURCE-001` | `ARC-SRC-001` | Security and aggregation steward and Classification and Operational Security reviewer concur where applicable | Admits only safe public evidence and re-admits every derived, retained, visualized, composed, or emitted output from `ARC-SRC-001`, `ARC-AUTH-001`, RDY, ACQ, LOG, ALLY, DST, ECO, ADP, DEL, REV, and HND for its exact digest and context; unsafe or unresolved input/output is held or rejected, and `ARC-REL-001` emits nothing. |
| `CONTRACT-AUTH-001` | `ARC-AUTH-001` | Civilian mission and authority steward; Scope Keeper and Civilian Control, Law, Safety & Readiness assurance as specified | May narrow authority; cannot silently broaden it or delegate non-delegable decisions; the immutable authority manifest must receive a fresh digest/context-bound `ARC-SRC-001` security posture before dependent use. |
| `CONTRACT-RDY-001` | `ARC-RDY-001` | `ARC-ECO-001` and `ARC-DEL-001` consume accepted floors; `ARC-REV-001`, `ARC-ADP-001`, and `ARC-HND-001` require the resulting mandatory delivery posture | Missing or failed floors hold downstream candidate, fiscal, delivery, review, adaptive and handoff claims. |
| `CONTRACT-ACQ-001` | `ARC-ACQ-001` | `ARC-DST-001`, `ARC-ECO-001`, `ARC-DEL-001` | Preserves six commonality facets and supplier incidence without a composite score. |
| `CONTRACT-LOG-001` | `ARC-LOG-001` | `ARC-RDY-001`, `ARC-DST-001`, `ARC-ECO-001`, `ARC-DEL-001` | Preserves stock, time, tail, censoring, lifecycle and degraded semantics. |
| `CONTRACT-ALLY-001` | `ARC-ALLY-001` | `ARC-DST-001`, `ARC-ECO-001`, `ARC-DEL-001` | Keeps U.S., partner, shared and externalized effects separate. |
| `CONTRACT-DST-001` | `ARC-DST-001` | `ARC-ECO-001` and `ARC-DEL-001` consume distribution posture; `ARC-ADP-001` and `ARC-HND-001` require the resulting mandatory delivery posture | Preserves stakeholder, denominator, tail, burden, null and dissent semantics and blocks delivery bypass. |
| `CONTRACT-ECO-001` | `ARC-ECO-001` | `ARC-DEL-001` consumes immutable preliminary `ECO[n]`; only a custody/security/review-bound delivery posture may be consumed by `ARC-ECO-001` as input to final realization `ECO[n+1]`; Defense Comptroller retains accounting authority | Keeps six pathways, federal measures, horizons, units, prices, floors, delivery evidence and overlap non-additive and versioned. Preliminary `ECO[n]` never consumes same-version DEL; final realization `ECO[n+1]` is immutable and must exist before `ARC-ADP-001` may dispose. |
| `CONTRACT-TEST-001` | `ARC-REV-001` | Independent reviewers; all artifact owners answer findings | Blocks fixed point or downstream promotion on stale/conflicted review or unresolved critical/major findings. |
| `CONTRACT-DEL-001` | `ARC-DEL-001` | `ARC-SRC-001` and `ARC-REV-001` consume posture in its originating sequence; `ARC-ECO-001` consumes it only to create predecessor-linked final realization `ECO[n+1]`; `ARC-ADP-001` and `ARC-HND-001` consume only that final envelope plus its delivery bond | Every candidate, pathway, null, and research hypothesis has a delivery posture before realizability, final review, adaptive disposition, or handoff. Missing later authority retains held research-hypothesis state; holds/rejections create a held/rejected successor economic posture and block adaptation/handoff. No same-version DEL→ECO edge exists. |
| `CONTRACT-HND-001` | `ARC-HND-001` | Semantic-owner concurrence is required for mapped fields; Taxlane owns any external admission response | Produces only held or rejected BASTION-side state; never fabricates or implies admission. |
| `CONTRACT-REL-001` | `ARC-REL-001` | Scope Keeper; Classification and Operational Security review for any separately authorized future work | Remains closed; no public release compatibility is claimed. |
| `CONTRACT-TRACE-001` | `ARC-REV-001` | BASTION maintainer and role review steward | Blocks advancement when parent/child identity, gate or evidence trace is incomplete. |
| **Total** |  |  | **13 contracts allocated.** |

## Nonfunctional-constraint allocation

The primary custodian below coordinates verification; each constraint still
applies to every specification named in its baseline parent set.

| Constraint | Primary custodian | Cross-component effect |
|---|---|---|
| `SPEC-NF-001` | `ARC-SRC-001` | Absolute prohibited-data boundary at every ingest, retain, derive and emit boundary. |
| `SPEC-NF-002` | `ARC-AUTH-001` | Every component preserves civilian control and mission/risk authority. |
| `SPEC-NF-003` | `ARC-RDY-001` | Missing or failed readiness/safety floors block candidate, savings, delivery and handoff states. |
| `SPEC-NF-004` | `ARC-RDY-001` | `ARC-LOG-001` and `ARC-DST-001` cannot replace tails or concentrated effects with averages. |
| `SPEC-NF-005` | `ARC-ECO-001` | Units, horizons, prices, accounts, parties and overlap reconcile before combination. |
| `SPEC-NF-006` | `ARC-ECO-001` | Pathways and non-cash outcomes remain non-additive and cannot be automatically converted. |
| `SPEC-NF-007` | `ARC-REV-001` | All target components preserve null/N/A integrity; missing is never zero. |
| `SPEC-NF-008` | `ARC-REV-001` | Provenance, identity, ordering and supersession are deterministic and historical. |
| `SPEC-NF-009` | `ARC-REV-001` | Admission, review and handoff reject stale digest or context. |
| `SPEC-NF-010` | `ARC-AUTH-001` | No successful test can create operational, procurement, budget, Taxlane, allocation, rate, implementation, official-use or release authority. |
| **Total** |  | **10 constraints allocated.** |

## Role and harm-gate allocation

This matrix allocates all 21 role files to logical review surfaces. It does not
transfer the accountable owners settled in the specification baseline. Each
applicable role records an independent `pass`, `finding`, or `defer` against the
exact artifact digest. An author, component owner, fiscal analyst, delivery
owner, or adapter steward cannot approve their own output. Both assurance gates
must independently pass every safety-, authority-, security-, or
composition-sensitive promotion. A failed assurance gate blocks promotion.

Every `SPEC-DEL-006` observation keeps schedule, cost, burden, overlap, safety,
readiness, supplier, workforce, community, and alliance deviations separate.
The role rows below identify mandatory harm lenses for those deviations; no
average, fiscal result, or majority vote can erase an applicable harm.

| Role file / lens | Components and mandatory gates | Harm or misuse check |
|---|---|---|
| `parliament/civilian-strategy-force-planner.md` — Civilian Strategy & Force Planner | `ARC-AUTH-001`, `ARC-RDY-001`, `ARC-DEL-001`, `ARC-REV-001`; authority and delivery gates | Mission/authority drift, inherited-structure circularity, disguised force or operational planning; safety/readiness and authority deviations. |
| `parliament/operational-readiness.md` — Operational Readiness Officer | `ARC-RDY-001`, `ARC-LOG-001`, `ARC-DEL-001`, `ARC-REV-001`; floor and delivery gates | Spending/inventory proxies for readiness; staffing, training, integration, supply, repair, surge, recovery, safety and readiness deviations. |
| `parliament/acquisition-industrial-base.md` — Acquisition & Industrial-Base Lead | `ARC-ACQ-001`, `ARC-DST-001`, `ARC-DEL-001`, `ARC-ECO-001`; candidate/delivery gates | Brittle concentration, supplier hollowing, unpriced qualification/transition; schedule, cost, supplier and workforce deviations. |
| `parliament/logistics-sustainment.md` — Logistics & Sustainment Lead | `ARC-LOG-001`, `ARC-RDY-001`, `ARC-ALLY-001`, `ARC-DEL-001`; readiness/delivery gates | Purchase-price-only comparison, hidden repair tails, missing spares/data/facilities/energy; schedule, cost, readiness, supplier, workforce and alliance deviations. |
| `parliament/defense-comptroller.md` — Defense Comptroller | `ARC-ECO-001`, `ARC-DEL-001`, `ARC-HND-001`, `ARC-REV-001`; accounting/realization/handoff gates | Gross opportunity converted to booked savings; account, lifecycle, transition, overlap, downside, cost and schedule deviations. |
| `parliament/service-member-family.md` — Service-Member & Family Advocate | `ARC-RDY-001`, `ARC-DST-001`, `ARC-DEL-001`; floor/distribution/delivery gates | Hidden personnel/family burden; tempo, safety, staffing, retention, housing, health, caregiving, moves, burden, workforce and community deviations. |
| `parliament/independent-test-oversight.md` — Independent Test & Oversight Officer | `ARC-REV-001` and every producer; independent test/convergence gate | Advocacy, vendor assertion, unverifiable classified appeal, missing adverse case, and unobservable schedule/cost/performance/failure claims; cannot review own authored output. |
| `parliament/alliance-interoperability.md` — Alliance & Interoperability Strategist | `ARC-ALLY-001`, `ARC-LOG-001`, `ARC-DST-001`, `ARC-DEL-001`; interoperability/distribution/delivery gates | Unilateral optimization, lost compatibility, sovereignty/control drift, partner burden shift; alliance, supplier, schedule and readiness deviations. |
| `editorial/citation-auditor.md` — Citation Auditor | `ARC-SRC-001`, every producer, `ARC-REV-001`; custody and output re-admission gates | Uncited claim, assertion laundering, lost redaction/aggregation/proxy/scenario/derivation boundary, or stale source/digest. |
| `editorial/scope-keeper.md` — Scope Keeper | `ARC-AUTH-001`, `ARC-SRC-001`, `ARC-HND-001`, `ARC-REL-001`, `ARC-REV-001`; scope/authority/release gates | Classified/controlled/operational expansion, targeting/force-employment content, false official plan, Taxlane inference, or release claim. |
| `editorial/numeracy-checker.md` — Numeracy Checker | `ARC-ECO-001`, `ARC-DEL-001`, `ARC-HND-001`, `ARC-REV-001`; numeracy/accounting gates | Unit, denominator, price-year, horizon, lifecycle, transition, uncertainty, overlap or double-count error; cost/schedule deviation suppression. |
| `assurance/classification-operational-security.md` — Classification & Operational Security | `ARC-SRC-001` security re-admission for every component, `ARC-REV-001`, `ARC-HND-001`, `ARC-REL-001`; mandatory assurance gate | Direct prohibited content, dangerous public-field composition, reconstructive failure receipt, stale context, or unsafe visualization/audience/release context. |
| `assurance/civilian-control-law-safety-readiness.md` — Civilian Control, Law, Safety & Readiness | `ARC-AUTH-001`, `ARC-RDY-001`, `ARC-DEL-001`, `ARC-REV-001`, `ARC-HND-001`; mandatory assurance gate | Unlawful authority, mission/risk drift, personnel harm, failed floor, or financial optimization overriding safety/readiness; safety and readiness deviations. |
| `stakeholders/mission-user.md` — Mission User | `ARC-RDY-001`, `ARC-ACQ-001`, `ARC-LOG-001`, `ARC-ALLY-001`, `ARC-DEL-001`; readiness/delivery review | Unreliable, non-integrated, unsupported capability or hidden degraded behavior; readiness, schedule and alliance deviations without operational detail. |
| `stakeholders/service-member-family.md` — Service Member & Family | `ARC-RDY-001`, `ARC-DST-001`, `ARC-DEL-001`; floor/distribution/delivery review | Unsafe equipment/operations, unsustainable tempo, training/retention/housing/health/move/family burdens; safety, burden, workforce and community deviations. |
| `stakeholders/depot-logistics-workforce.md` — Depot & Logistics Workforce | `ARC-LOG-001`, `ARC-ACQ-001`, `ARC-DST-001`, `ARC-DEL-001`; sustainment/delivery review | Unrealistic repair schedule, missing technical data/facilities/skills/spares, unsafe workload; schedule, cost, safety, supplier and workforce deviations. |
| `stakeholders/prime-small-supplier.md` — Prime & Small Supplier | `ARC-ACQ-001`, `ARC-DST-001`, `ARC-DEL-001`; industrial/distribution/delivery review | Unstable requirements/demand, cash-flow/qualification/IP/competition burden, brittle capacity; supplier, workforce, schedule and cost deviations. |
| `stakeholders/installation-community.md` — Installation Community | `ARC-DST-001`, `ARC-DEL-001`, `ARC-ECO-001`; distribution/delivery review | Shifted employment, housing, utility, environment, safety, land, local-service or transition burden; community, burden, safety and cost deviations. |
| `stakeholders/ally-partner.md` — Ally & Partner | `ARC-ALLY-001`, `ARC-DST-001`, `ARC-DEL-001`; alliance/distribution/delivery review | Unexamined sovereign constraint, unstable commitment, incompatibility, burden transfer, or unpredictable collaboration; alliance and burden deviations. |
| `stakeholders/taxpayer-oversight.md` — Taxpayer & Oversight Body | `ARC-ECO-001`, `ARC-DEL-001`, `ARC-REV-001`, `ARC-HND-001`; accounting/delivery/handoff review | Weak mission linkage, unauditable lifecycle cost, hidden uncertainty/failure/null, noncompetitive burden, or unmeasured delivery; cost, schedule and overlap deviations. |
| `panel-reviewer/panel.md` — BASTION Methodology Panel | `ARC-REV-001` across all components; methodology and fixed-point gate | Method incoherence across public finance, acquisition, logistics, readiness, and civil-military/legal oversight; uses public aggregate unclassified evidence only and confers no external approval. |

All 21 roles are allocated exactly once in this matrix. Role dispositions are
retained with independence and dissent; role coverage never replaces the
accountable semantic owner or the two mandatory assurance decisions.

## Trust, authority and semantic boundaries

### Public-evidence and security boundary

Every component operates only on accepted public, aggregate, unclassified,
non-operational material. Every ingest, retention, derivation, visualization,
composition and emission is subject to source admission and compositional
security review.

This is a repeated output gate, not a one-time source check. Every derived,
retained, visualized, composed, or proposed emitted artifact produced by
`ARC-SRC-001`, `ARC-AUTH-001`, `ARC-RDY-001`, `ARC-ACQ-001`, `ARC-LOG-001`,
`ARC-ALLY-001`, `ARC-DST-001`, `ARC-ECO-001`, `ARC-ADP-001`, `ARC-DEL-001`,
`ARC-REV-001`, or `ARC-HND-001`
returns to `ARC-SRC-001` for a fresh security posture bound to the exact output
digest, input versions, joins, granularity, derivation, visualization,
composition, audience, release context, and expiry. No downstream use, review,
handoff, retention as an accepted result, or separately authorized future
release review may occur without that fresh posture. A changed join,
granularity, derivation, visualization, composition, audience, release context,
or expiry invalidates the posture, marks dependent artifacts stale, and reopens
source/security review. Failure is held or rejected and invalidates every bound
downstream branch; another component cannot waive or inherit the old posture.
Suppression, broadening, aggregation, redaction, or any other safe
transformation performed by `ARC-SRC-001` is itself a derived output and passes
the same gate; source/security custody cannot self-exempt a transformation.
`ARC-REL-001` is not an output producer and emits nothing.

The final handoff gate is finite. `ARC-HND-001` freezes one immutable candidate
bundle whose exact digest and complete context receive the latest accepted
`ARC-SRC-001` security posture. `ARC-REV-001` then records an independent
terminal decision/receipt over that unchanged admitted bundle. The terminal
record is governance metadata only: minimum non-reconstructive bundle identity,
admitted digest/context posture, review identity, decision, date, and dissent.
It contains no copied product values, new derivation, visualization,
composition, or product emission and therefore does not recursively create a
new candidate bundle. If review adds or changes any product/material content,
the bundle is no longer unchanged and must be frozen, security-re-admitted, and
reviewed as a new version. A terminal held handoff requires the bundle's latest
admitted digest/context and its matching independent decision; neither record
creates Taxlane or release authority.

No component may ingest, retain, derive, or emit classified information, CUI,
person-level service data, sensitive operational data, targeting content,
operational-planning content, or exploitable vulnerability content. This is an
absolute prohibition; `SPEC-UNK-SEC-001` holds the exact compositional-safety
method and cannot be used as permission to weaken the prohibition. Rejection
receipts retain only a safe, minimum, non-reconstructive identity, reason,
date, and review posture. They contain no rejected content, unsafe join or cell,
operational detail, reconstructive digest, visualization, or compositional
recipe and cannot enable reconstruction. The exact security method remains
held; this logical receipt boundary does not choose an encoding or threshold.

### Civilian control, law, safety and readiness

`ARC-AUTH-001` records the public mission abstraction, lawful civilian
authority, jurisdiction, owner, non-delegable decisions, period and analytic
boundary. It does not choose missions. No component may select or imply force
structure, force employment, targets, tactics, procurement, resource
allocation, budgets, rates, operational methods or official recommendations.

The Civilian Control, Law, Safety & Readiness assurance owner is accountable
for the prohibited-decision boundary and Scope Keeper concurrence is required
for promotion. `ARC-RDY-001` owns readiness and safety semantics; economics,
delivery and handoff components may consume an accepted posture but cannot
waive, redefine or trade away a floor.

### Fiscal and semantic authority

`ARC-ECO-001` preserves separate gross opportunity, realizable public delivery
savings, external benefit, private/operator revenue, legally
dedicated public receipts, collection/financing/transition/risk cost and net
public fiscal pressure. Avoided risk or service gain does not become booked
savings. Private or operator revenue is not a public receipt without a
reviewed legal claim. Peer gaps remain diagnostic only. `Gross opportunity` is
the controlled term from `BASTION-REQ-ECO-004`; `gross funding need` is not a
synonym and may be introduced only by a separately reviewed specification
change.

Each domain component owns its terms, units, limitations and acceptance
posture. Mapping a field across a contract requires the source semantic
owner's concurrence. `ARC-HND-001` preserves those meanings without conversion
and cannot resolve a conflict by relabeling, defaulting or selecting a more
convenient interpretation.

### Taxlane boundary

BASTION can research, validate, review, hold, revise, retire or replace a
domain pathway and can prepare a held `LaneEvidencePack` candidate. Taxlane
alone may admit a fiscal effect, combine it with other lanes, allocate funds,
rebalance a portfolio or test/set rates. A BASTION validation result is not a
Taxlane disposition. BASTION validates the lifecycle; it does not rebalance.

### Release boundary

`ARC-REL-001` is isolated and closed. No architecture path leads from a
research, reviewed, delivery or held artifact to public release. Any future
release needs separate authority, requirements, composition/security review
and fixed point. This architecture authorizes no public release.

## Logical interfaces

The following references identify semantic crossings only. Exact fields,
cardinality, encoding, transport and compatibility fixtures remain deferred to
`INTERFACES.md`.

| Interface reference | Producer → consumer | Required semantic posture |
|---|---|---|
| `LIF-SOURCE-ADMISSION` | `ARC-SRC-001` → every analytical component | Accepted source/version/digest/claim/security posture or an explicit hold/rejection. |
| `LIF-SECURITY-READMISSION` | `ARC-SRC-001`, `ARC-AUTH-001`, RDY, ACQ, LOG, ALLY, DST, ECO, ADP, DEL, REV, and HND derived-output producers → `ARC-SRC-001` security gate → every downstream consumer; `ARC-REL-001` has no output | Fresh posture bound to the exact output digest, sources, joins, granularity, derivation, visualization, composition, audience, release context, and expiry; source transformations use the same gate, failure invalidates downstream use, and only a safe non-reconstructive receipt remains. |
| `LIF-AUTHORITY-MANIFEST` | `ARC-AUTH-001` → `ARC-SRC-001` digest/context security posture → every dependent component | Immutable mission abstraction, lawful authority, owner, boundary, period and no-authority posture; missing/stale/unsafe AUTH output or bypass blocks every dependent use. |
| `LIF-READINESS-FLOOR` | `ARC-RDY-001` → `ARC-ECO-001`, then mandatory `ARC-DEL-001` posture before `ARC-REV-001`, `ARC-ADP-001`, or `ARC-HND-001` | Explicit result/null/reviewed-N/A, denominators, horizons, tails and degraded behavior; no downstream consumer may bypass delivery posture. |
| `LIF-DELIVERY-NETWORK` | `ARC-ACQ-001`, `ARC-LOG-001`, `ARC-ALLY-001` → `ARC-DST-001`, `ARC-ECO-001`, `ARC-DEL-001` | Domain-separated capacity, lifecycle, interoperability, risk and limitation results. |
| `LIF-DISTRIBUTION-POSTURE` | `ARC-DST-001` → `ARC-ECO-001`, then mandatory `ARC-DEL-001` posture before `ARC-ADP-001` or `ARC-HND-001` | Stakeholder/cohort, denominator, burden, concentration, tail, null and dissent; delivery must evaluate every applicable deviation. |
| `LIF-PATHWAY-ENVELOPE` | immutable preliminary `ARC-ECO-001[n]` → `ARC-DEL-001[n]` → `ARC-SRC-001` custody/security → `ARC-REV-001` → immutable final realization `ARC-ECO-001[n+1]` → `ARC-ADP-001[n+1]` | Six separate pathways, ledgers, horizons, uncertainty, overlap, owners, domain floors including `SPEC-ECO-006`, all-or-hold posture, and mandatory delivery posture. Missing predecessor, stale posture, bypass, or same-version DEL→ECO mutation holds; ADP receives only the final predecessor-linked envelope. |
| `LIF-REVIEW-PACKET` | every security-re-admitted artifact owner → `ARC-REV-001`; findings return through custody to a successor version | Frozen identity, digest, evidence, derivations, gates, delivery posture, negative cases, findings and dissent; no self-approval or in-place edit. For final handoff only, REV records a finite minimal non-reconstructive terminal decision over an unchanged admitted candidate bundle; new product content restarts freeze/re-admission/review. |
| `LIF-DELIVERY-POSTURE` | `ARC-DEL-001[n]` → `ARC-SRC-001` custody/security → `ARC-REV-001` → `ARC-ECO-001[n+1]`; `ARC-ADP-001` and `ARC-HND-001` only after the final envelope exists | Mandatory held/rejected/delivery-testable posture for every candidate, pathway, domain-floor result, null, and research hypothesis; missing later authority means held research hypothesis, never omission. Same-version ECO consumption is forbidden. |
| `LIF-DELIVERY-OBSERVATION` | later `ARC-DEL-001` observation → `ARC-SRC-001` custody/security → `ARC-REV-001` → later preliminary `ARC-ECO-001` successor → new mandatory `ARC-DEL-001` posture → custody/security/review → next final `ARC-ECO-001` realization | Exact baseline, accepted peer posture, all `DEL-006` deviations, trigger, action, owner, rollback, notification, and predecessor/successor identity; never same-version or stale backflow, and observation never bypasses the successor delivery gate. |
| `LIF-HELD-HANDOFF` | final realization envelope, ADP disposition, and mandatory delivery bond → `ARC-HND-001` immutable candidate bundle → `ARC-SRC-001` exact bundle security admission → `ARC-REV-001` finite terminal decision/receipt → external Taxlane boundary | Bundle digest/context must match the latest admitted posture. The terminal receipt is minimal non-reconstructive governance metadata with no copied/new product content and no recursive output gate; delivery/security hold or mismatch blocks external handoff and no state implies admission/allocation/rate. |
| `LIF-RELEASE-HOLD` | any component → `ARC-REL-001` | No-release posture only; no artifact emission. |

## Data and control flows

### Main evidence flow

```text
public candidate source
  -> ARC-SRC-001 admission / hold / safe rejection
  +  ARC-AUTH-001 civilian-authority manifest
  -> ARC-SRC-001 exact authority-output security re-admission
  -> ARC-RDY-001 | ARC-ACQ-001 | ARC-LOG-001 | ARC-ALLY-001
  -> ARC-DST-001 distribution and tail posture
  -> ARC-ECO-001[n] immutable preliminary bounded, non-additive pathway envelope
  -> ARC-DEL-001[n] mandatory delivery/research-hypothesis posture
  -> ARC-SRC-001 custody and digest/context-bound security re-admission
  -> ARC-REV-001 delivery-gate review
  -> ARC-ECO-001[n+1] immutable predecessor-linked final realization envelope
  -> ARC-ADP-001[n+1] proposed lifecycle disposition
  -> ARC-SRC-001 security re-admission
  -> ARC-REV-001 final frozen-packet review and convergence gate
  -> ARC-HND-001 immutable held LaneEvidencePack candidate bundle
  -> ARC-SRC-001 exact candidate-bundle security re-admission
  -> ARC-REV-001 finite terminal decision and minimal non-reconstructive receipt
  -> external Taxlane admit / hold / reject (Taxlane-owned)
```

`ARC-REV-001` is a sidecar control on every promotable arrow, not merely the
last step, and each reviewed derived artifact first passes
`LIF-SECURITY-READMISSION`. A review finding returns through custody to the
accountable producer as a requested new version; the review component cannot
edit the producer's artifact or approve its own work.

The terminal decision is finite because it records only bundle identity, the
admitted digest/context posture, review identity, decision, date and dissent.
It copies no product value and creates no new derivation, visualization,
composition or product content. Any product-content change creates a new
candidate bundle and restarts freeze, security re-admission and independent
review; the prior terminal receipt cannot admit the changed bundle.

### Observation and adaptation flow

```text
ARC-ECO-001 preliminary envelope[n]
  -> ARC-DEL-001 mandatory posture[n]
  -> ARC-SRC-001 custody + security re-admission
  -> ARC-REV-001 digest-bound review
  -> ARC-ECO-001 final realization envelope[n+1]
  -> ARC-ADP-001 preserve | revise | hold | retire | replace disposition[n+1]

ARC-ADP-001 disposition or ARC-DEL-001 observation (only when later authority exists)
  -> ARC-SRC-001 custody + security re-admission
  -> ARC-REV-001 digest-bound review
  -> ARC-ECO-001 new preliminary successor at a later version
  -> ARC-DEL-001 new mandatory posture at that version
  -> ARC-SRC-001 custody + security re-admission
  -> ARC-REV-001 delivery-gate review
  -> ARC-ECO-001 new final realization at the next version
```

An accepted trigger produces exactly one named action. Prior evidence,
rationale, owner, version, dissent and notification posture remain retained.
The preliminary `ARC-ECO-001[n]`, `ARC-DEL-001[n]` posture and final
`ARC-ECO-001[n+1]` are each immutable. DEL cannot update or be consumed by the
same ECO version: its reviewed posture realizes only the explicitly linked
final successor. ADP consumes only that final envelope. Disposition or later
delivery feedback can request a new preliminary successor, which must repeat
mandatory delivery, security custody and independent review before another
final envelope exists. Stale feedback, an absent or mismatched predecessor,
delivery bypass, or an in-place update fails closed. The loop can reopen
BASTION analysis; it cannot directly change Taxlane state. Any cross-lane
rebalance is a new Taxlane-owned decision outside this system.

### Null and negative-result flow

Null, rejected, held, failed, dissenting and negative results remain first-class
artifacts. Missing evidence routes to hold, never zero. A reviewed N/A must
include a reason and, where required, an alternative time boundary. A null
pathway can be the correct domain conclusion. It has a named observation and
custody owner, observation cadence, reopen triggers, predecessor/successor
history, evidence, rationale, dissent, and mandatory held delivery posture.
Only genuinely inapplicable realization-owner and realization-schedule fields
may use reasoned, independently reviewed N/A; the observation/custody owner is
never N/A. No component may fabricate a realization owner, schedule, cadence,
effect, or zero. The null travels through security re-admission, delivery and
independent review and may reach a held handoff without conversion into
savings, priority, realized effect, or omitted branch.

## Logical persistence and generated-artifact custody

This section assigns responsibility for retained logical records; it does not
choose storage, serialization or deployment.

| Logical artifact | Custodian | Retention and generation rule |
|---|---|---|
| Source inventory, source versions, output-security postures and safe rejection receipts | `ARC-SRC-001` | Stable identity, digest, custody, context and supersession; prohibited content and reconstructive failure detail are never retained, and every output posture binds its exact digest/context. |
| Authority manifests and concurrence records | `ARC-AUTH-001` | Versioned with mission, authority, jurisdiction, owner, period and scope; never inferred from an analysis result, and unusable until `ARC-SRC-001` records a fresh exact digest/context security posture. |
| Readiness/floor records | `ARC-RDY-001` | Preserve facet, denominator, horizon, tail, degraded case, null and failure evidence. |
| Acquisition/commonality results | `ARC-ACQ-001` | Preserve all six facets, lifecycle risks and safe aggregation separately. |
| Inventory, repair and sustainment records | `ARC-LOG-001` | Preserve units, boundaries, censoring, tails, lifecycle and security posture. |
| Alliance/interoperability results | `ARC-ALLY-001` | Preserve public commitment basis, sovereignty/control and party-separated effects. |
| Distribution/tail findings | `ARC-DST-001` | Preserve affected party, basis, concentration, burden, null and dissent. |
| Fiscal ledgers and pathway envelopes | `ARC-ECO-001` | Preserve separate measures, overlap keys, horizons, uncertainty and all-or-hold state; distinguish immutable preliminary and final realization versions and retain the reviewed DEL predecessor bond. |
| Lifecycle decisions and history | `ARC-ADP-001` | Preserve prior version, evidence, rationale, observation/custody owner, realization owner or reviewed N/A, schedule or reviewed N/A, cadence, trigger, notification and successor request without same-version mutation. |
| Review packets, findings and trace records | `ARC-REV-001` | Frozen digest, evidence, tests, findings, ownership, closure, independence and dissent remain reproducible; a terminal handoff receipt contains only minimal non-reconstructive governance metadata and no new product content. |
| Delivery postures, observations and actions | `ARC-DEL-001` | Preserve candidate/pathway/domain-floor identity, held/delivery-testable/rejected posture, exact baseline, schedule, cost, burden, overlap, safety, readiness, supplier, workforce, community, and alliance deviations, action, rollback and downstream notification. |
| Held handoff candidate or rejection | `ARC-HND-001` | Freeze an immutable candidate bundle and preserve semantic-owner concurrence, source digests, gates and external Taxlane ownership; handoff requires the exact bundle's fresh security posture and matching finite terminal decision, and never records inferred admission. |
| Release hold | `ARC-REL-001` | Preserve only no-release posture and future unknown references; generate no public artifact. |

Every generated artifact has one accountable logical custodian, a stable
identity, a version, its exact input identities and digests, a posture, an
owner and a supersession relation. Historical records are append-only in the
logical sense: correction creates a successor and never rewrites the evidence
used by an earlier run.

## Allowed and forbidden logical dependencies

### Allowed

- All analytical components depend on accepted `ARC-SRC-001` evidence and an
  applicable `ARC-AUTH-001` authority posture that itself has a fresh exact
  digest/context security posture; every derived or changed output, including
  an `ARC-SRC-001` transformation, returns through `ARC-SRC-001` security
  re-admission before another component may consume, review, retain as
  accepted, hand off, or consider it for a separately authorized future
  release review. `ARC-REL-001` emits no output.
- `ARC-RDY-001`, `ARC-ACQ-001`, `ARC-LOG-001` and `ARC-ALLY-001` may exchange
  explicit, versioned references through their settled contracts while each
  retains semantic ownership.
- `ARC-DST-001` may consume accepted domain results to test distribution and
  tails; `ARC-ECO-001` may consume accepted domain and distribution results to
  maintain separate ledgers and pathways.
- `ARC-DEL-001` must consume each candidate/pathway/domain-floor posture,
  including null and research-hypothesis branches, before realizability, final
  review, adaptive disposition, or handoff.
- `ARC-DEL-001[n]` consumes immutable preliminary `ECO[n]`. After DEL custody,
  security and independent review, `ARC-ECO-001` may create only the explicitly
  predecessor-linked immutable final realization `ECO[n+1]`, including the
  domain-floor fiscal realization required by `SPEC-ECO-006`.
- `ARC-ADP-001[n+1]` may consume only final `ECO[n+1]`; disposition and later
  delivery-observation feedback return through custody/security and review to
  request a later preliminary successor, which repeats mandatory delivery
  before another final realization.
- `ARC-REV-001` may inspect any frozen artifact and issue findings, holds or a
  gate result without changing the artifact.
- `ARC-HND-001` may consume only fresh, compatible, reviewed results with all
  applicable semantic-owner concurrences. It freezes an immutable candidate
  bundle; the exact bundle then receives security admission and a matching
  finite terminal `ARC-REV-001` decision before external handoff.

### Forbidden

- No analytical component may bypass source admission, per-output security
  re-admission, authority, delivery posture, floor, numeracy, independence,
  compatibility or digest gates.
- No `ARC-AUTH-001` manifest may bypass exact-output security re-admission, and
  no `ARC-SRC-001` suppression, broadening, aggregation, redaction or other
  transformation may self-exempt from the same gate.
- No downstream component may mutate an upstream source, authority, domain,
  ledger, review or historical record; feedback creates a new version.
- Preliminary `ECO[n]` may not consume `DEL[n]`, and DEL may not mutate its ECO
  predecessor. There is no same-version `ECO↔DEL` edge: only the acyclic
  `ECO[n]→DEL[n]→custody/security/review→ECO[n+1]→ADP[n+1]` sequence is
  permitted. Missing or mismatched predecessor identity, stale delivery
  posture, delivery bypass, or in-place ECO/DEL mutation is rejected.
- `ARC-ECO-001` and `ARC-ADP-001` may not exchange reciprocal same-version
  mutation. Disposition and observation may request only a later preliminary
  successor that repeats the delivery gate.
- `ARC-ECO-001` and `ARC-ADP-001` may not command readiness, force, acquisition,
  logistics, alliance, budget or operational decisions.
- No domain component may convert a peer gap, composite score, external
  benefit, avoided risk, capacity gain or private revenue into booked public
  savings.
- `ARC-REV-001` may not self-waive findings, rewrite evidence or create the
  authority it tests.
- A terminal review receipt may not copy product values or introduce a new
  derivation, visualization, composition or other product content. Any such
  change creates a new candidate bundle and restarts the terminal gate; a
  receipt for another digest/context cannot authorize handoff.
- `ARC-HND-001` may not set or imply Taxlane admission, cross-lane combination,
  allocation, rebalance, rates, official use or publication.
- `ARC-REL-001` has no outbound release edge. No other component may route
  around it.
- No component may depend on classified, controlled, person-level,
  operational-planning, targeting, exploitable-vulnerability or reconstructive
  detail.

## Failure containment and degraded behavior

| Failure or degraded condition | Containment boundary | Required behavior |
|---|---|---|
| Unsafe, ambiguous or composition-sensitive source | `ARC-SRC-001` | Hold or reject before analytical use; retain only a safe rejection receipt. |
| Derived, retained, visualized, composed, or proposed emitted output lacks a fresh exact-context security posture | Producer plus `ARC-SRC-001` | Hold/reject the output, retain only a safe non-reconstructive receipt, invalidate every bound downstream artifact, and block use/review/handoff/future release review. |
| Join, granularity, derivation, visualization, composition, audience, release context, or expiry changes | `ARC-SRC-001` | Invalidate the prior posture and dependent artifacts; reopen security review for the new digest/context. |
| Missing/stale identity, digest, provenance or context | Producer plus `ARC-REV-001` | Mark stale, block review/promotion/handoff and require a new bound version. |
| Missing civilian authority or scope ambiguity | `ARC-AUTH-001` | Hold all dependent analysis; do not infer permission. |
| AUTH output bypass, stale security posture, or dangerous authority composition | `ARC-AUTH-001` plus `ARC-SRC-001` | Hold every dependent use, invalidate artifacts bound to the unsafe manifest, and require a fresh exact digest/context security posture; authority meaning cannot waive the gate. |
| Missing or failed safety/readiness/resilience/alliance floor | `ARC-RDY-001` | Hold candidate, savings, delivery and handoff states while preserving failure evidence. |
| Unsafe supplier, logistics, partner or small-group detail | Owning domain plus `ARC-SRC-001` security control | Reduce only to an accepted safe abstraction or hold/reject; never retain unsafe detail for convenience. |
| Missing distribution or tail evidence | `ARC-DST-001` | Hold efficiency, savings, readiness and handoff claims; national averages cannot substitute. |
| Unit, horizon, price, account, party or overlap conflict | `ARC-ECO-001` | Keep ledgers separate and totals held; never choose an implicit conversion. |
| Missing support or fiscal evidence | Owning component | Record null/held, never zero and never a fabricated estimate. |
| Missing delivery authority, owner, capacity, floor, measurement, rollback, or other mandatory posture field | `ARC-DEL-001` | Retain held research-hypothesis posture for candidate, pathway, domain-floor, and null branches; reject the proposed final `ECO[n+1]` and block ADP and HND; never omit the delivery record or mutate preliminary `ECO[n]`. |
| Null lacks observation/custody owner, cadence, reopen trigger, history, or reviewed N/A for genuinely inapplicable realization fields | `ARC-ADP-001` plus `ARC-DEL-001` | Hold the null; never invent zero, realization owner, schedule, or cadence. |
| Review conflict, missing role, failed assurance or critical/major finding | `ARC-REV-001` | Block fixed point and downstream stage; retain dissent and negative evidence. |
| Delivery deviation or accepted reopen trigger | `ARC-DEL-001` with `ARC-ADP-001` | Choose exactly one stop/hold/revise/retire/replace action and create a reviewed successor if needed. |
| Same-version ECO/DEL cycle, missing or mismatched predecessor, stale DEL posture, delivery bypass, or in-place ECO/DEL mutation | `ARC-ECO-001`, `ARC-DEL-001`, and `ARC-REV-001` | Reject the candidate final realization; preserve preliminary `ECO[n]` and `DEL[n]`, and permit only a fresh custody/security/review-bound, explicitly predecessor-linked final `ECO[n+1]` before ADP. |
| Same-version ECO/ADP feedback, stale predecessor, or in-place mutation | `ARC-ECO-001`, `ARC-ADP-001`, and `ARC-REV-001` | Reject the mutation; preserve the final envelope and route feedback through custody/security/review to a later preliminary successor that repeats mandatory delivery. |
| Handoff incompatibility or missing semantic concurrence | `ARC-HND-001` | Reject or retain held state; do not translate away the conflict. |
| Terminal bundle digest/context mismatch, stale security posture, missing decision, or terminal record containing new product content | `ARC-HND-001`, `ARC-SRC-001`, and `ARC-REV-001` | Block external handoff. A matching unchanged bundle may receive only the finite minimal non-reconstructive decision/receipt; product-content change creates a new bundle and repeats freeze, security re-admission and independent review. |
| Taxlane response absent or unknown | External boundary | Keep BASTION state held; infer neither admission nor rejection. |
| Release authority absent | `ARC-REL-001` | Emit nothing publicly. |

Components may continue on unaffected, independently versioned evidence when
the failed dependency is not in their declared input graph. A dependent branch
must fail closed. No partial degradation may erase the causal hold or silently
broaden the artifact's scope.

## Exact open-unknown propagation

All thirteen specification unknowns remain open. This architecture closes none
of them and introduces no default. The affected SPEC rows below are copied from
the controlled baseline; the component column shows containment, not a change
to the settled dependency graph.

| Unknown | Exact affected SPEC rows | Affected logical components | Architecture hold |
|---|---|---|---|
| `SPEC-UNK-SEC-001` | As listed in each spec row; principally `SCP-001..003`, `SCP-009`, `SRC-005..006`, `LOG-006`, `HND-003/005`, `REL-002`, `SPEC-NF-001` | `ARC-SRC-001`, `ARC-LOG-001`, `ARC-HND-001`, `ARC-REL-001` | Hold affected admission, retention, derivation, emission, visualization, handoff and release. |
| `SPEC-UNK-RDY-001` | `SCP-006`, `RDY-*`, `DEL-001..003/006`, `HND-003/005` | `ARC-AUTH-001`, `ARC-RDY-001`, `ARC-DEL-001`, `ARC-HND-001` | Hold readiness, candidate, savings and handoff. |
| `SPEC-UNK-SRC-001` | `SCP-004`, `SRC-001..004/007..008`, `TST-001/004/005`, `VTR-001..002` | `ARC-SRC-001`, `ARC-REV-001` | Hold unrepresentable custody, version and review behavior. |
| `SPEC-UNK-QNT-001` | `RDY-004`, `ECO-003/007..010/014..015/018..020`, `HND-002/005` | `ARC-RDY-001`, `ARC-ECO-001`, `ARC-HND-001` | Hold projections, peers, horizons, totals and handoffs. |
| `SPEC-UNK-ACQ-001` | `ACQ-*`, `DST-003`, `DEL-006` | `ARC-ACQ-001`, `ARC-DST-001`, `ARC-DEL-001` | Hold acquisition, commonality, capacity, schedule and savings claims. |
| `SPEC-UNK-LOG-001` | `LOG-*`, `RDY-002/004..006`, `DEL-006` | `ARC-LOG-001`, `ARC-RDY-001`, `ARC-DEL-001` | Hold sustainment, readiness, lifecycle and savings claims. |
| `SPEC-UNK-ALLY-001` | `ALLY-*`, `DST-001/002/004/005`, `DEL-006` | `ARC-ALLY-001`, `ARC-DST-001`, `ARC-DEL-001` | Hold joint, interoperability, burden and fiscal claims. |
| `SPEC-UNK-DST-001` | `DST-*`, `ACQ-005`, `ALLY-002/005`, `ECO-003/006/020`, `DEL-006`, `HND-001..003/005` | `ARC-DST-001`, `ARC-ACQ-001`, `ARC-ALLY-001`, `ARC-ECO-001`, `ARC-DEL-001`, `ARC-HND-001` | Hold efficiency, savings, readiness, distribution and handoff claims. |
| `SPEC-UNK-ECO-001` | `ECO-001..006/009..013/015..017/019..020`, `HND-001..006` | `ARC-ECO-001`, `ARC-ADP-001`, `ARC-HND-001` | Hold monetization, savings, receipts, net pressure and handoff. |
| `SPEC-UNK-TST-001` | `TST-*`, `VTR-001..002` | `ARC-REV-001` | Hold fixed point and every downstream stage. |
| `SPEC-UNK-DEL-001` | `ECO-012/013/017/020`, `DEL-*`, `HND-001..003/005` | `ARC-ECO-001`, `ARC-ADP-001`, `ARC-DEL-001`, `ARC-HND-001` | Retain research-hypothesis state and block savings, implementation and handoff. |
| `SPEC-UNK-HND-001` | `ECO-011/016`, `HND-*` | `ARC-ECO-001`, `ARC-HND-001` | Hold adapter package and infer no Taxlane admission. |
| `SPEC-UNK-REL-001` | `SRC-005/006`, `REL-*` | `ARC-SRC-001`, `ARC-REL-001` | No public release. |

## Adaptive pathway lifecycle and custody

A pathway begins as a bounded research hypothesis. Its candidate envelope
preserves mechanism, owner, affected boundary, exclusions, baseline,
counterfactual, near/medium/long horizons, uncertainty, downside, transition
cost, realization schedule, service-floor and distribution result, overlap,
observation cadence and reopen triggers. It occupies exactly one explicit
logical posture such as result, null, reviewed N/A, held or rejected where the
controlled specification permits that posture. Before any realizability claim,
final review, adaptive disposition, or held handoff, `ARC-DEL-001` records the
mandatory delivery posture for the candidate, pathway, every applicable domain
floor, and the null/research-hypothesis branch. Missing later authority is a
held research hypothesis, never an omitted delivery record.

`ARC-ECO-001` owns the quantitative and accounting envelope. It first freezes
an immutable preliminary `ECO[n]`; `ARC-DEL-001[n]` then records the mandatory
delivery posture, which passes source custody/security and independent review.
Only that reviewed posture may support the explicitly predecessor-linked final
realization `ECO[n+1]`, including the domain-floor fiscal realization required
by `SPEC-ECO-006`. No delivery change mutates `ECO[n]`, and no preliminary ECO
consumes same-version DEL. `ARC-ADP-001` consumes only the final realization
and owns lifecycle evaluation and history. The named realization owner retains
custody of the realization claim, schedule, evidence and deviations; neither
component may substitute itself for that real-world owner. `ARC-DEL-001`
preserves the required delivery posture, all `DEL-006` deviations,
observations, and trigger-driven actions when later authority exists. A null
instead retains a named observation/custody owner, cadence, reopen triggers and
history; genuinely inapplicable realization-owner and schedule fields use only
reasoned independently reviewed N/A and are never fabricated.
`ARC-REV-001` independently reviews each new digest-bound version.

At an observation point the pathway may be preserved, revised, held, retired
or replaced. The preliminary envelope, delivery posture and final realization
remain immutable. `ARC-ADP-001` disposition and later `ARC-DEL-001` observation
return through `ARC-SRC-001` custody/security and `ARC-REV-001` review to
request a new preliminary successor; that successor repeats mandatory DEL,
security and review before a new final realization. Same-version ECO/DEL or
ECO/ADP reciprocity, a stale or missing predecessor, delivery bypass and
in-place mutation are forbidden. Nulls and failures remain available to later
review. A lifecycle transition never books savings and never carries an
earlier Taxlane decision forward by implication. Any proposed fiscal effect
returns through a fresh held handoff and Taxlane's separate admission process.

## Planned architecture verification mapping

No verification listed here has been executed. The mapping identifies the
evidence expected from later verification planning.

| Architecture responsibility | Planned verification references | Expected architecture evidence |
|---|---|---|
| Source/security and authority boundaries | `VER-SCP-*`, `VER-SRC-*`, `VER-NF-001..002`, `VER-NF-009..010` | Admission/refusal and exact-prohibition fixtures plus one per-output security-re-admission bypass fixture for SRC transformations, AUTH, RDY, ACQ, LOG, ALLY, DST, ECO, ADP, DEL, REV, and HND; AUTH-bypass and dangerous-authority-composition fixtures; dangerous-composition fixtures cover changed join, granularity, derivation, visualization, composition, audience, release context, and expiry. Verify downstream invalidation, safe non-reconstructive receipts, and that REL emits nothing. |
| Readiness and safety floors | `VER-RDY-*`, `VER-NF-003..004` | Complete promises, separate facets, floor failures, degraded cases, tails and proxy-only rejection. |
| Acquisition, logistics and alliance separation | `VER-ACQ-*`, `VER-LOG-*`, `VER-ALLY-*` | Facet completeness, non-composite commonality, inventory/repair distributions, lifecycle custody and party-separated effects. |
| Distribution and concentrated effects | `VER-DST-*`, `VER-NF-004` | Stakeholder/denominator/tail coverage and average-only rejection. |
| Economics and adaptive lifecycle | `VER-ECO-*`, `VER-NF-005..007` | Dimensional reconciliation, separate ledgers, non-additivity, overlap, gross-opportunity terminology, all-or-hold and history fixtures. Preliminary `ECO[n]`, reviewed DEL predecessor bond, final `ECO[n+1]`, `SPEC-ECO-006` realization and ADP-after-final ordering must verify; ECO/DEL cycle, stale posture, missing or mismatched predecessor, bypass, and in-place mutation cases must fail. ECO/ADP same-version feedback remains a failing fixture. Null fixtures require observation/custody owner, cadence, reopen/history and reviewed N/A without fabricated realization values. |
| Independent review, roles and trace | `VER-TST-*`, `VER-VTR-*`, `VER-NF-008..010` | Frozen packet, seeded adverse cases, finding closure, dissent, regeneration, trace, premature-stage refusal, all 21 role rows, both assurance gates, and author/owner self-approval rejection. |
| Delivery posture and feedback | `VER-DEL-*` | Mandatory-delivery bypass fixtures for candidate, pathway, each domain-floor input, null and research-hypothesis branches; missing later authority must produce held research hypothesis. Delivery hold/rejection must prevent final realization, ADP and HND while preserving preliminary ECO. Fixtures reject same-version ECO↔DEL, missing predecessor, stale DEL, and in-place mutation, and accept only `ECO[n]→DEL[n]→custody/security/review→ECO[n+1]→ADP[n+1]`. Observation fixtures preserve all schedule, cost, burden, overlap, safety, readiness, supplier, workforce, community and alliance deviations. |
| Held handoff and external ownership | `VER-HND-*` | Semantic round trip, rejection matrix, fresh security and delivery gates, missing-value refusal and no inferred Taxlane state; a missing/held/rejected delivery posture must prevent handoff. Terminal fixtures reject bundle digest/context mismatch, stale posture, missing decision and new product content in the receipt; an unchanged admitted bundle may receive only the finite minimal non-reconstructive terminal decision without recursive re-review. |
| Closed release boundary | `VER-REL-*` only after separate authority | Unauthorized-release refusal and, only in a future authorized stage, release-threat/context fixtures. |

Total coverage remains 98 one-to-one `VER-*` identities, 13 logical-contract
surfaces and 10 nonfunctional constraints. Later verification must also prove
that each of the 13 open unknowns continues to hold its exact dependent set
until separately closed.

## Decisions and rationale

| Decision | Rationale |
|---|---|
| Use domain-aligned conceptual components with exclusive primary SPEC custody. | Preserves defense-specific semantics, accountable ownership and complete trace without selecting physical packages. |
| Separate source admission from civilian authority. | Evidence safety/provenance and lawful decision authority require distinct accountable owners and concurrence. |
| Re-admit every derived or changed output, including AUTH and SRC transformations, through source/security custody; REL emits nothing. | Public source status or authority meaning does not make a composition, visualization, retained result, audience, or emitted artifact safe; posture must bind exact digest and context. |
| Keep readiness, acquisition, logistics, alliance and distribution distinct. | Their measures, floors, risks and affected parties are not interchangeable and cannot safely collapse into one score. |
| Separate quantitative/accounting custody from adaptive lifecycle evaluation. | Supports repeated evidence-bound adjustment while preventing lifecycle state from becoming a booked fiscal effect. |
| Require delivery posture before realizability, final review, adaptive disposition, or handoff. | Delivery authority, capacity, floors, measurement, deviations and rollback are part of every candidate/pathway/null branch, and missing later authority must remain visible as held research-hypothesis state. |
| Require the acyclic `preliminary ECO[n]→DEL[n]→custody/security/review→final ECO[n+1]→ADP[n+1]` sequence. | Preserves mandatory delivery and `SPEC-ECO-006` while eliminating reciprocal same-version ECO/DEL mutation; later feedback requests a new preliminary successor and repeats the gate. |
| Make independent review a sidecar gate. | Allows every promotable artifact to be frozen and challenged without reviewers mutating source artifacts. |
| End final handoff with a digest-bound finite terminal decision over an immutable admitted bundle. | Minimal non-reconstructive governance metadata closes the gate without creating a recursive product-output loop; any new product content restarts the gate. |
| Make the Taxlane adapter held and semantically one-way. | Preserves domain meaning and external fiscal ownership; avoids implied admission or rebalance. |
| Keep release isolated and closed. | Public release composition and misuse controls are unresolved and require separate authority. |
| Defer physical boundaries and contract encodings. | The current stage authorizes logical allocation only; premature implementation choices would overstate maturity. |

## Alternatives considered and rejected

| Alternative | Disposition | Reason |
|---|---|---|
| One monolithic analysis component | rejected | Obscures semantic ownership, security gates, floor authority, failure containment and trace. |
| Mirror the current organization, account or platform structure | rejected | Infrastructure 2.0 models mission promises, flows, bottlenecks and lifecycle rather than preserving an inherited org chart. |
| Use one composite readiness/commonality/value score | rejected | Hides facets, tails, uncertainty, dissent and non-additivity and could become an unauthorized funding rank. |
| Let economics optimize or command domain decisions | rejected | Fiscal analysis cannot select missions, force, procurement, operational methods or safety/readiness tradeoffs. |
| Treat peer differences as savings targets | rejected | Peer gaps are diagnostics and can be incomparable or policy-dependent. |
| Let BASTION admit effects or rebalance Taxlane | rejected | Admission, cross-lane combination, allocation, rebalance and rates belong solely to Taxlane. |
| Central data lake including controlled, operational or person-level detail | rejected | Violates the absolute public/aggregate/unclassified/non-operational boundary and creates composition risk. |
| Begin Rust/package/API/schema design now | rejected | Those are later-stage choices and the required methods and interfaces remain held. |
| Enable public release from reviewed state | rejected | Review success does not create release authority; the release contract remains unknown. |

## Assumptions

- The controlled specification digest remains the architecture input; any
  changed digest requires a new architecture review.
- Logical component boundaries may map many-to-many to later physical packages;
  this document makes no physical-placement claim.
- Domain owners remain available for semantic concurrence and retain their
  settled authority; no component ID replaces a human accountable owner.
- Taxlane remains an external system with independently governed admission and
  fiscal semantics.
- Planned evidence can be defined without claiming that a corpus,
  implementation, interface, test harness or delivery capability currently
  exists.

## Open risks and next-stage gates

- All thirteen `SPEC-UNK-*` controls remain open and promotion-gating. They are
  the principal architecture risk and intentionally prevent false precision.
- Compositional inference may make individually public aggregate artifacts
  unsafe together; exact controls remain held by `SPEC-UNK-SEC-001`.
- Public evidence may not support readiness, logistics, industrial-base,
  alliance, distribution or delivery claims at the required specificity;
  bounded null or held results are acceptable.
- Physical decomposition could accidentally merge semantic or authority
  boundaries; `PACKAGE_BOUNDARIES.md` must preserve accountable ownership,
  direction rules and failure containment.
- Interface encoding could erase nulls, N/A rationale, units, uncertainty,
  dissent, overlap or external Taxlane ownership; `INTERFACES.md` must prove
  semantic preservation.
- Delivery and realization evidence is not currently available. No component
  may claim implementation-ready, realized savings or an observed outcome.
- Release composition remains unresolved and unauthorized; no release work is
  eligible under this architecture.

## Architecture change control

This architecture is controlled by its content digest. Any change to a logical
component identity or responsibility; primary SPEC allocation; dependency edge
or direction; contract producer, consumer, semantics, version, concurrence,
failure behavior, or change trigger; security re-admission or safe-receipt
boundary or finite terminal decision; persistence/custody rule; delivery
prerequisite; preliminary/final ECO/DEL ordering or ECO/ADP version feedback;
null lifecycle; role/harm allocation; Taxlane or release isolation;
unknown propagation; nonfunctional allocation; planned verification; accepted
risk; or stage eligibility creates a new architecture version and digest.

Such a change immediately makes the prior architecture review stale, requires
fresh independent architecture, applicable domain-role, editorial, and both
assurance reviews, and requires every dependent later-stage artifact to be
invalidated or explicitly reconciled to the successor digest before use. A
later package, interface, design, verification, work-package, or implementation
artifact cannot silently change this logical baseline.

A proposed change to specification meaning, accountable owner or required
concurrence, direct `SPEC-UNK-*` dependency/hold set, controlled term such as
`gross opportunity`, no-authority boundary, or expected verification result
must reopen `SPECIFICATION_BASELINE.md` under its own change control before the
architecture may change. Closing an unknown additionally requires its settled
TBD acceptance condition. Architecture review cannot close a TBD or create an
implementation, Taxlane, fiscal, official-use, or release authority.

## Architecture disposition

This draft allocates all 98 controlled SPEC rows, all 13 logical contracts and
all 10 nonfunctional constraints; preserves all 98 REQ relationships and all
13 exact unknown holds; allocates all 21 role/harm lenses; and defines
conceptual dependencies, custody, change control, failure containment and
verification intent without selecting physical or interface design.

Disposition: **review-ready with 13 inherited open holds; not a fixed point**.
Independent digest-bound architecture and assurance review is required before
any decision to advance. No implementation, Taxlane, budget, allocation, rate,
official-use or release authority is conferred.
