# Pulse 09 — implementation-planning bootstrap successor record

Date: 2026-07-28

Assignment: `ASG-BASTION-IMPLEMENTATION-PLANNING-001`

Writer lease: exclusive `docs/vtrace/IMPLEMENTATION_PLAN.md`,
`docs/vtrace/WORK_PACKAGES.md`, and Pulse 09 author

## Objective

Record the controlled bootstrap successor to the fixed BASTION implementation
plan and work-package graph without creating code, workspace
state, a dependency, a concrete command, a toolchain selection, executed
evidence, hold closure, HND emission, or implementation authority.

## Controlled inputs and outputs

| Controlled artifact | SHA-256 |
|---|---|
| Fixed `MISSION.md` | `39a22bff4344a9bb0af8cdd87eb726d48a2f729c68cf1503c4145ba8253b2d5a` |
| Fixed `CONOPS.md` | `a93936dc6b344311f515bd18043355fd6794bf234778a24f6e9c20c8067b8602` |
| Fixed `REQUIREMENTS.md` | `9d6ea120ac2fd47ff13729a0259c776443984ed0dfe617a9c904553a65906d0e` |
| Fixed `SPECIFICATION_BASELINE.md` | `48ee88da65e633e7c5400b10e383f21b9bc66ac608eb23ee3d70646a9520751b` |
| Fixed `ARCHITECTURE.md` | `c5e125e7f5b40089962d0f9c5ef6580b3886caf1e54745c4e5e36268aadbef9b` |
| Fixed `PACKAGE_BOUNDARIES.md` | `43c10ac20da41640735ce1d1916566b2b8494acf7fe465a8d5c9f628af7e3695` |
| Fixed `INTERFACES.md` | `18e87ea802b252f379b739b484da954ccbe676f3f7c5244f3f8bfe0dbb6fb882` |
| Review-ready `CHANGE_CONTROL.md` / prospective `CHG-BA-TST-BOOT-002` | `ed259afd24fc39ea87d8b21c4b5a535a2b369c5558e9dd6e2f25b1e070deaa3c` |
| Fixed-point `DESIGN.md` | `4ca1d487936b2f83807c1d6f06660005c7055cf3b4ec0c9ccfcb21d254eeb781` |
| Fixed-point `CODE_RIGOR.md` | `3ce31b808f038291b79bf348547ed43029ca1b0a797520fbb72546d3964801d9` |
| CODE_RIGOR fixed-point record `pulse-08-code-rigor.md` | `501c1f23136b2939d94647204536f0b9d49902b97987533d5968f2c768e1eee6` |
| Bootstrap-successor `IMPLEMENTATION_PLAN.md` output | `e1ce5cfa603e491d21d7a71859c610316e62fc49caba1c0687c59b8b8baa15c7` |
| Bootstrap-successor `WORK_PACKAGES.md` output | `5fa703f2f93c5748da5f4cb800cbffe0b153747f006d6f116c577b6f90b412e3` |

The ten unchanged upstream artifact digests matched before successor
authoring. The review-ready change-control digest is the prospective control
input; it is not represented as an accepted fixed point. No other artifact
was changed.

## Author result

The planning bundle now defines:

- an exact 298-identity source universe: 98 requirements, 98 functional
  specifications, 10 non-functional specifications, 13 controlled unknowns,
  14 DESIGN decisions, 13 contracts, 12 package boundaries, and 40 code-rigor
  constraints;
- exactly 20 current work packages: one planning-only discovery package and
  19 proposed implementation/support packages whose entry is blocked;
- a finite, acyclic dependency graph with bootstrap-only REV and TST
  substrates, producer-owned sidecar deltas, separate preliminary and final
  economic packages, and final integration only after GEN and DOC;
- exact current package aliases without silently counting future handoff
  emission as a current package;
- `CTRL-HND-EMIT-001` as a deferred non-WP control requiring a new assignment,
  successor plan, accepted work package, exact mapping, and closure of all 12
  applicable non-release holds before any future pack emission; `TBD-REL-001`
  never becomes an HND-emission prerequisite and remains separately
  release-blocking;
- per-WP rows for all ten V-closure areas, all eight parliament assurance
  lanes, domain assurance, both independent formal assurance gates, and
  symbolic L0/L1/L2 obligations;
- universal accepted VERIFICATION and VALIDATION plans, accepted exact WP
  scope, applicable hold closure, representation, bounds, fixtures, commands,
  evidence destinations, owners, and assurance as implementation-entry gates;
- the universal producer `freeze -> SOURCE -> TEST -> consumer` sequence;
- finite HND and TERM behavior, no receipt/product backflow, external Taxlane
  exclusivity, and REL no-output/no-consumer behavior; and
- final INT closure against 13 contracts, 25 invariants, 18 transitions, all
  13 holds, the final source spine, support isolation, and forbidden edges.

The bundle applies the final ANCHOR remediation lessons at initial authoring:
the full controlled source disposition is explicit; the current WP set and
future HND emission control cannot be conflated; V-closure and assurance are
transposed per WP; REV/TST bootstrap boundaries do not own producer evidence;
and INT cannot bind a stale pre-GEN/DOC state.

## Bounded remediation

Independent review identified three planning defects. `BA-IP-B-001` moved the
empty REV/TST scaffolds before CST and removed every product prerequisite from
their bootstrap WPs; CST is now the first semantic producer and every later
sidecar delta remains producer-owned. `BA-IP-B-002` added exact 20-row forward
and 12-row reverse boundary-touch transposes: all 15 semantic/support producers
touch their primary boundary plus `PB-REV-001` and `PB-TST-001`, TST owns the
assigned `PB-FIX-001` bootstrap custody, and all 63 touch pairs reconcile.
`BA-IP-B-003` replaced prose hold propagation with exact blocker,
proof-input, and deferred-control sets plus their WP reverse transpose; all 137
blocker pairs and 8 proof-input pairs reconcile.

Final review identified one wording contradiction. `BA-IP-B-004` aligns the
plan with the already authoritative deferred-control allocation: HND emission
is gated by exactly 12 applicable non-release holds, while `TBD-REL-001` is
excluded from that set, never becomes an HND-emission prerequisite, and
continues to block the separate future release chain.

Prospective `BA-IP-B-005` applies `CHG-BA-TST-BOOT-002`: the TST held pair now
blocks the exact 16-member product/post-product set, is a proof input for pure
REV/TST bootstrap only, and has no WS relationship. This removes exactly three
TST blocker pairs, adds exactly two TST proof-input pairs, and yields 134
blocker plus 10 proof-input pairs. Every non-TST allocation remains unchanged.
Future eligibility is V&V fixed point, separate WS acceptance/entry/exit, then
separate REV/TST bootstrap acceptance/entry; TST closure still requires later
executed independent fixtures and accepted evidence.

No controlled source identity, WP identity, hold identity, semantic decision,
authority, current acceptance, executed evidence, hold closure, or emission
posture changed.

## Author validation

| Check | Result |
|---|---|
| Controlled input digests | 10 unchanged upstream exact matches plus one review-ready change-control successor digest |
| Controlled source disposition | 298 expected and unique: 98 REQ + 98 functional SPEC + 10 NF + 13 UNK + 14 DES + 13 contracts + 12 PB + 40 CR; zero missing or extra |
| Current WP register | 20 unique: 1 `discovery`; 19 `proposed; entry_blocked` |
| Dependency graph | 20 nodes; acyclic; REV/TST bootstrap after WS and before CST; GEN and DOC precede final INT |
| Package-boundary touch transpose | 20 WP rows and 12 PB rows; 63 exact pairs; zero forward-only or reverse-only pairs; all 15 sidecar producers touch REV and TST |
| Deferred HND emission | One non-WP control; exact 12 non-release-hold reverse allocation; `TBD-REL-001` excluded and separately release-blocking; no emission authorized |
| V-closure transpose | 20 WP rows; each covers exact `VCL-ALL` with a WP-specific focus or reasoned planning N/A |
| Assurance transpose | 20 WP rows; each addresses exact `PAR-ALL`, domain assurance, and both formal assurance gates |
| Verification levels | 20 WP rows; L0/L1/L2 slots symbolic and unavailable; no concrete command selected |
| Entry and exit matrix | 20 WP rows; universal accepted-plan and hold gates retained |
| Bootstrap boundary | WS has no TST relationship; REV/TST carry TST as proof input only after separate future acceptance; exact producer evidence deltas remain producer-owned |
| Controlled unknown preservation | 13 unique `SPEC-UNK-*` / `TBD-*` pairs; 134 blocker and 10 proof-input pairs in exact forward/reverse equality; one-to-one; zero closed |
| Finite graph protections | Preliminary ECO before DEL, final ECO after DEL, ADP successor only; HND/TERM finite; REL no output; no backflow |
| Representation boundary | No code, workspace, schema, API, dependency, concrete command, toolchain, fixture value, generator, deployment, or runtime selected |
| Assigned-file scope | Only the two planning documents and this Pulse 09 authored |
| Whitespace | Zero trailing-whitespace lines in both planning outputs |

These are author-side structural and semantic checks. Standing alone, they are
not independent review, executed verification or validation evidence,
acceptance, or convergence. The independent planning reviews and bounded
governance decision are recorded below.

## Superseded historical convergence decision

Decision date: 2026-07-29

Two independent, digest-bound reviews converged on the prior exact planning
bundle. Their decisions remain historical and do not apply to the successor
digests:

| Review | Bound scope | Result |
|---|---|---|
| Independent implementation-planning cross-review | Exact 298-source disposition; 20-WP register and acyclic bootstrap/DAG; 63-pair package-boundary transpose; 137-pair blocker and 8-pair proof-input hold transposes; all V-closure, parliament/domain/formal assurance, L0/L1/L2, entry/exit, HND/TERM/REL, and authority boundaries | Zero findings; recommended governance-only `pass_with_risk`. |
| Independent final convergence recheck | `BA-IP-B-001` through `BA-IP-B-004`; exact pre-CST REV/TST scaffolding; producer-owned sidecars; final post-GEN/DOC INT; exact 12 non-release-hold HND-control allocation with `TBD-REL-001` separately release-blocking; regression of counts, transposes, blocks, and prohibited authority | Zero findings; remediation converged; recommended governance-only `pass_with_risk`. |

The historical BASTION maintainer/stage-controller decision was a
**governance-only `pass_with_risk` implementation-planning fixed point** for
`IMPLEMENTATION_PLAN.md` SHA-256
`adf5f7d30f76aac7c8aad10bc3ec61220f13ef4181fc8b87bd89d1a5fe4a9900`
and `WORK_PACKAGES.md` SHA-256
`04089d6de81ca0acfecb309eeff5c53988a500ab5778edcfd4c2c4dd0736908c`.

That decision fixed only its repo-local planning baseline. The successor
content and digests above supersede it for future use and require new
independent convergence, recorded below. The historical decision accepted no
implementation WP, method, command, evidence result, hold closure, HND
emission, external action, or authority. All 19 implementation WPs remain
unaccepted and `entry_blocked`; all 13 holds remain open; and accepted
`VERIFICATION.md` and `VALIDATION.md` fixed points remain absent.

## Successor independent convergence and sequential decisions

Two independent zero-finding reviews bound the prospective change and exact
successor planning bundle:

| Review | Bound scope | Result |
|---|---|---|
| Independent bootstrap change/planning cross-review | `CHANGE_CONTROL.md` `ed259afd24fc39ea87d8b21c4b5a535a2b369c5558e9dd6e2f25b1e070deaa3c`; PLAN `e1ce5cfa603e491d21d7a71859c610316e62fc49caba1c0687c59b8b8baa15c7`; WP `5fa703f2f93c5748da5f4cb800cbffe0b153747f006d6f116c577b6f90b412e3`; exact WS→REV/TST eligibility; TST 16-blocker bootstrap allocation | Zero findings; recommended governance-only acceptance of `CHG-BA-TST-BOOT-002`, then `pass_with_risk` for the bound planning successors. |
| Independent final transpose and authority recheck | 13 holds; 134 blocker and 10 proof-input pairs in exact forward/reverse equality; all non-TST allocations unchanged; 298 controlled identities; 20 WPs; producer-owned sidecars; HND/TERM/REL and no-authority boundaries | Zero findings; remediation converged; recommended the same sequential governance decisions. |

The decisions are recorded in dependency order:

1. The BASTION maintainer/stage-controller **accepts the governance-only
   prospective decision `CHG-BA-TST-BOOT-002`** at `CHANGE_CONTROL.md`
   SHA-256
   `ed259afd24fc39ea87d8b21c4b5a535a2b369c5558e9dd6e2f25b1e070deaa3c`.
   This accepts only the exact stage/WP allocation meaning. It accepts no WP,
   fixture, command, evidence, execution, hold closure, implementation, or
   external authority.
2. Only after decision 1, the BASTION maintainer/stage-controller records a
   **governance-only `pass_with_risk` superseding implementation-planning
   fixed point** for `IMPLEMENTATION_PLAN.md` SHA-256
   `e1ce5cfa603e491d21d7a71859c610316e62fc49caba1c0687c59b8b8baa15c7`
   and `WORK_PACKAGES.md` SHA-256
   `5fa703f2f93c5748da5f4cb800cbffe0b153747f006d6f116c577b6f90b412e3`.

This fixed point supersedes the prior PLAN/WP digests for future use while
retaining them as history. It fixes the planning graph and transposes only.
It neither accepts nor enters any of the 19 proposed WPs. All 13 holds remain
open, and the successor V&V plans still require their own later fixed-point
decision.

## Protected boundaries and residual risk

- All 13 holds remain open and conjunctive. `TBD-TST-001` /
  `SPEC-UNK-TST-001` blocks all 16 semantic/post-product WPs. It is a proof
  input only for future pure REV/TST bootstrap, while WS has no TST
  relationship; planning permission closes neither hold.
- No current WP has accepted entry. The named L0/L1/L2 obligations have no
  accepted methods, commands, fixtures, bounds, expected results, or executed
  evidence.
- HND emits no pack. The terminal receipt is minimal and non-product, has no
  return edge, and cannot authorize Taxlane admission. REL emits no output and
  has no consumer.
- Public-aggregate, unclassified, non-operational, civilian-control,
  law/safety/readiness, distribution, stakeholder, and assurance protections
  remain mandatory. Classified, controlled, targeting, operational-planning,
  exploitable-vulnerability, or person-level service content remains
  prohibited.
- Any content change invalidates the two output digests and requires a
  controlled successor plus new independent review.

## Disposition

Disposition: **accepted governance-only `CHG-BA-TST-BOOT-002` followed by a
governance-only `pass_with_risk` superseding implementation-planning fixed
point** for the exact two successor output digests above.

The prior planning fixed point is retained as history but superseded for
future use. The successor is fixed only as planning. Implementation readiness
remains wholly blocked: all 19 implementation
WPs are unaccepted and `entry_blocked`, all 13 holds remain open, and successor
VERIFICATION and VALIDATION fixed points are absent. Any change to either
successor planning document invalidates this fixed point and requires another
controlled successor plus new independent convergence.

No code, Cargo/workspace state, dependency, schema, API, concrete command,
toolchain choice, executed evidence, hold closure, HND emission, Taxlane
action, official action, procurement, budget, allocation, rate, deployment,
or public release is authorized or created.
