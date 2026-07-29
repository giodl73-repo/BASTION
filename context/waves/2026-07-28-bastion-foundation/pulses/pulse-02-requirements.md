# Pulse 02 — REQUIREMENTS author baseline

Date: 2026-07-28
Assignment: `ASG-BASTION-REQUIREMENTS-001`
Writer lease: exclusive to the BASTION REQUIREMENTS author for this pulse
CONOPS input SHA-256:
`a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602`

Remediation assignment: `ASG-BASTION-REQUIREMENTS-REMEDIATION-001`
Remediation input SHA-256:
`87c15970b396dd8083acd266b0e0df8ebac8da2e855dae02b985fc08ffbf41e1`

## Objective

Translate the settled BASTION CONOPS into a review-ready requirements baseline
without opening specification, architecture, interfaces, design, work packages,
corpus construction, Rust implementation, Taxlane admission, official action,
or public release.

## Required inputs read

- all BASTION governance, product, mission, CONOPS, wave, pulse, and `.roles`
  files;
- the VTRACE adoption `REQUIREMENTS.md` template and adoption README;
- VTRACE process, staged-execution, review-process, requirements-traceability,
  and VTRACE-repo requirements guidance; and
- TRACKER's `infrastructure-2.0-common-pattern.md`.

## Baseline produced

- Created `docs/vtrace/REQUIREMENTS.md` with stable BASTION requirement IDs.
- Required every normative row to carry `shall` or `shall not`, parent trace,
  rationale, priority, owner, planned verification, and `pending_review`.
- Covered all ten CONOPS gates, all ten operating scenarios, all 13 deferrals,
  and all 11 open questions.
- Preserved public aggregate and compositional-security controls; civilian
  authority; safety/readiness/resilience floors; source and claim custody;
  quantitative and peer methods; six commonality facets; inventory/stock and
  repair distribution/tails; alliance/sovereignty; stakeholder distribution;
  independent test; delivery feedback and rollback; held Taxlane handoff; and
  release controls.
- Encoded six separate, non-interchangeable adaptive pathway postures: direct
  public cost reduction, delivery/process efficiency, avoided future cost/risk,
  readiness/capacity/resilience gain, lawful receipt effect, and null.
- Distinguished budget authority, obligations, outlays, transfers, and
  offsetting receipts while preserving appropriation and fiscal ownership.
- Created owned `TBD-*` records for unresolved exact values and methods; each
  has a destination, substantive closure condition, explicit hold behavior,
  and open posture.

## Remediation finding dispositions

| Finding group | Remediation | Disposition |
|---|---|---|
| Pathway horizons did not require separate treatment for all three time bands. | `BASTION-REQ-ECO-003`, `ECO-018`, and `ECO-019` now require near-, medium-, and long-horizon result/null treatment, reasoned independently reviewed N/A with an alternative time boundary, and a hold for missing or unaccepted treatment. Exact durations remain open under `TBD-QNT-001`. | remediated; independent recheck passed |
| Cross-stakeholder distribution ownership was too narrow. | `BASTION-REQ-DST-003` is now owned by the Personnel/family/workforce/community analyst and requires Service-Member/Family, supplier, depot/workforce, installation, taxpayer/oversight, and both assurance reviews. | remediated; independent recheck passed |
| Twenty compound rows were not independently falsifiable. | Split the cited action/prohibition pairs into stable IDs, producing 98 normative rows. Added branch-complete verification and the all-or-hold `PATHWAY-ENVELOPE-001` invariant. | remediated; independent recheck passed |
| Normative dependencies on unresolved values were indirect. | Added a direct requirement-to-TBD dependency trace for all 13 TBDs; listed requirements inherit each TBD's hold until independent acceptance. | remediated; independent recheck passed |

All 13 TBD records remain open. No exact duration, threshold, method, force,
procurement, budget, allocation, rate, or sensitive value was selected.

## Scope and authority posture

- No force structure, procurement, budget, allocation, rate, operational
  method, sensitive threshold, or implementation design was selected.
- No classified information, CUI, person-level service data, sensitive
  operational data, targeting, operational planning, vulnerability
  exploitation, official recommendation, or public release is authorized.
- Taxlane alone may admit a fiscal effect, combine lanes, rebalance allocations,
  or test rates.
- No Cargo workspace, Rust code, implementation, or accepted work package was
  created.

## Author validation

Requirements output SHA-256:
`b517e376f47d53eca11e7658166af966c7baeef513b1ec7e3862c94556337c55`

- Normative requirements: **98**, with 98 unique `BASTION-REQ-*` IDs.
- Controlled TBD records: **13**, with 13 unique `TBD-*` IDs.
- Parent coverage: `OPS-001` through `OPS-010`, `GATE-01` through `GATE-10`,
  `DEF-CONOPS-001` through `DEF-CONOPS-013`, and `OQ-001` through `OQ-011`
  all present in normative rows.
- Row quality: every normative row passed `shall`/`shall not`, parent,
  rationale, priority, owner, verification-method, and `pending_review` checks.
- TBD quality: every unresolved row has an owner, destination, substantive
  closure condition, hold behavior, and open posture.
- Atomicity: none of the 20 cited compound rows retains multiple normative
  `shall` clauses; split rows have branch-specific verification.
- Direct dependency: every `TBD-*` appears in the direct normative dependency
  trace with affected requirement IDs and inherited hold behavior.
- Governance: `.roles/ROLE.md` and all 21 role files are present.
- Hygiene and boundary checks: zero trailing-whitespace findings; no missing
  required scope terms; fixed CONOPS digest unchanged; no Cargo manifest or
  Rust files.

## Fixed-point review sequence

| Stage | Decision | Result |
|---|---|---|
| Initial author baseline | review-ready | 75 normative requirements and 13 open TBD records. |
| Independent review lane one | finding | Two grouped major findings: atomicity and direct TBD dependency trace. |
| Independent review lane two | finding | Two grouped major findings: horizon treatment and cross-stakeholder ownership. |
| Bounded remediation | remediated | 98 atomic requirements, corrected horizon and ownership controls, and direct trace for all 13 TBDs. |
| Independent cross-convergence recheck | pass_with_risk | Atomicity, ownership, TBD dependency, coverage, boundaries, and decision consistency passed. |

Current requirements findings: **zero critical; zero major; zero minor**.

## Decision

REQUIREMENTS status: **pass_with_risk; fixed point reached**.

All 13 TBDs remain open, owned, directly referenced, and promotion-gating.
Their verification is planned; it is not implementation evidence.

Next eligible artifact: `SPECIFICATION_BASELINE.md`, only under a new
assignment. No specification, design, interface, work package, implementation,
operational, procurement, budget, Taxlane-admission, allocation, rate-setting,
official-use, or public-release authority is conferred.
