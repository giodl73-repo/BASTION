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
| Product status | One bounded semantic feature is executable: a safe-synthetic readiness package can be assessed across nine support facets and compared with one bounded alternative. |

## Features available now

BASTION now has one defense-readiness feature in addition to two cross-cutting
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

## Defense feature scorecard

| User-visible capability | Status |
|---|---|
| Load a public aggregate, unclassified defense corpus | **Not built** |
| Define one fictional, non-operational mission/readiness package | **Built** |
| Ingest a sample formation, unit, asset, program, supplier, or inventory | **Not built** |
| Measure personnel, training, asset, maintenance, spares, logistics, and safety facets | **Built in first slice** |
| Measure supplier resilience and interoperability | **Built in first slice** |
| Calculate a transparent synthetic readiness floor or held result | **Built in first slice** |
| Run surge, disruption, mobilization, transition, or lifecycle scenarios | **Not built** |
| Compare one bounded support alternative | **Built in first slice** |
| Estimate lifecycle cost, realizable savings, or transition risk | **Not built** |
| Produce a held TAXLANE evidence handoff | **Not built** |

**Bottom line:** BASTION has completed exactly one bounded semantic slice. It
can identify the limiting facet in a fictional readiness package and show how
a specified support alternative changes the result. It cannot validate a real
force, predict mission success, plan a deployment, recommend procurement, or
set spending, savings, or tax rates.

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

Expected result on this branch: **21 passed, 0 failed**—16 review-kernel tests
and 5 semantic-slice tests. The separate parked typed-boundary checkpoint and
its unfinished evidence acceptance are not claimed by this branch.

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

BASTION is an early semantic research and tooling repository. It performs no
targeting, operational planning, vulnerability exploitation, force-employment
recommendation, official scoring, procurement instruction, savings claim,
budget allocation, or public release. Its achieved result is limited to the
included fictional package; a public aggregate corpus, real-force analysis,
lifecycle cost model, and flagship real-world scenario remain future work.
