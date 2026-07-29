# Pulse 04 — Logical architecture draft

Date: 2026-07-28
Assignment: `ASG-BASTION-ARCHITECTURE-001`
Writer lease: exclusive to the BASTION architecture author

Remediation assignment: `ASG-BASTION-ARCHITECTURE-REMEDIATION-001`
Frozen remediation input SHA-256:
`bbba7ce9a25036ed274f5c0f76117a197f27470e02f9d916672d861245ce76a4`

Second remediation assignment: `ASG-BASTION-ARCHITECTURE-REMEDIATION-002`
Frozen second-remediation input SHA-256:
`ea191083d20ae98364cd1af016ae84a7936c4674c5d816215e5919d13eb004c4`

Controlled SPECIFICATION_BASELINE input SHA-256:
`48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b`

Review-ready `docs/vtrace/ARCHITECTURE.md` output SHA-256:
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`

Frozen Pulse 04 input reviewed by convergence-002 SHA-256:
`f83c14aa6db19f25a05e642ccdd808bbc0fcf935b4b81396b984a0ea58b23775`

## Objective

Allocate the fixed-point specification baseline to reviewable logical
responsibilities, flows, trust boundaries, failure containment and planned
verification without choosing physical packages, interface encodings or an
implementation.

## Inputs read

- complete BASTION governance, product, role-manifest and all 21 role files;
- complete `MISSION.md`, `CONOPS.md`, `REQUIREMENTS.md` and settled
  `SPECIFICATION_BASELINE.md`;
- VTRACE architecture adoption template, process, NASA specificity map and
  staged-execution guidance; and
- TRACKER Infrastructure 2.0 common pattern.

The assignment named the common-pattern file with `2.0`; the controlled repo
filename is `docs/reading/infrastructure-2-0-common-pattern.md`. That file was
found and read in full.

## Output summary

- Defined 13 conceptual components with exclusive primary allocation of all 98
  controlled SPEC rows and retention of their one-to-one REQ relationships.
- Allocated all 13 logical contracts and all 10 nonfunctional constraints.
- Defined logical interface references, evidence/control flows, generated
  artifact custody, allowed/forbidden dependency directions and fail-closed
  degraded behavior.
- Preserved exact propagation of all 13 open `SPEC-UNK-*` holds.
- Kept public aggregate unclassified scope and the absolute prohibited-data
  boundary across ingest, retention, derivation and emission.
- Preserved civilian-control, law, safety/readiness, semantic-owner and fiscal
  authority boundaries.
- Kept peer gaps diagnostic only and Taxlane solely responsible for admission,
  cross-lane combination, allocation, rebalance and rates. BASTION validates
  lifecycle state and cannot rebalance.
- Preserved nulls, reviewed N/A, negative evidence, realization-owner custody,
  adaptive reopen triggers and new-version review.
- Added mandatory digest/context-bound security re-admission for every derived,
  retained, visualized, composed, or emitted output, with downstream
  invalidation, safe non-reconstructive receipts, and planned per-output bypass
  and dangerous-composition fixtures. The producer set explicitly includes
  AUTH and SRC transformations; AUTH manifests pass the same gate before use,
  and REL emits nothing.
- Closed final-handoff security recursion with an immutable candidate bundle,
  exact bundle admission, and an independent finite terminal decision whose
  receipt contains only minimal non-reconstructive governance metadata. New
  product content starts a new freeze/admission/review sequence.
- Made delivery posture mandatory for candidate, pathway, domain-floor, null,
  and research-hypothesis branches before realizability, final review,
  adaptive disposition, or handoff; all ten `DEL-006` deviations remain
  separate and delivery holds propagate to ECO, REV, ADP, and HND.
- Replaced same-version ECO/DEL reciprocity with the acyclic immutable
  `preliminary ECO[n] → DEL[n] → custody/security/review → final ECO[n+1]
  → ADP[n+1]` sequence, preserving mandatory delivery and `SPEC-ECO-006`.
  Planned negative checks cover cycle, stale posture, missing/mismatched
  predecessor, bypass and in-place mutation; later feedback must repeat the
  delivery gate through a new preliminary successor.
- Completed null custody with observation owner, cadence, reopen/history and
  reviewed N/A for genuinely inapplicable realization owner/schedule fields,
  without fabricated values.
- Restored the controlled `gross opportunity` term and made any proposed
  `gross funding need` concept subject to a specification change.
- Allocated all 21 parliament, stakeholder, editorial, assurance, and
  methodology roles to components, gates, harms, and misuse checks, including
  Mission User, Taxpayer & Oversight Body, both assurance gates, and
  self-approval refusal.
- Added architecture-local digest change control, independent re-review,
  downstream invalidation/reconciliation, and specification-reopen triggers.
- Explicitly deferred physical/package/language boundaries to
  `PACKAGE_BOUNDARIES.md` and contract fields/encodings to `INTERFACES.md`.

## Remediation dispositions

| Consolidated finding | Disposition |
|---|---|
| Derived-output composition-security gap | remediated with a mandatory per-output `LIF-SECURITY-READMISSION` edge, exact context triggers, fail-closed downstream invalidation, safe receipts, and planned bypass/composition fixtures; `SPEC-UNK-SEC-001` remains open. |
| Delivery bypass and optional-delivery posture | remediated with mandatory `ARC-DEL-001`/`CONTRACT-DEL-001` posture before realizability, final review, adaptive disposition and handoff for every candidate/pathway/domain-floor/null/research branch; missing authority remains held. |
| Same-version ECO/ADP cycle | remediated with immutable version direction and custody/security/review-bound successor creation only. |
| Incomplete null custody | remediated with observation/custody owner, cadence, reopen triggers, retained history, reviewed N/A, and no fabricated owner/schedule/zero. |
| Fiscal term drift | remediated by restoring `gross opportunity`; changing specification semantics remains outside architecture authority. |
| Incomplete role/harm allocation | remediated with an exact 21-role matrix, all `DEL-006` deviations, both assurance gates, independence, dissent, and no self-approval. |
| Missing architecture change control | remediated with controlled triggers, new digest, stale-review handling, independent re-review, downstream reconciliation, and specification-stage reopen conditions. |
| AUTH/security readmission and final-review recursion | remediated by explicitly gating AUTH and SRC transformation outputs, making REL output-free, and ending handoff with a digest-bound finite terminal decision over an unchanged admitted bundle; AUTH bypass, dangerous composition, stale/mismatched bundle and product-bearing receipt fixtures are planned. |
| Same-version ECO/DEL cycle | remediated with immutable preliminary and final economic versions separated by mandatory DEL custody/security/review; ADP consumes only the final envelope, no delivery change mutates its predecessor, and cycle/predecessor/staleness/bypass/in-place negative fixtures are planned. |

No TBD, direct unknown dependency set, physical boundary, interface encoding,
implementation choice, Taxlane state, release authority, or official action was
selected by remediation.

## Review, remediation, dissent and convergence sequence

| Sequence | Digest-bound record | Result |
|---|---|---|
| Initial architecture review | Architecture `bbba7ce9a25036ed274f5c0f76117a197f27470e02f9d916672d861245ce76a4` | Findings required remediation; no fixed point was recorded. |
| Remediation 001 | Architecture successor `ea191083d20ae98364cd1af016ae84a7936c4674c5d816215e5919d13eb004c4` | Addressed composition-security, delivery, ECO/ADP, null-custody, fiscal-term, role/harm and change-control findings while retaining all protected sets and holds. |
| Convergence 001 | Architecture `ea191083d20ae98364cd1af016ae84a7936c4674c5d816215e5919d13eb004c4` | Two major findings remained: complete AUTH/security readmission with a finite terminal review, and removal of the same-version ECO/DEL cycle. Advancement remained held. |
| Remediation 002 | Architecture successor `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` | Closed both majors with explicit AUTH/SRC output gates, output-free REL, finite immutable-bundle terminal review, and acyclic preliminary-ECO/DEL/final-ECO/ADP ordering. |
| Two independent convergence-002 reviews | Architecture `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`; Pulse input `f83c14aa6db19f25a05e642ccdd808bbc0fcf935b4b81396b984a0ea58b23775` | Both reviewers found both majors closed, no new findings, exact protected sets retained, and architecture eligible for `pass_with_risk`. |
| Dissent disposition | Same frozen architecture and Pulse inputs | No unresolved dissent was reported by either convergence-002 reviewer. The architecture still requires future review records to retain any later dissent; absence of current dissent does not close an inherited unknown. |

## Digest-bound fixed-point decision

Decision: **`pass_with_risk` architecture fixed point** for
`ARCHITECTURE.md` digest
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`,
based only on the two independent convergence-002 reviews of that exact
architecture digest and frozen Pulse input digest
`f83c14aa6db19f25a05e642ccdd808bbc0fcf935b4b81396b984a0ea58b23775`.

The accepted risk is bounded to the 13 inherited `SPEC-UNK-*` holds. They
remain open, exact, non-defaulted, and binding on every dependent method,
value, interface, package, verification, delivery, implementation, handoff and
release claim. This decision fixes the logical architecture only. It does not
represent planned verification as executed and creates no delivery evidence,
corpus, interface, package, implementation, Taxlane, budget, allocation, rate,
official-use, public-release or other external authority.

`PACKAGE_BOUNDARIES.md` is the next eligible architecture stage, but work on it
may begin only under a new explicit assignment bound to this fixed architecture
digest. This decision does not itself authorize package work.

## Validation

Author checks found:

- controlled SPEC digest: exact match;
- 98 unique controlled SPEC IDs and 98 exclusive architecture allocations;
- zero missing, extra or duplicate primary allocations;
- 13 architecture components, 13 logical contracts, 13 open unknowns and 10
  nonfunctional constraints represented;
- all 13 unknown dependent-item sets exactly equal to the controlled
  specification rows, including the SEC, ECO, and HND hold sets;
- 21 role files present, all 21 represented exactly once in the role/harm
  matrix, and zero role-manifest paths missing;
- no permitted reciprocal same-version ECO/DEL or ECO/ADP edge; the documented
  negative occurrences are explicit rejection and verification fixtures;
- AUTH and SRC transformations are explicit per-output security producers,
  REL emits nothing, and final handoff has a finite non-product terminal
  decision bound to the exact admitted bundle;
- preliminary ECO, mandatory DEL, custody/security/review, final ECO and ADP
  ordering is explicit, including `SPEC-ECO-006`;
- controlled `gross opportunity` terminology present; and
- zero trailing-whitespace findings in `ARCHITECTURE.md`.

Verification mappings are planned evidence only; no verification or validation
result is represented as executed.

## Limitations and blockers

- Status is **digest-bound `pass_with_risk` architecture fixed point** for the
  exact architecture digest recorded above. Any architecture content change
  invalidates this decision and requires a new review sequence.
- All 13 inherited unknowns remain open and promotion-gating; none was closed,
  defaulted or converted to zero.
- No corpus, interface, package, schema, persistence technology, runtime,
  algorithm, test harness, delivery capability or implementation is claimed.
- No force, procurement, operational, budget, Taxlane, allocation, rebalance,
  rate, official-use or public-release authority is created.
- Release remains closed pending separate authority and a release-specific
  fixed point.

Current author blocker count: **zero**. Inherited promotion holds: **13**.

## Disposition

`ARCHITECTURE.md` digest
`c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b`
is recorded as a `pass_with_risk` logical-architecture fixed point after two
independent convergence-002 reviews. The 13 inherited unknown holds and all
no-authority boundaries remain in force. `PACKAGE_BOUNDARIES.md` is next
eligible only under a new assignment. No package work, commit, push, remote
mutation, Taxlane action, official action or public release was performed.
