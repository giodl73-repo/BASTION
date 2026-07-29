# Pulse 17 — WP-TST-001 retained R5 acceptance candidate

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
| Retained R2 pulse SHA-256 / blob | `2a2868748ce53369d68e6978b8d3d02d3a684d7aec98f6f5f0c3d6fea9a2110a` / `88b11c49e7d8ced29e1ebcb40f68bf5dc6b519ad` |
| Retained R3 governance commit | `ae64448e98744668e3b80e3411255503bfbdd4ae` |
| Retained R3 candidate SHA-256 / blob | `76f259e3189cbb53be5e88b84dc922a13673ec52572efbe842f55fe85a67c2ae` / `655f38734b4f52ed7ff740fd2117c3cd5916f977` |
| Retained R3 pulse SHA-256 / blob | `954e17de5d0833d98f0a44c932476af60c0163b126c50e1be741646ee8d65bc4` / `4730684c910689009d2b81604c021b91862264ae` |
| Retained R4 governance commit | `b919512fb73472149afea5a55d1a022bf6aec8da` |
| Retained R4 candidate SHA-256 / blob | `eaff0bd15d34afb533306ab5a4a967cb672149422e14b634ae263fea512f4f70` / `18e616868d9f94b97264e4b744961d85b6442f3d` |
| Retained R4 pulse SHA-256 / blob | `42416f3d638d06e4930413f7e3ed5ef211143f6de19ee6f31cf4eb70d3ac434d` / `2d211769e4adfb4d9d0b6171909cdeb947d76492` |
| R5 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R5` |
| R5 candidate SHA-256 | `c618af6d61d05c51fe689a791f7a8bc9f2ed908c4c42e7e48dd07badec2a633d` |
| R5 candidate Git blob | `42f8ff4bd0e9350ac269b0a3a137209b1be1f120` |

R1 through R4 are immutable, were not accepted, and remain recoverable at the
exact commits, digests, and blobs above. R5 is their proposed successor, not an
in-place finding erasure.

## Retained R1 finding disposition

| Finding | R1 disposition | R2 remediation |
|---|---|---|
| `BA-TST-R1-M01` | retained major; R1 not accepted | R2's attempted separate WS-only executable lineage is superseded by R3's linear current-main lineage; WS remains only the logical WP predecessor and the context co-member graph has zero edges. |
| `BA-TST-R1-M02` | retained major; R1 not accepted | immutable successor-addressed mode/set records carry the full verification schema, per-mode/aggregate/tree custody, failures, counterexamples, review, assurance, history, rollback, and reproduction. |
| `BA-TST-R1-M03` | retained major; R1 not accepted | exact digest-bound forward and reverse transpose covers every required TST/REL/NF/DES/contract/CR/VCL/validation/actor/lane/hold source and rejects completeness, duplicate, or mismatch errors. |
| `BA-TST-R1-M04` | retained major; R1 not accepted | fixture rows bind custody, expected-reason, predecessor identity/digest/version, supersession state, exact bounds, and positive/negative/replay behavior. |

## Retained R2 finding disposition

| Finding | R2 disposition | R3 remediation |
|---|---|---|
| `BA-TST-R2-M01` | retained major; R2 not accepted | one linear current-main acceptance/entry/implementation chain; WS is the sole logical WP predecessor; REV is an unchanged context co-member; the exact two-node graph has zero edges and TST remains invariant when REV is unselected or removed from the in-memory graph projection. |
| `BA-TST-R2-M02` | retained major; R2 not accepted | 123 individually keyed canonical identities and 123 exact reverse edges replace every pairing, alias, range, shorthand, and bootstrap-suffixed name; 38 CRs, all actors/lanes, four separate SPEC-UNK identities and four separate TBD identities are explicit; the old 59-ID sample is retired. |
| `BA-TST-R2-M03` | retained major; R2 not accepted | source ID/digest and a non-self-referential canonical custody preimage are exact, bounded, LF-ordered, and covered by missing/substitution/successor/replay cases. |

## Retained R3 finding disposition

| Finding | R3 disposition | R4 remediation |
|---|---|---|
| `BA-TST-R3-M01` | retained major; R3 not accepted | exact closed `test-gate-evidence.v2` and set schemas define ordered keys, types, cardinalities, enums, nullability, path/ID/mode/version equality, duplicate/extra/missing rejection, non-self-referential acceptance/entry sequencing, initially empty/partial review decisions, create-new review successors, and digest-field-omitted mode/set custody. |
| `BA-TST-R3-M02` | retained major; R3 not accepted | all 38 CR identities are remapped from their exact `CODE_RIGOR.md` obligation to 63 actually executed static, supply-chain, contract, property, model, adversarial, hold, no-authority, or source-spine assertions; the exact 148 forward/reverse edges and per-mode target lists replace count alternation. |

## Retained R4 finding disposition

| Finding | R4 disposition | R5 remediation |
|---|---|---|
| `BA-TST-R4-M01` | retained major; R4 not accepted | each mode JSON embeds the only three bounded observed-output fragments plus closed rollback/reproduction plans; fixed RFC 6901 pointers resolve only to fragment content, hashes bind decoded/canonical bytes, and external/self/ancestor/future/unknown pointers and raw rejected bytes reject. |
| `BA-TST-R4-M02` | retained major; R4 not accepted | exactly 22 indexed decision slots hold null or closed digest-field-omitted immutable decision records with per-lane predecessor chains; a successor changes exactly one slot, retains 21, and can close finding/defer only by a later bound `pass`/`remediated` record; set successors use the same nested preimage. |

## Candidate boundary

The exact candidate limits future work to `PB-TST-001`, assigned
`PB-FIX-001`, configuration-only `PB-WS-001` membership, a current-lineage
two-package/zero-edge graph, four inert synthetic fixtures,
eight explicit integration-test targets, one supervised evidence runner, and
the separately retained 16-mode evidence set.

It fixes the future path allowlist, package graph, fixture representation and
bounds, all three L0, six L1, and seven L2 commands, runner bounds, bounded
source-spine/contract/property/model/adversarial/hold-non-closure/
no-authority cases, unchanged exact 123-identity/148-edge trace transpose and
closed embedded mode/set evidence schema, append-only
custody, required roles, stop condition, exit
criteria, and rollback. It pre-claims no implementation or result.

## Required independent review

Acceptance remains pending. Every required decision must bind the exact
R5 candidate digest above. Review must include all eight parliament lanes,
Independent Test & Oversight, Methodology Panel, Role review, Citation, Scope,
Numeracy, all seven stakeholder lenses, Classification & Operational Security,
and Civilian Control/Law/Safety/Readiness. Any changed digest, unresolved
critical or major finding, open evidence conflict, failed assurance gate, or
scope/authority ambiguity requires a retained successor candidate.

## No decision or authority

This pulse records a candidate for review only. It does not accept or enter
the WP; create a crate, fixture, runner, command, or evidence result; close
`TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001`, `TBD-REL-001`, or any other held
pair; accept product evidence; authorize a semantic producer; emit HND, TERM,
REL, or Taxlane state; support operational, force, procurement, budget,
allocation, or rate action; authorize official use; publish; push; or release.

Disposition: **R1/R2/R3/R4 findings retained and remediated only in proposed R5;
independent acceptance review required;
not accepted; not entered**.
