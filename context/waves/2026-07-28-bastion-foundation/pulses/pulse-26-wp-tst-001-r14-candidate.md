# Pulse 26 — WP-TST-001 retained R14 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R13 governance commit | `3166b0cf86af45f3fd04454ff1c9734cac37843d` |
| Retained R13 candidate SHA-256 / blob | `6bebe5ec95924acbc02f90d14869695ea8d7bc19b331c16c7a34a27b0cbca5a9` / `677191e393d275ddb1044ccb7369e667f44b22cb` |
| Retained R13 pulse SHA-256 / blob | `47a1fb382babd7872b7394660bb1d12dd530995f9eab9ff20808b501407d5d9c` / `536b759b15254c8b79143e4319b0922df4885a22` |
| R14 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R14` |
| R14 candidate SHA-256 | `0c909cb0aab010d4b936c93ae770ebf98fdabc421b5c4883ba967ef6a5c6955b` |
| R14 candidate Git blob | `182b36ffba985c7e8d432bb5a3b18aa0b76a557a` |

```vtrace-author-custody.v1
subject=WP-TST-001-R14
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=0c909cb0aab010d4b936c93ae770ebf98fdabc421b5c4883ba967ef6a5c6955b
```

R1 through R13 remain immutable, unaccepted review history. R14 is a proposed
successor and does not rewrite their records, findings, observations, or pulse
artifacts.

## R14 consolidated finding disposition

| Finding group | R14 remediation |
|---|---|
| mandatory hard bound | retained `JOB_OBJECT_LIMIT_JOB_MEMORY` and exact extended `JobMemoryLimit=1073741824` remain the non-substitutable aggregate committed-memory enforcement mechanism |
| documented terminal signal | an inactive Job is associated with a private completion port before assignment; exact key/message dequeue custody accepts the documented `JOB_OBJECT_MSG_JOB_MEMORY_LIMIT` hard-attempt signal without fabricating violation flags |
| supplemental notification | set/query-verified `JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2` at the same Job-memory threshold permits `JOB_OBJECT_MSG_NOTIFICATION_LIMIT` only with the required immediate `JobObjectLimitViolationInformation2` query; it never substitutes for the hard limit |
| immutable early pulse history | R1–R7 candidate pulses are independently commit:path-selected and raw-blob hashed; the five distinct Pulse 17 historical versions cannot collapse into current pathname bytes |
| retained R13 closures | named Job/PID/FILETIME recovery, independent execution ordinals, full generated-artifact recomputation, immutable materialization, mutation watches, and acyclic receipt/finalization remain intact |

## R1–R7 pulse ledger

| Revision | Commit | Path | SHA-256 / blob |
|---|---|---|---|
| R1 | `62116481b7b3e7d671517b6053c8cc3f20f93fce` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` | `f52849a3908decbae20724986026ac42c00d7e938d7d193a1c17fd8eb0a9a80e` / `b387aa17a9d50ca510c52552a83bb6d0581cbccd` |
| R2 | `21c8066445c72358a444c0b506422ec3b9dc63e0` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` | `2a2868748ce53369d68e6978b8d3d02d3a684d7aec98f6f5f0c3d6fea9a2110a` / `88b11c49e7d8ced29e1ebcb40f68bf5dc6b519ad` |
| R3 | `ae64448e98744668e3b80e3411255503bfbdd4ae` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` | `954e17de5d0833d98f0a44c932476af60c0163b126c50e1be741646ee8d65bc4` / `4730684c910689009d2b81604c021b91862264ae` |
| R4 | `b919512fb73472149afea5a55d1a022bf6aec8da` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` | `42416f3d638d06e4930413f7e3ed5ef211143f6de19ee6f31cf4eb70d3ac434d` / `2d211769e4adfb4d9d0b6171909cdeb947d76492` |
| R5 | `77e0abb94a427a1f824e4f5659e580b1aae74137` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-17-wp-tst-001-candidate.md` | `f231cc4684943275771cd06056abad619b8d5d8ea6c2587de52776a9da114382` / `30b7c92da8e21de9e7177779ef6d4e4127f095a4` |
| R6 | `1e0157aca9e20eb78cf1cd345fa5cc5bfc5729f3` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-18-wp-tst-001-r6-candidate.md` | `cfc4b2268fd45682d655d3543f1daa06889baea16dad4c3bf370c37a6760fe4b` / `8944d7b91e7abe90fff004433d613465b601b75a` |
| R7 | `3550c5deece2ec97207fbe8c1b4dda4c44d62a97` | `context/waves/2026-07-28-bastion-foundation/pulses/pulse-19-wp-tst-001-r7-candidate.md` | `ec12c55fa7a08d3e764c596b25ce5af96daafcef9a0a314eb9e2cded31aeba13` / `97249bc4a20bc7dd4dff4950dd169c78cf25590a` |

Each ledger row was derived from the named committed blob, not the current
worktree pathname.

## Preserved boundary and review

R14 retains R13's exact 18-path implementation allowlist, four fixtures, 16
modes, 123 identities, 148 forward/reverse transpose edges, 38 CR identities,
63 CR edges, and 22 indexed review lanes. Section 7 is byte-identical to R13.
Mode evidence advances to `test-gate-evidence.v11`, set evidence to
`test-gate-evidence-set.v10`, and ledger evidence remains byte-compatible
`test-gate-ledger.v2`.

All 22 lanes must independently bind the R14 digest. Any hard/notification
limit substitution, completion-port/key/message/query ambiguity, synthesized
violation flag, historical pulse version collapse, or retained R13 failure
condition requires another retained successor.

This pulse does not accept or enter the WP, execute implementation, create
evidence, authorize official use, emit HND/TERM/REL/Taxlane state, publish,
push, or release.

Disposition: **R1–R13 retained; API-valid memory-terminal and R1–R7 pulse-
history findings remediated only in proposed R14; independent acceptance review
required; not accepted; not entered**.
