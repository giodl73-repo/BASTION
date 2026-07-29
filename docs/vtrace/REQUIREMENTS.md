# BASTION Requirements

## Status and controlled scope

This baseline reached a requirements fixed point with a **pass_with_risk**
decision after two independent review lanes, bounded remediation, and an
independent cross-convergence recheck. It translates the settled BASTION
mission and CONOPS into testable product requirements for a future
public-aggregate, unclassified Defense 2.0 research and tooling system.

Controlled CONOPS input SHA-256:
`a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602`

The requirements define behavior and evidence obligations, not implementation
design. They do not select force structure, procurement, budgets, rates,
operational methods, sensitive thresholds, or official policy. They authorize
no classified or controlled handling, person-level processing, operational
planning, targeting, acquisition action, budget request, Taxlane admission,
allocation, rate-setting, public release, or Rust implementation.

## Normative conventions

- `shall` and `shall not` are normative.
- Every normative row is pending independent review.
- `must` means promotion-gating for the controlled scope; `should` means
  required unless a later fixed-point review accepts a bounded defer.
- `result`, `null`, `rejected`, `held`, and independently reviewed
  `not_applicable` are distinct postures. Missing and N/A are not synonyms.
- Gates are conjunctive. No favorable fiscal, readiness, or peer result can
  compensate for an omitted or failed gate.
- Verification methods are planned evidence methods, not claims that evidence
  or implementation already exists.
- Exact values and methods not safely selectable at requirements scope use a
  stable `TBD-*` record with owner, destination, closure condition, and hold
  behavior.

### Atomic state invariant

`PATHWAY-ENVELOPE-001` is an indivisible completeness invariant for an adaptive
pathway record. It contains a cited baseline and counterfactual, unit, price
year, uncertainty and method, downside case, transition and implementation
cost, realization owner and schedule, floor and distribution results, overlap
keys, observation cadence, reopen triggers, and valid-null behavior. Verification
must exercise one accepted complete branch and a rejection branch for every
missing, stale, incompatible, or unaccepted field. A partial envelope is held;
no branch may infer a default.

## Normative requirement table

### Scope, authority, and promotion control

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-SCP-001` | BASTION shall admit only public, aggregate, unclassified, non-operational evidence whose direct fields and foreseeable combinations pass the accepted security posture. | `OPS-001`; `GATE-01`; `DEF-CONOPS-001`; `OQ-001` | Establishes the admissible evidence boundary. | must | Security and aggregation steward | Source-manifest inspection plus admitted/rejected/combination-risk fixtures | pending_review |
| `BASTION-REQ-SCP-002` | BASTION shall not ingest, retain, derive, or emit classified information, CUI, person-level service data, sensitive operational data, targeting content, operational-planning content, or exploitable vulnerability content. | `OPS-001`; `OPS-002`; `GATE-01`; `GATE-02` | Prevents direct and inferred unsafe use. | must | Classification and Operational Security reviewer | Prohibited-field and dangerous-combination negative fixtures plus corpus inspection | pending_review |
| `BASTION-REQ-SCP-003` | BASTION shall hold or reject any artifact with missing, stale, non-comparable, unsafe, unreconciled, unowned, falsely precise, or double-counted material evidence. | `OPS-001`; `OPS-008`; `OPS-009`; `GATE-01`; `GATE-09`; `GATE-10` | Makes uncertainty and unsafe posture fail closed. | must | BASTION maintainer | One state-transition fixture for each enumerated hold or rejection cause | pending_review |
| `BASTION-REQ-SCP-004` | BASTION shall bind every promoted artifact and review to a stable artifact identity, version, digest, gate posture, owner, and supersession relation. | `OPS-008`; `OPS-010`; `GATE-09`; `DEF-CONOPS-003`; `DEF-CONOPS-010` | Prevents stale review and silent replacement. | must | Role review steward | Digest-mismatch and supersession fixtures plus trace inspection | pending_review |
| `BASTION-REQ-SCP-005` | BASTION shall require every applicable gate field or facet to carry evidence and a `result`, `null`, or reasoned independently reviewed `not_applicable` posture. | `OPS-003` through `OPS-009`; `GATE-01` through `GATE-10` | Makes gate dispositions explicit. | must | BASTION maintainer | Gate-matrix schema inspection plus result/null/reviewed-N/A branch fixtures | pending_review |
| `BASTION-REQ-SCP-006` | BASTION shall record the public mission abstraction, lawful civilian authority, jurisdiction, decision owner, non-delegable decisions, effective period, analytic boundary, and unresolved ambiguity before dependent analysis may be promoted. | `OPS-002`; `GATE-02`; `DEF-CONOPS-002`; `OQ-002` | Preserves civilian control and lawful authority. | must | Civilian mission and authority steward | Mission-manifest inspection plus absent/ambiguous-authority negative fixtures | pending_review |
| `BASTION-REQ-SCP-007` | BASTION shall not choose missions, force structure, force employment, targets, tactics, procurement, resource allocation, budgets, rates, operational methods, or official recommendations. | `OPS-002`; `OPS-007`; `GATE-02`; `GATE-10` | Keeps research separate from governmental decisions. | must | Civilian Control, Law, Safety & Readiness reviewer | Scope-lint inspection plus prohibited-output negative fixtures | pending_review |
| `BASTION-REQ-SCP-008` | BASTION shall not treat a requirements, review, candidate, or held handoff artifact as implementation, acquisition, budget, Taxlane, rate, official-use, or release authority. | `OPS-009`; `OPS-010`; `GATE-10`; `DEF-CONOPS-012`; `OQ-010` | Prevents false approval and downstream authority drift. | must | Scope Keeper | Claim-language inspection plus false-authority fixtures | pending_review |
| `BASTION-REQ-SCP-009` | BASTION shall preserve a safe hold or rejection reason without retaining or reconstructing prohibited content. | `OPS-001`; `OPS-008`; `GATE-01`; `DEF-CONOPS-001` | Keeps rejection auditable without recreating harm. | must | Security and aggregation steward | Rejection-receipt inspection plus prohibited-content reconstruction fixture | pending_review |
| `BASTION-REQ-SCP-010` | BASTION shall hold promotion when any applicable gate field or facet is missing, implicit, or author-waived. | `OPS-003` through `OPS-010`; `GATE-01` through `GATE-10` | Makes conjunctive gates non-bypassable. | must | BASTION maintainer | Separate missing-field, implicit-field, and author-waiver blocked-promotion fixtures | pending_review |

### Source, identity, and claim custody

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-SRC-001` | BASTION shall assign a stable identity to every source and source version. | `OPS-001`; `GATE-01`; `DEF-CONOPS-003`; `OQ-003` | Makes source versions addressable and reproducible. | must | Public-evidence steward | Identity uniqueness, stability, and supersession fixtures | pending_review |
| `BASTION-REQ-SRC-002` | BASTION shall label each material claim as `source_fact`, `derived_measure`, `analytic_assumption`, `scenario_result`, `finding`, `null`, or `rejected`. | `OPS-001`; `OPS-008`; `GATE-09`; `DEF-CONOPS-003` | Distinguishes evidence from interpretation and result. | must | Public-evidence steward | Exhaustive claim-label schema fixtures | pending_review |
| `BASTION-REQ-SRC-003` | BASTION shall record each material derivation's exact input versions, method identity, assumptions, units, uncertainty, output identity, and reproducibility posture. | `OPS-001`; `OPS-003`; `GATE-01`; `GATE-09`; `DEF-CONOPS-003` | Establishes claim custody from evidence to result. | must | Public-evidence steward | Derivation-manifest inspection plus regeneration and missing-input negative fixtures | pending_review |
| `BASTION-REQ-SRC-004` | BASTION shall preserve superseded, revised, rejected, held, and null records without silently changing historical runs or substituting zero for missing evidence. | `OPS-001`; `OPS-002`; `GATE-01`; `DEF-CONOPS-003` | Preserves auditability and valid null paths. | must | BASTION maintainer | Version-history inspection plus revision/null regression fixtures | pending_review |
| `BASTION-REQ-SRC-005` | BASTION shall evaluate direct sensitivity, small-group exposure, joins, mosaicing, derived granularity, visualization, repeated release, and foreseeable cross-artifact inference before source or artifact promotion. | `OPS-001`; `GATE-01`; `DEF-CONOPS-001`; `DEF-CONOPS-013`; `OQ-001` | Public fragments can compose into sensitive information. | must | Security and aggregation steward | Threat-model inspection plus compositional-inference and repeated-release negative fixtures | pending_review |
| `BASTION-REQ-SRC-006` | BASTION shall re-open security and source admission when a source, join, granularity, derivation, visualization, audience, release context, or accepted expiry condition changes. | `OPS-001`; `GATE-01`; `DEF-CONOPS-001`; `DEF-CONOPS-013` | Admission is contextual and time-bound. | must | Security and aggregation steward | Change-trigger transition tests plus expiry fixtures | pending_review |
| `BASTION-REQ-SRC-007` | BASTION shall record each source version's publisher, custody location, access date, vintage, license/reuse posture, scope, aggregation, units, denominators, update cadence, revisions, exclusions, and known limitations. | `OPS-001`; `GATE-01`; `DEF-CONOPS-003`; `OQ-003` | Makes source custody complete. | must | Public-evidence steward | Metadata-field completeness fixtures with one rejection branch per field | pending_review |
| `BASTION-REQ-SRC-008` | BASTION shall preserve vendor, advocacy, official-statement, and independent-test provenance as separate evidence postures. | `OPS-001`; `OPS-008`; `GATE-09`; `DEF-CONOPS-003`; `DEF-CONOPS-010` | Prevents assertion laundering by provenance collapse. | must | Public-evidence steward | Provenance-class separation and attempted-collapse fixtures | pending_review |

### Mission readiness, safety, and resilience

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-RDY-001` | BASTION shall define every readiness or safety promise with its public authority, system boundary, measure, denominator, period, horizon, evidence rule, distribution or tail treatment, degraded-mode treatment, and hold rule. | `OPS-002`; `OPS-003`; `GATE-03`; `DEF-CONOPS-002`; `OQ-002` | Converts broad readiness language into verifiable promises. | must | Readiness-system analyst | Promise-schema inspection plus complete/incomplete promise fixtures | pending_review |
| `BASTION-REQ-RDY-002` | BASTION shall separately evaluate staffing, training, personnel safety, availability, integration, maintenance, supply, repair, resilience, surge, mobilization, and recovery for every applicable readiness artifact. | `OPS-003`; `GATE-03`; `DEF-CONOPS-002` | Prevents a single aggregate from masking readiness dependencies. | must | Readiness-system analyst | Facet-coverage matrix inspection plus omitted-facet negative fixtures | pending_review |
| `BASTION-REQ-RDY-003` | BASTION shall reconcile derived readiness measures to cited public aggregates across definitions, denominators, system boundaries, periods, and vintages. | `OPS-003`; `GATE-03`; `DEF-CONOPS-002`; `DEF-CONOPS-003` | Establishes comparability to public baselines. | must | Readiness-system analyst | Reconciliation report with one mismatch fixture for each named basis | pending_review |
| `BASTION-REQ-RDY-004` | BASTION shall evaluate a bounded non-operational downside or degraded-support case and retain failure evidence for every promoted readiness candidate. | `OPS-003`; `OPS-007`; `GATE-03`; `GATE-07`; `DEF-CONOPS-002`; `DEF-CONOPS-004` | Normal-period performance does not prove resilience. | must | Operational Readiness Officer | Scenario inspection plus missing-downside negative fixture | pending_review |
| `BASTION-REQ-RDY-005` | BASTION shall hold any candidate that degrades or lacks evidence for lawful civilian authority, personnel safety, readiness, resilience, surge, recovery, or alliance floors. | `OPS-003`; `OPS-007`; `GATE-02`; `GATE-03`; `GATE-06` | Makes hard floors non-waivable. | must | Civilian Control, Law, Safety & Readiness reviewer | Floor-failure and missing-evidence negative fixtures plus assurance review | pending_review |
| `BASTION-REQ-RDY-006` | BASTION shall not infer employable readiness, resilience, or safety from spending, inventory counts, planned values, or averages alone. | `OPS-003`; `OPS-005`; `GATE-03`; `OQ-002` | Blocks common readiness proxies and hidden tails. | must | Operational Readiness Officer | Proxy-only and average-only negative fixtures | pending_review |
| `BASTION-REQ-RDY-007` | BASTION shall keep readiness series separate when their definitions, denominators, system boundaries, periods, or vintages remain unreconciled. | `OPS-003`; `GATE-03`; `DEF-CONOPS-002`; `DEF-CONOPS-003` | Prevents unresolved series from becoming false comparisons. | must | Readiness-system analyst | Incompatible-series separation fixtures for each reconciliation basis | pending_review |

### Acquisition, industrial base, and commonality

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-ACQ-001` | BASTION shall evaluate requirements stability, cycle time, competition, qualification, production and repair capacity, supplier concentration, technical-data and intellectual-property constraints, workforce and facilities, cash flow, demand stability, learning, transition, and surge at safe public abstraction. | `OPS-004`; `GATE-04`; `DEF-CONOPS-005`; `OQ-005` | Defines the industrial delivery boundary. | must | Acquisition and industrial-base analyst | Acquisition-facet matrix inspection plus omitted-facet fixtures | pending_review |
| `BASTION-REQ-ACQ-002` | BASTION shall separately expose shared-support value, unique-system need, concentration effect, transition effect, interoperability effect, and common-mode or unique-system failure risk for every applicable acquisition or candidate artifact. | `OPS-004`; `OPS-007`; `GATE-04`; `DEF-CONOPS-005` | Makes commonality benefits and risks jointly visible. | must | Acquisition and industrial-base analyst | Six-facet commonality schema inspection plus missing-facet negative fixtures | pending_review |
| `BASTION-REQ-ACQ-003` | BASTION shall assign each commonality facet its own evidence, method, units where quantitative, uncertainty, and `result`, `null`, or independently reviewed `not_applicable` posture. | `OPS-004`; `GATE-04`; `DEF-CONOPS-005` | Makes every commonality facet independently reviewable. | must | Acquisition and Industrial-Base Lead | Per-facet evidence/posture fixtures including implicit-N/A rejection | pending_review |
| `BASTION-REQ-ACQ-004` | BASTION shall include requalification, tooling, workforce, facility, transition, support, schedule, supplier-exit, concentration, and failure costs and risks in acquisition candidate comparisons. | `OPS-004`; `OPS-007`; `GATE-04`; `GATE-07`; `DEF-CONOPS-005` | Keeps acquisition economics lifecycle-complete. | must | Acquisition and industrial-base analyst | Lifecycle bridge inspection plus omitted-cost negative fixtures | pending_review |
| `BASTION-REQ-ACQ-005` | BASTION shall separately report prime, small-supplier, government, workforce, and facility dependencies and incidence at an approved aggregation. | `OPS-004`; `GATE-04`; `GATE-08`; `DEF-CONOPS-005`; `DEF-CONOPS-008` | Exposes concentration and burden shifts. | must | Acquisition and industrial-base analyst | Dependency/incidence report inspection plus unsafe-granularity fixture | pending_review |
| `BASTION-REQ-ACQ-006` | BASTION shall hold performance, schedule, capacity, competition, or savings claims supported only by vendor advocacy, gross obligations, unsafe detail, or incomplete competition evidence. | `OPS-004`; `OPS-008`; `GATE-04`; `GATE-09` | Weak evidence cannot establish delivery or savings. | must | Independent Test and Oversight Officer | Evidence-tier and vendor-only negative fixtures | pending_review |
| `BASTION-REQ-ACQ-007` | BASTION shall not classify supplier exit, brittle concentration, unpriced qualification, delayed capability, or shifted industrial burden as an efficiency. | `OPS-004`; `GATE-04`; `GATE-07`; `GATE-08` | Prevents cost and risk externalization. | must | Acquisition and Industrial-Base Lead | Candidate-ledger inspection plus false-efficiency negative fixtures | pending_review |
| `BASTION-REQ-ACQ-008` | BASTION shall not collapse the six commonality facets into a composite score. | `OPS-004`; `OPS-007`; `GATE-04`; `DEF-CONOPS-005` | Prevents benefits from concealing concentration or failure risk. | must | Acquisition and Industrial-Base Lead | One composite-score rejection fixture and six-facet preservation inspection | pending_review |

### Logistics, inventory, sustainment, and repair

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-LOG-001` | BASTION shall define every applicable inventory posture and stock boundary with stocks, condition states, ownership or custody, period, units, inclusions, exclusions, stock-policy basis, and reconciliation posture. | `OPS-005`; `GATE-05`; `DEF-CONOPS-006`; `OQ-006` | Makes inventory measures interpretable. | must | Logistics and sustainment analyst | Inventory-schema inspection plus incomplete-boundary fixtures | pending_review |
| `BASTION-REQ-LOG-002` | BASTION shall record every applicable repair-time distribution with population, denominator, start and stop events, censoring posture, period, central measures, accepted tail measures, uncertainty, and degraded-mode treatment. | `OPS-005`; `GATE-05`; `DEF-CONOPS-006`; `OQ-006` | Averages alone hide repair tails. | must | Logistics and sustainment analyst | Distribution-schema inspection plus average-only and censoring negative fixtures | pending_review |
| `BASTION-REQ-LOG-003` | BASTION shall assign inventory posture and repair-time distribution separate evidence and separate `result`, `null`, or independently reviewed `not_applicable` postures. | `OPS-005`; `GATE-05`; `DEF-CONOPS-006` | Makes both sustainment facets independently reviewable. | must | Logistics and Sustainment Lead | Separate inventory and repair-posture fixtures including N/A review | pending_review |
| `BASTION-REQ-LOG-004` | BASTION shall trace acquisition-to-sustainment custody across technical data, workforce, facilities, spares, distribution, energy, maintenance, repair, upgrades, disposal, and transition at safe abstraction. | `OPS-005`; `GATE-05`; `DEF-CONOPS-006` | Captures whole lifecycle support. | must | Logistics and sustainment analyst | Custody-map inspection plus omitted-stage fixtures | pending_review |
| `BASTION-REQ-LOG-005` | BASTION shall include deferred maintenance, obsolescence, cannibalization, queue and tail effects, supplier and workforce constraints, surge, recovery, and lifecycle costs in sustainment candidate results. | `OPS-005`; `OPS-007`; `GATE-05`; `GATE-07`; `DEF-CONOPS-006` | Prevents purchase-price and steady-state bias. | must | Logistics and sustainment analyst | Sustainment-ledger inspection plus missing-degraded-cost fixtures | pending_review |
| `BASTION-REQ-LOG-006` | BASTION shall hold any logistics map, stock detail, repair result, or derived combination rejected by security review. | `OPS-005`; `GATE-01`; `GATE-05`; `DEF-CONOPS-001`; `DEF-CONOPS-006` | Security overrides analytic utility. | must | Classification and Operational Security reviewer | Separate security-rejection fixture for each named artifact class | pending_review |
| `BASTION-REQ-LOG-007` | BASTION shall hold promotion when inventory or repair evidence is missing or incompatible. | `OPS-005`; `GATE-05`; `DEF-CONOPS-006` | Makes the two sustainment facets non-bypassable. | must | Logistics and Sustainment Lead | Missing-inventory, missing-repair, and incompatibility blocked-promotion fixtures | pending_review |
| `BASTION-REQ-LOG-008` | BASTION shall not infer a zero-cost repair path from missing support evidence. | `OPS-005`; `GATE-05`; `GATE-07`; `DEF-CONOPS-006` | Null support evidence is not free repair. | must | Logistics and Sustainment Lead | Missing-support-to-zero-cost negative fixture | pending_review |

### Alliance, interoperability, and sovereignty

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-ALLY-001` | BASTION shall identify applicable public commitments, standards, compatibility boundaries, common logistics, partner capacity assumptions, sovereign constraints, export or control boundaries, and transition dependencies without inferring partner operations or intent. | `OPS-006`; `GATE-06`; `DEF-CONOPS-007`; `OQ-007` | Defines a lawful non-operational interoperability boundary. | must | Alliance and interoperability analyst | Interoperability-manifest inspection plus sensitive-inference fixtures | pending_review |
| `BASTION-REQ-ALLY-002` | BASTION shall separate U.S., partner, shared, and externalized costs, benefits, burdens, receipts, and risks. | `OPS-006`; `GATE-06`; `GATE-07`; `GATE-08`; `DEF-CONOPS-007` | Prevents unilateral netting and burden concealment. | must | Alliance and interoperability analyst | Party-by-ledger fixture matrix plus cross-party netting rejection | pending_review |
| `BASTION-REQ-ALLY-003` | BASTION shall test alliance and interoperability candidates under normal and degraded institutional support at safe abstraction. | `OPS-006`; `OPS-007`; `GATE-06`; `DEF-CONOPS-007` | Compatibility must survive plausible support degradation. | must | Alliance and Interoperability Strategist | Normal/degraded paired scenario fixture plus no-degraded-case rejection | pending_review |
| `BASTION-REQ-ALLY-004` | BASTION shall hold a joint or burden claim when commitment authority, common-system obligations, partner evidence, transition cost, sovereign constraints, or incidence cannot be evaluated safely. | `OPS-006`; `GATE-01`; `GATE-02`; `GATE-06`; `DEF-CONOPS-007` | Missing partner evidence does not imply consent or compatibility. | must | Alliance and Interoperability Strategist | Hold-state and incomplete-partner-evidence negative fixtures | pending_review |
| `BASTION-REQ-ALLY-005` | BASTION shall preserve conflicting domestic and partner results as separate results. | `OPS-006`; `OPS-007`; `GATE-06`; `GATE-08`; `DEF-CONOPS-007` | Prevents one party's gain from erasing another's burden. | must | Alliance and interoperability analyst | Conflicting-result preservation and attempted-netting fixtures | pending_review |
| `BASTION-REQ-ALLY-006` | BASTION shall retain uncertainty and dissent for every alliance and interoperability scenario result. | `OPS-006`; `OPS-007`; `GATE-09`; `DEF-CONOPS-007` | Preserves unresolved partner and methodology risk. | must | Alliance and Interoperability Strategist | Uncertainty/dissent completeness fixture plus omitted-dissent rejection | pending_review |

### Distribution and affected parties

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-DST-001` | BASTION shall separately evaluate applicable effects on service members and families, mission users, civilian and depot/logistics workforces, prime and small suppliers, installation communities, taxpayers and oversight bodies, and allies or partners. | `OPS-003` through `OPS-007`; `GATE-08`; `DEF-CONOPS-008`; `OQ-008` | Prevents one aggregate from erasing affected groups. | must | Personnel, family, workforce, and community analyst | Stakeholder-coverage matrix plus omitted-lens fixtures | pending_review |
| `BASTION-REQ-DST-002` | BASTION shall define each distribution measure with affected aggregate, denominator, period, horizon, baseline, uncertainty, burden category, central result, tail or concentrated result, and evidence posture. | `OPS-007`; `GATE-08`; `DEF-CONOPS-008`; `OQ-008` | Makes burden and pain distribution verifiable. | must | Personnel, family, workforce, and community analyst | Distribution-schema inspection plus average-only fixture | pending_review |
| `BASTION-REQ-DST-003` | BASTION shall test safety, tempo, staffing, retention, skills, moves, housing, health, caregiving, local services, environment, employment transition, supplier cash flow, and burden shifts where applicable. | `OPS-004`; `OPS-005`; `OPS-007`; `GATE-08`; `DEF-CONOPS-008` | Captures non-fiscal effects and hidden delivery costs. | must | Personnel, family, workforce, and community analyst | Cross-stakeholder facet matrix and burden-shift fixtures; required review by Service-Member and Family Advocate, Prime and Small Supplier, Depot and Logistics Workforce, Installation Community, Taxpayer and Oversight, and both assurance roles | pending_review |
| `BASTION-REQ-DST-004` | BASTION shall preserve conflicting stakeholder results and valid nulls as separate result records. | `OPS-007`; `GATE-08`; `GATE-09`; `DEF-CONOPS-008` | Keeps distributional conflict and nulls visible. | must | Personnel, family, workforce, and community analyst | Conflicting-result and valid-null preservation fixtures | pending_review |
| `BASTION-REQ-DST-005` | BASTION shall not net stakeholder results into a composite priority, readiness, or savings score. | `OPS-007`; `GATE-08`; `GATE-09`; `DEF-CONOPS-008` | Dissent and distribution cannot be averaged away. | must | Personnel, family, workforce, and community analyst | Priority/readiness/savings composite-score rejection fixtures | pending_review |

### Quantitative methods, peer comparison, and adaptive accounting

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-ECO-001` | BASTION shall represent direct public cost reduction, delivery or process efficiency, avoided future cost or risk, readiness or capacity or resilience gain, lawful domain-relevant receipt effect, and a null pathway as six distinct non-interchangeable pathway postures. | `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-009`; `OQ-004`; `OQ-011` | Implements adaptive Taxlane support without a one-time quota. | must | Defense resource analyst | Pathway-enum inspection plus all-six-path fixture | pending_review |
| `BASTION-REQ-ECO-002` | BASTION shall not automatically add pathway values or convert avoided cost or risk, readiness or capacity or resilience gain, external benefit, private revenue, or operator revenue into booked public savings or public receipts. | `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-009`; `OQ-011` | Prevents category error and double counting. | must | Defense Comptroller | Non-additivity invariants plus forbidden-conversion fixtures | pending_review |
| `BASTION-REQ-ECO-003` | BASTION shall record a separate near-, medium-, and long-horizon `result` or `null` for every adaptive pathway where each horizon applies. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004`; `DEF-CONOPS-009`; `OQ-011` | Makes pathway effects explicit across time without selecting exact durations. | must | Defense resource analyst | Three-horizon result/null branch fixtures controlled by `TBD-QNT-001` | pending_review |
| `BASTION-REQ-ECO-004` | BASTION shall distinguish gross opportunity, realizable public savings, external benefit, lawful dedicated public receipts, lifecycle cost, collection or financing cost, transition and implementation cost, risk cost, realization timing, uncertainty, overlap, and net public fiscal pressure. | `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-009`; `OQ-004` | Preserves the whole-system fiscal bridge. | must | Defense resource analyst | Accounting-schema inspection plus category-collapse negative fixtures | pending_review |
| `BASTION-REQ-ECO-005` | BASTION shall distinguish budget authority, obligations, outlays, transfers, and offsetting receipts while preserving appropriation, account, period, and accountable fiscal-owner boundaries. | `OPS-004`; `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-009`; `OQ-004` | Prevents incomparable federal fiscal measures from being netted. | must | Defense Comptroller | Federal-measure fixture matrix plus boundary-reconciliation inspection | pending_review |
| `BASTION-REQ-ECO-006` | BASTION shall require realizable public savings to have lawful authority, a named realization owner, an evidence-supported capture mechanism, implementation and transition path, realization cost and schedule, cited-baseline reconciliation, and passed safety, readiness, distribution, supplier, alliance, and overlap gates. | `OPS-007`; `GATE-02` through `GATE-08`; `DEF-CONOPS-009`; `DEF-CONOPS-011` | Gross opportunity is not cashable savings. | must | Defense Comptroller | Realization-gate inspection plus ownerless and floor-failing savings fixtures | pending_review |
| `BASTION-REQ-ECO-007` | BASTION shall identify normalized peer functions and peer sets, source vintages, institutional and mission differences, input and output measures, units, price-year and purchasing-power treatment where applicable, uncertainty, and non-comparable portions before using a peer comparator. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004`; `OQ-004`; `OQ-011` | Makes peer comparison bounded and reproducible. | must | Defense resource analyst | Peer-manifest inspection plus incompatible-peer negative fixtures | pending_review |
| `BASTION-REQ-ECO-008` | BASTION shall treat a peer value as a diagnostic scenario reference. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004`; `OQ-011` | Defines the bounded role of peer evidence. | must | Scope Keeper | Peer-record inspection plus missing-diagnostic-posture fixture | pending_review |
| `BASTION-REQ-ECO-009` | BASTION shall record quantitative horizon, range, uncertainty method, cited-baseline reconciliation, downside case, and evidence-supported probability or independently reviewed `not_applicable` for every promoted projection. | `OPS-003`; `OPS-007`; `GATE-07`; `DEF-CONOPS-004`; `OQ-004` | Blocks false precision and one-path forecasts. | must | Defense resource analyst | Projection-schema inspection plus missing-range/downside fixtures | pending_review |
| `BASTION-REQ-ECO-010` | BASTION shall record discount, present-value, inflation, exchange-rate, and purchasing-power conventions when applicable. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004` | Makes quantitative bases explicit. | must | Numeracy Checker | Method-manifest branch fixture for each applicable convention | pending_review |
| `BASTION-REQ-ECO-011` | BASTION shall assign stable overlap keys to pathways, candidates, accounts, periods, organizations, suppliers, partners, and other fiscal lanes. | `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-009`; `DEF-CONOPS-012`; `OQ-010` | Makes possible overlaps addressable. | must | Defense Comptroller | Overlap-key completeness, uniqueness, and stability fixtures | pending_review |
| `BASTION-REQ-ECO-012` | BASTION shall re-evaluate every active pathway at its accepted cadence or upon a source revision, peer-comparability change, observed delivery variance, shock, burden shift, floor failure, authority change, overlap change, or ownership change. | `OPS-007`; `OPS-009`; `GATE-07`; `GATE-10`; `DEF-CONOPS-011`; `OQ-011` | Makes the fiscal bridge adaptive to outcomes and shocks. | must | Delivery owner | Reopen-trigger transition tests plus cadence inspection | pending_review |
| `BASTION-REQ-ECO-013` | BASTION shall record each re-evaluation outcome as preserve, revise, hold, retire, or replace. | `OPS-007`; `OPS-009`; `GATE-10`; `DEF-CONOPS-011`; `OQ-011` | Defines the adaptive pathway lifecycle. | must | Delivery owner | One accepted state-transition fixture for each lifecycle outcome | pending_review |
| `BASTION-REQ-ECO-014` | BASTION shall not treat a peer value or peer gap as a target mandate, savings amount, funding quota, allocation, or rate instruction. | `OPS-007`; `GATE-02`; `GATE-07`; `DEF-CONOPS-004`; `OQ-011` | Peer evidence does not create domestic realizability or authority. | must | Scope Keeper | Separate mandate, savings, quota, allocation, and rate-conversion rejection fixtures | pending_review |
| `BASTION-REQ-ECO-015` | BASTION shall hold cross-scenario totals whose units, horizons, price bases, or methods remain unreconciled. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004` | Incompatible quantitative bases cannot be totaled. | must | Numeracy Checker | One blocked-total fixture for each incompatibility basis | pending_review |
| `BASTION-REQ-ECO-016` | BASTION shall prevent double counting across pathways, candidates, accounts, periods, organizations, suppliers, partners, and other fiscal lanes. | `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-009`; `DEF-CONOPS-012`; `OQ-010` | Protects portfolio accounting integrity. | must | Defense Comptroller | One duplicate-effect fixture for each named overlap surface | pending_review |
| `BASTION-REQ-ECO-017` | BASTION shall retain the prior version, evidence, rationale, owner, and downstream notification posture for every pathway re-evaluation. | `OPS-007`; `OPS-009`; `GATE-10`; `DEF-CONOPS-011`; `OQ-011` | Makes adaptive decisions auditable. | must | Delivery owner | Retention completeness and attempted-history-loss fixtures | pending_review |
| `BASTION-REQ-ECO-018` | BASTION shall require a reasoned independently reviewed `not_applicable` posture and an explicit alternative time boundary when a near-, medium-, or long-horizon treatment is genuinely inapplicable. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004`; `OQ-011` | Prevents a horizon from being omitted by author discretion. | must | Defense resource analyst | N/A reason, independent-review, and alternative-boundary fixtures controlled by `TBD-QNT-001` | pending_review |
| `BASTION-REQ-ECO-019` | BASTION shall hold pathway promotion when any required horizon result, null, reviewed N/A, or alternative time boundary is missing or not accepted. | `OPS-007`; `GATE-07`; `DEF-CONOPS-004`; `DEF-CONOPS-009`; `OQ-011` | Makes the horizon gate fail closed. | must | Defense resource analyst | Missing and unaccepted horizon-treatment blocked-promotion fixtures | pending_review |
| `BASTION-REQ-ECO-020` | BASTION shall satisfy `PATHWAY-ENVELOPE-001` for every adaptive pathway. | `OPS-007`; `OPS-009`; `GATE-07`; `DEF-CONOPS-004`; `DEF-CONOPS-009`; `DEF-CONOPS-011`; `OQ-011` | Retains the complete evidence envelope as one all-or-hold state invariant. | must | Defense resource analyst | Branch-complete `PATHWAY-ENVELOPE-001` verification | pending_review |

### Independent test, review, and convergence

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-TST-001` | BASTION shall freeze and digest-bind each review artifact, evidence manifest, derivation set, gate matrix, negative case set, and unresolved-question set before independent review. | `OPS-008`; `GATE-09`; `DEF-CONOPS-010`; `OQ-003`; `OQ-009` | Ensures reviewers assess the same artifact. | must | Role review steward | Review-packet inspection plus stale-digest fixture | pending_review |
| `BASTION-REQ-TST-002` | BASTION shall require an independent reviewer to reproduce permitted quantitative results, inspect qualitative claim custody, and test adverse cases, failures, uncertainty, denominators, price years, lifecycle and transition arithmetic, and double counts. | `OPS-008`; `GATE-09`; `DEF-CONOPS-010`; `OQ-009` | Makes verification adversarial and evidence-based. | must | Independent Test and Oversight Officer | Reproduction report and seeded-error fixture suite | pending_review |
| `BASTION-REQ-TST-003` | BASTION shall retain negative results, failed tests, nulls, rejected candidates, dissent, and unresolved evidence. | `OPS-008`; `GATE-09`; `DEF-CONOPS-010` | Failure evidence is part of research truth. | must | Independent Test and Oversight Officer | Retention fixture for each named evidence posture | pending_review |
| `BASTION-REQ-TST-004` | BASTION shall record each finding with stable identity, reviewed digest, role, severity, affected claim, evidence pointer, disposition, owner, destination, substantive closure condition, reviewer independence, and dissent. | `OPS-008`; `OPS-010`; `GATE-09`; `DEF-CONOPS-010` | Makes remediation and deferral auditable. | must | Role review steward | Finding-schema inspection plus incomplete-finding fixtures | pending_review |
| `BASTION-REQ-TST-005` | BASTION shall hold fixed-point promotion for a stale review, conflicted reviewer, absent required role, failed assurance gate, unowned defer, false approval claim, or unresolved critical or major actionable finding. | `OPS-008`; `OPS-010`; `GATE-09`; `GATE-10`; `DEF-CONOPS-010` | Enforces VTRACE convergence. | must | BASTION maintainer | Convergence rule inspection plus each blocking-condition fixture | pending_review |
| `BASTION-REQ-TST-006` | BASTION shall not replace retained evidence with advocacy, credentials, or inaccessible classified appeals. | `OPS-008`; `GATE-01`; `GATE-09`; `DEF-CONOPS-010` | Prevents non-evidence from overriding failure or null results. | must | Independent Test and Oversight Officer | Advocacy, credential, and classified-appeal substitution rejection fixtures | pending_review |

### Delivery, feedback, and rollback

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-DEL-001` | BASTION shall require every delivery-testable candidate to name lawful authority, accountable owner, dependencies, resources, milestones, measures, safety and readiness floors, observation cadence, stop conditions, evaluation design, rollback boundary, and realization evidence plan. | `OPS-007`; `GATE-10`; `DEF-CONOPS-011`; `OQ-009` | Delivery feasibility is part of the candidate. | must | Delivery owner | Candidate-delivery schema inspection plus missing-owner/floor fixtures | pending_review |
| `BASTION-REQ-DEL-002` | BASTION shall hold a candidate as a research hypothesis when authority, ownership, resources, transition capacity, measurement, stop conditions, or rollback protection is absent or unresolved. | `OPS-007`; `GATE-10`; `DEF-CONOPS-011`; `OQ-009` | Prevents modeled gain from becoming false execution readiness. | must | Delivery owner | Candidate-state transition tests plus incomplete-delivery fixtures | pending_review |
| `BASTION-REQ-DEL-003` | BASTION shall compare observed delivery with the cited baseline and accepted peer posture at each observation point. | `OPS-007`; `GATE-07`; `GATE-10`; `DEF-CONOPS-011` | Anchors re-evaluation to accepted evidence. | must | Delivery owner | Baseline and peer comparison fixtures including missing-reference rejection | pending_review |
| `BASTION-REQ-DEL-004` | BASTION shall stop, hold, revise, retire, or replace a pathway when an accepted trigger fires. | `OPS-007`; `OPS-009`; `GATE-10`; `DEF-CONOPS-011`; `OQ-011` | Requires action when adaptive controls fire. | must | Delivery owner | One trigger-to-action fixture for each permitted disposition | pending_review |
| `BASTION-REQ-DEL-005` | BASTION shall not claim implementation or delivery readiness until later accepted specifications, design, interfaces, verification plans, and a relevant accepted work package supply the required evidence. | `OPS-010`; `GATE-10`; `DEF-CONOPS-011`; `OQ-009` | Requirements are not implementation authority. | must | BASTION maintainer | Stage-gate inspection plus premature-readiness negative fixture | pending_review |
| `BASTION-REQ-DEL-006` | BASTION shall detect schedule, cost, burden, overlap, safety, readiness, supplier, workforce, community, and alliance deviations at each observation point. | `OPS-007`; `GATE-03` through `GATE-08`; `GATE-10`; `DEF-CONOPS-011` | Re-evaluation must include floors and incidence, not only cost. | must | Delivery owner | One seeded-deviation fixture for each named deviation surface | pending_review |
| `BASTION-REQ-DEL-007` | BASTION shall preserve the reason, evidence, version, owner, rollback posture, and downstream notification for every trigger-driven pathway action. | `OPS-007`; `OPS-009`; `GATE-10`; `DEF-CONOPS-011`; `OQ-011` | Makes adaptive correction auditable. | must | Delivery owner | Action-record completeness and missing-notification fixtures | pending_review |

### Held LaneEvidencePack and Taxlane boundary

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-HND-001` | BASTION shall prepare only a `held` LaneEvidencePack candidate containing BASTION and source identities, artifact and adapter digests, gate and security postures, distinct pathway and fiscal ledgers, nulls, uncertainty, downside, distribution, delivery state, overlap keys, provenance, residual risk, and dissent. | `OPS-009`; `GATE-10`; `DEF-CONOPS-012`; `OQ-010` | Preserves the bounded cross-repo evidence contract. | must | Taxlane adapter steward | Interface-record inspection plus complete/incomplete handoff fixtures | pending_review |
| `BASTION-REQ-HND-002` | BASTION shall preserve the six pathway postures, federal fiscal measures, peer-comparability limits, realization owner, cadence, transition costs, uncertainty, overlap, floors, and valid nulls without converting them during adapter mapping. | `OPS-009`; `GATE-07`; `GATE-10`; `DEF-CONOPS-009`; `DEF-CONOPS-012`; `OQ-010`; `OQ-011` | Prevents semantic loss at the fiscal boundary. | must | Taxlane adapter steward | Round-trip mapping inspection plus category-loss fixtures | pending_review |
| `BASTION-REQ-HND-003` | BASTION shall reject a stale, incomplete, unsafe, unreconciled, unowned, double-counted, falsely precise, or floor-failing handoff. | `OPS-009`; `GATE-01`; `GATE-03` through `GATE-10`; `DEF-CONOPS-012` | Handoff failures must fail closed. | must | Taxlane adapter steward | One adapter rejection fixture for each named failure state | pending_review |
| `BASTION-REQ-HND-004` | BASTION shall mark Taxlane admission state as externally controlled. | `OPS-009`; `GATE-10`; `DEF-CONOPS-012`; `OQ-010` | Makes admission ownership explicit. | must | Taxlane adapter steward | Ownership-state inspection plus missing-external-control fixture | pending_review |
| `BASTION-REQ-HND-005` | BASTION shall revalidate scope, source custody, compositional security, civilian authority, safety and readiness floors, numeracy, reviewer independence, adapter compatibility, and digest freshness before each held handoff. | `OPS-009`; `GATE-01` through `GATE-10`; `DEF-CONOPS-012` | Prevents stale or context-shifted evidence transfer. | must | Taxlane adapter steward | Pre-handoff gate report plus stale/context-change fixtures | pending_review |
| `BASTION-REQ-HND-006` | BASTION shall not fabricate a required interface value. | `OPS-009`; `GATE-10`; `DEF-CONOPS-012` | Missing interface evidence must remain null or rejected. | must | Taxlane adapter steward | Missing-required-value fabrication rejection fixture | pending_review |
| `BASTION-REQ-HND-007` | BASTION shall not infer Taxlane admission, cross-lane combination, allocation, rebalance, rate, official use, or publication from handoff creation. | `OPS-009`; `GATE-10`; `DEF-CONOPS-012`; `OQ-010` | Taxlane alone owns downstream fiscal and official actions. | must | Taxlane adapter steward | Separate false-inference fixture for each prohibited downstream action | pending_review |

### Release and misuse controls

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-REL-001` | BASTION shall not publish, publicly release, or represent an artifact as approved without separate explicit release authority and a release-specific fixed-point review. | `OPS-008`; `OPS-010`; `GATE-01`; `GATE-10`; `DEF-CONOPS-013` | Current planning creates no release authority. | must | Scope Keeper | Release-state inspection plus unauthorized-release fixture | pending_review |
| `BASTION-REQ-REL-002` | BASTION shall require any separately authorized future release plan to assess direct and cross-release composition, linkage, sensitive context, audience misuse, source and review staleness, provenance retention, correction or takedown, and security and scope acceptance. | `OPS-001`; `OPS-008`; `GATE-01`; `DEF-CONOPS-013`; `OQ-001` | Repeated safe-looking releases can create unsafe inference. | must | Classification and Operational Security reviewer | Release-threat-model inspection plus mosaicing and stale-release fixtures | pending_review |
| `BASTION-REQ-REL-003` | BASTION shall preserve source, derivation, limitation, uncertainty, dissent, security posture, and non-authority context in any future separately authorized communication. | `OPS-008`; `GATE-01`; `GATE-09`; `GATE-10`; `DEF-CONOPS-013` | Prevents context stripping and claim inflation. | must | Citation Auditor | Communication inspection plus context-loss negative fixture | pending_review |

### VTRACE stage control

| ID | Requirement | Parent need / scenario | Rationale | Priority | Owner | Verification method | Status |
|---|---|---|---|---|---|---|---|
| `BASTION-REQ-VTR-001` | BASTION shall retain stable links from every accepted requirement to parent CONOPS gates, scenarios, deferrals, and open questions and later to specification, design, interface, verification, validation, and evidence identities. | `OPS-010`; `GATE-09`; `DEF-CONOPS-003`; `DEF-CONOPS-010`; `OQ-003` | Establishes the VTRACE source-to-evidence chain. | must | BASTION maintainer | Trace-orphan inspection plus unknown-parent fixture | pending_review |
| `BASTION-REQ-VTR-002` | BASTION shall not advance the requirements stage until independent review confirms unique stable IDs, clear normative grammar, ownership, credible verification, complete `OPS-001` through `OPS-010`, `GATE-01` through `GATE-10`, `DEF-CONOPS-001` through `DEF-CONOPS-013`, and `OQ-001` through `OQ-011` coverage, and zero unresolved critical or major findings. | `OPS-010`; `GATE-09`; `GATE-10` | Defines requirements fixed-point convergence. | must | Role review steward | Requirements validator plus coverage and finding-ledger inspection | pending_review |
| `BASTION-REQ-VTR-003` | BASTION shall not start Rust or other implementation until the applicable requirements, specifications, architecture, interfaces, design, verification plan, and work package have reached their required recorded gates. | `OPS-010`; `GATE-10`; `DEF-CONOPS-011` | Prevents premature implementation. | must | BASTION maintainer | Stage-state inspection plus premature-work-package negative fixture | pending_review |

## Coverage matrix

### Gate and scenario coverage

| Parent | Primary requirement coverage |
|---|---|
| `GATE-01` / `OPS-001` | `BASTION-REQ-SCP-001` through `SCP-005`; `SCP-009`; `SCP-010`; `SRC-001` through `SRC-008`; `REL-002` |
| `GATE-02` / `OPS-002` | `BASTION-REQ-SCP-006`; `SCP-007`; `RDY-001`; `ALLY-004`; `ECO-014` |
| `GATE-03` / `OPS-003` | `BASTION-REQ-RDY-001` through `RDY-007`; `DEL-003`; `DEL-006`; `HND-003` |
| `GATE-04` / `OPS-004` | `BASTION-REQ-ACQ-001` through `ACQ-008`; `ECO-005` |
| `GATE-05` / `OPS-005` | `BASTION-REQ-LOG-001` through `LOG-008`; `RDY-006` |
| `GATE-06` / `OPS-006` | `BASTION-REQ-ALLY-001` through `ALLY-006`; `ECO-006` |
| `GATE-07` / `OPS-007` | `BASTION-REQ-ECO-001` through `ECO-020`; `ACQ-004`; `LOG-005`; `DST-001` through `DST-005` |
| `GATE-09` / `OPS-008` | `BASTION-REQ-TST-001` through `TST-006`; `SRC-002`; `SRC-008`; `REL-001` through `REL-003` |
| `GATE-10` / `OPS-009` | `BASTION-REQ-HND-001` through `HND-007`; `ECO-001` through `ECO-020`; `DEL-004`; `DEL-007` |
| `GATE-10` / `OPS-010` | `BASTION-REQ-SCP-008`; `SCP-010`; `TST-004`; `TST-005`; `DEL-005`; `VTR-001` through `VTR-003` |
| `GATE-08` across `OPS-003` through `OPS-007` | `BASTION-REQ-DST-001` through `DST-005`; `ACQ-005`; `ALLY-002`; `ALLY-005`; `ECO-006`; `DEL-006` |

### Deferral and open-question coverage

| Parent defer / question | Requirement coverage | Remaining controlled TBD |
|---|---|---|
| `DEF-CONOPS-001`; `OQ-001` | `BASTION-REQ-SCP-001` through `SCP-003`; `SCP-009`; `SRC-005`; `SRC-006`; `LOG-006`; `REL-002` | `TBD-SEC-001` |
| `DEF-CONOPS-002`; `OQ-002` | `BASTION-REQ-SCP-006`; `RDY-001` through `RDY-007` | `TBD-RDY-001` |
| `DEF-CONOPS-003`; `OQ-003` | `BASTION-REQ-SCP-004`; `SRC-001` through `SRC-004`; `SRC-007`; `SRC-008`; `VTR-001` | `TBD-SRC-001` |
| `DEF-CONOPS-004`; `OQ-004` | `BASTION-REQ-RDY-004`; `ECO-003`; `ECO-007` through `ECO-010`; `ECO-014`; `ECO-015`; `ECO-018` through `ECO-020` | `TBD-QNT-001` |
| `DEF-CONOPS-005`; `OQ-005` | `BASTION-REQ-ACQ-001` through `ACQ-008` | `TBD-ACQ-001` |
| `DEF-CONOPS-006`; `OQ-006` | `BASTION-REQ-LOG-001` through `LOG-008`; `RDY-001`; `RDY-004` | `TBD-LOG-001` |
| `DEF-CONOPS-007`; `OQ-007` | `BASTION-REQ-ALLY-001` through `ALLY-006` | `TBD-ALLY-001` |
| `DEF-CONOPS-008`; `OQ-008` | `BASTION-REQ-DST-001` through `DST-005`; `ACQ-005`; `ALLY-002`; `ALLY-005` | `TBD-DST-001` |
| `DEF-CONOPS-009`; `OQ-004`; `OQ-011` | `BASTION-REQ-ECO-001` through `ECO-020`; `HND-002` | `TBD-ECO-001` |
| `DEF-CONOPS-010`; `OQ-003`; `OQ-009` | `BASTION-REQ-TST-001` through `TST-006`; `VTR-001`; `VTR-002` | `TBD-TST-001` |
| `DEF-CONOPS-011`; `OQ-009`; `OQ-011` | `BASTION-REQ-ECO-012`; `ECO-013`; `ECO-017`; `ECO-020`; `DEL-001` through `DEL-007` | `TBD-DEL-001` |
| `DEF-CONOPS-012`; `OQ-010`; `OQ-011` | `BASTION-REQ-ECO-011`; `ECO-016`; `HND-001` through `HND-007` | `TBD-HND-001` |
| `DEF-CONOPS-013`; `OQ-001` | `BASTION-REQ-SRC-005`; `SRC-006`; `REL-001` through `REL-003` | `TBD-REL-001` |

## Direct requirement-to-TBD dependency trace

The following is a normative dependency trace. Every listed requirement
inherits the controlling TBD's hold behavior until that TBD is independently
accepted. Requirements not listed here depend only on values already explicit
at this stage; they may still be held by their ordinary gate conditions.

| Controlling TBD | Directly dependent normative requirements | Inherited hold |
|---|---|---|
| `TBD-SEC-001` | `BASTION-REQ-SCP-001` through `SCP-003`; `SCP-009`; `SRC-005`; `SRC-006`; `LOG-006`; `HND-003`; `HND-005`; `REL-002` | Hold affected admission, derivation, visualization, handoff, and release. |
| `TBD-RDY-001` | `BASTION-REQ-SCP-006`; `RDY-001` through `RDY-007`; `DEL-001` through `DEL-003`; `DEL-006`; `HND-003`; `HND-005` | Hold affected readiness, candidate, savings, and handoff claims. |
| `TBD-SRC-001` | `BASTION-REQ-SCP-004`; `SRC-001` through `SRC-004`; `SRC-007`; `SRC-008`; `TST-001`; `TST-004`; `TST-005`; `VTR-001`; `VTR-002` | Hold artifacts whose custody, version, derivation, or review binding cannot be represented. |
| `TBD-QNT-001` | `BASTION-REQ-RDY-004`; `ECO-003`; `ECO-007` through `ECO-010`; `ECO-014`; `ECO-015`; `ECO-018` through `ECO-020`; `HND-002`; `HND-005` | Hold affected projections, peer comparisons, horizons, totals, and handoffs. |
| `TBD-ACQ-001` | `BASTION-REQ-ACQ-001` through `ACQ-008`; `DST-003`; `DEL-006` | Hold affected acquisition, commonality, capacity, schedule, and savings results. |
| `TBD-LOG-001` | `BASTION-REQ-LOG-001` through `LOG-008`; `RDY-002`; `RDY-004` through `RDY-006`; `DEL-006` | Hold affected sustainment, readiness, lifecycle, and savings results. |
| `TBD-ALLY-001` | `BASTION-REQ-ALLY-001` through `ALLY-006`; `DST-001`; `DST-002`; `DST-004`; `DST-005`; `DEL-006` | Hold joint, interoperability, commitment, burden, and fiscal claims. |
| `TBD-DST-001` | `BASTION-REQ-DST-001` through `DST-005`; `ACQ-005`; `ALLY-002`; `ALLY-005`; `ECO-003`; `ECO-006`; `ECO-020`; `DEL-006`; `HND-001` through `HND-003`; `HND-005` | Hold affected efficiency, savings, readiness, distribution, and handoff claims. |
| `TBD-ECO-001` | `BASTION-REQ-ECO-001` through `ECO-006`; `ECO-009` through `ECO-013`; `ECO-015` through `ECO-017`; `ECO-019`; `ECO-020`; `HND-001` through `HND-006` | Hold monetization, realizable-savings, receipt, net-pressure, and Taxlane handoff claims. |
| `TBD-TST-001` | `BASTION-REQ-TST-001` through `TST-006`; `VTR-001`; `VTR-002` | Hold fixed point and every downstream stage. |
| `TBD-DEL-001` | `BASTION-REQ-ECO-012`; `ECO-013`; `ECO-017`; `ECO-020`; `DEL-001` through `DEL-007`; `HND-001` through `HND-003`; `HND-005` | Retain research-hypothesis posture and block realizable savings, implementation, and handoff. |
| `TBD-HND-001` | `BASTION-REQ-ECO-011`; `ECO-016`; `HND-001` through `HND-007` | Hold every adapter package and infer no Taxlane admission. |
| `TBD-REL-001` | `BASTION-REQ-SRC-005`; `SRC-006`; `REL-001` through `REL-003` | Authorize no public release. |

## Controlled unresolved values

Every record is open. A future specification may close it only through the
named condition and independent review. Until then, affected promotion is held;
no implementation may choose a hidden default.

| TBD ID | Unresolved value or method | Owner | Destination | Closure condition | Hold behavior | Status |
|---|---|---|---|---|---|---|
| `TBD-SEC-001` | Safe aggregation, suppression or equivalent control, compositional-inference limits, expiry, and re-review triggers by analysis class. | Security and aggregation steward | Security specification and verification | Threat model, machine-checkable fields, direct/composition/repeated-release negative fixtures, and independent security acceptance establish safe rules without publishing exploitable thresholds. | Hold admission, derivation, visualization, handoff, and release for affected fields or combinations. | open |
| `TBD-RDY-001` | Exact readiness, safety, resilience, surge, mobilization, recovery measures, horizons, distributions, and non-waivable floors. | Readiness-system analyst | Promise specification and verification | Public authority, definitions, denominators, horizons, tail/degraded treatment, evidence rules, thresholds or explicit decisions, negative fixtures, and both assurance acceptances exist. | Hold the affected readiness result, candidate, savings claim, and handoff. | open |
| `TBD-SRC-001` | Exact source, identity, evidence-label, claim, derivation, version, null, rejection, and review-digest schemas. | Public-evidence steward | Corpus and interface specifications | Schemas and fixtures prove supersession, stale-review rejection, provenance separation, deterministic reproduction, null, and rejection. | Hold any artifact whose custody cannot be represented without loss. | open |
| `TBD-QNT-001` | Exact horizon representation, uncertainty and probability rules, price-year, inflation, exchange, purchasing-power, discount, present-value, peer normalization, and reconciliation tolerances. | Defense resource analyst | Quantitative methods specification | Methods state applicability, evidence basis, units, ranges, downside, peer limits, null behavior, tolerances, and pass numeracy/methodology fixtures. | Hold affected projection, peer comparison, cross-scenario total, and fiscal handoff. | open |
| `TBD-ACQ-001` | Exact acquisition, capacity, competition, concentration, qualification, learning, workforce, transition, and six commonality-facet measures and limits. | Acquisition and industrial-base analyst | Industrial-base specification | Safe measures cover both commonality benefits and risks, denominators, evidence tiers, small-supplier incidence, null/N/A, negative cases, and security/industrial-base acceptance. | Hold affected acquisition, commonality, capacity, schedule, or savings result. | open |
| `TBD-LOG-001` | Exact stock-policy, inventory/condition/custody reconciliation, repair-time start/stop/censoring, percentile/tail, workload, availability, and degraded-recovery measures. | Logistics and sustainment analyst | Readiness and sustainment specification | Compatible lifecycle and stock boundaries, accepted distribution/tail methods, safe aggregation, null/N/A behavior, negative fixtures, and readiness/security acceptance exist. | Hold affected sustainment, readiness, lifecycle, and savings result. | open |
| `TBD-ALLY-001` | Exact public commitment, compatibility, standard, sovereignty/control, partner-capacity, separated-ledger, and burden-incidence representation. | Alliance and interoperability analyst | Interoperability specification and interface | Public authority sources, safe semantics, uncertainty/null rules, partner-risk fixtures, and security/legal review exist without operational inference. | Hold joint, interoperability, commitment, burden, and fiscal claims. | open |
| `TBD-DST-001` | Exact stakeholder burden, distribution, incidence, concentrated-effect, and tail measures. | Personnel, family, workforce, and community analyst | Distribution specification | Each stakeholder lens has denominator, time/horizon, baseline, evidence, tail treatment, burden-shift tests, null/N/A, and independent stakeholder/assurance acceptance. | Hold affected efficiency, savings, readiness, and handoff claims. | open |
| `TBD-ECO-001` | Exact multi-path accounting schema, federal fiscal field semantics, overlap rules, realization thresholds, and net-pressure equation fixtures. | Defense Comptroller | Economics specification and shared accounting interface | Schema keeps six pathways distinct, distinguishes budget authority/obligations/outlays/transfers/offsetting receipts, preserves appropriation/fiscal ownership, costs, timing, uncertainty, peer limits, overlap, valid nulls, and passes numeracy and Taxlane interface review. | Hold monetization, realizable-savings, receipt, net-pressure, and Taxlane handoff claims. | open |
| `TBD-TST-001` | Exact evidence tiers, reproduction criteria, reviewer conflicts, severity/disposition schema, and convergence fixtures. | Independent Test and Oversight Officer | Verification plan | Positive and negative fixtures prove digest binding, independence, failure retention, finding completeness, assurance presence, and zero-major convergence. | Hold fixed point and every downstream stage. | open |
| `TBD-DEL-001` | Exact delivery-readiness evidence, observation cadence, reopen triggers, milestone, stop, rollback, transition, and realization criteria. | Delivery owner | Delivery specification and future accepted work package | Authority, ownership, resources, floors, observed-versus-baseline and peer refresh, burden detection, stop/rollback, realization proof, and independent validation are accepted. | Retain research-hypothesis posture; block realizable savings, implementation, and handoff. | open |
| `TBD-HND-001` | Exact LaneEvidencePack fields, BASTION/Taxlane identities, adapter ownership, compatibility, rejection, overlap, and held/admitted state semantics. | Taxlane adapter steward | Interface specification after shared Taxlane review | Both repos accept schema and fixtures for separated ledgers, pathways, nulls, uncertainty, security, gates, delivery, overlap, provenance, rejection, and authority ownership. | Hold every adapter package; infer no Taxlane admission. | open |
| `TBD-REL-001` | Exact release composition, audience, misuse, correction, takedown, repeated-release, and provenance-retention controls if release is separately authorized. | Scope Keeper | Release-specific requirements and validation under new authority | Separately authorized plan passes compositional threat model, stale/context-loss/misuse fixtures, and independent security, citation, and scope acceptance. | No public release. | open |

## Preliminary role-review notes

These are author checks only; they do not record a role pass.

| Review surface | Author check | Independent review focus |
|---|---|---|
| Civilian strategy and law | Mission authority and prohibited decisions are explicit. | Detect disguised force planning, procurement, resource allocation, or authority transfer. |
| Operational readiness | Eleven readiness facets, degraded paths, floors, and nulls are mandatory. | Test measurability without sensitive operational inference or proxy optimism. |
| Acquisition and industrial base | Delivery factors and six commonality facets are separately gated. | Test commonality benefits and concentration/common-cause risks, including small suppliers. |
| Logistics and sustainment | Inventory boundaries and repair distributions/tails are mandatory. | Test stock reconciliation, censoring, degraded recovery, lifecycle costs, and safe abstraction. |
| Comptroller and numeracy | Six pathways, fiscal measures, costs, timing, uncertainty, peers, and overlap remain distinct. | Test gross-to-net logic, non-additivity, federal field semantics, and false precision. |
| Service members, families, workforce, suppliers, communities, taxpayers, allies | Distribution, burden shifts, concentrated effects, and valid nulls are explicit. | Test missing facets, unsafe aggregation, and burdens hidden by averages. |
| Independent test | Frozen digests, reproducibility, negative evidence, dissent, and convergence are required. | Test whether planned verification can actually falsify every material claim. |
| Alliance and interoperability | Sovereignty, controls, partner constraints, separated ledgers, and degraded cases are explicit. | Test domestic optimization that shifts partner burden or infers partner posture. |
| Citation, scope, and security | Source custody, compositional inference, authority, and release controls are explicit. | Test joins, repeated release, stale context, false approval, and dangerous public combinations. |
| Methodology panel | Requirements trace from every scenario, gate, defer, and question. | Challenge feasibility, clarity, measurable conditions, and TBD closure sufficiency. |

## Requirements quality and fixed-point decision

| Review stage | Decision | Result |
|---|---|---|
| Initial author baseline | review-ready | 75 normative requirements and 13 open TBD records. |
| Independent substance and traceability review | finding | Two grouped major findings: compound requirement atomicity and indirect requirement-to-TBD dependency trace. |
| Independent stakeholder, numeracy, and assurance review | finding | Two grouped major findings: incomplete three-horizon treatment and overly narrow cross-stakeholder ownership. |
| Bounded remediation | remediated | 98 atomic normative requirements; explicit horizon result/null/reviewed-N/A/alternative-boundary holds; corrected stakeholder ownership and required reviewers; direct dependency trace for all 13 TBDs. |
| Independent cross-convergence recheck | pass_with_risk | Counts, identities, atomicity, ownership, direct TBD links, coverage, boundaries, and decision consistency passed. |

Current requirements findings: **zero critical; zero major; zero minor**.

Fixed-point decision: **pass_with_risk**. All 13 `TBD-*` records remain open,
owned, directly referenced, and promotion-gating. Their verification methods
are planned evidence, not implementation evidence or proof that a method or
value has been accepted.

The next eligible VTRACE artifact is `SPECIFICATION_BASELINE.md`, only under a
new assignment. This fixed point confers no specification, design, interface,
work-package, implementation, operational, procurement, budget, Taxlane-
admission, allocation, rate-setting, official-use, or public-release authority.
