# BASTION Specification Baseline

## Status and scope

Repo: BASTION

Baseline type: **mixed** — target logical behavior, current planning-only
authority boundaries, and explicitly unknown methods or values.

Status: **pass_with_risk; specification fixed point reached**.

Controlled REQUIREMENTS input SHA-256:
`9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e`

This baseline defines logical, versioned behavior and acceptance boundaries. It
does not define APIs, schemas, crates, packages, modules, languages, algorithms,
storage, deployment, force structure, procurement, budgets, rates, operational
methods, or sensitive thresholds. Every verification item is planned evidence,
not an executed result.

## Baseline sources

| Source | Evidence used | State | Boundary |
|---|---|---|---|
| `MISSION.md` | Purpose, users, outcomes, constraints, success | current | Planning authority only. |
| `CONOPS.md` | Actors, gates, scenarios, holds, handoffs, deferrals | current | No implementation or operational authority. |
| `REQUIREMENTS.md` | 98 fixed-point normative requirements and 13 open TBDs | current | Requirements fixed point; methods and values remain held. |
| `.roles/` | Parliament, stakeholder, editorial, assurance, methodology lenses | current | Internal role review; no external approval. |
| Product implementation, corpus, tests, released interfaces | None accepted | unknown | No current product behavior is claimed. |

## Specification conventions

- Every `SPEC-<family>-<number>` maps one-to-one to
  `BASTION-REQ-<family>-<number>` unless a row explicitly states otherwise.
- Every `SPEC-*` maps one-to-one to the planned `VER-*` item with the same
  family and number.
- `target` describes controlled future behavior, not current implementation.
- `current` is limited to repo-local planning and no-authority controls already
  stated in settled governance.
- `held` means a named `SPEC-UNK-*` / `TBD-*` dependency blocks promotion.
- `proposed` means independently reviewable at specification scope; it does not
  authorize implementation.
- A missing, stale, unsafe, incompatible, unreconciled, or unowned input fails
  closed. Unknown is never default, zero, or inferred.

## Controlled specification items

| Spec ID | Parent REQ IDs | Type | State | Precise specification statement | Planned verification | Planned validation | Owner | Risk | Inherited hold | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| `SPEC-SCP-001` | `BASTION-REQ-SCP-001` | assurance | target | Source admission returns accepted only for public, aggregate, unclassified, non-operational evidence with an accepted direct-and-compositional security posture; every other branch is held or rejected. | `VER-SCP-001`: admission decision-table fixtures | `VAL-SRC-001`: safe-source scenario | Security and aggregation steward | high | `SPEC-UNK-SEC-001` | held |
| `SPEC-SCP-002` | `BASTION-REQ-SCP-002` | assurance | current | Every controlled boundary rejects ingest, retention, derivation, and emission of classified information, CUI, person-level service data, sensitive operational data, targeting content, operational-planning content, and exploitable vulnerability content. | `VER-SCP-002`: one prohibited-content fixture for each ingest, retain, derive, and emit boundary | `VAL-SRC-002`: unsafe-source and unsafe-output refusal | Classification and Operational Security reviewer | high | `SPEC-UNK-SEC-001` | held |
| `SPEC-SCP-003` | `BASTION-REQ-SCP-003` | control | target | Artifact state becomes held or rejected for each enumerated missing, stale, non-comparable, unsafe, unreconciled, unowned, falsely precise, or double-counted evidence condition. | `VER-SCP-003`: one transition fixture per condition | `VAL-HOLD-001`: fail-closed scenario | BASTION maintainer | high | `SPEC-UNK-SEC-001` | held |
| `SPEC-SCP-004` | `BASTION-REQ-SCP-004` | control | target | Promotion and review records resolve to one stable artifact identity, version, digest, gate posture, owner, and supersession relation; a mismatch is stale. | `VER-SCP-004`: identity and digest mismatch fixtures | `VAL-REV-001`: frozen-review scenario | Role review steward | high | `SPEC-UNK-SRC-001` | held |
| `SPEC-SCP-005` | `BASTION-REQ-SCP-005` | control | target | Each applicable gate field stores evidence plus exactly one `result`, `null`, or reasoned independently reviewed `not_applicable` posture. | `VER-SCP-005`: posture exhaustiveness fixtures | `VAL-GATE-001`: conjunctive-gate scenario | BASTION maintainer | high | none | proposed |
| `SPEC-SCP-006` | `BASTION-REQ-SCP-006` | authority | target | A dependent analysis is promotable only after its public mission abstraction, lawful civilian authority, jurisdiction, owner, non-delegable decisions, period, analytic boundary, and ambiguity posture are recorded. | `VER-SCP-006`: authority-manifest completeness fixtures | `VAL-AUTH-001`: civilian-authority scenario | Civilian mission and authority steward | high | `SPEC-UNK-RDY-001` | held |
| `SPEC-SCP-007` | `BASTION-REQ-SCP-007` | authority | current | No controlled output field or disposition selects missions, force structure, force employment, targets, tactics, procurement, resource allocation, budgets, rates, operational methods, or official recommendations; the Civilian Control, Law, Safety & Readiness assurance owner is accountable and promotion requires recorded Scope Keeper concurrence. | `VER-SCP-007`: prohibited-decision output fixtures plus assurance/scope concurrence inspection | `VAL-AUTH-002`: civilian-authority and scope refusal scenario | Civilian Control, Law, Safety & Readiness reviewer | high | none | proposed |
| `SPEC-SCP-008` | `BASTION-REQ-SCP-008` | authority | current | Planning, review, candidate, and held-handoff states cannot encode implementation, acquisition, budget, Taxlane, rate, official-use, or release approval. | `VER-SCP-008`: false-authority fixtures | `VAL-AUTH-003`: no-approval scenario | Scope Keeper | high | none | proposed |
| `SPEC-SCP-009` | `BASTION-REQ-SCP-009` | assurance | target | A rejection receipt retains only safe reason, identity, date, and review posture sufficient for audit without prohibited content or reconstructive detail. | `VER-SCP-009`: safe-receipt and reconstruction fixtures | `VAL-SRC-003`: safe rejection custody | Security and aggregation steward | high | `SPEC-UNK-SEC-001` | held |
| `SPEC-SCP-010` | `BASTION-REQ-SCP-010` | control | target | Promotion is blocked when any applicable gate field is missing, implicit, or author-waived. | `VER-SCP-010`: three blocked-promotion branches | `VAL-GATE-002`: incomplete-gate scenario | BASTION maintainer | high | none | proposed |
| `SPEC-SRC-001` | `BASTION-REQ-SRC-001` | data | target | Each source and source version has a stable, non-reused identity and explicit supersession relation. | `VER-SRC-001`: identity stability fixtures | `VAL-SRC-004`: source revision scenario | Public-evidence steward | medium | `SPEC-UNK-SRC-001` | held |
| `SPEC-SRC-002` | `BASTION-REQ-SRC-002` | data | target | Each material claim has exactly one declared claim class: `source_fact`, `derived_measure`, `analytic_assumption`, `scenario_result`, `finding`, `null`, or `rejected`. | `VER-SRC-002`: exhaustiveness, exclusivity, and unknown-label fixtures over all seven named claim classes | `VAL-CLAIM-001`: source-to-claim scenario | Public-evidence steward | high | `SPEC-UNK-SRC-001` | held |
| `SPEC-SRC-003` | `BASTION-REQ-SRC-003` | data | target | A derivation record identifies exact input versions, method identity, assumptions, units, uncertainty, output identity, and reproducibility posture. | `VER-SRC-003`: derivation completeness fixtures | `VAL-CLAIM-002`: regeneration scenario | Public-evidence steward | high | `SPEC-UNK-SRC-001` | held |
| `SPEC-SRC-004` | `BASTION-REQ-SRC-004` | data | target | Superseded, revised, rejected, held, and null records remain distinct and historical runs retain their original inputs; missing never becomes zero. | `VER-SRC-004`: version/null regression fixtures | `VAL-CLAIM-003`: historical reproduction | BASTION maintainer | high | `SPEC-UNK-SRC-001` | held |
| `SPEC-SRC-005` | `BASTION-REQ-SRC-005` | assurance | target | Promotion requires a recorded assessment of direct sensitivity, small groups, joins, mosaicing, derived granularity, visualization, repeated release, and cross-artifact inference. | `VER-SRC-005`: threat-surface fixture matrix | `VAL-PRIV-001`: composition-risk scenario | Security and aggregation steward | high | `SPEC-UNK-SEC-001`; `SPEC-UNK-REL-001` | held |
| `SPEC-SRC-006` | `BASTION-REQ-SRC-006` | control | target | Any source, join, granularity, derivation, visualization, audience, release-context, or expiry change reopens admission and security review. | `VER-SRC-006`: change-trigger transitions | `VAL-PRIV-002`: context-change scenario | Security and aggregation steward | high | `SPEC-UNK-SEC-001`; `SPEC-UNK-REL-001` | held |
| `SPEC-SRC-007` | `BASTION-REQ-SRC-007` | data | target | Each source-version record contains the complete publisher, custody, access, vintage, reuse, scope, aggregation, unit, denominator, cadence, revision, exclusion, and limitation field set. | `VER-SRC-007`: one missing-field rejection per field | `VAL-SRC-005`: source custody review | Public-evidence steward | medium | `SPEC-UNK-SRC-001` | held |
| `SPEC-SRC-008` | `BASTION-REQ-SRC-008` | data | target | Vendor, advocacy, official-statement, and independent-test provenance remain distinct and cannot be promoted by relabeling. | `VER-SRC-008`: provenance-collapse fixtures | `VAL-CLAIM-004`: conflicting-source scenario | Public-evidence steward | high | `SPEC-UNK-SRC-001` | held |
| `SPEC-RDY-001` | `BASTION-REQ-RDY-001` | product | target | A readiness or safety promise records authority, boundary, measure, denominator, period, horizon, evidence rule, tail, degraded mode, and hold rule as one complete promise record. | `VER-RDY-001`: promise completeness fixtures | `VAL-RDY-001`: public-promise scenario | Readiness-system analyst | high | `SPEC-UNK-RDY-001` | held |
| `SPEC-RDY-002` | `BASTION-REQ-RDY-002` | product | target | Staffing, training, personnel safety, availability, integration, maintenance, supply, repair, resilience, surge, mobilization, and recovery have separate evidence and postures. | `VER-RDY-002`: facet coverage fixtures | `VAL-RDY-002`: readiness-network scenario | Readiness-system analyst | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-LOG-001` | held |
| `SPEC-RDY-003` | `BASTION-REQ-RDY-003` | product | target | Readiness reconciliation reports differences in definitions, denominators, system boundaries, periods, and vintages against cited aggregates. | `VER-RDY-003`: mismatch reconciliation fixtures | `VAL-RDY-003`: baseline reproduction | Readiness-system analyst | high | `SPEC-UNK-RDY-001` | held |
| `SPEC-RDY-004` | `BASTION-REQ-RDY-004` | product | target | Every promoted readiness candidate contains a bounded non-operational downside or degraded-support result and retained failure evidence. | `VER-RDY-004`: missing-downside fixture | `VAL-RDY-004`: degraded-support scenario | Operational Readiness Officer | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-QNT-001`; `SPEC-UNK-LOG-001` | held |
| `SPEC-RDY-005` | `BASTION-REQ-RDY-005` | assurance | target | A candidate state becomes held when lawful authority, safety, readiness, resilience, surge, recovery, or alliance-floor evidence fails or is absent. | `VER-RDY-005`: one floor-failure fixture per floor | `VAL-RDY-005`: non-degradation scenario | Civilian Control, Law, Safety & Readiness reviewer | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-LOG-001` | held |
| `SPEC-RDY-006` | `BASTION-REQ-RDY-006` | assurance | target | Spending, inventory counts, planned values, and averages alone are invalid evidence for employable readiness, resilience, or safety. | `VER-RDY-006`: proxy-only rejection fixtures | `VAL-RDY-006`: proxy-misuse scenario | Operational Readiness Officer | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-LOG-001` | held |
| `SPEC-RDY-007` | `BASTION-REQ-RDY-007` | product | target | Unreconciled readiness series remain separate and cannot populate a comparative result. | `VER-RDY-007`: series-separation fixtures | `VAL-RDY-007`: incompatible-series scenario | Readiness-system analyst | high | `SPEC-UNK-RDY-001` | held |
| `SPEC-ACQ-001` | `BASTION-REQ-ACQ-001` | product | target | The acquisition/industrial-base record separately evaluates requirements stability, cycle time, competition, qualification, production and repair capacity, supplier concentration, technical-data and intellectual-property constraints, workforce and facilities, cash flow, demand stability, learning, transition, and surge at safe public abstraction without supplier-sensitive detail. | `VER-ACQ-001`: one completeness/security fixture for each named acquisition facet | `VAL-ACQ-001`: industrial-delivery scenario | Acquisition and industrial-base analyst | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-ACQ-002` | `BASTION-REQ-ACQ-002` | product | target | Shared-support value, unique-system need, concentration, transition, interoperability, and common-mode or unique-system failure are six distinct commonality results. | `VER-ACQ-002`: six-facet fixtures | `VAL-ACQ-002`: commonality scenario | Acquisition and industrial-base analyst | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-ACQ-003` | `BASTION-REQ-ACQ-003` | product | target | Each commonality facet carries its own evidence, method, quantitative units where used, uncertainty, and result/null/reviewed-N/A posture. | `VER-ACQ-003`: per-facet posture fixtures | `VAL-ACQ-003`: commonality evidence review | Acquisition and Industrial-Base Lead | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-ACQ-004` | `BASTION-REQ-ACQ-004` | economics | target | Candidate cost/risk records include requalification, tooling, workforce, facility, transition, support, schedule, supplier exit, concentration, and failure effects separately. | `VER-ACQ-004`: omitted-cost fixtures | `VAL-ACQ-004`: lifecycle comparison | Acquisition and industrial-base analyst | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-ACQ-005` | `BASTION-REQ-ACQ-005` | distribution | target | Prime, small-supplier, government, workforce, and facility dependencies and incidence are separate at an accepted safe aggregation. | `VER-ACQ-005`: incidence/aggregation fixtures | `VAL-ACQ-005`: supplier burden scenario | Acquisition and industrial-base analyst | high | `SPEC-UNK-ACQ-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-ACQ-006` | `BASTION-REQ-ACQ-006` | assurance | target | Vendor-only, gross-obligation-only, unsafe-detail, or incomplete-competition evidence produces a held performance, schedule, capacity, competition, or savings claim. | `VER-ACQ-006`: weak-evidence fixtures | `VAL-ACQ-006`: independent-evidence scenario | Independent Test and Oversight Officer | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-ACQ-007` | `BASTION-REQ-ACQ-007` | assurance | target | Supplier exit, brittle concentration, unpriced qualification, delayed capability, and shifted industrial burden are classified as costs or risks, never efficiency. | `VER-ACQ-007`: false-efficiency fixtures | `VAL-ACQ-007`: supplier-resilience scenario | Acquisition and Industrial-Base Lead | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-ACQ-008` | `BASTION-REQ-ACQ-008` | assurance | target | A commonality output preserves all six facets and rejects any composite replacement score. | `VER-ACQ-008`: composite-score rejection | `VAL-ACQ-008`: commonality review | Acquisition and Industrial-Base Lead | high | `SPEC-UNK-ACQ-001` | held |
| `SPEC-LOG-001` | `BASTION-REQ-LOG-001` | product | target | Inventory posture records stocks, condition states, custody, period, units, inclusions, exclusions, stock-policy basis, and reconciliation posture as one boundary. | `VER-LOG-001`: stock-boundary fixtures | `VAL-LOG-001`: inventory scenario | Logistics and sustainment analyst | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-002` | `BASTION-REQ-LOG-002` | product | target | Repair-time distribution records population, denominator, start/stop events, censoring, period, center, accepted tails, uncertainty, and degraded treatment. | `VER-LOG-002`: distribution/censoring fixtures | `VAL-LOG-002`: repair-tail scenario | Logistics and sustainment analyst | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-003` | `BASTION-REQ-LOG-003` | product | target | Inventory and repair distributions have independent evidence and independent result/null/reviewed-N/A postures. | `VER-LOG-003`: separate-posture fixtures | `VAL-LOG-003`: sustainment evidence review | Logistics and Sustainment Lead | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-004` | `BASTION-REQ-LOG-004` | product | target | Sustainment custody traces technical data, workforce, facilities, spares, distribution, energy, maintenance, repair, upgrades, disposal, and transition at safe abstraction. | `VER-LOG-004`: custody-stage fixtures | `VAL-LOG-004`: lifecycle handoff scenario | Logistics and sustainment analyst | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-005` | `BASTION-REQ-LOG-005` | economics | target | Sustainment candidate results preserve deferred maintenance, obsolescence, cannibalization, queues/tails, supplier/workforce constraints, surge, recovery, and lifecycle costs separately. | `VER-LOG-005`: degraded-cost fixtures | `VAL-LOG-005`: sustainment downside | Logistics and sustainment analyst | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-006` | `BASTION-REQ-LOG-006` | assurance | target | A security rejection forces the corresponding logistics map, stock detail, repair result, or combination to held state. | `VER-LOG-006`: security-rejection fixtures | `VAL-LOG-006`: safe-abstraction scenario | Classification and Operational Security reviewer | high | `SPEC-UNK-SEC-001`; `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-007` | `BASTION-REQ-LOG-007` | control | target | Missing or incompatible inventory or repair evidence blocks promotion. | `VER-LOG-007`: missing/incompatible fixtures | `VAL-LOG-007`: incomplete-sustainment scenario | Logistics and Sustainment Lead | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-LOG-008` | `BASTION-REQ-LOG-008` | assurance | target | Missing support evidence yields null/held repair cost and cannot yield zero cost. | `VER-LOG-008`: missing-to-zero rejection | `VAL-LOG-008`: null-repair scenario | Logistics and Sustainment Lead | high | `SPEC-UNK-LOG-001` | held |
| `SPEC-ALLY-001` | `BASTION-REQ-ALLY-001` | product | target | Interoperability records public commitments, standards, compatibility, common logistics, partner assumptions, sovereignty, controls, and transition without partner-operation or intent inference. | `VER-ALLY-001`: safe-boundary fixtures | `VAL-ALLY-001`: public-commitment scenario | Alliance and interoperability analyst | high | `SPEC-UNK-ALLY-001` | held |
| `SPEC-ALLY-002` | `BASTION-REQ-ALLY-002` | economics | target | U.S., partner, shared, and externalized costs, benefits, burdens, receipts, and risks occupy separate ledger partitions. | `VER-ALLY-002`: party-ledger fixtures | `VAL-ALLY-002`: burden-incidence scenario | Alliance and interoperability analyst | high | `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-ALLY-003` | `BASTION-REQ-ALLY-003` | product | target | Every interoperability candidate has paired normal and degraded institutional-support results at safe abstraction. | `VER-ALLY-003`: paired-scenario fixtures | `VAL-ALLY-003`: degraded-partner-support scenario | Alliance and Interoperability Strategist | high | `SPEC-UNK-ALLY-001` | held |
| `SPEC-ALLY-004` | `BASTION-REQ-ALLY-004` | control | target | Missing or unsafe authority, obligation, partner evidence, transition cost, sovereignty, or incidence yields a held joint or burden claim. | `VER-ALLY-004`: one hold fixture per cause | `VAL-ALLY-004`: incomplete-partner scenario | Alliance and Interoperability Strategist | high | `SPEC-UNK-ALLY-001` | held |
| `SPEC-ALLY-005` | `BASTION-REQ-ALLY-005` | distribution | target | Conflicting domestic and partner results remain distinct and cannot be netted. | `VER-ALLY-005`: conflict-preservation fixtures | `VAL-ALLY-005`: domestic/partner conflict | Alliance and interoperability analyst | high | `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-ALLY-006` | `BASTION-REQ-ALLY-006` | review | target | Every alliance/interoperability result carries uncertainty and dissent postures. | `VER-ALLY-006`: omitted-posture fixtures | `VAL-ALLY-006`: alliance review scenario | Alliance and Interoperability Strategist | high | `SPEC-UNK-ALLY-001` | held |
| `SPEC-DST-001` | `BASTION-REQ-DST-001` | distribution | target | Applicable effects are separate for service-member/family, mission-user, workforce, supplier, installation-community, taxpayer/oversight, and ally/partner lenses. | `VER-DST-001`: stakeholder coverage fixtures | `VAL-DST-001`: multi-stakeholder scenario | Personnel/family/workforce/community analyst | high | `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-DST-002` | `BASTION-REQ-DST-002` | distribution | target | Each distribution result records affected aggregate, denominator, period, horizon, baseline, uncertainty, burden category, central result, tail/concentration, and evidence posture. | `VER-DST-002`: measure completeness fixtures | `VAL-DST-002`: pain/tail scenario | Personnel/family/workforce/community analyst | high | `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-DST-003` | `BASTION-REQ-DST-003` | distribution | target | Safety, tempo, staffing, retention, skills, moves, housing, health, caregiving, local services, environment, employment transition, supplier cash flow, and burden shifts each receive a separate applicable outcome and recorded review by the Service-Member and Family Advocate, Prime and Small Supplier, Depot and Logistics Workforce, Installation Community, Taxpayer and Oversight, and both assurance roles. | `VER-DST-003`: one facet fixture for each named outcome and one concurrence fixture for each named review voice | `VAL-DST-003`: cross-stakeholder burden-shift scenario | Personnel/family/workforce/community analyst | high | `SPEC-UNK-ACQ-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-DST-004` | `BASTION-REQ-DST-004` | distribution | target | Conflicting stakeholder outcomes and valid nulls remain separate versioned result records. | `VER-DST-004`: conflict/null preservation | `VAL-DST-004`: conflicting-effects scenario | Personnel/family/workforce/community analyst | high | `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-DST-005` | `BASTION-REQ-DST-005` | assurance | target | Composite priority, readiness, or savings scores cannot replace stakeholder result records. | `VER-DST-005`: composite rejection fixtures | `VAL-DST-005`: distribution review | Personnel/family/workforce/community analyst | high | `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001` | held |
| `SPEC-ECO-001` | `BASTION-REQ-ECO-001` | economics | target | Direct cost reduction, process efficiency, avoided cost/risk, readiness/capacity/resilience gain, lawful receipt effect, and null are six distinct pathway states. | `VER-ECO-001`: pathway-state fixtures | `VAL-ECO-001`: adaptive-pathway scenario | Defense resource analyst | high | `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-002` | `BASTION-REQ-ECO-002` | economics | target | The accounting relation rejects automatic pathway addition and prohibited conversions to booked savings or receipts. | `VER-ECO-002`: non-additivity/conversion invariants | `VAL-ECO-002`: category-misuse scenario | Defense Comptroller | high | `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-003` | `BASTION-REQ-ECO-003` | economics | target | Each pathway has separate near-, medium-, and long-horizon result or null branches where applicable. | `VER-ECO-003`: three-horizon fixtures | `VAL-ECO-003`: horizon comparison | Defense resource analyst | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-004` | `BASTION-REQ-ECO-004` | economics | target | Gross opportunity, realizable savings, benefit, lawful receipt, lifecycle, financing, transition/implementation, risk, timing, uncertainty, overlap, and net pressure are separate ledger fields. | `VER-ECO-004`: ledger partition fixtures | `VAL-ECO-004`: gross-to-net scenario | Defense resource analyst | high | `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-005` | `BASTION-REQ-ECO-005` | economics | target | Budget authority, obligations, outlays, transfers, and offsetting receipts remain distinct within appropriation, account, period, and fiscal-owner boundaries. | `VER-ECO-005`: federal-measure fixtures | `VAL-ECO-005`: account reconciliation | Defense Comptroller | high | `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-006` | `BASTION-REQ-ECO-006` | economics | target | A realizable-savings state exists only with authority, owner, capture evidence, transition path, cost/schedule, baseline reconciliation, and passed safety/readiness/distribution/supplier/alliance/overlap gates. | `VER-ECO-006`: realization-gate fixtures | `VAL-ECO-006`: realizability scenario | Defense Comptroller | high | `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-007` | `BASTION-REQ-ECO-007` | economics | target | A peer record includes normalized peer functions and peer sets, source vintages, institutional and mission differences, input and output measures, units, price-year and purchasing-power treatment where applicable, uncertainty, and non-comparable portions. | `VER-ECO-007`: peer-record completeness and incompatible-peer fixtures | `VAL-ECO-007`: peer-comparison scenario | Defense resource analyst | high | `SPEC-UNK-QNT-001` | held |
| `SPEC-ECO-008` | `BASTION-REQ-ECO-008` | authority | target | Every peer value is labelled diagnostic scenario evidence. | `VER-ECO-008`: diagnostic-label fixtures | `VAL-ECO-008`: peer-use review | Scope Keeper | high | `SPEC-UNK-QNT-001` | held |
| `SPEC-ECO-009` | `BASTION-REQ-ECO-009` | economics | target | Each projection records horizon, range, uncertainty method, cited-baseline reconciliation, downside, and evidence-supported probability or reviewed N/A. | `VER-ECO-009`: projection completeness fixtures | `VAL-ECO-009`: downside projection | Defense resource analyst | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-010` | `BASTION-REQ-ECO-010` | economics | target | Applicable discount, present-value, inflation, exchange-rate, and purchasing-power conventions are explicit and versioned. | `VER-ECO-010`: convention fixtures | `VAL-ECO-010`: quantitative-method review | Numeracy Checker | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-011` | `BASTION-REQ-ECO-011` | economics | target | Every pathway, candidate, account, period, organization, supplier, partner, and other fiscal lane has a stable versioned overlap key. | `VER-ECO-011`: completeness, uniqueness, and stability fixtures for each named overlap surface | `VAL-ECO-011`: overlap scenario | Defense Comptroller | high | `SPEC-UNK-ECO-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-ECO-012` | `BASTION-REQ-ECO-012` | operations | target | An active pathway reopens for observation and review at its accepted cadence or upon a source revision, peer-comparability change, observed delivery variance, shock, burden shift, floor failure, authority change, overlap change, or ownership change. | `VER-ECO-012`: one reopen transition for cadence and each named change trigger | `VAL-ECO-012`: adaptive-reopen scenario | Delivery owner | high | `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-ECO-013` | `BASTION-REQ-ECO-013` | operations | target | Re-evaluation produces exactly one preserve, revise, hold, retire, or replace disposition. | `VER-ECO-013`: lifecycle transition fixtures | `VAL-ECO-013`: adaptive-pathway lifecycle and re-evaluation scenario | Delivery owner | high | `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-ECO-014` | `BASTION-REQ-ECO-014` | authority | target | Peer values and gaps cannot populate target-mandate, savings, funding-quota, allocation, or rate-instruction fields. | `VER-ECO-014`: peer-misuse fixtures | `VAL-ECO-014`: peer-authority scenario | Scope Keeper | high | `SPEC-UNK-QNT-001` | held |
| `SPEC-ECO-015` | `BASTION-REQ-ECO-015` | control | target | An unreconciled unit, horizon, price basis, or method places a cross-scenario total in held state. | `VER-ECO-015`: basis-mismatch fixtures | `VAL-ECO-015`: incompatible-total scenario | Numeracy Checker | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-016` | `BASTION-REQ-ECO-016` | economics | target | Duplicate effects sharing overlap keys cannot be counted more than once across pathways, candidates, accounts, periods, organizations, suppliers, partners, or other fiscal lanes. | `VER-ECO-016`: one duplicate-effect fixture for each named overlap surface | `VAL-ECO-016`: portfolio-overlap scenario | Defense Comptroller | high | `SPEC-UNK-ECO-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-ECO-017` | `BASTION-REQ-ECO-017` | operations | target | Each re-evaluation retains prior version, evidence, rationale, owner, and notification posture. | `VER-ECO-017`: history-retention fixtures | `VAL-ECO-017`: pathway-history scenario | Delivery owner | medium | `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-ECO-018` | `BASTION-REQ-ECO-018` | economics | target | A genuinely inapplicable horizon carries reasoned independently reviewed N/A plus an explicit alternative time boundary. | `VER-ECO-018`: N/A/alternative-boundary fixtures | `VAL-ECO-018`: horizon-N/A scenario | Defense resource analyst | high | `SPEC-UNK-QNT-001` | held |
| `SPEC-ECO-019` | `BASTION-REQ-ECO-019` | control | target | Missing or unaccepted result, null, reviewed N/A, or alternative time boundary blocks pathway promotion. | `VER-ECO-019`: horizon-hold fixtures | `VAL-ECO-019`: incomplete-horizon scenario | Defense resource analyst | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-ECO-001` | held |
| `SPEC-ECO-020` | `BASTION-REQ-ECO-020` | economics | target | `PATHWAY-ENVELOPE-001` is all-or-hold: every accepted field is present and compatible or the pathway remains held. | `VER-ECO-020`: branch-complete envelope fixtures | `VAL-ECO-020`: pathway-envelope scenario | Defense resource analyst | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-TST-001` | `BASTION-REQ-TST-001` | review | target | Review begins only from one frozen digest-bound packet containing artifact, evidence, derivations, gates, negative cases, and unresolved questions. | `VER-TST-001`: packet/digest fixtures | `VAL-TST-001`: frozen-review scenario | Role review steward | high | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` | held |
| `SPEC-TST-002` | `BASTION-REQ-TST-002` | review | target | Independent review includes quantitative reproduction, qualitative custody inspection, adverse/failure testing, uncertainty, dimensional, lifecycle/transition, and double-count checks. | `VER-TST-002`: seeded-error review fixtures | `VAL-TST-002`: adversarial-review scenario | Independent Test and Oversight Officer | high | `SPEC-UNK-TST-001` | held |
| `SPEC-TST-003` | `BASTION-REQ-TST-003` | review | target | Negative results, failed tests, nulls, rejected candidates, dissent, and unresolved evidence remain in the evidence ledger. | `VER-TST-003`: retention fixtures | `VAL-TST-003`: negative-evidence scenario | Independent Test and Oversight Officer | high | `SPEC-UNK-TST-001` | held |
| `SPEC-TST-004` | `BASTION-REQ-TST-004` | review | target | Every finding record contains stable identity, digest, role, severity, claim, evidence, disposition, owner, destination, closure, independence, and dissent. | `VER-TST-004`: finding completeness fixtures | `VAL-TST-004`: finding-remediation scenario | Role review steward | high | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` | held |
| `SPEC-TST-005` | `BASTION-REQ-TST-005` | control | target | Stale/conflicted review, absent role, failed assurance, unowned defer, false approval, or unresolved critical/major finding blocks fixed point. | `VER-TST-005`: convergence-block fixtures | `VAL-TST-005`: fixed-point scenario | BASTION maintainer | high | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` | held |
| `SPEC-TST-006` | `BASTION-REQ-TST-006` | assurance | target | Advocacy, credentials, and inaccessible classified appeals cannot replace retained evidence. | `VER-TST-006`: substitution-rejection fixtures | `VAL-TST-006`: evidence-conflict scenario | Independent Test and Oversight Officer | high | `SPEC-UNK-TST-001` | held |
| `SPEC-DEL-001` | `BASTION-REQ-DEL-001` | operations | target | Delivery-testable state requires complete authority, owner, dependencies, resources, milestones, measures, floors, cadence, stop, evaluation, rollback, and realization-evidence fields. | `VER-DEL-001`: delivery-record fixtures | `VAL-DEL-001`: delivery-readiness scenario | Delivery owner | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-DEL-002` | `BASTION-REQ-DEL-002` | control | target | Missing authority, ownership, resources, transition capacity, measurement, stop, or rollback retains research-hypothesis state. | `VER-DEL-002`: one hold fixture per cause | `VAL-DEL-002`: held-hypothesis scenario | Delivery owner | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-DEL-003` | `BASTION-REQ-DEL-003` | operations | target | Each observation compares delivery with the exact cited baseline and accepted peer posture. | `VER-DEL-003`: comparison fixtures | `VAL-DEL-003`: observed-delivery scenario | Delivery owner | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-DEL-004` | `BASTION-REQ-DEL-004` | operations | target | An accepted trigger causes exactly one stop, hold, revise, retire, or replace action. | `VER-DEL-004`: trigger-action fixtures | `VAL-DEL-004`: corrective-action scenario | Delivery owner | high | `SPEC-UNK-DEL-001` | held |
| `SPEC-DEL-005` | `BASTION-REQ-DEL-005` | authority | current | Delivery-readiness and implementation-ready states are unavailable until later specifications, design, interfaces, verification plans, and an accepted work package exist. | `VER-DEL-005`: premature-readiness rejection | `VAL-DEL-005`: stage-boundary scenario | BASTION maintainer | high | `SPEC-UNK-DEL-001` | held |
| `SPEC-DEL-006` | `BASTION-REQ-DEL-006` | operations | target | Every observation evaluates schedule, cost, burden, overlap, safety, readiness, supplier, workforce, community, and alliance deviations separately. | `VER-DEL-006`: seeded-deviation fixtures | `VAL-DEL-006`: delivery-deviation scenario | Delivery owner | high | `SPEC-UNK-RDY-001`; `SPEC-UNK-ACQ-001`; `SPEC-UNK-LOG-001`; `SPEC-UNK-ALLY-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-DEL-001` | held |
| `SPEC-DEL-007` | `BASTION-REQ-DEL-007` | operations | target | Every trigger-driven action retains reason, evidence, version, owner, rollback posture, and downstream notification. | `VER-DEL-007`: action-record fixtures | `VAL-DEL-007`: rollback/notification scenario | Delivery owner | high | `SPEC-UNK-DEL-001` | held |
| `SPEC-HND-001` | `BASTION-REQ-HND-001` | interface | target | The logical handoff state is only a held LaneEvidencePack candidate containing the required identities, digests, gates, security, ledgers, nulls, uncertainty, downside, distribution, delivery, overlap, provenance, risk, and dissent fields. | `VER-HND-001`: held-pack completeness fixtures | `VAL-HND-001`: held-handoff scenario | Taxlane adapter steward | high | `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-HND-002` | `BASTION-REQ-HND-002` | interface | target | Adapter mapping preserves six pathways, federal measures, peer limits, owner, cadence, transition costs, uncertainty, overlap, floors, and nulls without conversion. | `VER-HND-002`: round-trip category fixtures | `VAL-HND-002`: semantic-preservation scenario | Taxlane adapter steward | high | `SPEC-UNK-QNT-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-HND-003` | `BASTION-REQ-HND-003` | interface | target | Stale, incomplete, unsafe, unreconciled, unowned, double-counted, falsely precise, or floor-failing handoffs produce rejection state. | `VER-HND-003`: rejection fixture matrix | `VAL-HND-003`: failed-handoff scenario | Taxlane adapter steward | high | `SPEC-UNK-SEC-001`; `SPEC-UNK-RDY-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-HND-004` | `BASTION-REQ-HND-004` | interface | current | Taxlane admission state is external and cannot be set by BASTION. | `VER-HND-004`: ownership-state and missing-external-control fixtures | `VAL-HND-004`: external-admission scenario | Taxlane adapter steward | high | `SPEC-UNK-ECO-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-HND-005` | `BASTION-REQ-HND-005` | interface | target | Each handoff requires fresh scope, custody, composition-security, authority, floor, numeracy, independence, compatibility, and digest gate postures. | `VER-HND-005`: pre-handoff gate fixtures | `VAL-HND-005`: context-change handoff | Taxlane adapter steward | high | `SPEC-UNK-SEC-001`; `SPEC-UNK-RDY-001`; `SPEC-UNK-QNT-001`; `SPEC-UNK-DST-001`; `SPEC-UNK-ECO-001`; `SPEC-UNK-DEL-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-HND-006` | `BASTION-REQ-HND-006` | interface | target | A missing required interface value remains null/held/rejected and cannot be fabricated. | `VER-HND-006`: missing-value fixtures | `VAL-HND-006`: incomplete-pack scenario | Taxlane adapter steward | high | `SPEC-UNK-ECO-001`; `SPEC-UNK-HND-001` | held |
| `SPEC-HND-007` | `BASTION-REQ-HND-007` | authority | current | Handoff creation cannot set or imply admission, cross-lane combination, allocation, rebalance, rate, official-use, or publication state. | `VER-HND-007`: false-inference fixtures | `VAL-HND-007`: Taxlane-boundary scenario | Taxlane adapter steward | high | `SPEC-UNK-HND-001` | held |
| `SPEC-REL-001` | `BASTION-REQ-REL-001` | authority | current | Public-release and approved states are unavailable without separate release authority and a release-specific fixed point. | `VER-REL-001`: unauthorized-release fixture | `VAL-REL-001`: no-release scenario | Scope Keeper | high | `SPEC-UNK-REL-001` | held |
| `SPEC-REL-002` | `BASTION-REQ-REL-002` | assurance | unknown | A future separately authorized release contract remains unknown until direct/cross-release composition, linkage, context, misuse, staleness, provenance, correction/takedown, security, and scope controls are accepted. | `VER-REL-002`: planned release-threat fixtures | `VAL-REL-002`: future release review | Classification and Operational Security reviewer | high | `SPEC-UNK-SEC-001`; `SPEC-UNK-REL-001` | held |
| `SPEC-REL-003` | `BASTION-REQ-REL-003` | communications | unknown | Any future authorized communication must preserve source, derivation, limitations, uncertainty, dissent, security posture, and non-authority context. | `VER-REL-003`: planned context-retention fixtures | `VAL-REL-003`: future communication review | Citation Auditor | high | `SPEC-UNK-REL-001` | held |
| `SPEC-VTR-001` | `BASTION-REQ-VTR-001` | control | target | Trace records connect each requirement to scenario/gate/defer/question parents and later specification, design, interface, verification, validation, and evidence identities. | `VER-VTR-001`: orphan-link fixtures | `VAL-VTR-001`: end-to-end trace scenario | BASTION maintainer | high | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` | held |
| `SPEC-VTR-002` | `BASTION-REQ-VTR-002` | control | current | Requirements advancement requires reviewed unique IDs, grammar, ownership, verification, full parent coverage, and zero unresolved critical/major findings. | `VER-VTR-002`: requirements-gate inspection | `VAL-VTR-002`: stage-advance scenario | Role review steward | high | `SPEC-UNK-SRC-001`; `SPEC-UNK-TST-001` | held |
| `SPEC-VTR-003` | `BASTION-REQ-VTR-003` | authority | current | Rust or other implementation state is unavailable until applicable requirements, specifications, architecture, interfaces, design, verification plan, and work package gates pass. | `VER-VTR-003`: premature-implementation fixture | `VAL-VTR-003`: implementation-boundary scenario | BASTION maintainer | high | none | proposed |

## Logical versioned contracts

These are domain responsibility contracts, not implementation APIs or schemas.

| Contract ID | Spec IDs | Logical surface and owner | Compatibility rule | Missing/stale/unsafe behavior | Change-control trigger | Planned evidence |
|---|---|---|---|---|---|---|
| `CONTRACT-SOURCE-001` | `SPEC-SCP-001..005`, `SCP-009..010`, `SRC-001..008` | Source/security/claim posture — Public-evidence steward accountable; Security and aggregation steward and Classification and Operational Security reviewer concur on applicable security postures | Version, digest, claim state, and security posture must remain interpretable without weakening prior rejection. | Hold or reject; never default, retain prohibited content, or reconstruct prohibited content. | Source field, join, granularity, derivation, visualization, audience, release context, expiry, claim-state, or security-posture changes. | `VER-SCP-*`, `VER-SRC-*`; `VAL-SRC-*`, `VAL-PRIV-*`. |
| `CONTRACT-AUTH-001` | `SPEC-SCP-006..008` | Mission/civilian authority — Civilian mission and authority steward | A successor may narrow but cannot silently broaden authority or delegate a non-delegable decision. | Hold dependent analysis. | Mission, authority, jurisdiction, owner, period, or analytic boundary changes. | `VER-SCP-006..008`; `VAL-AUTH-*`. |
| `CONTRACT-RDY-001` | `SPEC-RDY-001..007` | Readiness/safety promises — Readiness-system analyst | Facet identity, denominator, horizon, tails, degraded treatment, and floor semantics remain explicit across versions. | Hold readiness/candidate/fiscal claims. | Promise, evidence rule, boundary, denominator, horizon, floor, or degraded case changes. | `VER-RDY-*`; `VAL-RDY-*`. |
| `CONTRACT-ACQ-001` | `SPEC-ACQ-001..008` | Acquisition/industrial base/commonality — Acquisition and industrial-base analyst | Six commonality facets and supplier incidence remain separate; no composite substitution. | Hold acquisition, schedule, capacity, or savings claims. | Facet definition, aggregation, supplier boundary, evidence tier, or lifecycle boundary changes. | `VER-ACQ-*`; `VAL-ACQ-*`. |
| `CONTRACT-LOG-001` | `SPEC-LOG-001..008` | Logistics/inventory/repair — Logistics and sustainment analyst | Stock boundary and repair distribution retain units, custody, time, tails, censoring, and degraded semantics. | Hold or reject; missing support never becomes zero cost. | Stock policy, inventory boundary, repair timing, censoring, lifecycle, aggregation, or security posture changes. | `VER-LOG-*`; `VAL-LOG-*`. |
| `CONTRACT-ALLY-001` | `SPEC-ALLY-001..006` | Alliance/interoperability/sovereignty — Alliance and interoperability analyst | Domestic, partner, shared, and externalized results remain separated and sovereignty/control boundaries remain visible. | Hold joint or burden claims. | Commitment, standard, compatibility, control, partner assumption, or incidence changes. | `VER-ALLY-*`; `VAL-ALLY-*`. |
| `CONTRACT-DST-001` | `SPEC-DST-001..005` | Cross-stakeholder distribution — Personnel/family/workforce/community analyst | Stakeholder, denominator, time, tail, burden, null, and dissent semantics remain separate. | Hold efficiency, savings, readiness, and handoff claims. | Stakeholder boundary, denominator, tail measure, burden category, or evidence posture changes. | `VER-DST-*`; `VAL-DST-*`. |
| `CONTRACT-ECO-001` | `SPEC-ECO-001..020` | Quantitative/peer/accounting/adaptive envelope — Defense resource analyst | Six pathways, federal measures, horizons, units, price bases, overlap, and lifecycle states remain non-additive and versioned. | Hold monetization, totals, realizable savings, receipts, and handoff. | Method, peer set, horizon, price basis, account, owner, overlap, cadence, floor, or pathway state changes. | `VER-ECO-*`; `VAL-ECO-*`. |
| `CONTRACT-TEST-001` | `SPEC-TST-001..006`, `VTR-001..002` | Independent test/convergence — Role review steward | Review digest, evidence state, finding severity/ownership, required roles, and dissent remain reproducible. | Block fixed point and downstream stage. | Artifact digest, evidence, method, reviewer independence, role set, finding, or defer changes. | `VER-TST-*`, `VER-VTR-001..002`; review scenarios. |
| `CONTRACT-DEL-001` | `SPEC-DEL-001..007` | Delivery/feedback/rollback — Delivery owner | Owner, authority, floors, cadence, triggers, actions, rollback, and notifications remain linked across versions. | Retain research-hypothesis state. | Authority, owner, dependency, resource, measure, floor, cadence, trigger, rollback, or observation changes. | `VER-DEL-*`; `VAL-DEL-*`. |
| `CONTRACT-HND-001` | `SPEC-HND-001..007` | Held LaneEvidencePack — Taxlane adapter steward | Mapping preserves identities, pathways, fiscal semantics, nulls, gates, risk, provenance, and external admission ownership. | Reject or retain held state; never fabricate or infer admission. | Shared contract, field mapping, digest, gate, overlap, security, owner, or Taxlane ownership changes. | `VER-HND-*`; `VAL-HND-*`. |
| `CONTRACT-REL-001` | `SPEC-REL-001..003` | Separately authorized release posture — Scope Keeper | No release compatibility is claimed until a separate contract is accepted; context and security cannot be weakened. | No public release. | Any proposed audience, artifact, composition, context, correction, or authority change. | `VER-REL-*`; `VAL-REL-*` only after separate authority. |
| `CONTRACT-TRACE-001` | `SPEC-VTR-001..003` | Stage/trace authority — BASTION maintainer | Parent/child identities and gate decisions remain visible; later artifacts cannot retroactively create authority. | Block stage advancement or implementation. | Requirement, spec, verification, validation, evidence, gate, or implementation-status change. | `VER-VTR-*`; `VAL-VTR-*`. |

## Logical responsibility allocation

| Spec families | Logical responsibility | Owner | Forbidden responsibility | Architecture/package posture |
|---|---|---|---|---|
| `SPEC-SCP-001..005`, `SPEC-SCP-009..010`, `SPEC-SRC-*` | Public-evidence admission, identity, provenance, claims, and security custody | Public-evidence steward accountable; Security and aggregation steward and Classification and Operational Security reviewer concur where security applies | Operational intelligence, identity reconstruction, release approval | deferred |
| `SPEC-SCP-006`, `SPEC-SCP-008` | Research/review and downstream-authority semantics | Scope Keeper | External approval, official action, implementation, fiscal use, or release authority | deferred |
| `SPEC-SCP-007` | Civilian decision and prohibited-output authority semantics | Civilian Control, Law, Safety & Readiness reviewer accountable; Scope Keeper concurrence required | Mission, force, procurement, budget, allocation, rate, operational, or official decision | deferred |
| `SPEC-RDY-*` | Public aggregate readiness/safety promises | Readiness-system analyst | Force employment or unit-level posture | deferred |
| `SPEC-ACQ-*`, `SPEC-LOG-*` | Industrial delivery and lifecycle support | Acquisition and logistics analysts | Procurement selection or sensitive supplier/logistics detail | deferred |
| `SPEC-ALLY-*` | Public alliance/interoperability evidence | Alliance analyst | Partner intent or operational inference | deferred |
| `SPEC-DST-*` | Stakeholder distribution and tails | Cross-stakeholder analyst | Composite priority or human-worth ranking | deferred |
| `SPEC-ECO-*` | Quantitative, peer, fiscal, adaptive analysis | Defense resource analyst | Budget, allocation, rate, or Taxlane admission | deferred |
| `SPEC-TST-*`, `SPEC-VTR-*` | Independent review, trace, convergence | Role review steward | Author self-approval or external-approval claim | deferred |
| `SPEC-DEL-*` | Delivery evidence and feedback control | Delivery owner | Implementation readiness without later gates | deferred |
| `SPEC-HND-*` | Held handoff semantics | Taxlane adapter steward | Taxlane admission or fiscal use | deferred |
| `SPEC-REL-*` | No-release and future release posture | Scope Keeper | Public release without separate authority | deferred |

No crate, package, module, language, persistence, API, or runtime allocation is
made here. Those remain architecture, interface, package-boundary, and design
questions after this baseline reaches fixed point.

## Nonfunctional constraints

| Constraint ID | Parent specs | Constraint | Controlled rule | Planned verification | Status |
|---|---|---|---|---|---|
| `SPEC-NF-001` | `SPEC-SCP-002`, `SPEC-SCP-009` | Absolute prohibited-data boundary | No classified information, CUI, person-level service data, sensitive operational data, targeting content, operational-planning content, or exploitable vulnerability content is ingested, retained, derived, or emitted; the unknown compositional safety method remains separately held by `SPEC-UNK-SEC-001`. | `VER-NF-001`: one direct prohibited-data fixture for each ingest, retain, derive, and emit boundary | held by `SPEC-UNK-SEC-001` |
| `SPEC-NF-002` | `SPEC-SCP-006..008`, `RDY-*`, `DEL-*` | Civilian control | No state transition broadens lawful authority or silently changes mission/risk. | `VER-NF-002`: authority-transition fixtures | proposed |
| `SPEC-NF-003` | `SPEC-RDY-*`, `ECO-006`, `DEL-*` | Readiness/safety floors | A missing or failed floor blocks candidate, savings, delivery, and handoff states. | `VER-NF-003`: cross-contract floor invariants | held |
| `SPEC-NF-004` | `SPEC-RDY-*`, `LOG-*`, `DST-*` | Tail visibility | Central values cannot replace required distribution, repair-tail, degraded, or concentrated-effect fields. | `VER-NF-004`: average-only rejection fixtures | held |
| `SPEC-NF-005` | `SPEC-ECO-*`, `ALLY-002`, `HND-*` | Dimensional/accounting integrity | Units, horizons, price bases, account measures, parties, and overlap must reconcile before combination. | `VER-NF-005`: dimensional and ledger invariants | held |
| `SPEC-NF-006` | `SPEC-ECO-001..020`, `HND-002` | Non-additivity | Pathways and non-cash outcomes cannot be automatically summed or converted. | `VER-NF-006`: non-additivity property fixtures | held |
| `SPEC-NF-007` | all target specs | Null/N/A integrity | Missing is never zero; N/A requires reason, alternative boundary where required, and independent review. | `VER-NF-007`: null/N/A/default fixtures | proposed |
| `SPEC-NF-008` | `SPEC-SCP-004`, `SRC-*`, `TST-*`, `VTR-*` | Deterministic provenance/versioning | Equal accepted inputs and versions yield equal logical artifact identity and ordering; supersession never rewrites history. | `VER-NF-008`: regeneration/version fixtures | held |
| `SPEC-NF-009` | `SPEC-SCP-004`, `SRC-006`, `TST-001`, `HND-005` | Stale-digest rejection | Review, admission, and handoff fail when bound digest or context differs. | `VER-NF-009`: stale-digest/context fixtures | held |
| `SPEC-NF-010` | all specs | No authority inflation | A successful verification or validation result cannot create operational, procurement, budget, Taxlane, allocation, rate, official, implementation, or release authority. | `VER-NF-010`: false-authority fixtures | proposed |

## Controlled assumptions and unknowns

All unknown records are open. A dependent specification inherits the named
hold until the REQUIREMENTS TBD closure condition is independently accepted.

| Unknown ID | Parent TBD | State | Unknown control | Directly dependent spec families | Owner | Destination and closure source | Inherited hold |
|---|---|---|---|---|---|---|---|
| `SPEC-UNK-SEC-001` | `TBD-SEC-001` | unknown | Safe aggregation, composition/inference, suppression, expiry, re-review | As listed in each spec row; principally `SCP-001..003`, `SCP-009`, `SRC-005..006`, `LOG-006`, `HND-003/005`, `REL-002`, `SPEC-NF-001` | Security and aggregation steward | Security specification/verification; exact REQUIREMENTS TBD closure | Hold affected admission, retention, derivation, emission, visualization, handoff, release. |
| `SPEC-UNK-RDY-001` | `TBD-RDY-001` | unknown | Readiness/safety/resilience/surge/recovery measures and floors | `SCP-006`, `RDY-*`, `DEL-001..003/006`, `HND-003/005` | Readiness-system analyst | Promise specification/verification; exact REQUIREMENTS TBD closure | Hold readiness, candidate, savings, handoff. |
| `SPEC-UNK-SRC-001` | `TBD-SRC-001` | unknown | Source, identity, claim, derivation, null/rejection, digest schemas | `SCP-004`, `SRC-001..004/007..008`, `TST-001/004/005`, `VTR-001..002` | Public-evidence steward | Corpus/interface specification; exact REQUIREMENTS TBD closure | Hold unrepresentable custody/version/review behavior. |
| `SPEC-UNK-QNT-001` | `TBD-QNT-001` | unknown | Horizons, uncertainty/probability, price/PV, peer normalization, reconciliation | `RDY-004`, `ECO-003/007..010/014..015/018..020`, `HND-002/005` | Defense resource analyst | Quantitative-method specification; exact REQUIREMENTS TBD closure | Hold projections, peers, horizons, totals, handoffs. |
| `SPEC-UNK-ACQ-001` | `TBD-ACQ-001` | unknown | Acquisition/capacity/commonality measures and interpretation limits | `ACQ-*`, `DST-003`, `DEL-006` | Acquisition and industrial-base analyst | Industrial-base specification; exact REQUIREMENTS TBD closure | Hold acquisition/commonality/capacity/schedule/savings. |
| `SPEC-UNK-LOG-001` | `TBD-LOG-001` | unknown | Stock policy, inventory, repair distributions/tails, availability, degraded recovery | `LOG-*`, `RDY-002/004..006`, `DEL-006` | Logistics and sustainment analyst | Sustainment specification; exact REQUIREMENTS TBD closure | Hold sustainment/readiness/lifecycle/savings. |
| `SPEC-UNK-ALLY-001` | `TBD-ALLY-001` | unknown | Commitments, compatibility, sovereignty/control, capacity, partner incidence | `ALLY-*`, `DST-001/002/004/005`, `DEL-006` | Alliance and interoperability analyst | Interoperability specification; exact REQUIREMENTS TBD closure | Hold joint/interoperability/burden/fiscal claims. |
| `SPEC-UNK-DST-001` | `TBD-DST-001` | unknown | Stakeholder burden/distribution/concentrated/tail measures | `DST-*`, `ACQ-005`, `ALLY-002/005`, `ECO-003/006/020`, `DEL-006`, `HND-001..003/005` | Cross-stakeholder analyst | Distribution specification; exact REQUIREMENTS TBD closure | Hold efficiency/savings/readiness/distribution/handoff. |
| `SPEC-UNK-ECO-001` | `TBD-ECO-001` | unknown | Multi-path accounting, federal measures, overlap, realization, net-pressure schema | `ECO-001..006/009..013/015..017/019..020`, `HND-001..006` | Defense Comptroller | Economics/shared-accounting specification; exact REQUIREMENTS TBD closure | Hold monetization/savings/receipt/net-pressure/handoff. |
| `SPEC-UNK-TST-001` | `TBD-TST-001` | unknown | Evidence tiers, reproduction, conflicts, findings, convergence | `TST-*`, `VTR-001..002` | Independent Test and Oversight Officer | Verification plan; exact REQUIREMENTS TBD closure | Hold fixed point and downstream stage. |
| `SPEC-UNK-DEL-001` | `TBD-DEL-001` | unknown | Delivery evidence, cadence, triggers, milestones, stop/rollback, realization | `ECO-012/013/017/020`, `DEL-*`, `HND-001..003/005` | Delivery owner | Delivery specification/future accepted WP; exact REQUIREMENTS TBD closure | Retain research hypothesis; block savings/implementation/handoff. |
| `SPEC-UNK-HND-001` | `TBD-HND-001` | unknown | LaneEvidencePack fields, ownership, compatibility, rejection, held/admitted state | `ECO-011/016`, `HND-*` | Taxlane adapter steward | Shared interface review; exact REQUIREMENTS TBD closure | Hold adapter package; infer no Taxlane admission. |
| `SPEC-UNK-REL-001` | `TBD-REL-001` | unknown | Release composition, audience, misuse, correction/takedown, provenance | `SRC-005/006`, `REL-*` | Scope Keeper | Separately authorized release requirements; exact REQUIREMENTS TBD closure | No public release. |

## Requirement-to-spec coverage

The family mapping is one-to-one by suffix; no range implies coverage by a
single aggregate spec.

| Requirement IDs | Spec IDs | Count | Coverage status | Notes |
|---|---|---:|---|---|
| `BASTION-REQ-SCP-001..010` | `SPEC-SCP-001..010` | 10 | covered | One-to-one. |
| `BASTION-REQ-SRC-001..008` | `SPEC-SRC-001..008` | 8 | covered | One-to-one. |
| `BASTION-REQ-RDY-001..007` | `SPEC-RDY-001..007` | 7 | covered | One-to-one. |
| `BASTION-REQ-ACQ-001..008` | `SPEC-ACQ-001..008` | 8 | covered | One-to-one. |
| `BASTION-REQ-LOG-001..008` | `SPEC-LOG-001..008` | 8 | covered | One-to-one. |
| `BASTION-REQ-ALLY-001..006` | `SPEC-ALLY-001..006` | 6 | covered | One-to-one. |
| `BASTION-REQ-DST-001..005` | `SPEC-DST-001..005` | 5 | covered | One-to-one. |
| `BASTION-REQ-ECO-001..020` | `SPEC-ECO-001..020` | 20 | covered | One-to-one; held unknowns remain explicit. |
| `BASTION-REQ-TST-001..006` | `SPEC-TST-001..006` | 6 | covered | One-to-one. |
| `BASTION-REQ-DEL-001..007` | `SPEC-DEL-001..007` | 7 | covered | One-to-one. |
| `BASTION-REQ-HND-001..007` | `SPEC-HND-001..007` | 7 | covered | One-to-one. |
| `BASTION-REQ-REL-001..003` | `SPEC-REL-001..003` | 3 | covered | One-to-one; release remains unauthorized. |
| `BASTION-REQ-VTR-001..003` | `SPEC-VTR-001..003` | 3 | covered | One-to-one. |
| **Total** | **All controlled specs** | **98** | **covered with inherited holds** | No requirement is represented as implemented. |

## Spec-to-verification coverage

The `VER` family and suffix match the `SPEC` family and suffix one-to-one.
Evidence pointers are intentionally absent because no verification has run.

| Spec IDs | Planned VER IDs | Count | Expected result | Evidence pointer | Status |
|---|---|---:|---|---|---|
| `SPEC-SCP-001..010` | `VER-SCP-001..010` | 10 | Each authority/security/control branch accepts only its specified valid state and rejects or holds every named invalid state. | none; planned | planned |
| `SPEC-SRC-001..008` | `VER-SRC-001..008` | 8 | Source, claim, derivation, version, provenance, and security-change records satisfy their field and transition rules. | none; planned | planned |
| `SPEC-RDY-001..007` | `VER-RDY-001..007` | 7 | Readiness facets, reconciliations, degraded cases, floors, proxy rejection, and series separation behave as specified. | none; planned | planned |
| `SPEC-ACQ-001..008` | `VER-ACQ-001..008` | 8 | Acquisition/commonality facets and supplier/cost evidence remain separate, safe, and fail closed. | none; planned | planned |
| `SPEC-LOG-001..008` | `VER-LOG-001..008` | 8 | Inventory boundaries, repair tails, sustainment custody, security holds, and null-cost rules behave as specified. | none; planned | planned |
| `SPEC-ALLY-001..006` | `VER-ALLY-001..006` | 6 | Alliance/partner boundaries, ledgers, degraded cases, holds, conflict, uncertainty, and dissent remain separate. | none; planned | planned |
| `SPEC-DST-001..005` | `VER-DST-001..005` | 5 | Stakeholder coverage, distributions/tails, burden facets, conflicts/nulls, and composite rejection behave as specified. | none; planned | planned |
| `SPEC-ECO-001..020` | `VER-ECO-001..020` | 20 | Pathways, horizons, peer/accounting bases, realization, overlap, lifecycle, N/A, holds, and envelope invariants behave as specified. | none; planned | planned |
| `SPEC-TST-001..006` | `VER-TST-001..006` | 6 | Frozen review, reproduction, evidence retention, findings, convergence, and substitution rejection behave as specified. | none; planned | planned |
| `SPEC-DEL-001..007` | `VER-DEL-001..007` | 7 | Delivery readiness, holds, observation, triggers/actions, deviations, rollback, and notifications behave as specified. | none; planned | planned |
| `SPEC-HND-001..007` | `VER-HND-001..007` | 7 | Held-pack completeness, semantic preservation, rejection, external ownership, revalidation, nulls, and no inference behave as specified. | none; planned | planned |
| `SPEC-REL-001..003` | `VER-REL-001..003` | 3 | No release occurs; any future-authorized posture remains context- and security-bounded. | none; planned | planned |
| `SPEC-VTR-001..003` | `VER-VTR-001..003` | 3 | Trace, stage gates, and implementation boundary behave as specified. | none; planned | planned |
| **Total** | **All planned VER items** | **98** | Every controlled spec has an expected result. | none; planned | planned |

## Assumptions and explicit non-goals

- There is no accepted BASTION corpus, runtime, API, schema, CLI, package,
  crate, database, deployment, or public release to describe as current.
- Public aggregate availability does not imply security suitability.
- A target logical contract does not prove feasibility or implementation.
- Peer evidence is diagnostic and cannot create a target, savings amount,
  allocation, or rate instruction.
- Taxlane alone owns admission, cross-lane combination, allocation, rebalance,
  and rate testing.
- Architecture, package/language allocation, interfaces, algorithms,
  persistence, performance thresholds, and operational methods are non-goals.

## Change control

Independent review and a recorded change are required when any of the following
changes: parent requirement meaning; spec statement or state; unknown/TBD hold;
logical contract compatibility or owner; source/security context; mission or
civilian authority; readiness/safety floor; stakeholder or alliance boundary;
unit, horizon, price basis, fiscal measure, pathway, peer, overlap, or cadence;
verification or validation method/expected result; held-handoff semantics;
release posture; or accepted risk.

A change cannot close a `SPEC-UNK-*` record unless the parent `TBD-*` closure
condition passes independent review. Any incompatible change supersedes rather
than rewrites the prior version and reopens every affected review and handoff.

## Fixed-point review record

| Review stage | Decision | Recorded result |
|---|---|---|
| Initial author baseline | review-ready | The mixed baseline mapped 98 requirements to 98 controlled specs and 98 planned verification identities, with 13 unknowns open. |
| Independent substance review | finding | Two major findings plus minor findings required prohibited-boundary and handoff hold repair, exact enumeration anchors, settled terminology, validation naming, and accountable ownership/concurrence. |
| Independent assurance review | finding | Minor-only findings required complete security and economics hold inheritance and removal of a false proposed nonfunctional posture. |
| Bounded remediation | remediated | All findings were repaired without changing parent requirements, closing a TBD, selecting an interface or design, or representing planned evidence as executed. |
| Independent cross-convergence review | pass_with_risk | Requirement/spec/verification coverage, exact direct hold graph, contract ownership and concurrence, enumerations, terminology, no-authority boundaries, and remediation closures passed. |

Current specification findings: **zero critical; zero major; zero minor**.

## No-authority boundary and specification gate

This baseline confers no architecture, interface, design, package, work-package,
corpus, implementation, operational, force, procurement, budget, Taxlane-
admission, allocation, rebalance, rate-setting, official-use, or public-release
authority. It contains no classified information, CUI, person-level service
data, sensitive operational data, targeting, operational-planning, or
vulnerability-exploitation content.

Current gate decision: **pass_with_risk; specification fixed point reached**.

All 13 `SPEC-UNK-*` controls remain open and promotion-gating. Verification and
validation evidence remains planned, not executed. This fixed point selects no
interface encoding, architecture, design, work package, or implementation and
confers no classified/CUI/operational, Taxlane, budget, allocation, rate,
official-use, or release authority.

The next eligible VTRACE artifact is `ARCHITECTURE.md`, only under a new
assignment. No later-stage work is authorized by this decision.
