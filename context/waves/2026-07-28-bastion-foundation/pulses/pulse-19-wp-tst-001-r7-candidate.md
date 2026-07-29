# Pulse 19 — WP-TST-001 retained R7 acceptance candidate

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
| Retained R6 governance commit | `1e0157aca9e20eb78cf1cd345fa5cc5bfc5729f3` |
| Retained R6 candidate SHA-256 / blob | `e155df20adac753a6b92cf2f36205233626c98551b0c7ac8459ff3a975dd0ced` / `34ba2c245c49478e32186fb5f7e4581e4a755847` |
| Retained R6 pulse SHA-256 / blob | `cfc4b2268fd45682d655d3543f1daa06889baea16dad4c3bf370c37a6760fe4b` / `8944d7b91e7abe90fff004433d613465b601b75a` |
| R7 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R7` |
| R7 candidate SHA-256 | `e9b2c5e82a31eb6ee172f35fe06b2db46f3affcebae0f8b5391264cca59644ff` |
| R7 candidate Git blob | `89ab301d7e3596fbb32678b84c1356a41b3f2bc0` |

R1 through R6 remain immutable, unaccepted review history. R7 is a proposed
successor and does not rewrite their records or findings.

## Retained R6 finding disposition

| Finding | R6 disposition | R7 remediation |
|---|---|---|
| `BA-TST-R6-M01` | retained major; R6 not accepted | a closed observed preflight retains the bounded actual row union and overflow custody for matched, missing, extra, nonallowlisted, deleted, renamed, and substituted paths; expected/actual artifact and manifest digests are independently nullable with typed reasons; dirty/staged/untracked/ignored state remains distinct; only exact non-null happy-path equality promotes, while a null-exit not-run failure remains schema-valid and non-promotable. |
| `BA-TST-R6-M02` | retained major; R6 not accepted | one normative valid JSON map supplies literal arrays of argv arrays for all 16 modes, and a separate exact per-mode table fixes ordered cargo/rustfmt/clippy/test-gate tool tuples, versions, digest sources, digest preimages, and computed digests. |
| `BA-TST-R6-M03` | retained major; R6 not accepted | the set structured result uses exact selected-mode counts and a total four-row mapping from `partial|conflicted|failed|full` to exit/result/posture/reason; review successors retain counts and streams while changing only mapped structured state fields and derived hashes. |
| `BA-TST-R6-M04` | retained major; R6 not accepted | predecessor findings, defers, dissent, and conflicts reconcile one-to-one in immutable order with stable IDs/claims; exhaustive monotone state matrices, exact P+N cardinality, append-only new-item rules, pass restrictions, and negative cases prevent erasure or false closure. |
| `BA-TST-R6-M05` | retained major; R6 not accepted | reviewer identity is an absolute candidate-wide lane bijection across mode, set, current, and history decisions; reuse in another lane always rejects and is explicitly tested. |

## Preserved candidate boundary

R7 retains R6's exact implementation allowlist, two-node/zero-edge package
shape, four synthetic fixtures, 16 modes, 123 canonical identities, 148 exact
forward/reverse transpose edges, 38 CR identities, 63 CR edges, and 22 indexed
review lanes. Section 7 is byte-identical to committed R6. Evidence remains
create-new, bounded, embedded, pointer-local, digest-bound, and non-authorizing.

Mode evidence advances to `test-gate-evidence.v4`; set evidence advances to
`test-gate-evidence-set.v3`. Every nested digest preimage omits only its own
digest member and cannot bind an enclosing/future digest. The acceptance pulse
binds committed R7 and prior inputs, never its own future commit/digest; entry
then binds committed acceptance, never its own future identity.

## Required independent review and no authority

All 22 exact lanes must independently bind the R7 digest above. Any changed
digest, duplicate cross-lane reviewer, unresolved critical/major finding, open
defer/dissent/conflict, failed assurance, dishonest observation, schema drift,
or authority ambiguity requires another retained successor.

This pulse does not accept or enter the WP, implement or execute a command,
create evidence, close a held pair, accept product evidence, authorize a
producer or official use, emit HND/TERM/REL/Taxlane state, publish, push, or
release.

Disposition: **R1–R6 retained; R6 findings remediated only in proposed R7;
independent acceptance review required; not accepted; not entered**.
