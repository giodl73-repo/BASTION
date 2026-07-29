# Pulse 05 — Planned package boundaries

Date: 2026-07-28
Assignment: `ASG-BASTION-PACKAGE-BOUNDARIES-001`
Writer lease: exclusive to the BASTION package-boundaries author

Controlled fixed `docs/vtrace/ARCHITECTURE.md` input SHA-256:
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`

Repository-local fixed-point `docs/vtrace/PACKAGE_BOUNDARIES.md` SHA-256:
`43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695`

## Objective

Translate the fixed logical architecture into a minimal, reviewable planned
physical ownership and dependency baseline without creating a workspace,
package, crate, test, fixture, generated artifact, interface, corpus, or
implementation.

## Inputs read

- TRACKER `CONTEXT.md` and BASTION `AGENTS.md`, `CLAUDE.md`, `README.md`,
  `PRODUCT_PLAN.md`, foundation-wave ledger, and all prior pulses;
- complete BASTION `MISSION.md`, `CONOPS.md`, `REQUIREMENTS.md`,
  `SPECIFICATION_BASELINE.md`, and fixed `ARCHITECTURE.md` source chain;
- `.roles/ROLE.md` and all 21 parliament, stakeholder, editorial, assurance,
  and methodology role files;
- VTRACE adoption `PACKAGE_BOUNDARIES.md` template and package-boundary
  framework guidance;
- TRACKER `infrastructure-2-0-common-pattern.md`; and
- existing VTRACE and BISECT package-boundary baselines as non-authoritative
  structure references only.

The controlled architecture digest matched exactly before authoring.

## Planned baseline produced

- Defined 12 minimal planned physical boundaries: one workspace-control
  surface; six product libraries; one thin BASTION-only runner; existing
  governance/docs; one non-product integration-test crate; one fixture
  boundary; and one generated-output custody boundary.
- Rejected a one-crate-per-component interpretation. Grouped the five domain
  analysis components in one separately owned-module crate and grouped ECO,
  DEL, and ADP in one pathway crate to enforce their version-order invariant.
- Kept source/security custody, civilian authority, independent review, held
  handoff, and closed release responsibilities in separate trust boundaries.
- Allocated all 13 `ARC-*` logical components exactly once to an accountable
  physical boundary, with zero missing or duplicate allocations.
- Placed all 13 controlled `CONTRACT-*` identities exactly once in the public
  interface column. No other public-interface identifier, field, encoding,
  signature, cardinality, transport, or compatibility rule was selected.
- Preserved all 98 SPEC allocations, 13 contracts, 10 nonfunctional
  constraints, 13 exact unknown holds, and 98 planned verification identities
  by reference without changing or closing a protected set.
- Allocated all 21 role files to cross-boundary participation surfaces while
  retaining semantic owners, reviewer independence, dissent, and both
  conjunctive assurance gates.
- Defined an acyclic star-shaped planned build graph: six sibling product
  libraries feed a thin runner; the test-only crate depends downward on the
  runner/libraries and fixtures; no product depends on a sibling, runner, test,
  fixture, generated output, or docs boundary.
- Separately defined runtime/control flow so every output, including AUTH and
  custody transformations, passes exact-context security re-admission before
  use and review.
- Preserved immutable preliminary `ECO[n] → DEL[n] → custody/security/review
  → final ECO[n+1] → ADP[n+1]` ordering, successor-only feedback, mandatory
  delivery, and same-version-cycle rejection.
- Preserved the finite handoff gate: immutable held bundle, exact bundle
  security posture, independent minimal non-reconstructive terminal decision,
  and restart on new product content.
- Defined generated-artifact custody and a no-hand-edit rule while explicitly
  leaving every generator, command, representation, storage, and retention
  technology unavailable and unselected.
- Closed `BAS-PB-MIN-001` by limiting `PB-GEN-001` consumers to read-only
  inspection by `PB-TST-001` and `PB-REV-001`. Accepted producer, handoff, or
  review output flows into generated custody only after production; no
  product, runner, handoff, Taxlane, or release boundary consumes it as source.
  In particular, `PB-HND-001` produces the held candidate from accepted inputs
  before any optional generated mirror.
- Defined work-package touch/integration rules, owner/change-control triggers,
  rejected splits, risks, and the ban on product dependencies on tests or
  fixtures.
- Rejected a premature generic Infrastructure 2.0 shared crate. Extraction
  remains gated on at least three independent domain proofs and a reviewed
  semantic change.

## Planned validation posture

Rust and docs L0/L1/L2 commands are recorded as prospective commands only.
They are unavailable as accepted implementation evidence and were not executed
because no Cargo workspace, crate, Rust file, test target, accepted fixture
inventory, or generator exists. The baseline selects no toolchain version,
dependency library, feature, unsafe policy, API, schema, serialization,
storage, algorithm, runtime, or deployment.

Author structural checks found:

- fixed architecture digest: exact match;
- package-boundary rows: 12 unique planned boundaries;
- logical-component allocation: 13 rows, 13 unique `ARC-*` IDs, zero missing
  or duplicate allocations;
- public contract placement: 13 references, 13 unique controlled contract IDs;
- protected unknown register: 13 unique `SPEC-UNK-*` IDs, with exact dependent
  sets retained by architecture/specification reference;
- role participation: 21 rows, 21 unique role-file paths, and every repo role
  file represented exactly once;
- build dependency direction: acyclic by the declared layer/adjacency rules,
  with all unspecified edges forbidden;
- generated custody direction: accepted producer/`PB-HND-001`/`PB-REV-001`
  output → `PB-GEN-001` → read-only `PB-TST-001`/`PB-REV-001` inspection, with
  zero product, runner, handoff, Taxlane, or release consumers;
- held-candidate direction: `PB-HND-001` produces from accepted inputs before
  any optional generated mirror, and neither `PB-HND-001` nor Taxlane consumes
  that mirror as source;
- all ten `DEL-006` deviation surfaces and both assurance gates retained by
  reference and boundary rules;
- no same-version ECO/DEL or ECO/ADP edge permitted;
- no test/fixture/generated/docs-to-product dependency permitted; and
- zero trailing-whitespace findings in both authored files.

These are author structural checks, not executed VTRACE verification or an
independent role decision.

## Review, remediation, and convergence sequence

1. The author baseline at SHA-256
   `c6440f451e83dbc4633961f74673a61b9c769211a67d82506c4d154355c21d74`
   entered independent digest-bound convergence review with all 13 inherited
   holds open.
2. The first independent review reported no critical or major finding and one
   minor finding, `BAS-PB-MIN-001`: generated-output custody could be read as
   allowing the held handoff to consume `PB-GEN-001`.
3. Bounded remediation changed only `PACKAGE_BOUNDARIES.md` and this pulse.
   It made `PB-TST-001` and `PB-REV-001` the only read-only inspectors of
   `PB-GEN-001`, prohibited every product, runner, handoff, Taxlane, and
   release consumer, and fixed the direction as accepted producer/handoff/
   review output → generated custody → read-only test/review inspection.
   `PB-HND-001` produces a held candidate from accepted inputs before an
   optional mirror and never consumes that mirror as source.
4. The remediated package digest is
   `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695`.
   A second independent digest-bound convergence review found zero current
   critical, major, or minor findings. The package, applicable domain,
   editorial, methodology, and both independent assurance gates converged on
   `pass_with_risk` eligibility without closing or narrowing an inherited
   hold.

The review sequence confirmed the exact baseline counts: 12 physical
boundaries; 13 uniquely allocated logical components; 98 protected SPEC
allocations; 13 controlled contracts; 10 nonfunctional constraints; 13 exact
unknown holds; 98 planned verification identities; and 21 role files. The
planned build graph remains acyclic, and the generated, test, fixture, docs,
handoff, Taxlane, and release directions remain deny-by-default as recorded in
the frozen package digest.

## Scope, safety, and authority posture

- Public, aggregate, unclassified, non-operational evidence only.
- No classified information, CUI, person-level service data, sensitive
  operational data, targeting, operational planning, or exploitable
  vulnerability content is admitted, retained, derived, or emitted.
- Civilian authority remains distinct from evidence custody. Personnel safety,
  readiness, resilience, surge, recovery, and alliance obligations remain hard
  floors; both assurance gates remain mandatory and independent.
- Taxlane alone owns admission, cross-lane combination, allocation, rebalance,
  and rates. The planned handoff boundary produces only held/rejected state.
- Release remains closed and output-free. No crate, runner command, generated
  artifact, test result, or fixed point implies publication.
- No delivery or realization evidence, accepted corpus, interface, package,
  implementation, official plan, procurement action, budget request, or rate
  instruction exists.

## Limitations and blockers

- All 13 inherited `SPEC-UNK-*` holds remain exact, open, non-defaulted, and
  promotion-gating. None was narrowed or closed.
- Planned package names and paths are boundary proposals, not created files or
  evidence that the decomposition compiles or is feasible.
- Exact contract fields, types, signatures, encoding, transport, and
  compatibility remain deferred to `INTERFACES.md`.
- Exact Rust module structure, algorithms, dependencies, versions, toolchain,
  tests, fixtures, generator, storage, runtime, and deployment remain deferred
  to later fixed artifacts and accepted work packages.
- There is no accepted corpus, test harness, generated artifact, delivery
  capability, realization proof, Taxlane integration, or release plan.

Current author blocker count: **zero**. Inherited promotion holds: **13**.

## Disposition

Repository-local decision: **`pass_with_risk` fixed point** for
`PACKAGE_BOUNDARIES.md` at SHA-256
`43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695`,
bound to fixed `ARCHITECTURE.md` SHA-256
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`.

The accepted residual risk is bounded to the 13 inherited, exact, open
promotion holds and to the planned-only nature of every unborn package,
interface, command, fixture, generator, and runtime surface. L0, L1, and L2
commands remain prospective, unavailable as accepted evidence, and
unexecuted. This decision creates no implementation readiness, delivery or
realization evidence, Taxlane admission, official authority, or release
authority.

`INTERFACES.md` is the next eligible VTRACE stage only under a new explicit
assignment. This pulse does not start, authorize, or pre-accept interface,
design, verification, work-package, package, Cargo, Rust, corpus, fixture,
generated-artifact, implementation, Taxlane, or release work.

No workspace/package/crate creation, Cargo or Rust work, interface work, test
or fixture work, generated output, corpus work, delivery work, commit, push,
remote mutation, Taxlane action, official action, or public release was
performed.
