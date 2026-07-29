# BASTION Concept of Operations

## Status and scope

This document reached a role-panel fixed point with a **pass_with_risk**
decision after independent review, bounded remediation, and editorial/method
recheck. It translates the fixed-point `MISSION.md` into observable operating
scenarios for a future Defense 2.0 research and tooling system using public,
aggregate, unclassified evidence. It defines custody, review, rejection, and
handoff behavior before requirements or implementation are accepted.

This CONOPS authorizes no targeting, force employment, operational planning,
vulnerability exploitation, acquisition decision, budget request, official
score, policy recommendation, allocation, rate change, Taxlane admission, or
public release. It admits no classified information, controlled unclassified
information (CUI), person-level service data, sensitive operational data, or
dangerous combinations of otherwise public fields. No Rust implementation is
authorized.

## Operating principles

1. Treat defense as a mission-readiness and delivery network, not as an account
   total, platform count, or procurement list.
2. Begin with declared public mission abstractions and lawful civilian
   authority. Research may test whether a support system can meet an authorized
   public mission; it may not select missions or prescribe force employment.
3. Use public, aggregate, unclassified, non-operational evidence only. Public
   discoverability is necessary but not sufficient for admission.
4. Preserve source, vintage, scope, organization, program or asset class,
   scenario, horizon, unit, price year, evidence label, uncertainty, security
   posture, and derivation for every material input and output.
5. Test staffing, training, safety, availability, integration, maintenance,
   supply, repair, resilience, mobilization, surge, recovery, interoperability,
   and family/workforce effects together. Spending and inventory do not proxy
   readiness.
6. Keep gross opportunity, realizable public savings, external benefit,
   receipts, lifecycle cost, transition cost, risk cost, timing, uncertainty,
   and net public pressure separate.
7. Do not call degraded readiness, reduced safety, hollowed suppliers, deferred
   maintenance, workforce or family burden, impaired interoperability, shifted
   cost, or unsupported gross exposure a saving.
8. Hold missing, conflicting, sensitive, non-comparable, or unresolved evidence
   as null. Missing evidence is never zero, and a null or rejected candidate is
   a valid result.
9. Preserve adversarial role findings and dissent. No author may waive a
   promotion gate or record an independent pass on the author's own artifact.

## Authority, security, and abstraction boundary

BASTION operates below the level at which analysis could direct forces, reveal
exploitable weaknesses, or substitute for an authorized decision-maker. Its
safe unit of analysis is a public aggregate institution, mission-support
function, broad asset or program class, supplier or workforce segment,
non-operational scenario, or public fiscal aggregate. A mission abstraction
describes a publicly stated purpose and support promise; it does not encode
locations, routes, tactics, threat-response sequences, target characteristics,
specific vulnerabilities, or current unit-level posture.

The Classification & Operational Security role may reject a field, source,
granularity, join, derived result, visualization, or combination even when each
component is individually public. Rejected material may retain a minimal
inventory record—identity, rejection category, date, and reviewer—but not the
sensitive value or an explanation that recreates it. BASTION does not accept
classification markings as evidence and does not solicit, infer, summarize, or
simulate classified expertise.

The Civilian Control, Law, Safety & Readiness role separately confirms that an
artifact stays within declared lawful authority and does not silently alter a
mission, transfer a governmental decision, or degrade personnel safety,
readiness, resilience, surge, or alliance commitments. Security and civilian-
control review are independent conjunctive gates: passing either does not
compensate for failing the other.

## Actors and responsibilities

| Actor | Responsibility | Required handoff information |
|---|---|---|
| BASTION maintainer | Own repository truth, active VTRACE stage, versions, and promotion posture. | Artifact identity, scope, digest, evidence posture, validation result, role disposition, and next owner. |
| Public-evidence steward | Acquire and inventory candidate public aggregate sources. | Publisher, custody location, access date, vintage, license/reuse posture, aggregation, update cadence, limitations, and source identity. |
| Security and aggregation steward | Test direct sensitivity, combinations, joins, granularity, inference, and release risk. | Field/combination decision, safe abstraction, rejection category, residual risk, reviewer, and expiry/recheck trigger. |
| Civilian mission and authority steward | Version public mission abstractions, authorities, constraints, and analytic scenarios without selecting operations. | Mission source, authority boundary, effective period, scenario assumptions, exclusions, and unresolved ambiguity. |
| Readiness-system analyst | Trace aggregate staffing, training, availability, integration, safety, maintenance, supply, surge, and recovery dependencies. | Promise, denominator, period, evidence, uncertainty, dependencies, degraded path, and unresolved gaps. |
| Acquisition and industrial-base analyst | Examine requirements stability, cycle time, competition, capacity, qualification, workforce, concentration, and transition. | Program/asset class, supplier segment, lifecycle stage, capacity basis, competition evidence, dependencies, and transition risks. |
| Logistics and sustainment analyst | Examine maintenance, depots, spares, distribution, technical data, facilities, energy, repair, resilience, and lifecycle support. | Support boundary, workload/capacity basis, repair and recovery measures, dependencies, lifecycle cost, and uncertainty. |
| Alliance and interoperability analyst | Test standards, common logistics, partner capacity, commitments, sovereign constraints, and burden incidence. | Public commitment/standard source, compatibility boundary, partner assumptions, controls, distribution, and unresolved conflicts. |
| Defense resource analyst | Build auditable fiscal ledgers and compare bounded alternatives without converting gross opportunity into booked savings. | Baseline, counterfactual, authority, appropriation/account scope, price year, horizon, realization bridge, costs, timing, uncertainty, overlap, and downside. |
| Personnel, family, workforce, and community analyst | Test safety, tempo, staffing, retention, skills, housing, health, moves, caregiving, local services, environment, and transition incidence. | Affected aggregate group, measure and denominator, distribution/tail result, burden, mitigation, evidence, and nulls. |
| Independent test and oversight reviewer | Challenge performance, cost, schedule, failure, uncertainty, and reproducibility claims using permitted evidence. | Test identity, method, independence posture, result/null, limitations, failure evidence, and disposition. |
| Delivery owner | Accept or reject ownership of a bounded future candidate and its measurement plan. | Authority, dependencies, resources, milestones, safety/readiness floors, stop conditions, evaluation design, and rollback boundary. |
| Role review steward | Run parliament, stakeholder, editorial, assurance, and methodology review while preserving dissent. | Stable finding ID, role, severity, affected claim, digest, disposition, defer destination, closure condition, and reviewer independence. |
| Taxlane adapter steward | Prepare only a held evidence package conforming to the later accepted shared interface. | Candidate identity, separated ledgers, uncertainty, overlap, delivery posture, gate results, security posture, provenance digest, and held/admission state. |

Researchers and portfolio analysts are intended users. Service members,
families, civilian employees, depot and logistics workers, suppliers,
installation communities, taxpayers, allies, and partners are stakeholder
lenses—not data subjects whose records are processed.

## Identity, evidence, and claim custody

Every artifact uses stable identities for its source and source version,
mission abstraction, organization or system boundary, program or broad asset
class, supplier/workforce segment, scenario, horizon, claim, derivation, review,
and handoff. Supersession creates a new version; it does not rewrite a prior
result. Derived claims cite exact input versions and carry a reproducible
derivation or an explicit qualitative method.

Claims are labelled at least as `source_fact`, `derived_measure`,
`analytic_assumption`, `scenario_result`, `finding`, `null`, or `rejected`.
Vendor assertions, advocacy claims, official statements, and independent test
evidence retain distinct evidence labels. Repetition does not convert an
assertion into independent verification. An appeal to inaccessible or
classified evidence cannot prove a BASTION claim.

Each handoff includes the artifact digest and current gate posture. Review is
stale when its digest differs from the artifact under promotion. A rejected or
held input cannot silently enter a downstream derivation; a superseded source
remains reproducible; an unresolved value remains null through every handoff.

## Conjunctive promotion gates

All gates below are mandatory and non-bypassable for a promotable artifact.
Passing one cannot offset failure, omission, or uncertainty in another. A field
that truly does not apply must carry a reasoned `not_applicable` disposition,
the applicable alternative boundary, and independent review. Author discretion,
favorable economics, schedule pressure, or lack of evidence cannot produce an
implicit waiver.

### GATE-01: Scope, source, and security

The artifact uses only admitted public aggregate unclassified evidence, has
complete source custody, and passes both direct-field and combination/inference
review. It contains no classified, CUI, person-level, targeting, operational-
planning, sensitive operational, or exploitable vulnerability content. Changed
joins, granularity, visualization, or release context trigger re-review.

### GATE-02: Civilian mission, authority, and law

The public mission abstraction, lawful civilian authority, decision owner,
non-delegable decisions, jurisdiction, and analytic boundary are explicit. The
artifact neither chooses a mission nor disguises force planning, acquisition
approval, resource allocation, or official advice as research.

### GATE-03: Safety, readiness, and resilience

The artifact separately evaluates, at safe abstraction, staffing, training,
personnel safety, availability, integration, maintenance, supply, repair,
resilience, surge, mobilization, and recovery where relevant. Each facet has an
evidence reference and `result`, `null`, or independently reviewed
`not_applicable` posture. Inventory, spending, or an average cannot substitute
for these facets or hide a degraded/tail condition.

### GATE-04: Acquisition and industrial base

The artifact tests requirements stability, cycle time, competition, production
and repair capacity, supplier concentration, qualification, technical-data and
intellectual-property constraints, workforce/facility needs, cash-flow and
demand signals, production learning, transition, and surge where relevant.
Supplier exit, brittle concentration, unpriced requalification, and capacity
shift are costs or risks—not free savings.

Every applicable acquisition or candidate artifact separately exposes these
platform/system-commonality facets at safe abstraction:

1. shared-support value;
2. unique-system need;
3. concentration effect;
4. transition effect;
5. interoperability effect; and
6. common-mode or unique-system failure risk.

Each facet carries its own evidence reference and `result`, `null`, or reasoned
and independently reviewed `not_applicable` posture. The facets may not be
collapsed into one commonality score. Missing or implicit evidence, analyst
discretion, or an author-waived facet holds promotion; commonality's benefits
may not offset an unexamined concentration, transition, interoperability, or
failure risk.

### GATE-05: Logistics and lifecycle support

The artifact includes acquisition, operation, maintenance, depot, spares,
distribution, facilities, energy, technical data, training, upgrade, disposal,
and transition effects applicable to the declared boundary. It distinguishes
scheduled performance from degraded recovery at a non-operational level and
does not reduce lifecycle comparison to purchase price.

Every applicable logistics, sustainment, or candidate artifact separately
records (a) its inventory posture and stock boundary—including which stocks,
condition states, ownership/custody, period, units, inclusions, exclusions, and
stock-policy basis are represented—and (b) the repair-time distribution and
tail treatment rather than only an average. Each carries an evidence reference
and `result`, `null`, or reasoned and independently reviewed `not_applicable`
posture. Missing or implicit evidence, incompatible stock boundaries, absent
repair-time distributions, or author waiver holds promotion.

### GATE-06: Alliance and interoperability

The artifact identifies applicable public commitments, standards, common
logistics, compatibility, partner capacity, sovereign and export/control
boundaries, transition effects, and burden incidence. A unilateral financial
gain may not erase an interoperability loss or shift unacknowledged cost/risk
to an ally or partner.

### GATE-07: Fiscal and resource bridge

Every promoted quantitative opportunity records a cited baseline and
counterfactual, appropriation/account boundary, unit, quantity, price year,
horizon, gross opportunity, realizable public savings, external benefit,
receipts, lifecycle cost, transition cost, risk cost, realization timing,
uncertainty range and method, downside case, overlap/double-count treatment,
and net public pressure. Unsupported values remain null. A probability appears
only with evidence; otherwise it carries reviewed `not_applicable`.

The bridge keeps five possible contribution pathways separate and prevents
double counting among them:

1. direct public cost reduction;
2. delivery or process efficiency;
3. avoided future cost or risk;
4. readiness or capacity gain; and
5. lawful domain-relevant revenue or receipt effect.

Each pathway has its own baseline, unit, horizon, realization owner, transition
and implementation cost, uncertainty, overlap keys, and valid null path. A
readiness or capacity gain is not monetized unless a later accepted method and
evidence support that conversion; an avoided cost is not current cash savings;
and receipts remain distinct from expenditure reduction.

Where a peer goal is used, the comparison records why the peers and functions
are comparable, institutional and mission differences, input and output
measures, price-year and purchasing-power treatment where applicable, source
vintages, uncertainty, and the portion that cannot be compared. A peer value is
a scenario reference, not a target mandate or evidence that the United States
can realize the same result.

Gross opportunity is never a savings claim. Realizable public savings require
an identified authority and delivery owner, implementation and transition
path, cost and time to realize, evidence-supported capture mechanism, safety
and readiness non-degradation, and reconciliation to the cited public baseline.
Promoted candidates also state a feedback and re-evaluation cadence: refresh
evidence and peer comparability, compare observed delivery with the baseline,
re-estimate costs and effects, detect burden shifts or floor degradation, and
hold, revise, or retire the claim. Re-evaluation never converts BASTION into a
budget, allocation, or rate authority.

### GATE-08: People, suppliers, taxpayers, and communities

The artifact separately tests incidence on service members and families,
civilian and depot/logistics workforces, prime and small suppliers,
installation communities, taxpayers, and allies/partners where relevant.
Safety, tempo, retention, skills, moves, housing, health, caregiving, local
services, environmental effects, employment transition, cash flow, and burden
distribution receive `result`, `null`, or independently reviewed
`not_applicable` postures. Averages cannot conceal concentrated or tail harms.

### GATE-09: Independent test, uncertainty, and dissent

Material performance, readiness, cost, schedule, failure, and uncertainty
claims are independently observable in permitted evidence or remain null.
Negative evidence and failed tests are retained. All required parliament,
stakeholder, editorial, assurance, and methodology surfaces are present; the
author and assurance-sensitive reviewers differ; major dissent is preserved.

### GATE-10: Delivery and downstream authority

The candidate has a bounded owner, dependencies, resources, milestones,
measures, stop conditions, evaluation design, and rollback boundary appropriate
to its later stage. A BASTION result remains research evidence. Any Taxlane
package is held; only Taxlane may admit, compare, allocate, or use it in shared
accounting. No artifact implies official adoption, obligation, procurement,
rate-setting, or release.

## Operating scenarios

### OPS-001: Admit or reject a public aggregate source

**Trigger:** a steward proposes a public report, dataset, table, audit,
test/evaluation report, statute, standard, budget document, supplier or
workforce aggregate, or methodological source.

**Inputs:** source material; publisher; custody location; access date; vintage;
license/reuse posture; aggregation and geography; program, mission, supplier,
or workforce applicability; units; known revisions and limitations.

**Normal path:**

1. Assign stable source and source-version identities and retain the original
   public context.
2. Label the evidence type, provenance, applicable period, denominators,
   update cadence, uncertainty, exclusions, and advocacy/independence posture.
3. Inspect direct content, metadata, tables, joins, small groups, derived
   granularity, mosaicing, and foreseeable combinations for sensitive or
   exploitable inference.
4. Map only admissible fields to a safe public mission, readiness, acquisition,
   logistics, supplier, workforce, alliance, or fiscal concept.
5. Validate units, totals, suppression, revisions, and derivations.
6. Route the bounded source record through citation, scope, numeracy, security,
   and—as relevant—civilian-control review.

**Degraded or failure path:** reject classified, CUI, person-level, sensitive
operational, targeting, operational-planning, vulnerability, inadequately
aggregated, or dangerous-combination content. Quarantine inaccessible,
uncitable, stale, incompatible, or unexplained values. Preserve only a safe
rejection receipt; do not reproduce the rejected content or infer a substitute.

**Outputs and handoff:** admitted source record or safe rejection receipt,
field posture, limitations, validations, reviewer identity, and expiry/recheck
trigger. Downstream stewards receive the exact admitted version and digest.

### OPS-002: Register a public mission abstraction and scenario

**Trigger:** an analysis requires a stated public mission, authority, support
promise, baseline, sensitivity, stress, or bounded institutional alternative.

**Inputs:** admitted public mission and authority sources; public organization
and support relationships; assumptions; horizon; units; exclusions; and
supersession context.

**Normal path:**

1. Assign mission-abstraction and scenario identities.
2. Separate public source facts, BASTION interpretations, and analytic
   assumptions.
3. Record lawful civilian authority, decision ownership, jurisdiction,
   effective period, publicly declared outcome, safe support boundary, and
   decisions BASTION cannot make.
4. Define a cited baseline before a sensitivity, stress, or counterfactual.
5. Exclude locations, routes, targets, tactics, timelines, vulnerabilities,
   current unit posture, and other operationally sensitive detail.
6. Route ambiguity to a hold; do not resolve it through software behavior or
   analyst preference.

**Degraded or failure path:** if authority, mission scope, safety boundary, or
safe abstraction is unclear, register the unresolved question and block
dependent claims. An abstraction that could support operational planning is
rejected even if its components are public.

**Outputs and handoff:** versioned mission/scenario manifest, authority and
exclusion record, assumptions, horizon, uncertainty posture, and hold reasons;
all later artifacts cite the exact versions.

### OPS-003: Build a public aggregate readiness-system baseline

**Trigger:** analysts ask whether an institutional support network can satisfy
a declared public mission promise at safe abstraction.

**Inputs:** admitted sources; mission and scenario versions; aggregate staffing,
training, availability, safety, maintenance, supply, repair, integration,
resilience, surge, recovery, cost, and interoperability evidence.

**Normal path:**

1. Declare the organizational and system boundary, promise, period,
   denominators, and known exclusions.
2. Reproduce cited public aggregates before deriving measures.
3. Trace institutional dependencies across personnel, acquisition, suppliers,
   logistics, sustainment, facilities, information, and partners without
   encoding operational deployment or vulnerabilities.
4. Evaluate every applicable readiness/safety facet separately under
   `GATE-03`, including distributions or tails where public evidence permits.
5. Reconcile inconsistent definitions, vintages, denominators, and system
   boundaries; keep unresolved joins separate.
6. Run a bounded non-operational downside or degraded-support case and retain
   failure evidence.

**Degraded or failure path:** broaden or suppress unsafe groups; reject unsafe
joins; hold measures whose denominator, source independence, or mission link
cannot be established. Emit a null rather than inferring employable readiness
from inventories, budgets, averages, or classified appeals.

**Outputs and handoff:** reproducible aggregate readiness-network artifact,
facet results/nulls, dependency map at safe abstraction, evidence gaps,
uncertainty, and bounded finding or null. It proceeds to full role review.

### OPS-004: Examine acquisition and industrial-base delivery

**Trigger:** a public aggregate finding concerns requirements stability,
acquisition cycle, competition, supplier capacity, production, qualification,
workforce, or transition.

**Inputs:** admitted program/asset-class and supplier-segment aggregates;
public requirements and authorities; acquisition, schedule, test, competition,
capacity, workforce, facility, cost, and transition evidence.

**Normal path:**

1. Define the bounded outcome and lifecycle stage without making an acquisition
   selection or revealing sensitive production or vulnerability detail.
2. Separate prime, small-supplier, government, workforce, and facility
   dependencies and test concentration at an approved aggregation.
3. Separately evaluate shared-support value, unique-system need, concentration,
   transition, interoperability, and common-mode or unique-system failure risk.
   Give each facet its own evidence and `result`, `null`, or independently
   reviewed `not_applicable`; otherwise hold promotion.
4. Examine requirements changes, qualification, technical-data/IP constraints,
   competition, cycle time, capacity, cash flow, learning, and demand stability.
5. Include requalification, tooling, workforce, facility, transition, support,
   schedule, and failure costs.
6. Test whether apparent speed or savings hollow capacity, create a fragile
   sole dependency, shift risk, or reduce safe mission support.
7. Route performance and schedule claims to independent test/oversight review.

**Degraded or failure path:** suppress/reject sensitive supplier or capacity
detail and hold causal or savings claims when only vendor advocacy, incomplete
competition data, or gross obligation totals are available. Supplier exit and
delayed capability are not efficiencies.

**Outputs and handoff:** aggregate delivery map, supplier/workforce incidence,
lifecycle/transition bridge, capacity and concentration posture, uncertainties,
and candidate/null. Logistics, alliance, personnel, fiscal, and assurance roles
receive it.

### OPS-005: Examine logistics, sustainment, and degraded recovery

**Trigger:** a gap concerns maintenance, depots, spares, distribution,
facilities, technical data, energy, repair, availability, surge, or recovery.

**Inputs:** public aggregate workload, capacity, availability, maintenance,
repair, supply, workforce, facility, lifecycle-cost, and resilience evidence;
mission and scenario versions.

**Normal path:**

1. Declare a non-operational support boundary and compatible denominators.
2. Define the inventory posture and stock boundary: stocks and condition states,
   ownership/custody, period, units, inclusions, exclusions, and stock-policy
   basis. Require evidence and a `result`, `null`, or independently reviewed
   `not_applicable`; missing or incompatible stock evidence holds promotion.
3. Trace acquisition-to-sustainment custody, including technical data,
   workforce, facilities, spares, repair, upgrades, and disposal.
4. Compare planned, observed, and degraded recovery performance without
   publishing operationally useful bottleneck detail.
5. Record the repair-time distribution and explicit tail treatment, with its
   evidence and `result`, `null`, or independently reviewed `not_applicable`;
   an average alone or missing distribution evidence holds promotion.
6. Test schedule, workload, capacity, safety, supplier, and workforce
   assumptions; preserve queues and other tail effects when safely aggregated.
7. Include deferred-maintenance, cannibalization, obsolescence, transition,
   surge, and recovery consequences in lifecycle and risk ledgers.
8. Obtain security and readiness review of every derived map or combination.

**Degraded or failure path:** security-review rejection overrides analytic
utility. Incompatible workload or availability measures remain separate.
Missing support evidence produces a gap/null, not an assumption of availability
or a zero-cost repair path.

**Outputs and handoff:** safe aggregate sustainment artifact, lifecycle and
degraded-path results, dependencies, risks, costs, uncertainty, and nulls for
industrial-base, readiness, fiscal, community, and assurance review.

### OPS-006: Examine alliance and interoperability handoffs

**Trigger:** a finding or candidate could affect public commitments, standards,
shared logistics, coalition compatibility, partner capacity, sovereign
constraints, export/control boundaries, or burden distribution.

**Inputs:** admitted public commitments, standards, aggregate compatibility and
capacity evidence, scenario assumptions, lifecycle and transition results, and
partner/alliance stakeholder review.

**Normal path:**

1. State the public commitment and authority boundary without simulating an
   operation or inferring partner intent.
2. Trace standards, common support, training, logistics, qualification,
   controls, and transition dependencies at safe abstraction.
3. Record sovereign constraints and distinguish U.S., partner, shared, and
   externalized costs, benefits, burdens, and risks.
4. Test the candidate under both normal and degraded institutional support,
   preserving conflicting partner and domestic results.
5. Require alliance/interoperability, legal/civilian-control, security, and
   fiscal review before promotion.

**Degraded or failure path:** do not fill partner-data gaps with assumptions or
use public fragments to infer sensitive partner readiness. Hold a candidate
when common-system obligations, transition costs, or burden incidence cannot be
evaluated safely.

**Outputs and handoff:** interoperability and commitment posture, separated
partner ledgers, sovereign/control constraints, dissent, uncertainty, and held
candidate/null.

### OPS-007: Develop a bounded Defense 2.0 candidate

**Trigger:** a reviewed readiness, acquisition, logistics, organizational,
supplier, interoperability, workforce, or fiscal gap suggests a bounded
institutional alternative.

**Inputs:** reviewed gap; cited baseline; candidate scenario; authority
boundary; affected stakeholders; applicable gates; delivery owner; evidence
about dependencies, costs, timing, risks, and overlap.

**Normal path:**

1. State the hypothesis, declared public outcome, non-goals, owner, and valid
   null path.
2. Construct a reproducible counterfactual at non-operational abstraction; do
   not recommend forces, targets, employment, procurement, or allocation.
3. Test all ten conjunctive gates and preserve distinct facet results.
4. Build the gross-to-net resource bridge. Separate gross opportunity from
   realizable savings, benefits, receipts, lifecycle/transition/risk costs,
   timing, uncertainty, downside, overlap, and net pressure.
5. Record direct cost reduction, delivery/process efficiency, avoided future
   cost/risk, readiness/capacity gain, and lawful revenue/receipt effects as
   separate non-double-counted pathways, including valid nulls.
6. If a peer goal informs the scenario, test functional comparability,
   institutional/mission differences, vintages, units, price treatment,
   uncertainty, and non-comparable portions; never treat a peer rate as a
   mandate or automatically realizable result.
7. Test concentrated and tail effects on personnel, families, workforces,
   suppliers, communities, taxpayers, and partners.
8. Define later-stage measures, feedback/re-evaluation cadence, stop
   conditions, evaluation, and rollback concepts without claiming
   implementation readiness.
9. Route the frozen digest to independent substantive, editorial, assurance,
   stakeholder, and methodology review.

**Degraded or failure path:** reject a candidate that needs prohibited evidence,
degrades a hard floor, lacks authority, hides burden/cost, or cannot survive
security review. Hold a candidate with insufficient evidence or ownership.
Record a null when the candidate does not improve the declared outcome.

**Outputs and handoff:** candidate or null artifact, gate matrix, resource
bridge, distribution, delivery questions, review findings, dissent, and held
posture. This is research evidence, not a recommendation.

### OPS-008: Conduct independent test and adversarial review

**Trigger:** an artifact requests promotion or makes a material readiness,
performance, cost, schedule, safety, security, or delivery claim.

**Inputs:** frozen artifact and digest; evidence manifest; derivations; gate
matrix; negative cases; unresolved questions; required role packet.

**Normal path:**

1. Verify the digest and reviewer independence before review.
2. Attempt to reproduce permitted quantitative results and inspect qualitative
   claim custody.
3. Test adverse cases, failure evidence, uncertainty, denominator and price-
   year integrity, lifecycle/transition arithmetic, and double counts.
4. Run every parliament and stakeholder lens; then citation, scope, numeracy,
   both assurance lanes, and the five-archetype methodology panel.
5. Record stable findings as `pass`, `finding`, or `defer`, with severity,
   evidence pointer, owner, destination, substantive closure condition, and
   dissent.
6. Return changed artifacts for a new digest and independent recheck.

**Degraded or failure path:** stale digest, conflicted reviewer, missing role,
unsafe evidence, unowned defer, or unresolved critical/major finding blocks
promotion. Inability to reproduce a claim produces a finding or null, never a
credential-based waiver.

**Outputs and handoff:** immutable review packet, findings, dispositions,
residual risks, dissent, and fixed-point eligibility. Review does not confer
government, military, vendor, ally, Taxlane, or public approval.

### OPS-009: Prepare a held Taxlane evidence handoff

**Trigger:** a later-stage, independently reviewed BASTION candidate has a
complete future shared interface and requests consideration outside BASTION.

**Inputs:** exact artifact digest; sources and derivations; all gate results;
separated fiscal/resource ledgers; distribution; uncertainty and downside;
delivery posture; overlap keys; security posture; residual risk and dissent.

**Normal path:**

1. Revalidate scope, source custody, security combinations, civilian authority,
   safety/readiness floors, numeracy, and reviewer independence.
2. Map only accepted shared-interface fields while retaining BASTION identities,
   nulls, uncertainty, exclusions, and provenance.
3. Preserve gross opportunity, realizable public savings, external benefit,
   receipts, lifecycle/transition/risk cost, timing, overlap, and net pressure
   as separate values.
4. Preserve each distinct contribution pathway—direct cost reduction,
   delivery/process efficiency, avoided future cost/risk, readiness/capacity
   gain, and lawful revenue/receipt effect—with its peer-comparability posture,
   realization owner, feedback cadence, uncertainty, costs, overlap keys, and
   valid nulls.
5. Mark the package `held`; record adapter and source digests and the Taxlane-
   controlled admission state.

**Degraded or failure path:** reject stale, incomplete, unsafe, unreconciled,
unowned, falsely precise, or floor-failing handoffs. Missing values remain null;
BASTION does not fabricate shared-interface values or infer Taxlane acceptance.

**Outputs and handoff:** held LaneEvidencePack candidate or rejection receipt.
Only Taxlane can admit or use the package for cross-lane accounting; the
handoff is not an official score, recommendation, request, allocation, rate, or
release.

### OPS-010: Advance or hold a VTRACE deliverable

**Trigger:** the active deliverable seeks fixed-point review.

**Inputs:** artifact digest; validation receipts; full role packet; findings,
dissent, deferrals, and active-wave record.

**Normal path:** freeze the artifact, obtain independent review against its
digest, remediate bounded actionable findings, recheck independently, and
advance only with zero unresolved critical/major findings, both assurance gates
passed, and every defer open with an owner, destination, and substantive closure
condition.

**Degraded or failure path:** hold on stale review, scope expansion, missing
role, false approval language, ownerless defer, failed assurance, or unresolved
critical/major finding. After bounded remediation cycles, escalate rather than
polishing indefinitely.

**Outputs and handoff:** fixed-point decision or held decision, residual risks,
next eligible deliverable, and explicit non-authorizations. A fixed point does
not authorize implementation; Rust awaits an accepted work package.

## Degraded operating modes

| Mode | Required behavior | Prohibited inference |
|---|---|---|
| Source unavailable, revised, or irreconcilable | Pin the last reproducible version, mark staleness, hold changed claims, and seek a public replacement. | Silence, staleness, or revision is not zero and does not prove trend. |
| Security or combination concern | Stop propagation, record a safe rejection, invalidate dependent promotion, and route to security review. | Public fragments may not be recombined to reconstruct sensitive posture. |
| Incompatible definitions or denominators | Keep series separate, expose incompatibility, and emit a null if comparison fails. | Similar labels do not establish equivalence. |
| Missing degraded/recovery evidence | Hold readiness and savings conclusions and record the missing facet. | Normal-period averages do not prove resilience or surge. |
| Supplier/workforce evidence gap | Hold capacity, transition, schedule, and realizable-savings claims. | Obligations or contract counts do not prove healthy capacity or competition. |
| Alliance/partner evidence gap | Preserve the domestic result separately and hold joint or burden claims. | Missing partner evidence does not imply compatibility, consent, or zero burden. |
| Delivery owner or authority absent | Retain a research hypothesis only; block realizable savings and downstream promotion. | A modeled gross opportunity is not executable or cashable. |
| Review disagreement | Preserve dissent, isolate the disputed claim, and hold if critical/major. | Averages across reviewer opinions do not create approval. |

## Role review surfaces

### Parliament

- **Civilian Strategy & Force Planner:** mission abstraction, public priority,
  authority, tradeoffs, organizational boundary, and protection against covert
  operational planning.
- **Operational Readiness:** staffing, training, availability, integration,
  maintenance, supply, repair, safety, surge, and recovery beyond inventory.
- **Acquisition & Industrial Base:** requirements stability, competition,
  qualification, capacity, concentration, workforce, learning, and transition.
- **Logistics & Sustainment:** lifecycle support, depots, spares, distribution,
  facilities, energy, repair, degraded recovery, and surge.
- **Defense Comptroller:** accounts, price years, lifecycle, transition,
  realization, timing, risk, overlap, auditability, and downside.
- **Service-Member & Family:** safety, tempo, staffing, retention, housing,
  health, moves, caregiving, and hidden family burden.
- **Independent Test & Oversight:** observable performance, cost, schedule,
  failure, uncertainty, independence, and reproducibility.
- **Alliance & Interoperability:** standards, shared logistics, compatibility,
  commitments, sovereign/control boundaries, partner capacity, and incidence.

### Stakeholders

Every applicable artifact shows separate results/nulls for service members and
families; mission users; depot and logistics workforces; prime and small
suppliers; installation communities; allies and partners; and taxpayers and
oversight bodies. Conflicting effects are preserved rather than netted into a
single score.

### Editorial, assurance, and methodology

Citation review tests source custody and evidence labels. Numeracy review tests
units, denominators, price years, horizons, lifecycle and transition cost,
uncertainty, arithmetic, and double counts. Scope review tests the aggregate,
unclassified, non-operational, non-official, and non-release boundary. Both
assurance roles must independently pass any promotion. The methodology panel
uses the five declared archetypes, public evidence only, no named-person
impersonation, and no claim of external approval.

Dissent is a first-class output: it names the role, affected claim, artifact
digest, evidence, severity, requested disposition, and whether it blocks the
current stage. It remains visible after remediation.

## Explicit deferrals

Every deferral below is **open**. Each has a later destination and a substantive
closure condition. No deferral authorizes its own assumed answer, and a reasoned
`not_applicable` posture also requires independent review.

| ID | Deferred detail | Owner | Destination | Substantive closure condition | Posture |
|---|---|---|---|---|---|
| `DEF-CONOPS-001` | Exact safe aggregation, combination/inference, suppression, expiry, and re-review controls. | Security and aggregation steward | REQUIREMENTS, security specification, interface, and verification | Public-data threat model covers direct and compositional inference; machine-checkable admission/rejection fields, minimum safe aggregation or equivalent control, expiry triggers, negative fixtures, and independent security acceptance exist. | open; promotion-gating |
| `DEF-CONOPS-002` | Exact mission, readiness, safety, resilience, surge, and recovery promise measures and floors. | Civilian mission and readiness stewards | REQUIREMENTS and promise specification | Each measure has authority, definition, denominator, horizon, tail/degraded treatment, evidence rule, threshold or explicit decision procedure, and non-waivable failure/hold behavior accepted by both assurance lanes. | open; promotion-gating |
| `DEF-CONOPS-003` | Source ontology, evidence labels, claim schema, identity/versioning, and derivation custody. | Public-evidence and role review stewards | REQUIREMENTS, corpus/interface specification, and verification | Schemas cover versions, supersession, advocacy/independence, null/rejection, derivation, digest binding, stale-review rejection, and reproducibility with positive and negative fixtures. | open; promotion-gating |
| `DEF-CONOPS-004` | Quantitative horizon, uncertainty, probability, price-year, lifecycle, present-value, peer-comparability, and reconciliation methods. | Defense resource analyst and numeracy reviewer | REQUIREMENTS and economics specification | Permitted methods, units, horizon representation, ranges, downside, evidence-supported probability rules, discount/present-value conventions where applicable, functional peer selection and comparability limits, purchasing-power/price treatment, reconciliation tolerances, and null behavior are independently accepted and fixture-tested. | open; promotion-gating |
| `DEF-CONOPS-005` | Acquisition, platform/system commonality, supplier capacity/concentration, competition, qualification, production learning, workforce, and transition measures safe for public aggregation. | Acquisition and industrial-base analyst | REQUIREMENTS and industrial-base specification | Exact commonality measures and interpretation limits cover both benefits and risks across shared-support value, unique-system need, concentration, transition, interoperability, and common-mode/unique-system failure; all measures have safe abstraction, denominators, evidence hierarchy, capacity limits, small-supplier incidence, explicit null/N/A behavior, negative cases, and security/industrial-base acceptance. | open; promotion-gating |
| `DEF-CONOPS-006` | Logistics, sustainment, inventory posture/stock boundary, availability, repair-time distribution/tails, workload, depot, spares, resilience, and degraded-recovery measures. | Logistics and sustainment analyst | REQUIREMENTS and readiness/sustainment specification | Exact stock-policy and inventory measures define stocks/condition states, custody, period, units, inclusions/exclusions, and reconciliation; exact repair-time distribution and tail measures define denominators, censoring, percentiles/tails, and degraded treatment; compatible lifecycle boundaries, safe aggregation, null/N/A rules, negative fixtures, and readiness/security acceptance are verified. | open; promotion-gating |
| `DEF-CONOPS-007` | Alliance/interoperability standards, commitment, sovereign/control, partner-capacity, and burden-incidence representation. | Alliance and interoperability analyst | REQUIREMENTS and interoperability interface | Public authority sources, compatibility semantics, control boundaries, separated partner ledgers, uncertainty/null rules, partner-risk review, and security/legal acceptance exist without operational inference. | open; promotion-gating |
| `DEF-CONOPS-008` | Personnel, family, workforce, supplier, taxpayer, and installation-community burden, distribution, and tail measures. | Personnel/family/workforce/community analyst | REQUIREMENTS and distribution specification | Each stakeholder lens has measures, denominators, time and tail treatment, burden-shift tests, evidence rules, explicit null/N/A behavior, and independent stakeholder and assurance acceptance. | open; promotion-gating |
| `DEF-CONOPS-009` | Gross-to-net opportunity and multi-pathway accounting for direct cost reduction, delivery/process efficiency, avoided future cost/risk, readiness/capacity gain, lawful revenue/receipt effect, realizable savings, cost, timing, uncertainty, overlap, and net pressure. | Defense resource analyst | REQUIREMENTS, economics specification, and shared accounting interface | The schema prevents gross-to-savings promotion; keeps all five pathways separate with valid nulls; explicitly distinguishes budget authority, obligations, outlays, transfers, and offsetting receipts while preserving appropriation and accountable fiscal-owner boundaries; records realization owner, capture, lifecycle/transition/implementation/risk cost, horizon, timing, feedback cadence, uncertainty, peer-comparability posture, and overlap; reconciles cited baselines; and passes numeracy plus Taxlane interface review. | open; promotion-gating |
| `DEF-CONOPS-010` | Independent-test evidence hierarchy, reproducibility, failure retention, reviewer independence, and convergence mechanics. | Independent test and role review stewards | REQUIREMENTS and verification plan | Evidence tiers, reproducibility criteria, failed/negative test retention, conflict rules, digest binding, reviewer independence, severity/disposition schema, and fixed-point negative fixtures are accepted. | open; promotion-gating |
| `DEF-CONOPS-011` | Delivery-owner readiness, milestone, feedback/re-evaluation cadence, stop-condition, evaluation, rollback, transition, and realization evidence. | Delivery owner | REQUIREMENTS, delivery specification, and accepted future work package | Required ownership/authority, dependencies/resources, measures, safety/readiness floors, phased evidence, observed-versus-baseline and peer-comparability refresh, burden-shift detection, stop/rollback logic, realization proof, and independent validation are specified; implementation still awaits an accepted work package. | open; promotion-gating |
| `DEF-CONOPS-012` | Exact held LaneEvidencePack fields, adapter ownership, Taxlane admission semantics, and shared overlap/accounting contract. | Taxlane adapter steward | INTERFACES after BASTION requirements and shared Taxlane review | BASTION and Taxlane identities, separated ledgers, nulls, uncertainty, security, gate/delivery state, overlap keys, provenance digests, rejection behavior, held/admitted ownership, and contract fixtures are accepted by both repositories. | open; promotion-gating |
| `DEF-CONOPS-013` | Public-release composition, context-loss, misuse, and repeated-release controls if release is ever separately authorized. | Scope, security, and citation reviewers | Release-specific requirements and validation under separate authority | A separately authorized release plan assesses cross-artifact composition and linkage, sensitive context, stale posture, audience misuse, provenance retention, takedown/correction, and independent security/scope acceptance. This CONOPS supplies no release authority. | open; not current-stage authorization |

## Open questions and traceability

| ID | Open question | Deferral destination | Required reviewers |
|---|---|---|---|
| `OQ-001` | Which public aggregates and combinations are safe enough for each analysis class, and when must they expire or be re-reviewed? | `DEF-CONOPS-001`, `DEF-CONOPS-013` | Security, citation, scope, methodology. |
| `OQ-002` | Which mission/readiness promises and floors are measurable without crossing into operational planning? | `DEF-CONOPS-002`, `DEF-CONOPS-006` | Civilian strategy, readiness, logistics, both assurance lanes. |
| `OQ-003` | What source and claim schemas make every result reproducible while retaining null, rejection, supersession, and dissent? | `DEF-CONOPS-003`, `DEF-CONOPS-010` | Citation, numeracy, independent test, methodology. |
| `OQ-004` | Which fiscal/economic methods and accounting fields prevent gross opportunity from becoming false savings? | `DEF-CONOPS-004`, `DEF-CONOPS-009` | Comptroller, numeracy, independent test, taxpayer oversight. |
| `OQ-005` | Which industrial-base measures reveal capacity and concentration without exposing sensitive supplier detail? | `DEF-CONOPS-001`, `DEF-CONOPS-005` | Acquisition/industrial base, suppliers, security, scope. |
| `OQ-006` | How are lifecycle logistics and degraded recovery tested safely and comparably? | `DEF-CONOPS-002`, `DEF-CONOPS-006` | Logistics, readiness, mission user, security, numeracy. |
| `OQ-007` | How are alliance obligations, interoperability, sovereign constraints, controls, and burden represented without inferring partner operations? | `DEF-CONOPS-001`, `DEF-CONOPS-007` | Alliance/interoperability, ally/partner, civilian/legal, security. |
| `OQ-008` | Which distribution and tail measures expose burdens on people, suppliers, taxpayers, and communities? | `DEF-CONOPS-008` | All stakeholder lenses, family advocate, numeracy, assurance. |
| `OQ-009` | What evidence turns a bounded hypothesis into a delivery-testable candidate with stop and rollback protection? | `DEF-CONOPS-010`, `DEF-CONOPS-011` | Independent test, delivery owner, full parliament, assurance. |
| `OQ-010` | Which fields belong in BASTION's held adapter, and which decisions remain exclusively Taxlane-controlled? | `DEF-CONOPS-009`, `DEF-CONOPS-012` | Comptroller, Taxlane adapter, interface owners, scope, assurance. |
| `OQ-011` | How should adaptive peer-informed goals and the five contribution pathways be compared, re-evaluated, and retired without double counting or implying Taxlane authority? | `DEF-CONOPS-004`, `DEF-CONOPS-009`, `DEF-CONOPS-011`, `DEF-CONOPS-012` | Comptroller, numeracy, delivery owner, taxpayer oversight, Taxlane adapter, both assurance lanes. |

## Preliminary author role check

This author check does not confer a pass. The draft exposes the following
surfaces for independent review:

- Civilian mission authority precedes system and fiscal optimization.
- Readiness is decomposed beyond inventory and spending, with safety, degraded
  recovery, and null paths explicit.
- Acquisition, supplier capacity, concentration, competition, transition, and
  small-supplier effects are inside the candidate boundary.
- Logistics, lifecycle sustainment, depots, spares, workforce, and recovery
  precede realizable-savings claims.
- Price year, horizon, lifecycle, transition, realization, timing, risk,
  overlap, uncertainty, and downside remain separate.
- Service-member/family, workforce, supplier, community, taxpayer, and partner
  burdens cannot be treated as free inputs.
- Independent test and negative evidence can reject advocacy claims, including
  unsupported appeals to classified evidence.
- Alliance commitments and interoperability cannot be erased by unilateral
  cost minimization.
- Citation, scope, numeracy, security-by-combination, civilian control, law,
  safety, readiness, and release boundaries are explicit conjunctive gates.

Independent reviewers must challenge, rather than inherit, these author
assessments and preserve disagreement.

## Review handoff and current decision

| Review stage | Decision | Result |
|---|---|---|
| Independent substantive review | finding | Two major findings: platform/system commonality; inventory posture and repair-time distribution/tails. |
| Independent stakeholder, editorial, and assurance review | pass_with_risk | One minor later-stage accounting deferral required explicit federal fiscal-measure distinctions. |
| Bounded remediation | remediated | Mandatory commonality and inventory/repair treatments added; accounting closure sharpened without resolving later detail. |
| Independent editorial/method recheck | pass_with_risk | Remediations, deferral mapping, decision language, scope, and boundary consistency passed. |

Current-CONOPS unresolved findings: **zero critical; zero major**.

Fixed-point decision: **pass_with_risk**. All 13 later-stage
`DEF-CONOPS-*` records remain open and promotion-gating. They confer no
operational, acquisition, budget, Taxlane-admission, allocation, rate-setting,
official-use, implementation, or public-release authority.

The next eligible VTRACE deliverable is `REQUIREMENTS.md`, only under a new
assignment. Implementation remains blocked until a relevant work package is
accepted.
