# BASTION Package Boundaries

## Status and controlled input

Repo: BASTION

Assignment: `ASG-BASTION-PACKAGE-BOUNDARIES-001`

Package-boundary state: **review-ready planned baseline; not a fixed point**.

Controlled `ARCHITECTURE.md` input SHA-256:
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`

The architecture fixed-point decision is recorded in Pulse 04. This artifact
plans physical ownership and dependency boundaries only. No listed workspace,
package, crate, test target, fixture directory, generated directory, command,
interface representation, or artifact exists merely because it is named here.
No file or package described below has been created by this stage.

This baseline does not select APIs, schemas, serialization, storage,
algorithms, dependency libraries or versions, Rust toolchain version, runtime,
process topology, deployment, security mechanism, or sensitive threshold.
Controlled contract IDs name semantic surfaces only; fields, signatures,
cardinality, encodings, transports, and compatibility fixtures remain deferred
to `INTERFACES.md`.

## Boundary strategy

The minimal planned implementation shape is one BASTION-local Rust workspace,
six independently owned product libraries, one thin orchestration binary, one
non-product integration-test crate, one fixture boundary, one generated-output
boundary, and the existing governance/docs boundary. It is intentionally not
one crate per logical component.

The split follows four rules:

1. Source/security custody, civilian authority, independent review, and the
   held Taxlane handoff remain separate physical trust boundaries.
2. The five mutually related domain-analysis components share one crate but
   retain separate modules, semantic owners, contracts, results, and holds.
3. Economics, mandatory delivery, and adaptive lifecycle share one crate so a
   single versioned control boundary can enforce preliminary
   `ECO[n]→DEL[n]→final ECO[n+1]→ADP[n+1]` ordering without a crate cycle.
4. Sibling product libraries have no planned direct build dependency. A thin
   BASTION-only runner composes them through the controlled contract surfaces,
   preventing reciprocal package dependencies and premature extraction of a
   generic shared crate.

`ARC-REL-001` remains a docs/governance responsibility because it emits
nothing. Creating a release crate would falsely imply release behavior.

## Planned boundary inventory

Every path and package name in this table is planned, except the existing docs,
roles, and wave paths. The `Public interfaces` column contains only controlled
contract IDs or `none`; it does not define an API.

| Boundary ID | Planned boundary unit | Profile | Accountable physical owner | Responsibility | Public interfaces | Planned consumers |
|---|---|---|---|---|---|---|
| `PB-WS-001` | Repository-root Rust workspace membership and lock/configuration surface | Rust workspace metadata | BASTION maintainer | Own the later workspace member list and whole-workspace validation entry point; own no domain semantics. | none | All planned Rust boundaries. |
| `PB-CST-001` | `crates/bastion-custody/` library | Rust product library | Public-evidence steward; security concurrence remains mandatory | Source admission, claim/provenance custody, safe rejection, and exact digest/context security re-admission, including custody transformations. | `CONTRACT-SOURCE-001` | Runner and every controlled downstream use. |
| `PB-AUT-001` | `crates/bastion-authority/` library | Rust product library | Civilian mission and authority steward | Mission abstraction, lawful civilian authority, scope, and no-authority-inflation posture; output remains unusable before custody re-admission. | `CONTRACT-AUTH-001` | Runner, custody gate, and dependent analysis after admission. |
| `PB-DOM-001` | `crates/bastion-domain/` library with separately owned readiness, acquisition, logistics, alliance, and distribution modules | Rust product library | BASTION maintainer for placement; the five architecture semantic owners retain exclusive meaning and concurrence | Public-aggregate domain analysis, hard floors, commonality, lifecycle support, interoperability, stakeholder incidence, and tails without composite replacement. | `CONTRACT-RDY-001`; `CONTRACT-ACQ-001`; `CONTRACT-LOG-001`; `CONTRACT-ALLY-001`; `CONTRACT-DST-001` | Runner, pathway boundary, review, and handoff after required gates. |
| `PB-PTH-001` | `crates/bastion-pathway/` library with separately owned economics, delivery, and adaptation modules | Rust product library | BASTION maintainer for placement; Defense resource analyst and Delivery owner retain their settled semantic authority | Six non-additive pathways, preliminary/final economic envelopes, mandatory delivery posture, feedback, rollback, and adaptive history with immutable version ordering. | `CONTRACT-ECO-001`; `CONTRACT-DEL-001` | Runner, review, and handoff after required gates. |
| `PB-REV-001` | `crates/bastion-review/` library | Rust product library | Role review steward; Independent Test and Oversight Officer owns applicable independent test | Frozen review packets, findings, negative evidence, dissent, trace, convergence, and the finite terminal decision without producer mutation or self-approval. | `CONTRACT-TEST-001`; `CONTRACT-TRACE-001` | Runner, accountable producers, stage governance, and terminal handoff. |
| `PB-HND-001` | `crates/bastion-handoff/` library | Rust product library | Taxlane adapter steward | Build only an immutable held or rejected BASTION-side `LaneEvidencePack` candidate while preserving semantic-owner concurrence and external admission ownership. | `CONTRACT-HND-001` | Runner, custody re-admission, terminal review, and the external Taxlane boundary. |
| `PB-RUN-001` | `crates/bastion-runner/` binary/orchestration package | Rust application shell | BASTION maintainer | Invoke product boundaries in the fixed architecture order, route every output through custody and review, and expose no new semantic decision. | none | Authorized internal researchers only after later implementation gates. |
| `PB-DOC-001` | Existing root governance, `.roles/`, `docs/vtrace/`, and `context/waves/` | docs/governance | BASTION maintainer and role review steward | Control VTRACE artifacts, role participation, reviews, pulses, no-authority posture, and the closed no-output release responsibility. | `CONTRACT-REL-001` | Maintainers, reviewers, and later controlled stages. |
| `PB-TST-001` | `crates/bastion-boundary-tests/` non-product integration-test package | Rust test-only | Independent Test and Oversight Officer with role review steward | Exercise controlled contract crossings, failure containment, security/readmission bypasses, version-order invariants, role coverage, and no-authority behavior. | none | Review and verification evidence only. |
| `PB-FIX-001` | `fixtures/` non-product inputs | fixture data/docs; representation deferred | Public-evidence steward; security and both assurance reviews apply where relevant | Hold inert synthetic or accepted public-aggregate positive, null, negative, stale, dangerous-composition, cycle, and mismatch inputs for planned tests. | none | `PB-TST-001` only. |
| `PB-GEN-001` | `generated/` non-source output custody | generated artifacts; representation deferred | Producing boundary remains semantic custodian; BASTION maintainer owns path policy | Hold reproducible outputs, receipts, manifests, and review evidence after a later generator exists; never become source of truth or a product dependency. | none | `PB-TST-001` and `PB-REV-001` read-only inspection only; no product, runner, handoff, Taxlane, or release consumer. |

## Exact logical-component allocation

Each architecture component has exactly one accountable physical home. A
grouped crate does not merge component identity, primary SPEC custody,
contract ownership, owner concurrence, failure behavior, or unknown holds.

| Logical component | Accountable physical boundary | Internal separation and cross-cutting participation |
|---|---|---|
| `ARC-SRC-001` | `PB-CST-001` | Separate admission, custody, transformation, rejection, and re-admission responsibilities; Classification and Operational Security participation cannot be waived. |
| `ARC-AUTH-001` | `PB-AUT-001` | Authority remains distinct from evidence safety; Scope Keeper and Civilian Control, Law, Safety & Readiness participate in promotion. |
| `ARC-RDY-001` | `PB-DOM-001` | Separate readiness module and semantic owner; logistics, alliance, stakeholder, and both assurance lenses participate where applicable. |
| `ARC-ACQ-001` | `PB-DOM-001` | Separate acquisition/commonality module and semantic owner; supplier, workforce, distribution, security, and review lenses participate. |
| `ARC-LOG-001` | `PB-DOM-001` | Separate inventory/repair/sustainment module and semantic owner; readiness, workforce, supplier, alliance, and security lenses participate. |
| `ARC-ALLY-001` | `PB-DOM-001` | Separate alliance/interoperability module and semantic owner; partner, legal/civilian, distribution, and security lenses participate. |
| `ARC-DST-001` | `PB-DOM-001` | Separate distribution/tail module and semantic owner; all seven stakeholder lenses and both assurance gates participate as applicable. |
| `ARC-ECO-001` | `PB-PTH-001` | Separate accounting module owns preliminary and final envelopes; Defense Comptroller and Numeracy Checker participate; no budget or Taxlane authority. |
| `ARC-ADP-001` | `PB-PTH-001` | Separate lifecycle module consumes only final predecessor-linked ECO and may request only a later preliminary successor. |
| `ARC-DEL-001` | `PB-PTH-001` | Separate mandatory-delivery module; Delivery owner and all deviation/harm lenses participate; it cannot mutate preliminary ECO. |
| `ARC-REV-001` | `PB-REV-001` | Physically separate independent-review boundary; every applicable role participates without self-approval. |
| `ARC-HND-001` | `PB-HND-001` | Physically separate held adapter; all semantic owners concur on mapped meaning and Taxlane remains external. |
| `ARC-REL-001` | `PB-DOC-001` | Closed governance-only responsibility; produces no artifact and has no outbound release edge. |

Allocation totals: **13 logical components, 13 unique component-to-boundary
assignments, zero unallocated components, and zero multiply allocated
components**.

By reference to the controlled architecture, these placements preserve all 98
exclusive SPEC allocations, all 13 controlled contracts, all 10
nonfunctional constraints, all 13 exact `SPEC-UNK-*` dependent sets, and all
98 planned `VER-*` identities. This stage changes none of those protected
sets and closes no unknown.

The unknown register below records physical containment only. The exact
dependent SPEC rows and hold behavior remain the controlled architecture and
specification text and are not redefined here.

| Preserved unknown | Containing physical boundaries |
|---|---|
| `SPEC-UNK-SEC-001` | `PB-CST-001`, applicable `PB-DOM-001` logistics work, `PB-HND-001`, `PB-DOC-001` |
| `SPEC-UNK-RDY-001` | `PB-AUT-001`, `PB-DOM-001`, `PB-PTH-001`, `PB-HND-001` |
| `SPEC-UNK-SRC-001` | `PB-CST-001`, `PB-REV-001` |
| `SPEC-UNK-QNT-001` | applicable `PB-DOM-001` readiness work, `PB-PTH-001`, `PB-HND-001` |
| `SPEC-UNK-ACQ-001` | `PB-DOM-001`, `PB-PTH-001` |
| `SPEC-UNK-LOG-001` | `PB-DOM-001`, `PB-PTH-001` |
| `SPEC-UNK-ALLY-001` | `PB-DOM-001`, `PB-PTH-001` |
| `SPEC-UNK-DST-001` | `PB-DOM-001`, `PB-PTH-001`, `PB-HND-001` |
| `SPEC-UNK-ECO-001` | `PB-PTH-001`, `PB-HND-001` |
| `SPEC-UNK-TST-001` | `PB-REV-001` |
| `SPEC-UNK-DEL-001` | `PB-PTH-001`, `PB-HND-001` |
| `SPEC-UNK-HND-001` | `PB-PTH-001`, `PB-HND-001` |
| `SPEC-UNK-REL-001` | `PB-CST-001`, `PB-DOC-001` |

## Role participation across physical boundaries

Roles participate in gates and review; they do not become package dependencies
or displace the accountable physical or semantic owner. Each of the 21 role
files appears exactly once below. Both assurance roles are independent,
conjunctive gates for every applicable promotion.

| Role file | Participating boundaries | Required physical-boundary concern |
|---|---|---|
| `parliament/civilian-strategy-force-planner.md` | `PB-AUT-001`, `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Mission/authority drift, disguised force or operational planning, and readiness/delivery authority. |
| `parliament/operational-readiness.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Proxy optimism, degraded support, tails, safety, readiness, surge, and recovery. |
| `parliament/acquisition-industrial-base.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Commonality, concentration, qualification, supplier/workforce, transition, schedule, and cost. |
| `parliament/logistics-sustainment.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Inventory boundaries, repair tails, lifecycle support, degraded recovery, and delivery. |
| `parliament/defense-comptroller.md` | `PB-PTH-001`, `PB-HND-001`, `PB-REV-001` | Fiscal measures, gross opportunity, realization, overlap, lifecycle, transition, and handoff meaning. |
| `parliament/service-member-family.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Safety, tempo, staffing, retention, family burden, and delivery deviations. |
| `parliament/independent-test-oversight.md` | `PB-REV-001`, `PB-TST-001`, and every producing boundary | Reproduction, adverse cases, failure retention, independence, and no self-approval. |
| `parliament/alliance-interoperability.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Compatibility, sovereignty/control, shared logistics, partner burden, and degraded support. |
| `editorial/citation-auditor.md` | `PB-CST-001`, `PB-DOC-001`, `PB-REV-001`, and every producing boundary | Source custody, evidence labels, derivation/aggregation boundaries, and digest freshness. |
| `editorial/scope-keeper.md` | `PB-CST-001`, `PB-AUT-001`, `PB-HND-001`, `PB-DOC-001`, `PB-REV-001` | Public aggregate scope, no operational/official claim, Taxlane isolation, and no release. |
| `editorial/numeracy-checker.md` | `PB-PTH-001`, `PB-HND-001`, `PB-REV-001`, `PB-TST-001` | Units, denominators, horizons, prices, uncertainty, overlap, and double counting. |
| `assurance/classification-operational-security.md` | `PB-CST-001`, every producing boundary, `PB-HND-001`, `PB-REV-001`, `PB-DOC-001` | Direct and compositional security, exact-context re-admission, safe receipts, and output-free release boundary. |
| `assurance/civilian-control-law-safety-readiness.md` | `PB-AUT-001`, `PB-DOM-001`, `PB-PTH-001`, `PB-HND-001`, `PB-REV-001` | Lawful authority, personnel safety, hard floors, mission/risk stability, and no fiscal override. |
| `stakeholders/mission-user.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Reliable integrated support and degraded behavior without operational detail. |
| `stakeholders/service-member-family.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Safety, training, tempo, retention, housing, health, moves, and family stability. |
| `stakeholders/depot-logistics-workforce.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Maintainability, data, facilities, skills, spares, workload, safety, and realistic repair schedules. |
| `stakeholders/prime-small-supplier.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Competition, cash flow, capacity, qualification, IP, workforce, and resilient demand. |
| `stakeholders/installation-community.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Employment, housing, utilities, environment, safety, services, and transition burden. |
| `stakeholders/ally-partner.md` | `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001` | Commitments, sovereign constraints, interoperability, shared logistics, and burden distribution. |
| `stakeholders/taxpayer-oversight.md` | `PB-PTH-001`, `PB-HND-001`, `PB-REV-001` | Mission linkage, auditability, lifecycle affordability, uncertainty, failure/nulls, and delivery. |
| `panel-reviewer/panel.md` | `PB-REV-001`, `PB-TST-001`, and every producing boundary | Cross-method coherence using public aggregate unclassified evidence; no external approval. |

## Planned build-dependency DAG

In this section, `A → B` means **A may depend on B at build time**. It does not
mean B consumes A's product. Any edge not explicitly allowed below is
forbidden until this baseline and the later interface/design artifacts are
changed through review.

```text
PB-RUN-001 -> PB-CST-001, PB-AUT-001, PB-DOM-001,
              PB-PTH-001, PB-REV-001, PB-HND-001

PB-TST-001 -> PB-RUN-001, each Layer-0 product library, PB-FIX-001

Independent non-build boundaries: PB-DOC-001, PB-GEN-001
Workspace membership/control only: PB-WS-001
```

The six Layer-0 product libraries are siblings. Their controlled runtime data
flow is composed by `PB-RUN-001`; no sibling product-to-product Cargo edge is
selected at this stage. `INTERFACES.md` must settle how contract values cross
those seams before any dependency can be implemented. This is a deliberate
planned boundary, not a claim that the uncreated crates currently compile.

### Exhaustive build-dependency rules

| Dependent | Dependency | Allowed? | Reason and planned verification |
|---|---|---|---|
| `PB-RUN-001` | `PB-CST-001`, `PB-AUT-001`, `PB-DOM-001`, `PB-PTH-001`, `PB-REV-001`, `PB-HND-001` | yes, later | The BASTION-only application shell must orchestrate all product libraries without acquiring their semantics. Later workspace metadata inspection proves only these downward edges. |
| `PB-TST-001` | `PB-RUN-001` and each Layer-0 product library | yes, test-only | Independent integration tests exercise complete and isolated contract paths. Later Cargo target inspection proves the dependency is absent from product builds. |
| `PB-TST-001` | `PB-FIX-001` | yes, test input only | Fixtures feed verification; they do not define production behavior. Later test-manifest and path inspection prove direction. |
| Any Layer-0 product library | another Layer-0 product library | no at this stage | Direct sibling edges could create cycles, bypass orchestration, or force an unreviewed representation. Any proposed exception requires fixed `INTERFACES.md`, DAG recheck, and package-boundary review. |
| Any Layer-0 product library | `PB-RUN-001` | no | A product library cannot depend upward on its application shell. |
| Any product boundary, including `PB-RUN-001` | `PB-TST-001` or `PB-FIX-001` | no | Test and fixture boundaries are never product dependencies. |
| Any product or test boundary | `PB-GEN-001` as source or build input | no | Generated outputs are sinks and evidence, never source of truth or compile-time authority. `PB-TST-001` and `PB-REV-001` may inspect accepted mirrored outputs read-only; inspection is not a build, runtime, source, or product dependency. No other consumer is permitted. |
| Any Rust boundary | `PB-DOC-001` as executable or build dependency | no | Governance is authoritative by review and trace, not compiled into hidden behavior. Controlled contract references may be checked later without making docs a runtime dependency. |
| `PB-DOC-001` | any Rust boundary | no build dependency | Docs may cite planned or later verified evidence, but no implementation result is accepted now. |
| `PB-WS-001` | member crates | membership only | Workspace membership is configuration, not semantic ownership or a runtime edge. |
| External Taxlane | `PB-HND-001` | no BASTION build edge selected | The handoff is a separately governed external boundary. Compatibility and dependency form remain deferred to shared interface review. |
| Public-release system | any BASTION boundary | no | Release is closed; `PB-DOC-001`/`ARC-REL-001` emits nothing and no route-around edge exists. |

The resulting planned Cargo graph is acyclic: six independent libraries feed
one runner; the test-only boundary depends downward on the runner/libraries;
no product edge points upward or sideways.

## Controlled runtime and artifact direction

Runtime/control direction is distinct from Cargo dependency direction. The
runner may only sequence the controlled contracts; it cannot add an interface,
default, conversion, decision, or authority.

| Sequence | Controlled producer and consumer direction | Mandatory posture |
|---|---|---|
| 1 | `PB-CST-001` → runner → every applicable producer via `CONTRACT-SOURCE-001` | Exact admitted public source/version/digest/claim/security posture or explicit hold/rejection. |
| 2 | `PB-AUT-001` → runner → `PB-CST-001` via `CONTRACT-AUTH-001` and `CONTRACT-SOURCE-001` | AUTH output receives fresh exact digest/context security re-admission before any dependent use. |
| 3 | `PB-DOM-001` produces the five domain contracts; runner routes every derived/changed domain output back to `PB-CST-001`, then to `PB-REV-001` | No result is consumed, accepted, retained, or reviewed before fresh output security posture; semantic owners remain distinct inside the grouped crate. |
| 4 | `PB-PTH-001` freezes preliminary `ECO[n]`, records mandatory `DEL[n]`, runner routes DEL through custody and review, then `PB-PTH-001` may create final `ECO[n+1]` and only then `ADP[n+1]` | `CONTRACT-ECO-001` and `CONTRACT-DEL-001`; immutable predecessor bond, `SPEC-ECO-006`, no delivery bypass, no same-version or in-place mutation. |
| 5 | ADP disposition or later DEL observation → runner → custody → review → a later preliminary ECO successor → mandatory new DEL → custody/review → next final ECO | Feedback always creates a successor and repeats the mandatory delivery gate; stale/missing predecessor fails closed. |
| 6 | Every producer, including custody transformations and review product outputs, → runner → `PB-CST-001` → `PB-REV-001` | Exact-context security re-admission precedes independent review; findings return only to the accountable producer for a new version. |
| 7 | Final ECO plus ADP and delivery bond → `PB-HND-001` immutable candidate bundle → runner → `PB-CST-001` exact bundle admission → `PB-REV-001` finite terminal decision | Terminal receipt contains only minimum non-reconstructive governance metadata and no product content. Any product change restarts freeze/admission/review. |
| 8 | Accepted producer, `PB-HND-001`, or `PB-REV-001` output → `PB-GEN-001` custody → `PB-TST-001` and `PB-REV-001` read-only inspection | Custody is a sink after production, never an input to product, runner, handoff, Taxlane, or release behavior. A held candidate bundle is produced by `PB-HND-001` from accepted inputs and may be mirrored to `PB-GEN-001` only after production; neither `PB-HND-001` nor Taxlane consumes `PB-GEN-001` as source. |
| 9 | Reviewed held candidate → external Taxlane boundary via `CONTRACT-HND-001` | BASTION can produce only held/rejected state. Taxlane alone owns admission, combination, allocation, rebalance, and rates. |
| 10 | Any no-release posture → `PB-DOC-001` via `CONTRACT-REL-001` | Governance retention only; no artifact emission and no public-release edge. |

## Boundary rules and change control

| Boundary | Allowed later changes under an accepted work package | Forbidden changes | Package-boundary change trigger |
|---|---|---|---|
| `PB-WS-001` | Add only reviewed members and validation configuration named by an accepted work package. | Hidden member, external path dependency, toolchain/version default, or dependency library not accepted by later design/work. | Member, workspace validation, lock/configuration policy, toolchain, or cross-repo dependency changes. |
| `PB-CST-001` | Implement settled source/security contracts and safe receipts after interface/design gates. | Prohibited data, self-exempt transformation, stale posture reuse, reconstructive receipt, release decision, or authority decision. | Source/security responsibility, concurrence, context trigger, receipt, or dependency edge changes. |
| `PB-AUT-001` | Implement settled authority contract after interface/design gates. | Broaden authority silently; choose mission, force, procurement, budget, rate, operation, or official action; bypass security re-admission. | Mission/authority field, owner, concurrence, scope, period, or security route changes. |
| `PB-DOM-001` | Implement separately owned domain modules and their settled contracts. | Merge semantic owners, collapse facets/tails/parties, turn peer gaps into targets, retain unsafe detail, or command operations/procurement. | Component placement, module ownership, contract, floor, aggregation, domain dependency, or split/merge changes. |
| `PB-PTH-001` | Implement immutable preliminary/final envelopes, mandatory delivery, and successor-only adaptation. | Same-version ECO/DEL or ECO/ADP cycle, in-place edit, delivery bypass, false savings, fabricated owner/schedule/value, budget or Taxlane action. | Version/order invariant, contract, fiscal owner, delivery prerequisite, feedback, overlap, or disposition changes. |
| `PB-REV-001` | Implement frozen independent review, trace, findings, dissent, and minimal terminal receipt. | Review own authored output, edit producer artifacts, self-waive, create authority, or put product content in terminal receipt. | Reviewer independence, role set, finding/convergence semantics, terminal metadata, or review dependency changes. |
| `PB-HND-001` | Implement semantic-preserving held/rejected adapter behavior after shared interface review. | Fabricate, convert away null/risk/owner meaning, set Taxlane state, publish, or bypass delivery/security/review. | Field mapping, semantic owner, shared contract, digest, gate, overlap, external ownership, or handoff dependency changes. |
| `PB-RUN-001` | Sequence accepted calls and surface explicit failures. | Own domain rules, infer defaults, bypass gates, mutate artifacts, create an unreviewed interface, or become a reusable cross-domain framework. | Orchestration order, product dependency, command/output behavior, failure propagation, or public surface changes. |
| `PB-DOC-001` | Controlled VTRACE, role, review, pulse, and no-release governance updates. | Close an unknown without its acceptance condition, claim planned evidence as executed, or imply external approval/release. | Accepted artifact, role set, fixed-point status, release posture, or no-authority language changes. |
| `PB-TST-001` | Add independently owned tests tied to controlled `VER-*` identities after verification planning. | Become a product dependency, encode an unaccepted method as truth, use real prohibited data, or turn a passing test into authority. | Test target, evidence method, dependency, reviewer independence, or expected-result changes. |
| `PB-FIX-001` | Add synthetic or accepted public-aggregate fixtures with custody, identity, purpose, expected posture, and security review. | Real classified/CUI/person/operational/targeting/vulnerability content; silent golden update; product dependency. | Fixture semantics, source/custody, sensitivity, expected posture, or promotion changes. |
| `PB-GEN-001` | Receive later reproducible outputs with producer, inputs, digest, command identity, posture, and supersession. | Hand edit, treat output as source of truth, accept missing provenance, or publish without separate authority. | Artifact class, source-of-truth, regeneration, retention, path, handoff, or release disposition changes. |

Any boundary, component allocation, controlled contract placement, allowed
dependency, owner, source-of-truth rule, validation profile, test/fixture rule,
or generated-artifact rule change creates a new package-boundary digest and
requires independent package, applicable role, editorial, and both assurance
reviews. A change that alters logical responsibility, contract semantics,
security re-admission, terminal review, pathway ordering, Taxlane/release
isolation, protected sets, or accepted risk must first reopen the architecture
or specification artifact that owns that meaning.

## Planned language and validation tailoring

These are prospective commands, not executed evidence. There is no Cargo
workspace, crate, test target, implementation, accepted fixture set, or
generator. Therefore every Rust/docs L0, L1, and L2 profile below is
**unavailable and unexecuted** until later accepted implementation work creates
the named boundaries and a verification plan accepts the commands. Naming a
command here does not authorize running implementation work now.

| Boundary set | Planned rigor profile | Planned L0 | Planned L1 | Planned L2 | Current posture |
|---|---|---|---|---|---|
| `PB-WS-001` | Rust workspace coordination | accepted WP-WS format/N/A execution | accepted WP-WS workspace/static/supply-chain execution | accepted fixed-edge/no-semantics model and adversarial execution | present; accepted empty membership; no product crate or semantics. |
| All Rust product and runner boundaries | Rust product | `cargo fmt --all --check` | `cargo clippy --workspace --all-targets -- -D warnings` | `cargo test --workspace --all-targets` | unavailable; unexecuted; no product or runner crate exists. |
| `PB-TST-001` | Rust test-only | `cargo fmt --all --check` | `cargo test -p bastion-boundary-tests --no-run` | `cargo test -p bastion-boundary-tests` | unavailable; unexecuted; test package does not exist. |
| `PB-DOC-001` | docs/trace/role governance | `git diff --check` | `cargo test -p bastion-boundary-tests --test docs_contracts` | `cargo test -p bastion-boundary-tests --test trace_and_role_coverage` | planned profile unavailable as accepted evidence and unexecuted; later test targets do not exist. |
| `PB-FIX-001` | controlled test fixtures | `cargo test -p bastion-boundary-tests --test fixture_inventory --no-run` | `cargo test -p bastion-boundary-tests --test fixture_inventory` | `cargo test -p bastion-boundary-tests --test negative_and_invariant_fixtures` | unavailable; unexecuted; fixture inventory and tests do not exist. |
| `PB-GEN-001` | generated-output custody | unavailable until a generator is selected by later design | unavailable until custody verification is implemented | unavailable until deterministic regeneration and review are implemented | no generator selected; no command executed; no generated artifact accepted. |

No dependency library, feature, toolchain version, unsafe policy, performance
threshold, or deployment check is selected. Later `LANGUAGE_PROFILES.md`,
`DESIGN.md`, `VERIFICATION.md`, and accepted work packages must tailor these
profiles without weakening the controlled boundaries.

## Test and fixture isolation

- `PB-TST-001` and `PB-FIX-001` are non-product boundaries. Product libraries
  and the runner may never import, read, or require them for normal behavior.
- Test-only dependencies flow from `PB-TST-001` to products and fixtures. No
  reverse edge is permitted.
- Negative security fixtures use inert synthetic markers and structures, not
  real classified, CUI, person-level, sensitive operational, targeting,
  operational-planning, or exploitable-vulnerability content.
- A fixture records identity, purpose, source/custody posture, expected state,
  applicable unknown hold, and review posture. A changed expected output is a
  reviewed fixture successor, not a silent golden-file update.
- Planned tests must include per-producer security-readmission bypasses,
  AUTH bypass and dangerous composition, source-transformation self-exemption,
  output-free REL, role/self-approval failures, mandatory delivery bypass,
  ECO/DEL and ECO/ADP same-version cycles, stale/missing predecessor, in-place
  mutation, terminal bundle mismatch, and product-bearing terminal receipt.
- A passing test remains planned or verification evidence only. It cannot
  create delivery readiness, official approval, Taxlane state, or release.

## Generated-artifact custody

Generated artifacts are immutable outputs. Their accepted source of truth is
the exact accepted producer version plus exact input identities/digests,
contract/gate postures, command identity, and supersession relation—not the
file under `generated/`. Hand editing is forbidden. Correction requires a new
producer/input version and regeneration.

The only accepted direction is accepted producer, `PB-HND-001`, or
`PB-REV-001` output → `PB-GEN-001` custody → read-only inspection by
`PB-TST-001` or `PB-REV-001`. No product boundary, runner, handoff, Taxlane, or
release boundary consumes `PB-GEN-001`. In particular, `PB-HND-001` produces a
held candidate bundle from accepted inputs; the bundle may be mirrored to
`PB-GEN-001` only after production, and neither `PB-HND-001` nor Taxlane reads
that mirror as a source.

No generator, command, format, serialization, storage system, or retention
technology is selected at this stage. Accordingly, every regeneration command
is explicitly unavailable.

| Planned artifact class | Semantic custodian | Source of truth | Regeneration command | Planned verification and release posture |
|---|---|---|---|---|
| Source inventory, security posture, or safe rejection receipt | `PB-CST-001` | Accepted source/input identities plus custody/security decision and exact context | unavailable; generator not selected | Digest/context and safe non-reconstruction checks; never retain prohibited content. |
| Authority manifest | `PB-AUT-001` | Accepted authority sources, owner, scope, period, version, and fresh custody security posture | unavailable; generator not selected | Completeness, no-authority-inflation, and AUTH-output readmission checks. |
| Domain result, null, or hold | `PB-DOM-001` with the applicable semantic module owner | Accepted inputs, separate domain contract, evidence, limitations, and gate postures | unavailable; generator not selected | Domain invariants, tails/facets/parties, security, and both assurance gates. |
| Preliminary/final pathway envelope, delivery posture, or adaptive history | `PB-PTH-001` with ECO/DEL/ADP semantic owners | Accepted domain inputs and immutable predecessor/successor chain | unavailable; generator not selected | Ordering, non-additivity, delivery, null, overlap, history, and cycle-rejection checks. |
| Review packet, finding, dissent, or nonterminal decision | `PB-REV-001` | Frozen admitted artifact digest plus independent review inputs and role dispositions | unavailable; generator not selected | Reproduction, independence, finding completeness, dissent, and stale-digest checks. |
| Immutable held candidate bundle or rejection | `PB-HND-001` | Final accepted pathway/delivery/ADP inputs and semantic-owner concurrences; `PB-HND-001` produces the candidate before any optional mirror to `PB-GEN-001` | unavailable; generator not selected | Semantic round trip, exact bundle digest/context, holds, and no inferred Taxlane state; only `PB-TST-001`/`PB-REV-001` may inspect the mirror read-only, while `PB-HND-001` and Taxlane never consume it as source. |
| Finite terminal receipt | `PB-REV-001` | Unchanged admitted candidate-bundle identity/posture and independent terminal decision | unavailable; generator not selected | Minimum non-reconstructive metadata only; no product content and no recursive review loop. |

`ARC-REL-001`/`PB-DOC-001` has no generated product artifact. Nothing in
`PB-GEN-001` is public or publishable by default. Any future release requires
separate authority and a release-specific fixed point.

## Work-package touch and integration rules

1. Every future `WP-*` names exactly one primary `PB-*` boundary, the exact
   permitted paths, objective, stop condition, and validation/evidence scope.
2. A work package that touches a second boundary names the controlled contract
   crossing and requires an accepted `INTEGRATION_PLAN.md` before integration.
3. Any new or changed public seam requires fixed `INTERFACES.md` first. A work
   package cannot invent fields, signatures, encoding, defaults, compatibility,
   or a shared type.
4. Workspace membership changes belong to `PB-WS-001` and must be separated
   from product implementation unless the accepted work package explicitly
   names both the primary boundary and integration step.
5. Product work never edits `PB-TST-001`, `PB-FIX-001`, or `PB-GEN-001` as a
   convenience. Test, fixture, and regeneration changes are separately scoped
   review surfaces.
6. Generated outputs are never hand-edited. A work package may regenerate only
   when a later accepted design names the generator, source of truth, command,
   custody, and deterministic verification.
7. A cross-boundary security, civilian-authority, safety/readiness, handoff, or
   release touch requires the applicable semantic owners, independent review,
   Scope Keeper, and both assurance roles. Majority agreement cannot waive a
   failed assurance gate.
8. `PB-RUN-001` integration work may sequence only accepted contracts. It may
   not absorb domain logic to avoid an interface review.
9. No work package may close a `SPEC-UNK-*`, infer a missing value, fabricate a
   delivery owner or schedule, change the ECO/DEL version order, set Taxlane
   state, or create official/release authority.
10. Child-repo implementation and validation must complete before any separate
    TRACKER pointer update; this baseline authorizes neither action.

## Taxlane, release, data, and authority boundaries

- BASTION remains public-aggregate, unclassified, non-operational, internal
  research tooling. No boundary may ingest, retain, derive, or emit classified
  information, CUI, person-level service data, sensitive operational data,
  targeting content, operational-planning content, or exploitable
  vulnerability content.
- `PB-HND-001` creates only a held/rejected BASTION-side candidate. There is no
  planned Cargo, runtime, storage, or deployment coupling to Taxlane. Taxlane
  alone may admit, combine, allocate, rebalance, or test/set rates.
- `PB-DOC-001` holds `ARC-REL-001` as a closed, output-free governance
  boundary. No crate, runner command, generated path, test result, or reviewed
  handoff implies publication.
- Civilian authority remains in `PB-AUT-001`; hard readiness/safety semantics
  remain with their domain owners; economics, delivery, review, and handoff
  cannot redefine or trade away a floor.
- Classification & Operational Security and Civilian Control, Law, Safety &
  Readiness are independent mandatory assurance gates. Passing one cannot
  compensate for failing the other.

## Alternatives considered and rejected

| Alternative | Disposition | Reason |
|---|---|---|
| One crate for each of the 13 logical components | rejected | Logical accountability does not justify 13 compile units; it would add interface and dependency overhead and invite ECO/DEL or domain-reference crate cycles before interfaces exist. |
| One monolithic BASTION crate | rejected | It would collapse trust, civilian authority, independent review, held handoff, and domain ownership boundaries and make self-review/bypass harder to detect. |
| Copy the full Infrastructure 2.0 twelve-crate role profile now | rejected | The pattern is guidance, not a mandate. BASTION methods, interfaces, corpus, score, simulation, optimization, and CLI behavior remain unresolved and cannot justify those crates. |
| Generic cross-domain `infrastructure-core` or shared contracts crate | rejected | No three-domain portability proof and no accepted BASTION interface representation exists. Duplication risk is preferable to imposing road, health, education, or other domain semantics on defense. |
| Direct sibling product-crate dependencies | rejected for this baseline | They could create build cycles or bypass the runner's security/review sequencing. A later exception requires fixed interfaces, explicit direction, and DAG review. |
| Merge review with the producer, runner, or handoff crate | rejected | It would weaken reviewer independence, enable self-approval, or blur the finite terminal decision. |
| Implement `ARC-REL-001` as a release crate or runner command | rejected | The architecture makes REL output-free and release unauthorized. |
| Put fixtures or generated outputs inside product crates | rejected | Product behavior must not depend on test evidence or mutable/generated material. |
| Treat generated output as hand-maintained source | rejected | It destroys reproducibility, custody, digest binding, and supersession history. |

## Open risks and next-stage gates

- All 13 exact `SPEC-UNK-*` controls remain open and promotion-gating. They
  block affected methods, values, verification, delivery, implementation,
  handoff, and release without changing this boundary inventory.
- `PB-DOM-001` and `PB-PTH-001` are deliberately grouped. Later interface or
  ownership evidence may justify a split, but any split must preserve semantic
  owners, contracts, holds, DAG direction, and the pathway ordering invariant.
- The runner could accumulate semantic logic. Boundary and integration review
  must keep it a thin BASTION-only application shell.
- Avoiding a shared primitive crate may create early duplication. Extraction
  remains deferred until at least three independent domain implementations
  prove the same type or invariant and a reviewed change preserves BASTION
  semantics.
- Exact Rust types, crate features, test targets, fixtures, generated paths,
  and command behavior remain unknown until interfaces/design/verification and
  accepted work packages. None may be inferred from the planned names here.
- There is no accepted corpus, delivery evidence, realization evidence,
  implementation, test harness, generator, Taxlane interface, or release plan.
- Physical placement does not prove feasibility. Later verification must test
  build-DAG direction, component allocation, controlled contract ownership,
  role participation, security re-admission, failure containment, version
  ordering, terminal handoff, and no-authority behavior.

## Package-boundary disposition

This planned baseline assigns every one of the 13 logical components to one
accountable physical boundary, places all 13 controlled contracts exactly once
as public semantic surfaces, preserves all 98 SPEC allocations, 10
nonfunctional constraints, 13 exact unknown holds, and 21 role lenses by
reference, and defines an acyclic planned build graph plus controlled runtime
direction.

Disposition: **review-ready planned package-boundary baseline with 13 inherited
holds; not a fixed point**. Independent digest-bound package, domain-role,
editorial, methodology, and both assurance reviews are required before any
decision to advance. `INTERFACES.md` is not yet eligible merely because this
draft exists.

No workspace, Cargo manifest, crate, Rust source, test, fixture, generated
artifact, corpus, interface, API, schema, storage, algorithm, dependency,
runtime, deployment, delivery evidence, Taxlane action, official action,
commit, push, remote mutation, or public release was created or authorized.
