# Pulse 17 — WP-TST-001 retained R2 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole executable predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R1 governance commit | `62116481b7b3e7d671517b6053c8cc3f20f93fce` |
| Retained R1 candidate SHA-256 / blob | `93ea15ea87b140b7e45ae67db5a4133e24e8f18778db1ce41a891042b1157554` / `65fabe5060cb6c9b9cf7ea2f0c5e88ebec9c178d` |
| R2 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R2` |
| R2 candidate SHA-256 | `4ecd246d67bb5d07c94496a9975c99cdc8488295e8e74235be29391b3725e146` |
| R2 candidate Git blob | `47687aff86c392b7e30b237de1015b9d304d4fc4` |

R1 is immutable, was not accepted, and remains recoverable at the exact commit,
digest, and blob above. R2 is its proposed successor, not an in-place finding
erasure.

## Retained R1 finding disposition

| Finding | R1 disposition | R2 remediation |
|---|---|---|
| `BA-TST-R1-M01` | retained major; R1 not accepted | WS is the sole executable predecessor; REV is context-only; zero REV or other dependency edges are executable and both edge directions are rejected. |
| `BA-TST-R1-M02` | retained major; R1 not accepted | immutable successor-addressed mode/set records carry the full verification schema, per-mode/aggregate/tree custody, failures, counterexamples, review, assurance, history, rollback, and reproduction. |
| `BA-TST-R1-M03` | retained major; R1 not accepted | exact digest-bound forward and reverse transpose covers every required TST/REL/NF/DES/contract/CR/VCL/validation/actor/lane/hold source and rejects completeness, duplicate, or mismatch errors. |
| `BA-TST-R1-M04` | retained major; R1 not accepted | fixture rows bind custody, expected-reason, predecessor identity/digest/version, supersession state, exact bounds, and positive/negative/replay behavior. |

## Candidate boundary

The exact candidate limits future work to `PB-TST-001`, assigned
`PB-FIX-001`, configuration-only `PB-WS-001` membership, a WS-only one-package
zero-dependency graph, four inert synthetic fixtures,
eight explicit integration-test targets, one supervised evidence runner, and
the separately retained 16-mode evidence set.

It fixes the future path allowlist, package graph, fixture representation and
bounds, all three L0, six L1, and seven L2 commands, runner bounds, bounded
source-spine/contract/property/model/adversarial/hold-non-closure/
no-authority cases, full trace transpose and evidence schema, append-only
custody, required roles, stop condition, exit
criteria, and rollback. It pre-claims no implementation or result.

## Required independent review

Acceptance remains pending. Every required decision must bind the exact
R2 candidate digest above. Review must include all eight parliament lanes,
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

Disposition: **R1 findings retained and remediated only in proposed R2;
independent acceptance review required;
not accepted; not entered**.
