# BASTION

**Purchased inventory is not readiness.**

BASTION is Defense 2.0: a way to test whether public resources become
sustainable readiness rather than merely purchased inventory.

A purchased system is not readiness if it cannot be staffed, trained, supplied,
maintained, integrated, repaired, mobilized, or safely used for its declared
public mission. Purchase price is not lifecycle affordability. More inventory
does not fix a supplier, maintenance, workforce, logistics, or interoperability
bottleneck.

BASTION turns that insight into a replayable evidence chain. It keeps readiness
facets, capacity realization, lifecycle resources, alternatives, stakeholder
burden, delivery feasibility, safety floors, and adaptation visible together.
The result is not a force plan or procurement recommendation. It is a disciplined
way to test whether a claimed defense investment or efficiency can survive the
constraints required to make it real.

## The story in one view

```mermaid
flowchart LR
    A[Public mission] --> B[What was purchased?]
    B --> C[What is usable?]
    C --> D[What is the bottleneck?]
    D --> E[What is the lifecycle cost?]
    E --> F[Who carries burden and risk?]
    F --> G[Can a transition be delivered safely?]
    G --> H[Observe and adapt]
    H --> I[Held Taxlane evidence pack]
```

BASTION prevents shortcuts across this chain. Spending does not prove
readiness. Inventory does not prove availability. A projected saving is not
real until transition and lifecycle costs are included. A peer-country
percentage is context, not a target. Research cannot authorize procurement,
operations, allocation, or tax rates.

The transferable principle is simple: measure the bottleneck that limits the
mission, not the input that is easiest to count.

## What works today

BASTION now provides twelve executable, bounded semantic features:

- a fictional, non-operational readiness-package assessment across personnel,
  training, assets, maintenance, spares, logistics, suppliers,
  interoperability, and safety;
- a remedy comparison that tests inventory, maintenance/spares, and
  supplier/logistics responses against the same bottleneck;
- an admitted public, aggregate, unclassified evidence mini-corpus;
- explicit disruption and recovery scenarios;
- a purchase-to-usable-readiness realization chain;
- reconciled acquisition and full-lifecycle resource accounting;
- constrained procurement, sustainment, commonality, and workforce
  alternatives without automatic selection;
- stakeholder incidence across personnel, families, suppliers, communities,
  allies, and taxpayers;
- a transition gate requiring ownership, industrial and workforce capacity,
  milestones, measures, safety, stop conditions, and rollback;
- an adaptive cycle that creates a new immutable successor when observed
  readiness, cost, or supplier conditions cross declared triggers;
- definition-normalized NATO expenditure comparison; and
- a complete but held Taxlane evidence-pack candidate that cannot admit,
  allocate, price, procure, operate, or release itself.

Every example is deterministic and uses public aggregate unclassified evidence
or clearly labelled synthetic, non-operational data.

## What the demonstrations reveal

### Purchased inventory and usable readiness are different

In the fictional Cedar disaster-relief support package, the initial readiness
floor is 60%, constrained by supplier resilience. A bounded support alternative
raises the floor to 78%. This is a transparent minimum across synthetic support
facets—not a probability of mission success.

The remedy comparison sharpens the result. Inventory expansion improves asset
availability but leaves the supplier bottleneck and 60% floor unchanged.
Maintenance and spares raise the floor to 65%; supplier and logistics work
raise it to 80%. BASTION preserves resource use, lead time, transition burden,
safety posture, and the remaining bottleneck without selecting a remedy.

### Funding is not the same as delivered capacity

The realization demonstration traces its fictional funded-capacity index to
60% usable readiness and identifies maintenance as the largest handoff loss.
That makes the missing link visible: authorization and purchase must survive
staffing, training, integration, maintenance, spares, supply, and other
delivery constraints.

### More procurement can cost more without fixing the constraint

One synthetic portfolio alternative adds $20 billion in fictional lifecycle
cost while leaving the 60% readiness floor unchanged. The result is not an
argument against procurement; it is proof that procurement must be evaluated
against the actual bottleneck and full lifecycle.

### Transition and safety determine whether an efficiency is real

The lifecycle feature reconciles acquisition, personnel, operations,
maintenance, spares, infrastructure, transition, and unallocated resources
with zero residual. A candidate does not become transition-testable until it
has an accountable owner, industrial and workforce capacity, milestones,
measures, transition funding, a safety floor, stop conditions, and rollback.

## From question to evidence

| Stage | Question BASTION can answer | Demonstrated feature |
|---|---|---|
| SEM-001 | What constrains a fictional support package? | Nine visible readiness facets and a transparent floor |
| SEM-002 | Which remedy addresses the actual bottleneck? | Three remedy classes with cost, time, burden, safety, and residual constraint |
| SEM-003 | What official public baseline is admitted? | GAO 2026 aggregate weapon-program assessment |
| SEM-004 | Does readiness survive disruption and bounded recovery? | Immutable supplier/workforce/maintenance/spares scenarios |
| SEM-005 | How much funded capacity becomes usable readiness? | Explicit realization losses and limiting handoff |
| SEM-006 | Do lifecycle resources reconcile? | Eight-category, zero-residual resource envelope |
| SEM-007 | Does each alternative improve the floor or merely add cost? | Procurement, sustainment, commonality, and workforce comparison |
| SEM-008 | Who carries workload, transition burden, cost, and risk? | Seven-group, reconciled incidence map |
| SEM-009 | Is the transition deliverable and safe enough to test? | Owner, capacity, milestones, measures, safety, stop, and rollback gate |
| SEM-010 | How does the model learn without rewriting history? | Triggered immutable successor |
| SEM-011 | How does US spending compare under one definition? | Definition-normalized NATO comparison |
| SEM-012 | Is the evidence complete enough for external review? | Held, non-admitted LaneEvidencePack candidate |

The admitted public baseline uses
[GAO's 2026 Weapon Systems Annual Assessment](https://www.gao.gov/products/gao-26-108457).
It records 104 assessed programs, 23 middle-tier acquisitions, at least $49
billion of planned middle-tier investment, technology-maturity findings, and
bounded schedule and total-investment context. These aggregates establish the
public acquisition landscape; they do not establish readiness or support any
operational inference.

The peer feature uses NATO's common expenditure definition and keeps estimates
and category limits explicit. It demonstrates comparison discipline; it does
not infer readiness from spending share or turn a peer value into a US target.

## Start here

Install the pinned Rust 1.95.0 toolchain, then run one bounded example before
reading the full program record. The first command tests a readiness floor; the
second compares remedies against its constraint; the third shows the held
external handoff.

Assess the fictional support package:

```powershell
cargo +1.95.0 run --locked --offline -q -p bastion-readiness-slice --bin bastion -- assess fixtures/synthetic/readiness-package.fixture
```

Compare remedies against the same type of constraint:

```powershell
cargo +1.95.0 run --locked --offline -q -p bastion-remedy-slice --bin bastion-remedies -- compare fixtures/synthetic/readiness-remedies.fixture
```

Replay the culminating held handoff:

```powershell
cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-012 fixtures/synthetic/held-taxlane-pack.fixture
```

Each command emits deterministic JSON with its evidence state, measured result,
limits, holds, and explicit no-authority fields. Exact commands for SEM-003
through SEM-012 are recorded in
[the VTRACE work packages](docs/vtrace/WORK_PACKAGES.md).

## Why the Taxlane handoff is held

BASTION owns defense evidence; it does not own the national allocation or rate
decision. Its SEM-012 artifact packages:

- need, lifecycle, and transition ranges;
- readiness and safety floors;
- stakeholder distribution;
- delivery confidence and controls;
- uncertainty, overlap, dissent, and unresolved holds; and
- explicit proof that projected savings remain unrealized and the pack cannot
  admit or emit itself.

This makes defense evidence useful to Taxlane without allowing BASTION to turn
research into procurement or policy. A future efficiency claim must survive
transition, lifecycle, readiness, resilience, surge, workforce, supplier,
alliance, legal, civilian-control, and safety constraints—and must be
re-evaluated as observed results change.

## Reuse boundary

BASTION is intentionally a specialist defense-readiness evidence product, not a
general procurement, readiness, accounting, or policy library. Its crates,
fixtures, thresholds, scenarios, and held `LaneEvidencePack` candidate are
owned by this non-operational research program. The Taxlane handoff is explicitly
held and non-admitted, so it is not a downstream adoption record; no external
manifest or consumer-owned compatibility test protects a BASTION contract.

Reuse the evidence discipline and no-authority pattern by adaptation, not by
embedding BASTION's domain models. A direct contract should graduate only after
a named external admission process pins a versioned aggregate schema and proves
classification, civilian-control, safety, migration, and rollback boundaries in
the consumer repository.

## Trust and validation

BASTION is governed by VTRACE and reviewed through civilian strategy,
readiness, acquisition, industrial-base, logistics, comptroller, service-member
and family, independent-test, alliance, classification, safety, law, numeracy,
citation, editorial, and stakeholder roles. Missing or stale evidence is held,
never converted to zero. Dissent and null results remain visible.

The repository pins Rust 1.95.0 and uses no third-party Rust dependencies.

```powershell
cargo +1.95.0 test --locked --offline --workspace --all-targets
```

Expected result on this branch: **47 passed, 0 failed**—16 review-kernel tests,
5 tests for each of the first two product slices, and 21 tests for SEM-003
through SEM-012.

## What remains

The current achievement is a complete bounded semantic program, not a validated
real-force model. The next substantive work is to widen and independently
review the public corpus, test aggregate program and industrial-base examples,
validate a non-operational representative scenario, and submit a handoff to an
external admission process. Classified, controlled, exploitable, targeting,
operational-planning, and person-level information remain outside scope.

## Repository guide

- [Product plan](PRODUCT_PLAN.md) — thesis, product shape, and current boundary.
- [Program completion record](docs/program/SEMANTIC_DELIVERIES_003_012.md) —
  feature sequence and final audit evidence.
- [Mission](docs/vtrace/MISSION.md) — purpose, outcomes, exclusions, and authority.
- [Implementation plan](docs/vtrace/IMPLEMENTATION_PLAN.md) — package order and gates.
- [Work packages](docs/vtrace/WORK_PACKAGES.md) — exact feature contracts.
- [Verification](docs/vtrace/VERIFICATION.md) and
  [validation](docs/vtrace/VALIDATION.md) — proof and outcome checks.
- [Role panel](.roles/ROLE.md) — substantive, editorial, assurance, and stakeholder review.

## Boundary

BASTION is a public-aggregate, unclassified research and tooling repository.
It performs no targeting, operational planning, vulnerability exploitation,
real-force assessment, force-employment recommendation, procurement
instruction, official scoring, savings claim, budget allocation, tax-rate
instruction, or public release. Its admitted public corpus is deliberately
small; its readiness, scenario, accounting, alternative, transition, and
handoff demonstrations are synthetic and non-operational.

## License

BASTION uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.
