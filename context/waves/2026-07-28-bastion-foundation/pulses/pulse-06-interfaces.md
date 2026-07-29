# Pulse 06 — Encoding-neutral interface-controls fixed point

Date: 2026-07-28
Assignment: `ASG-BASTION-INTERFACES-001`
Writer lease: exclusive to the BASTION interfaces author

Controlled fixed `docs/vtrace/PACKAGE_BOUNDARIES.md` input SHA-256:
`43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695`

Controlled fixed `docs/vtrace/ARCHITECTURE.md` input SHA-256:
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`

Fixed `docs/vtrace/INTERFACES.md` SHA-256:
`18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882`

## Objective

Define an encoding-neutral interface-control baseline for exactly the thirteen
fixed BASTION contracts without choosing a Rust/API/schema/file/transport
representation, closing an inherited unknown, creating implementation, or
changing the fixed architecture or package boundaries.

## Required inputs read

- TRACKER `CONTEXT.md`; BASTION `AGENTS.md`, `README.md`, `PRODUCT_PLAN.md`,
  `CLAUDE.md`, and the foundation-wave context;
- complete fixed source chain: `MISSION.md`, `CONOPS.md`, `REQUIREMENTS.md`,
  `SPECIFICATION_BASELINE.md`, `ARCHITECTURE.md`, `PACKAGE_BOUNDARIES.md`, and
  Pulses 01 through 05;
- `.roles/ROLE.md` and all 21 parliament, stakeholder, editorial, assurance,
  and methodology role files;
- VTRACE adoption `INTERFACES.md` template, canonical interface example,
  encoding-model, process, staged-execution, and gate-checklist guidance; and
- TRACKER `infrastructure-2-0-common-pattern.md`.

The frozen package-boundary and architecture digests matched exactly before
authoring. No upstream controlled file was changed.

## Baseline produced

- Created `docs/vtrace/INTERFACES.md` with an exact 13-row inventory and one
  complete detailed record for every fixed `CONTRACT-*` identity.
- Recorded semantic version/status, physical custodian, logical producer,
  exact direct/version-qualified consumers, and named unknown blockers for
  every contract. Co-location cannot widen a consumer set.
- Required each record to define purpose/preconditions, required/conditional/
  prohibited semantic payload groups, identity/digest/context and predecessor/
  successor behavior, typed postures, fail-closed error/invalidation,
  accountable owner/concurrence/independent assurance, classification/OPSEC,
  civilian-control/readiness/harm constraints, compatibility/breaking
  triggers, and planned unexecuted fixtures.
- Limited the possible common envelope to non-domain identity, custody,
  provenance, digest/context, posture, owner/review, invalidation, and
  supersession metadata. It owns no defense or fiscal meaning.
- Preserved mandatory exact-output source/security re-admission for every
  product/material producer, including SRC transformations and AUTH; AUTH
  cannot bypass the gate, and REL emits nothing.
- Preserved the immutable acyclic sequence `preliminary ECO[n] → mandatory
  DEL[n] → custody/security/review → final ECO[n+1] → ADP[n+1]`, including
  predecessor bonds, later-successor feedback, mandatory repeated delivery,
  and fail-closed cycle/staleness/bypass/mutation cases.
- Preserved the finite HND gate: accepted final ECO, ADP, DEL bond, gates and
  semantic concurrences form one immutable candidate; the exact bundle receives
  source/security admission and independent TEST. Ordinary findings return to
  the accountable producer. A successful `IF-TERM-001` decision/receipt is not
  an `ARC-HND-001` semantic consumer or input; it advances the unchanged exact
  admitted HND bundle directly to the external Taxlane handoff boundary.
- Exempted only that minimal non-product terminal governance receipt from
  universal producer-output SOURCE re-admission and independent re-review. It
  contains only unchanged admitted bundle identity/posture, reviewer identity,
  decision, date, and dissent. Any product/material/context/digest change
  invalidates it and restarts bundle freeze, SOURCE admission, and independent
  TEST, so the exemption cannot create recursion or carry product content.
- Kept Taxlane external and exclusive for admission, combination, allocation,
  rebalance, and rate testing/setting. BASTION emits no pack while the handoff
  unknown is open and infers no external state.
- Kept generated output derivative-only and prohibited HND/Taxlane use of a
  generated mirror as source.
- Defined semantic compatibility and upstream reopen/downstream invalidation
  rules. A representation that preserves bytes but changes controlled meaning
  is incompatible.
- Required at least three independent domain proofs and a fixed reviewed
  contract before any common/shared implementation extraction; defense
  semantics cannot enter the common envelope.
- Added a planned interface-fixture register without selecting fixture paths,
  encodings, commands, generators, or expected byte sequences.
- Routed all 21 substantive role files exactly once while preserving semantic
  ownership, dissent, reviewer independence, both conjunctive assurance gates,
  and no self-approval.

## Author structural validation

Author inspection found:

| Check | Result |
|---|---|
| Controlled package digest | Exact match |
| Controlled architecture digest | Exact match |
| Interface inventory | 13 rows; 13 unique IDs; exact equality to the fixed contract set |
| Detailed interface records | 13 records; 13 unique IDs; exact equality to the inventory |
| Per-record required controls | 13 each for identity/custody/consumers/blockers, purpose/preconditions, required, conditional, prohibited, version, error/invalidation, owner/assurance, compatibility, and planned fixtures |
| Unknown preservation | 13 unique `SPEC-UNK-*` IDs; zero missing or additional unknowns; none closed |
| Role routing | 21 rows; 21 unique repository role paths; exact equality to the substantive role-file set |
| Protected sets | 98 SPEC, 98 planned VER, 13 contracts, 13 components, 10 nonfunctional constraints, and 13 unknowns preserved by reference |
| Source/security flow | SRC transformations and AUTH included; every product/material producer output re-admitted; only the unchanged minimal non-product terminal receipt is exempt; any product/material/context/digest change restarts freeze/admission/review; REL no-output preserved |
| ECO/DEL/ADP order | Preliminary/final distinction, mandatory DEL, custody/security/review, predecessor/successor, and negative cases explicit |
| TEST consumer split | Ordinary findings return to the exact accountable producer; successful `IF-TERM-001` receipt is consumed only by the external handoff gate and never by `ARC-HND-001` |
| HND/Taxlane boundary | Unchanged exact admitted HND bundle advances directly at the external gate; terminal receipt is non-product and non-recursive; no pack while held; no inferred admission; external/exclusive Taxlane authority explicit |
| Representation deferral | No concrete type, trait, function, schema/file/config/event name, serialization, transport, CLI, storage, algorithm, threshold, dependency, runtime, or deployment selected |
| Whitespace | Zero trailing-whitespace lines in `INTERFACES.md` and this pulse at final inspection |

These are author-side document and structural checks. They are not executed
interface verification, role decisions, assurance acceptance, implementation
evidence, or a fixed-point decision.

## Digest-bound review, remediation, and convergence sequence

| Stage | Frozen subject | Disposition |
|---|---|---|
| Initial author baseline | `INTERFACES.md` SHA-256 `d967b5c7d63bc16f6aaf08d31c30989ca13487da785dfca77eb7b534239fb9dc` | Review-ready author output only; no acceptance or fixed point claimed. |
| Independent substance and assurance review | Initial author baseline | Found the actionable `IF-TERM-001` ambiguity: ordinary TEST findings and the successful terminal decision/receipt were not separated exactly, `ARC-HND-001` appeared to consume the terminal receipt, and the universal producer-output SOURCE rule could create terminal re-admission/re-review recursion. Advancement remained held. |
| Bounded remediation 001 | Successor `INTERFACES.md` SHA-256 `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` | Ordinary findings return to the exact accountable producer; the successful terminal receipt is a minimal non-product governance sidecar consumed only by the external handoff gate, never by `ARC-HND-001`; the unchanged admitted HND bundle advances directly to the external Taxlane handoff boundary; any product/material/context/digest change restarts freeze, SOURCE admission, and independent TEST. Author validation only. |
| Independent convergence review A | Remediation-001 successor | `pass_with_risk`; zero current actionable findings; `IF-TERM-001`, exact consumers, no-recursion behavior, protected sets, holds, role routing, safety/authority gates, compatibility, and representation deferral passed. |
| Independent convergence review B | Remediation-001 successor | `pass_with_risk`; independently confirmed zero current actionable findings and closure of `IF-TERM-001`; exact 13-contract, 13-hold, and 21-role sets, unexecuted fixtures, finite terminal receipt, external Taxlane authority, and no design leakage passed. |
| Repo-local fixed-point decision | Remediation-001 successor | **`pass_with_risk`; INTERFACES fixed point declared for the exact frozen digest above.** Residual risk is carried by all 13 inherited holds and every planned but unexecuted fixture; no hold is closed and no later-stage evidence is inferred. |

The two convergence reviews are independent digest-bound evidence. Their
zero-finding decisions close the `IF-TERM-001` finding for this exact frozen
subject. They do not execute a fixture, select a representation, accept a
Taxlane mapping, grant Taxlane admission, or authorize implementation or
release. Any change to `INTERFACES.md` invalidates this decision and requires
successor review under its compatibility and change-control rules.

## Planned verification and command posture

The baseline plans positive, null, reviewed-N/A, hold, rejection, stale,
dangerous-composition, category-loss, incompatible, cycle, predecessor,
self-approval, route-around, and false-authority fixture branches. They remain
semantic plans only.

L0, L1, and L2 commands are unavailable as accepted interface evidence and
were not executed. No Cargo workspace, Rust source, interface representation,
schema, fixture inventory, test target, generator, or implementation exists.
No command, path, expected encoding, or output was invented.

## Scope, safety, and authority posture

- Public, aggregate, unclassified, non-operational internal research only.
- No classified information, CUI, person-level service data, sensitive
  operational data, targeting, operational planning, or exploitable
  vulnerability content is admitted, retained, derived, or emitted.
- Classification & Operational Security and Civilian Control, Law, Safety &
  Readiness remain independent conjunctive assurance gates. Safety, readiness,
  resilience, surge, recovery, alliance, distribution, and harm floors cannot
  be traded away by fiscal or delivery posture.
- All 13 inherited `SPEC-UNK-*` controls remain exact, open, non-defaulted, and
  promotion-gating.
- `CONTRACT-REL-001` remains closed and output-free. No review, test,
  generated artifact, handoff, or fixed point implies publication.
- Taxlane alone owns admission and cross-lane fiscal authority. No interface
  record creates an allocation, rebalance, rate, budget, official, or release
  decision.

## Fixed-point limitations and blockers

- Interface status is **repo-local fixed point with `pass_with_risk`** for the
  exact frozen digest. The risk qualification records 13 inherited open holds
  and wholly unexecuted planned fixtures; it does not weaken a gate.
- Exact source/claim schemas, security methods, readiness measures/floors,
  quantitative methods, acquisition/logistics/alliance/distribution methods,
  accounting semantics, review mechanics, delivery evidence, HND mapping, and
  release controls remain held by their named unknowns.
- The interface document selects semantic groups and invariants only. Exact
  field names, cardinality, types, errors, encoding, compatibility fixtures,
  and all implementation mechanics remain deferred.
- There is no accepted corpus, delivery evidence, realization evidence,
  Taxlane mapping, interface artifact, implementation, test harness,
  generated artifact, or release plan.

Current author blocker count: **zero**. Inherited promotion holds: **13**.

`DESIGN.md` is the next VTRACE deliverable eligible for consideration only
under a new explicit assignment. This fixed point does not itself create,
start, authorize, or accept DESIGN, verification, a work package, any
representation, or implementation.

## Disposition

`INTERFACES.md` at SHA-256
`18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882`
is the **repo-local encoding-neutral INTERFACES fixed point with
`pass_with_risk`**. It preserves exactly 13 contracts, 13 inherited open holds,
21 substantive role routes, and wholly unexecuted planned fixtures. The finite
terminal receipt remains minimal and non-product; it cannot enter HND, recurse
through SOURCE/TEST, imply Taxlane admission, or survive a product/material/
context/digest change.

No `DESIGN.md`, verification plan, work package, package, crate, Cargo/Rust
work, interface representation, schema, API, CLI, file format, transport,
fixture, test, generator, corpus, implementation, Taxlane action, official
action, commit, push, remote mutation, or public release was created,
performed, or authorized.
