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

## What BASTION has succeeded at

BASTION has converted a sensitive, expansive subject into a bounded and
mechanically testable research contract. Its controlled source universe covers
98 requirements and 121 specification identities within 298 controlled
identities, including explicit package, interface, design, and code-rigor
ownership.

The current typed boundary checkpoint demonstrates that:

- controlled requirements and sources have explicit, testable allocation;
- missing, stale, malformed, conflicting, or prohibited evidence fails closed;
- readiness, civilian-control, safety, financial, supplier, logistics,
  interoperability, and lifecycle concerns cannot silently substitute for one
  another;
- findings, dissent, holds, and null results remain visible;
- review output cannot become targeting, operational planning, force
  employment, procurement, allocation, rate, or release authority.

This is meaningful executable infrastructure. It provides the trust and
traceability substrate required before readiness, sustainment, acquisition,
supplier, economics, alternatives, and delivery models can make defensible
claims.

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
