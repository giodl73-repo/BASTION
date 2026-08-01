# BASTION VERDICT capability assessment

## Decision and scored object

- Assessment: `bastion-program-capability:2026-07-31`
- Decision supported: identify the next evidence-producing product slice.
- Object class: `program_capability`
- Object: BASTION at commit `cc81c61cd3e097449db072abac99d1613a2b2101`
- Perimeter: public, aggregate, unclassified, non-operational Defense 2.0 research and tooling.
- Not scored: US force readiness, a procurement or force-structure candidate,
  or a fiscal package.

Scale: `0 missing; 1 designed/partial; 2 executable/bounded; 3 demonstrated`.

## Dimension evidence

| ID | Dimension | Score | Evidence | Strength | Principal hold |
|---|---|---:|---|---|---|
| V | Value | 2 | `docs/vtrace/WP_SEM_006.md`; `docs/vtrace/WP_SEM_007.md`; `docs/vtrace/WP_SEM_012.md` | Reconciles lifecycle resources and compares constrained alternatives without equating purchase price with value. | Real candidate lifecycle prices, transition costs, and realized savings remain absent. |
| E | Effectiveness | 2 | `docs/vtrace/WP_SEM_001.md` through `WP_SEM_005.md`; executable SEM demonstrations | Distinguishes purchased inventory, usable readiness, bottlenecks, remedies, and realization. | Readiness and remedy results are synthetic rather than observed for a real public program. |
| R | Resilience | 2 | `docs/vtrace/WP_SEM_004.md`; supplier, workforce, maintenance, spares, disruption, and recovery facets | Stress and recovery affect a bounded readiness floor. | No real-force or program recovery observation is admitted. |
| D | Deliverability | 2 | `docs/vtrace/WP_SEM_009.md`; `docs/vtrace/WP_SEM_012.md` | Owner, capacity, milestones, measures, transition funding, safety, stop, and rollback gates are executable. | Enactment, appropriation, program capacity, and candidate-specific delivery remain held. |
| I | Iteration | 2 | `docs/vtrace/WP_SEM_010.md` | Observations can trigger an immutable analytical successor rather than rewriting history. | No institution has demonstrated operational response, outcome learning, or fiscal rebalancing. |
| C | Coverage and fair access | 1 | `docs/vtrace/WP_SEM_008.md`; service-member, family, workforce, supplier, community, ally, and taxpayer roles | Stakeholder incidence is explicit and reconciled in demonstrations. | Real distribution, personnel/family burden, small-supplier, installation-community, and allied incidence are not observed. |
| T | Trust | 3 | `CLAUDE.md`; `.roles/`; `docs/vtrace/`; public GAO/NATO source boundaries; 47-test validation record | Strong public/unclassified boundary, source labels, null handling, role gates, and no-authority outputs are demonstrated. | Trust maturity does not validate real readiness or authorize a defense decision. |

Total: **14/21**. This reproduces the TRACKER pilot; adoption creates no score
increase.

## Iteration evidence

| Loop | State | Evidence or hold |
|---|---|---|
| Analytical refresh | demonstrated for program capability | Triggered immutable successors are executable. |
| Operational response | held | No force, program, depot, supplier, or acquisition authority has acted from BASTION. |
| Outcome learning | held | No post-change real readiness, burden, or safety outcome exists. |
| Fiscal rebalancing | held | The SEM-012 Taxlane pack remains non-admitted. |

## Hard floors and claims

Civilian control, law, classification/security, personnel safety, readiness,
resilience, surge, alliance obligations, and service-member/family burden are
applicable. A failed or unresolved floor blocks promotion regardless of total.

This assessment allows the claim that BASTION has a candidate-capable semantic
program. It does not authorize force planning, procurement, operations,
allocation, savings, rates, Taxlane admission, or public release.

## Next evidence-producing action

Apply the existing chain to one public, aggregate, non-operational program
example with observed readiness proxies, actual lifecycle costs, distribution
evidence, and a later observed update. Preserve every classification,
operational-security, law, safety, and civilian-control boundary.

## `.roles` fixed point

The full parliament, editorial, assurance, and stakeholder panel retains the
14/21 result. The defense-comptroller and numeracy lenses prohibit translating
synthetic lifecycle values into savings; readiness and independent-test lenses
hold real effects; classification and civilian-control assurance retain the
non-operational boundary; service-member, family, supplier, community, ally,
and taxpayer lenses retain the coverage hold. No critical or major actionable
documentation finding remains.

## Validation

- Arithmetic: `2 + 2 + 2 + 2 + 2 + 1 + 3 = 14`.
- Repository whitespace: `git diff --check`.
- Expected implementation baseline: 47 workspace tests; this documentation-only
  adoption changes no executable behavior.
