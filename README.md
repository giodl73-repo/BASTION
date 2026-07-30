# BASTION

**Defense 2.0 — measure whether public resources produce sustainable mission
readiness, not merely purchased inventory.**

A purchased asset is not readiness if it cannot be staffed, trained, supplied,
maintained, integrated, repaired, mobilized, or safely employed for its declared
public mission. BASTION treats defense as a delivery network spanning people,
acquisition, maintenance, suppliers, logistics, interoperability, resilience,
and lifecycle cost.

## Baseline summary

| Area | Established baseline |
|---|---|
| Public question | Does public spending produce maintainable, supplied, staffed, interoperable, resilient readiness over the full lifecycle? |
| Evidence boundary | Public, aggregate, unclassified, source-labelled or clearly synthetic evidence only. |
| Systems baseline | Mission, CONOPS, requirements, specification, architecture, interfaces, design, package boundaries, verification, validation, and code-rigor controls are defined and traceable. |
| Working Rust | `bastion-review` provides a deterministic review kernel; `bastion-boundary-tests` implements typed control, source-allocation, contract, property, model, adversarial, hold, static, and no-authority checks. |
| Verified checkpoint | The parked `codex/wp-tst-001-amend` checkpoint passes 145 Rust tests across nine test targets with zero third-party Rust dependencies. |
| Evidence status | The typed implementation exists and is committed; its canonical 16-mode evidence publication and independent exit acceptance remain unfinished. |
| Product status | Twelve bounded semantic slices are executable, spanning readiness and remedies through admitted public aggregates, stress, realization, accounting, alternatives, distribution, transition, adaptation, peers, and a held Taxlane candidate. |

## Features available now

BASTION now has twelve defense-readiness features in addition to two cross-cutting
review controls:

1. The `bastion assess` command evaluates a fictional, non-operational support
   package across personnel, training, assets, maintenance, spares, logistics,
   suppliers, interoperability, and safety. It exposes the weakest supported
   facet and compares one bounded support alternative.
2. A Rust library can evaluate a fully constructed research-review packet and
   return **pass recommended**, **hold**, or **reject**, while preserving
   findings, conflicts, dissent, and accepted history.
3. A developer-facing boundary suite can verify that a proposed evidence
   implementation retains its required source, control, hold, review, and
   no-authority obligations.

In the included fictional Cedar disaster-relief support package, the baseline
readiness floor is 60%, constrained by supplier resilience, with five concerns.
The bounded alternative improves those five facets and raises the floor to 78%
with no remaining threshold concerns. The floor is a transparent minimum over
synthetic inputs—not a probability of mission success or deployment advice.

The `bastion-remedies compare` command now tests three remedy classes against
the same kind of fictional constraint. Inventory expansion raises asset
availability but leaves supplier resilience—and the 60% readiness floor—
unchanged. The maintenance/spares case raises the floor to 65%; the
supplier/logistics case raises it to 80%. Each result retains its synthetic
resource index, lead time, transition burden, safety posture, and remaining
bottleneck. BASTION deliberately does not select or recommend a remedy.

## Defense feature scorecard

| User-visible capability | Status |
|---|---|
| Load a source-labelled public aggregate, unclassified mini-corpus | **Built in SEM-003** |
| Define one fictional, non-operational mission/readiness package | **Built** |
| Ingest a sample formation, unit, asset, program, supplier, or inventory | **Not built** |
| Measure personnel, training, asset, maintenance, spares, logistics, and safety facets | **Built in first slice** |
| Measure supplier resilience and interoperability | **Built in first slice** |
| Calculate a transparent synthetic readiness floor or held result | **Built in first slice** |
| Run bounded disruption, recovery, and lifecycle scenarios | **Built in SEM-004–006** |
| Compare inventory, maintenance/spares, and supplier/logistics remedy classes | **Built in second slice** |
| Retain synthetic resource, lead-time, transition-burden, and safety limits | **Built in second slice** |
| Compare lifecycle resources and transition risk without claiming realized savings | **Built in SEM-006–009** |
| Run adaptive successors and normalized NATO expenditure comparisons | **Built in SEM-010–011** |
| Produce a held, non-admitted TAXLANE evidence candidate | **Built in SEM-012** |

**Bottom line:** BASTION has completed exactly twelve bounded semantic slices. It
can identify the limiting facet in a fictional readiness package and compare
how three remedy classes change that constraint while keeping their principal
burdens visible. It cannot validate a real force, predict mission success,
plan a deployment, recommend procurement, or set spending, savings, or tax
rates.

## First feature milestone achieved

The safe-synthetic readiness-package feature now:

- ingest a fictional, non-operational formation with staffing, asset,
  maintenance, spares, supplier, logistics, and interoperability facets;
- preserve missing, stale, incompatible, and non-composable facets;
- produce an assessable or held readiness result with explicit blockers;
- compare no-change and bounded support alternatives without deployment or
  targeting advice; and
- emit a source-labelled artifact that can be independently replayed.

A user can replay the included case with:

```powershell
cargo +1.95.0 run --locked --offline -q -p bastion-readiness-slice --bin bastion -- assess fixtures/synthetic/readiness-package.fixture
```

The command emits deterministic JSON with every facet, threshold, posture,
bottleneck, comparison, and explicit no-authority flags.

## Second feature milestone achieved

The safe-synthetic remedy comparison:

- compares inventory expansion, maintenance/spares, and supplier/logistics;
- keeps all nine readiness facets visible for baseline and every remedy;
- reports readiness-floor change and remaining bottleneck;
- retains resource index, lead time, and transition burden separately;
- holds remedies with missing/stale limits or a safety regression; and
- emits no selected remedy, procurement instruction, or monetized savings.

Replay it with:

```powershell
cargo +1.95.0 run --locked --offline -q -p bastion-remedy-slice --bin bastion-remedies -- compare fixtures/synthetic/readiness-remedies.fixture
```

The result demonstrates why purchased inventory is not automatically usable
readiness: strengthening a non-limiting facet can consume resources while the
actual constraint remains untouched.

## SEM-003 through SEM-012 achieved

| Slice | Executed feature achievement |
|---|---|
| SEM-003 | Admits GAO-26-108457 public aggregate evidence covering 104 programs, 23 middle-tier acquisitions, technology-maturity findings, and bounded investment/schedule context without readiness or operational inference. |
| SEM-004 | Compares baseline, disruption, and recovery supplier/workforce/maintenance/spares floors with an immutable successor and explicit recovery horizon. |
| SEM-005 | Traces a fictional funded-capacity index to 60% usable readiness and identifies maintenance as the largest handoff loss. |
| SEM-006 | Reconciles acquisition, personnel, operations, maintenance, spares, infrastructure, transition, and unallocated lifecycle resources with zero residual. |
| SEM-007 | Shows additional procurement adds $20 billion of fictional lifecycle cost while leaving the 60% readiness floor unchanged; no alternative is selected. |
| SEM-008 | Maps workload, transition burden, and reconciled cost incidence across seven stakeholder groups without ranking human worth. |
| SEM-009 | Requires owner, industrial/workforce capacity, milestones, measures, cadence, transition cost, safety floor, stop, and rollback before transition-testable research. |
| SEM-010 | Converts observed readiness, lifecycle-cost, and supplier triggers into exactly one immutable successor without in-place retry. |
| SEM-011 | Compares 2025 estimated US and NATO peer expenditure using NATO's definition while stating that spending share does not prove readiness. |
| SEM-012 | Builds a complete held LaneEvidencePack candidate with lifecycle/transition ranges, floors, distribution, uncertainty, overlap, dissent, and holds; savings remain unrealized and the pack cannot self-admit. |

Run any program slice with:

```powershell
cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-012 fixtures/synthetic/held-taxlane-pack.fixture
```

Replace `sem-012` and the fixture with the exact command recorded in
`docs/vtrace/WP_SEM_003.md` through `WP_SEM_012.md`.

## What the finished system is intended to show

Once the semantic packages and public aggregate corpus are implemented,
BASTION is designed to support questions such as:

- Is a purchased capability actually available after staffing, training,
  maintenance, spare-parts, supplier, and integration constraints?
- Would maintenance, inventory, common-platform, workforce, or acquisition
  reform improve readiness more than additional procurement?
- Where do lifecycle cost, repair-time tails, supplier concentration, or
  logistics constraints create persistent delivery risk?
- Which constrained alternatives improve resilience and readiness while
  preserving civilian control, safety, alliance, and transition floors?

These are intended examples, not current findings or operational advice.

## Verify the working feature branch

The repository pins Rust 1.95.0 and has no third-party Rust dependencies.

```powershell
cargo +1.95.0 test --locked --offline --workspace --all-targets
```

Expected result on this branch: **47 passed, 0 failed**—16 review-kernel tests,
5 tests for each of the first two slices, and 21 program tests for SEM-003
through SEM-012. The separate parked typed-boundary
checkpoint and its unfinished evidence acceptance are not claimed by this
branch.

## Repository map

- [Product plan](PRODUCT_PLAN.md) — thesis, intended product shape, and current boundary.
- [Mission](docs/vtrace/MISSION.md) — purpose, outcomes, exclusions, and authority.
- [Implementation plan](docs/vtrace/IMPLEMENTATION_PLAN.md) — package order and entry/exit gates.
- [Work packages](docs/vtrace/WORK_PACKAGES.md) — exact ownership and dependency allocation.
- [Verification](docs/vtrace/VERIFICATION.md) and [validation](docs/vtrace/VALIDATION.md) — required proof and outcome checks.
- [Role panel](.roles/ROLE.md) — readiness, acquisition, logistics, finance, civilian-control, safety, editorial, and stakeholder review.

## Relationship to TAXLANE

BASTION is the defense evidence owner. It may eventually prepare a held,
source-labelled handoff describing verified needs, efficiencies, lifecycle
effects, delivery confidence, and risks. It does not admit that evidence,
allocate a budget, or set a tax rate; those decisions remain outside this
repository.

## Boundary

BASTION is a bounded semantic research and tooling repository. It performs no
targeting, operational planning, vulnerability exploitation, force-employment
recommendation, official scoring, procurement instruction, savings claim,
budget allocation, or public release. Its achieved public evidence is a
deliberately small aggregate mini-corpus; readiness, accounting, alternatives,
transition, and handoff examples remain safe-synthetic and non-operational. A
general corpus, real-force analysis, admitted Taxlane handoff, and flagship
real-world scenario remain future work.
