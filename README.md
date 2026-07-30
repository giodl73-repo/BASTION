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
| Product status | No readiness model, acquisition model, public aggregate corpus, savings result, portfolio recommendation, or Taxlane handoff has been accepted yet. |

## Features available now

BASTION currently has two usable cross-cutting features:

1. A Rust library can evaluate a fully constructed research-review packet and
   return **pass recommended**, **hold**, or **reject**, while preserving
   findings, conflicts, dissent, and accepted history.
2. A developer-facing boundary suite can verify that a proposed evidence
   implementation retains its required source, control, hold, review, and
   no-authority obligations.

These are review and implementation-validation features. They do not yet
constitute a defense-readiness product.

## Defense feature scorecard

| User-visible capability | Status |
|---|---|
| Load a public aggregate, unclassified defense corpus | **Not built** |
| Define a fictional or aggregate mission/readiness package | **Not built** |
| Ingest a sample formation, unit, asset, program, supplier, or inventory | **Not built** |
| Measure staffing, training, availability, maintenance, repair, or supply | **Not built** |
| Measure acquisition cycle time, supplier resilience, or interoperability | **Not built** |
| Calculate a readiness score, gap, tail, or null result | **Not built** |
| Run surge, disruption, mobilization, transition, or lifecycle scenarios | **Not built** |
| Compare constrained portfolios or delivery alternatives | **Not built** |
| Estimate lifecycle cost, realizable savings, or transition risk | **Not built** |
| Produce a held TAXLANE evidence handoff | **Not built** |

**Bottom line:** BASTION has review and test-control features, but it has **zero
completed defense semantic features**. It cannot currently validate a sample
army or formation as a product feature. The earlier Cedar illustration was a
design example, not an executed capability, and has therefore been removed.

## First feature milestone

The first substantive milestone should be a safe-synthetic readiness-package
feature with a real CLI or library input and reproducible output. It should:

- ingest a fictional, non-operational formation with staffing, asset,
  maintenance, spares, supplier, logistics, and interoperability facets;
- preserve missing, stale, incompatible, and non-composable facets;
- produce an assessable or held readiness result with explicit blockers;
- compare no-change and bounded support alternatives without deployment or
  targeting advice; and
- emit a source-labelled artifact that can be independently replayed.

Until that executes, the repository should be described as a validated
foundation rather than a working readiness analyzer.

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

## Verify the working checkpoint

The repository pins Rust 1.95.0 and has no third-party Rust dependencies.

```powershell
git switch codex/wp-tst-001-amend
cargo +1.95.0 test --locked --offline --workspace --all-targets
```

Expected result at the parked typed-boundary checkpoint: **145 passed, 0
failed**. This test result does not substitute for the still-pending canonical
16-mode evidence publication and exit review.

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

BASTION is a foundation-stage research and tooling repository. It performs no
targeting, operational planning, vulnerability exploitation, force-employment
recommendation, official scoring, procurement instruction, savings claim,
budget allocation, or public release. The repository demonstrates a strong
typed foundation; the semantic product and flagship aggregate readiness
scenario remain future work.
