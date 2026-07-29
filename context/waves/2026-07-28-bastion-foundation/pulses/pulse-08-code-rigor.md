# Pulse 08 — CODE_RIGOR author baseline

Date: 2026-07-28

Assignment: `ASG-BASTION-CODE-RIGOR-001`

Writer lease: exclusive `docs/vtrace/CODE_RIGOR.md` and Pulse 08 author

## Objective

Translate the fixed BASTION DESIGN and accepted bounded change-control decision
into reviewable, deterministic, finite, high-assurance Rust pre-code constraints
without creating code, workspace state, a work package, dependency, encoding,
method value, executed evidence, or downstream authority.

## Controlled inputs and output

| Controlled artifact | SHA-256 |
|---|---|
| Fixed `MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| Fixed `CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| Fixed `REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| Fixed `SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| Fixed `ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| Fixed `PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| Fixed `INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| Accepted `CHANGE_CONTROL.md` / `CHG-BA-TST-001` | `147b67601e8e992332f47432a188d2c62d9b2e8f757d30ca4141dc94421c668b` |
| Fixed-point `DESIGN.md` | `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` |
| DESIGN fixed-point record `pulse-07-design.md` | `87bb645269d315534a321c19092f1ede757a7a432fd3544c1ca074921ce56a2d` |
| Author `CODE_RIGOR.md` output | `3ce31b808f038291b79bf348547ed43029ca1b0a797520fbb72546d3964801d9` |

All controlled input digests matched before authoring. No fixed artifact was
changed.

## Author result

`docs/vtrace/CODE_RIGOR.md` now defines:

- 40 normative constraints with exact DES, package-boundary, contract,
  invariant, hold, owner/concurrence, planned-verification, expected-result,
  and waiver allocation;
- exact forward allocation and mechanically equal reverse indexes for all 14
  DESIGN decisions, 12 planned package boundaries, 13 fixed contracts, and 25
  DESIGN invariants;
- typed, non-collapsing artifact, admission, review, compatibility, lifecycle,
  stale, and supersession meanings plus explicit proof coverage for all 18
  fixed state-transition rows;
- the universal per-generation `freeze -> SOURCE -> TEST -> fixed consumer`
  gate, including SOURCE transformations and AUTH outputs;
- the finite preliminary `ECO[n] -> DEL[n] -> final ECO[n+1] -> ADP[n+1]`
  successor path, later-feedback restart, and prohibition on same-version
  cycles, retry, revisit, or in-place mutation;
- deterministic HND no-emission, the minimal non-product and one-way
  `IF-TERM-001` receipt, receipt invalidation on bundle change, external
  Taxlane exclusivity, and REL no-output/no-consumer behavior;
- total and bounded future Rust behavior, no hidden panic/abort/unchecked
  shortcut, no unauthorized unsafe/foreign surface, no favorable semantic
  default, checked accounting, deterministic identity/order, immutable
  history, and fixed dependency direction;
- planned property, transition/model, adversarial, per-contract,
  parser/fuzz-if-authorized, regression, mode-isolation, generated-output,
  quality, dependency/license/advisory, and resource-bound evidence;
- twelve explicit waiver classes, including separate non-waivable security,
  civilian-control, safety/readiness, distribution, authority, semantics,
  graph, hold, evidence, HND/terminal, and release protections, with zero
  accepted waivers; and
- all 13 exact `TBD-*` / `SPEC-UNK-*` pairs open with their inherited hold
  meaning and direct/transitive propagation unchanged.

## Bounded remediation

`BA-CR-B-001` identified that universal gate constraint `CR-010` was stated
correctly but under-allocated. The forward allocation now covers all 13 active
DES product/material producers, all seven product boundaries plus the test
boundary, and all 12 active contracts. The affected DES, package-boundary, and
contract reverse sets were regenerated and are exact transposes.

`DES-REL-001` / `CONTRACT-REL-001` remains the fixed no-output exception.
Minimal non-product `IF-TERM-001` remains only a branch of
`CONTRACT-TEST-001`, not a fourteenth contract or a promotable producer. No
other constraint, invariant, hold, transition, waiver, evidence, role,
authority, or semantic allocation changed.

## Author validation

| Check | Result |
|---|---|
| Controlled input digests | 10 expected; 10 exact matches |
| Normative CR inventory | 40 expected; `CR-001` through `CR-040`; 40 unique |
| Authoritative allocation halves | 40 DES/PB/contract rows and 40 invariant/hold/owner/verification/expected/waiver rows; exact unique CR-key equality |
| DES forward/reverse equality | 14 reverse rows; zero transpose mismatches |
| Package-boundary forward/reverse equality | 12 reverse rows; zero transpose mismatches |
| Contract forward/reverse equality | 13 reverse rows; zero transpose mismatches; `IF-TERM-001` not added |
| Invariant forward/reverse equality | 25 reverse rows; zero transpose mismatches |
| `BA-CR-B-001` closure | `CR-010` covers 13 active DES, 8 relevant PB, and 12 active contracts; REL and minimal non-product TERM exceptions preserved |
| Transition coverage | 18 exact DESIGN rows represented; each has controlling CR coverage |
| Controlled unknown preservation | 13 unique TBDs and 13 unique paired `SPEC-UNK-*`; one-to-one; zero closed |
| Gate and finite graph | Universal SOURCE-before-TEST; monotone ECO/DEL/ADP sequence; finite HND/TERM branch; REL no-output; no receipt/product backflow |
| Waiver posture | 11 non-waivable classes plus process-only proposal path; zero accepted waivers |
| Evidence destinations | 10 `CR-EVID-*` groups cover every CR; all planned, absent, and unexecuted |
| Assurance and roles | Both assurance gates independent and non-waivable; all parliament and seven stakeholder lenses retained |
| Representation boundary | No source, workspace, package, schema, API, command, tool/version/configuration, dependency, runtime, storage, transport, deployment, fixture, generator, method, or resource value selected |
| Assigned-file scope | Only `docs/vtrace/CODE_RIGOR.md` and this Pulse 08 authored |
| Whitespace | Zero trailing-whitespace lines |

These are author-side structural and semantic checks. They are not independent
review, executed tool output, verification evidence, acceptance, or
convergence.

## Independent convergence decision

Two independent, digest-bound reviews converged on the remediated artifact at
SHA-256
`3ce31b808f038291b79bf348547ed43029ca1b0a797520fbb72546d3964801d9`:

| Review | Bound scope | Result |
|---|---|---|
| Independent code-rigor cross-review | All 40 CR rows and both authoritative halves; exact 14-DES, 12-PB, 13-contract, and 25-invariant transposes; all 18 transitions, 13 holds, typed states, finite graph, waiver classes, evidence posture, assurance, and authority boundaries | Zero findings; recommended `pass_with_risk`. |
| Independent remediation convergence recheck | Universal `CR-010` coverage across every active product/material producer and relevant boundary/contract; REL no-output and minimal non-product `IF-TERM-001` exceptions; regression of every transpose, hold, graph, evidence, and authority statement | Zero findings; `BA-CR-B-001` closed; recommended `pass_with_risk`. |

The BASTION maintainer/stage-controller decision is therefore a
**governance-only `pass_with_risk` CODE_RIGOR fixed point** for the exact digest
above. This decision accepts the planning constraint baseline only. It accepts
no implementation, work package, verification or validation method, evidence,
hold closure, HND emission, external admission, or release.

The next eligible planning work is an `IMPLEMENTATION_PLAN.md` /
`WORK_PACKAGES.md` planning bundle or a planning-only `VERIFICATION.md` draft.
Each requires a new explicit assignment before authoring, and the one-active-
VTRACE-deliverable rule still applies. Neither next artifact, any contained
work package, nor any method is accepted by this decision.

## Protected boundaries and residual risk

- All 13 holds remain open and conjunctive. In particular,
  `TBD-TST-001` / `SPEC-UNK-TST-001` remain product-evidence and downstream
  promotion gates under accepted `CHG-BA-TST-001`; this planning artifact
  supplies no Verification method or evidence.
- All public-aggregate, unclassified, non-operational, civilian-control,
  law/safety/readiness, distribution, stakeholder, and assurance gates remain
  mandatory. Classified, controlled, targeting, operational-planning,
  exploitable-vulnerability, or person-level service content remains
  prohibited.
- Every test, property, fixture, parser/fuzz obligation, tool result, static
  check, dependency/license/advisory audit, and resource-bound record remains
  planned, absent, and unexecuted.
- HND emits no pack, the terminal receipt carries no product and has no return
  edge, Taxlane retains external admission, and REL emits no output.

## Disposition

Disposition: **governance-only `pass_with_risk` CODE_RIGOR fixed point** for
`docs/vtrace/CODE_RIGOR.md` SHA-256
`3ce31b808f038291b79bf348547ed43029ca1b0a797520fbb72546d3964801d9`.

All 13 holds remain open. Every evidence destination remains planned, absent,
and unexecuted. Any content change invalidates this digest-bound decision and
requires a controlled successor plus new independent convergence.

No code, Cargo/workspace state, accepted work package, dependency, schema, API,
command, tool choice, method value, executed evidence, HND emission, Taxlane
action, official action, procurement, budget, allocation, rate, deployment, or
public release is authorized or created.
