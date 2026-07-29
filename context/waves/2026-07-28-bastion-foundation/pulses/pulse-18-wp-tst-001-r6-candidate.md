# Pulse 18 — WP-TST-001 retained R6 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R1 governance commit | `62116481b7b3e7d671517b6053c8cc3f20f93fce` |
| Retained R1 candidate SHA-256 / blob | `93ea15ea87b140b7e45ae67db5a4133e24e8f18778db1ce41a891042b1157554` / `65fabe5060cb6c9b9cf7ea2f0c5e88ebec9c178d` |
| Retained R2 governance commit | `21c8066445c72358a444c0b506422ec3b9dc63e0` |
| Retained R2 candidate SHA-256 / blob | `4ecd246d67bb5d07c94496a9975c99cdc8488295e8e74235be29391b3725e146` / `47687aff86c392b7e30b237de1015b9d304d4fc4` |
| Retained R3 governance commit | `ae64448e98744668e3b80e3411255503bfbdd4ae` |
| Retained R3 candidate SHA-256 / blob | `76f259e3189cbb53be5e88b84dc922a13673ec52572efbe842f55fe85a67c2ae` / `655f38734b4f52ed7ff740fd2117c3cd5916f977` |
| Retained R4 governance commit | `b919512fb73472149afea5a55d1a022bf6aec8da` |
| Retained R4 candidate SHA-256 / blob | `eaff0bd15d34afb533306ab5a4a967cb672149422e14b634ae263fea512f4f70` / `18e616868d9f94b97264e4b744961d85b6442f3d` |
| Retained R5 governance commit | `77e0abb94a427a1f824e4f5659e580b1aae74137` |
| Retained R5 candidate SHA-256 / blob | `c618af6d61d05c51fe689a791f7a8bc9f2ed908c4c42e7e48dd07badec2a633d` / `42f8ff4bd0e9350ac269b0a3a137209b1be1f120` |
| Retained R5 pulse SHA-256 / blob | `f231cc4684943275771cd06056abad619b8d5d8ea6c2587de52776a9da114382` / `30b7c92da8e21de9e7177779ef6d4e4127f095a4` |
| R6 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R6` |
| R6 candidate SHA-256 | `e155df20adac753a6b92cf2f36205233626c98551b0c7ac8459ff3a975dd0ced` |
| R6 candidate Git blob | `34ba2c245c49478e32186fb5f7e4581e4a755847` |

R1 through R5 are immutable, were not accepted, and remain recoverable at the
exact commits, digests, and blobs above. R6 is their proposed successor, not an
in-place finding erasure.

## Retained finding disposition

R1 through R4 findings and their R2 through R5 remediation statements remain
exactly retained in Pulse 17. R6 does not revise or claim acceptance of them.

| Finding | R5 disposition | R6 remediation |
|---|---|---|
| `BA-TST-R5-M01` | retained major; R5 not accepted | one normative canonical/type library and fully closed mode/set definitions now specify every field and nested member, scalar domain, null posture, enum/regex/equality, key order, array order/cardinality/uniqueness, conditional result/review coherence, embedded output/result/plan shape, diagnostic record, ID formula, reviewer identity, and assurance constraint; the set schema either imports an exact named closed definition or restates its complete specialization. |
| `BA-TST-R5-M02` | retained major; R5 not accepted | a one-lane successor now makes every named non-derived field and the other 21 slots byte-identical; the only synchronized delta is enumerated, decision/lane/version/predecessor equalities are exact, IDs are formula-bound and unique, projections reconcile only the changed decision, and the same constrained transition is independently stated for the set plus its 16 corresponding same-lane mode successors and aggregate. |

## Candidate boundary

R6 retains R5's exact 123 canonical identities, 148 forward/reverse transpose
edges, 38 CR identities, 63 CR edges, 16 command modes, 22 canonical review
lanes, implementation allowlist, package graph, fixtures, bounds, no-authority
surface, and create-new evidence custody. Section 7 is byte-identical to R5.

The only substantive change is schema and transition closure. Mode evidence is
`test-gate-evidence.v3`; set evidence is `test-gate-evidence-set.v2`. Embedded
fragments remain bounded and pointer-local. Digest preimages omit only their own
last digest member and are acyclic. Acceptance and entry retain the staged,
non-self-binding commit/pulse sequence.

## Required independent review

Acceptance remains pending. Every required decision must bind the exact R6
candidate digest above. Review must include all eight parliament lanes,
Independent Test & Oversight, Methodology Panel, Role review, Citation, Scope,
Numeracy, all seven stakeholder lenses, Classification & Operational Security,
and Civilian Control/Law/Safety/Readiness. Any changed digest, unresolved
critical or major finding, open evidence conflict, failed assurance gate, or
scope/authority ambiguity requires another retained successor candidate.

## No decision or authority

This pulse records a candidate for review only. It does not accept or enter the
WP; create a crate, fixture, runner, command, evidence record, or set; close any
held pair; accept product evidence; authorize a semantic producer; emit HND,
TERM, REL, or Taxlane state; support operational, force, procurement, budget,
allocation, or rate action; authorize official use; publish; push; or release.

Disposition: **R1/R2/R3/R4/R5 findings retained and remediated only in proposed
R6; independent acceptance review required; not accepted; not entered**.
