# Pulse 07 — Detailed conceptual design

Date: 2026-07-28

Assignment: `ASG-BASTION-DESIGN-001`

Governance remediation assignment: `ASG-BASTION-TST-GOVERNANCE-001`

Lease disposition: completed within the assigned three-file write scope.

## Objective

Allocate every fixed requirement and specification across the fixed
architecture, package boundaries, and interfaces; make the BASTION semantic
procedures, invariants, transitions, edge cases, rollout posture, and future
code-rigor hooks reviewable without making implementation or policy choices.
Resolve the bounded TST stage-governance deadlock through explicit change
control without editing or silently reinterpreting fixed upstream artifacts.

## Inputs read

- Repository instructions and operating context: `AGENTS.md`, `README.md`,
  `PRODUCT_PLAN.md`, `CLAUDE.md`, and `.roles/ROLE.md`.
- All 21 substantive role files under `.roles/`.
- Fixed VTRACE left side through `docs/vtrace/INTERFACES.md`.
- VTRACE DESIGN template, process, staged-execution, specificity-map, example,
  and CODE_RIGOR template guidance from the VTRACE repository.
- VTRACE CHANGE_CONTROL template and reference change-control artifact.
- Pulse 06 interface fixed-point record.

## Controlled input digests

| Input | SHA-256 |
|---|---|
| `docs/vtrace/MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| `docs/vtrace/CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| `docs/vtrace/REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| `docs/vtrace/SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| `docs/vtrace/ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| `docs/vtrace/PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| `docs/vtrace/INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| `pulse-06-interfaces.md` | `ebffbb2bb3106727088ff7ad79a7125046328d6e506bb7da8b5bceb2a877e92d` |
| Frozen pre-governance `docs/vtrace/DESIGN.md` | `268293f8758a64c7dc7d3453c078d26a87cd764da4a8cf63567d9eb046ef6b8d` |

## Output digest

| Output | SHA-256 |
|---|---|
| `docs/vtrace/CHANGE_CONTROL.md` | `147b67601e8e992332f47432a188d2c62d9b2e8f757d30ca4141dc94421c668b` |
| `docs/vtrace/DESIGN.md` | `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` |

## Design result

`docs/vtrace/DESIGN.md` now provides:

- 14 primary design decisions covering all 98 requirements and their 98
  one-to-one specifications, plus explicit allocation of all 10
  non-functional specifications (108 specification items total);
- allocation of all 13 components, 12 planned package boundaries, 13 fixed
  contracts, and 10 non-functional constraints;
- deterministic conceptual procedures enforcing
  `freeze -> SOURCE exact-output admission -> independent TEST -> authorized
  semantic consumer` for every promotable product/material output, including
  every stage of the versioned ECO/DEL/ADP sequence;
- an explicit per-generation DAG and finite-termination proof, plus inverse
  routing from every controlled hold to exact direct and transitive-only DES
  sets and an equal inverse DES-to-hold transpose;
- 25 invariants, 18 state-transition rows, 25 edge cases, 13 universal bypass
  fixtures, 16 future code-rigor hooks, and explicit rejected alternatives;
- synthesis of all 21 substantive role lenses; and
- all 13 `TBD-*` / `SPEC-UNK-*` holds still open with their fixed hold behavior.

`CHG-BA-TST-001` now provides the bounded prospective stage-term tailoring
required to resolve `BA-DES-M03`: the TST hold continues to govern
product-evidence convergence and downstream claims, while governance-only
planning-artifact fixed points do not claim verification success. After
independent acceptance of the exact change/design digests, a planning-only
VERIFICATION artifact may be authored before DESIGN fixed point solely to
propose the held method; it is not executed evidence or acceptance. DESIGN is
bound to the exact CHANGE_CONTROL digest.

The terminal branch remains finite: the minimal non-product `IF-TERM-001`
receipt is a branch of `CONTRACT-TEST-001`, not a fourteenth contract; it is
consumed only by the external handoff gate, never returns to HND/SOURCE/TEST,
and any product or bundle change invalidates it and restarts freeze, SOURCE
admission, and independent TEST. Taxlane remains the exclusive external
admission authority. `CONTRACT-REL-001` emits nothing.

## Bounded design remediation

| Finding | Disposition | Result |
|---|---|---|
| `BA-DES-M01` | remediated; independently rechecked and closed | Each of 13 holds has an explicit direct DES set and transitive-only DES set with no catch-all. The 14-row inverse DES-to-hold table is the exact set transpose. |
| `BA-DES-M02` | remediated; independently rechecked and closed | Procedures A, B, D, the finite DAG, state rules, and 13 producer bypass fixtures now require freeze, SOURCE exact-output admission, independent TEST, then authorized semantic consumption. Only unchanged minimal `IF-TERM-001` and REL no-output are outside product promotion. |
| `BA-DES-M03` | remediated by `CHG-BA-TST-001`; independently rechecked and closed for the bound digest pair | Controlled stage-term tailoring preserves the literal upstream hold, prior history, and every open method while permitting a bounded planning-only VERIFICATION artifact. DESIGN fixed point remains governance-only and grants no Verification-plan acceptance or downstream authority. |
| `BA-DES-M04` | remediated; independently rechecked and closed | Admission/compatibility `stale`, new successor identity, retained predecessor/successor/supersession references, and the five fixed lifecycle dispositions are separate states and transitions. |

## Change-control remediation

| Finding | Disposition | Result |
|---|---|---|
| `BA-CHG-M01` | remediated; independently rechecked and closed | CHANGE_CONTROL now records the complete 64-character fixed INTERFACES digest `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882`; all fixed inputs were rehashed after correction. |
| `BA-CHG-M02` | remediated; independently rechecked and closed for the bound digest pair | Independent digest-bound acceptance of the exact CHANGE_CONTROL + DESIGN pair is now a strict predecessor of any planning-only VERIFICATION draft. The draft may precede only the later administrative DESIGN fixed-point recording. |

## Author validation

- Seven fixed input digests rechecked against file bytes after governance
  remediation; the incomplete earlier CHANGE_CONTROL digest entry was corrected
  and is not represented as a passing check.
- Primary allocation arithmetic checked: 98 requirements, 108 specifications,
  13 components, 13 contracts, and no additional contract.
- All 12 package-boundary identities represented.
- All 13 controlled holds represented with no closure or invented default;
  direct/transitive and inverse set equality checked.
- All 21 substantive role files represented in author synthesis.
- One change record, one tailored stage DAG, three expressly rejected deadlock/history/reinterpretation alternatives plus two bypass alternatives, nine reviewer lenses, and eight reopen-trigger classes represented.
- Exact impact inventory includes both TST unknowns; 8 TST/VTR requirements;
  8 TST/VTR specifications; 8 planned VER identities; REV component and
  package; TEST/TRACE contracts; TEST/TRACE design decisions; BA-DES-M03 and
  stage text; plus an IF-TERM/HND no-edge impact check.
- DESIGN contains the corrected exact CHANGE_CONTROL digest; CHANGE_CONTROL contains the
  frozen pre-tailoring DESIGN digest, avoiding a circular digest claim.
- Prohibited choices scanned for language, package, schema, API,
  dependency, runtime, quantitative, operational, legal, policy, and release
  invention.
- Fixed-input digests, whitespace, file scope, and output SHA-256 checked after
  remediation.

## Independent convergence decision

Two independent, digest-bound reviews converged on the unchanged pair:

| Review | Bound scope | Result |
|---|---|---|
| Independent governance/change-control review | `CHG-BA-TST-001` at `147b67601e8e992332f47432a188d2c62d9b2e8f757d30ca4141dc94421c668b` and its binding to DESIGN | Zero findings; the prospective stage-term tailoring is bounded, preserves upstream history and both TST unknowns, and creates no verification, implementation, or downstream authority. Recommended `pass_with_risk`. |
| Independent DESIGN convergence review | DESIGN at `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` with the exact bound change-control digest | Zero findings; allocation, direct/transitive hold transpose, state/DAG behavior, review gates, terminal no-backflow branch, role routing, and scope/authority deferrals remain coherent. Recommended `pass_with_risk`. |

The BASTION maintainer/stage-controller decision is therefore to accept
`CHG-BA-TST-001` at the exact digest above and declare its exactly bound DESIGN
artifact a **governance-only `pass_with_risk` fixed point**. The residual risk
is explicit: all 13 `TBD-*` / `SPEC-UNK-*` holds remain open, and all planned
fixtures and evidence remain absent and unexecuted.

The next eligible stages are CODE_RIGOR and a planning-only VERIFICATION draft.
Each requires a new, explicit assignment before authoring. Neither stage nor
artifact is accepted by this decision, and the repository rule to advance one
VTRACE deliverable at a time still applies.

## Stage disposition

**Governance-only `pass_with_risk` DESIGN fixed point** for DESIGN SHA-256
`4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781`,
bound to accepted `CHG-BA-TST-001` / CHANGE_CONTROL SHA-256
`147b67601e8e992332f47432a188d2c62d9b2e8f757d30ca4141dc94421c668b`.
This is not CODE_RIGOR acceptance, VERIFICATION-plan acceptance, verification,
validation, implementation evidence, or external approval. All 13 holds,
including literal `TBD-TST-001` and `SPEC-UNK-TST-001`, remain unchanged, open,
conjunctive, and promotion-gating. All fixtures and evidence remain planned,
absent, and unexecuted.

No code, Rust, schema, package, API, dependency, runtime, work package,
deployment, executed evidence, HND emission, Taxlane admission, release,
official action, budget, allocation, rate, or procurement is authorized or
created.
