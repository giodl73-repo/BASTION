# Pulse 25 — WP-TST-001 retained R13 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R11 governance commit | `2cc8ef35d99a2b49878dce2943b639991df1feff` |
| Retained R11 candidate SHA-256 / blob | `b3082d4853c64c7f0f7505112ccb9bb22d504cdf61e21126f8f27a0c6a5e3b9e` / `f2f20f2fda1c73b3cd0924b1bc1a2c06867043e9` |
| Retained R11 pulse SHA-256 / blob | `8bb730b00a59634259d835e2fb82fe4346c035f9bb870bb252e5572d48c17f7c` / `6db09b5a99b328edd53cd03514b80e34049e2a11` |
| Retained R12 candidate SHA-256 / blob | `b6654694983513c99730ceb0a900f44a288f26a845db41bf4ec1a7395bb193aa` / `64c051acc6b5724661b7e7181a35deca219d5ac7` |
| Retained R12 pulse SHA-256 / blob | `8723e1cd1759f7ad29a5b0366e310b411e73a37f0971b779c3cb19edee9f2ba6` / `a9cb2cbecefa066a60602a341013443147650ab1` |
| Retained R12 governance commit | `cfb466029d759919c0f8ef5e6ab7a7fe3c1aab3c` |
| R13 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R13` |
| R13 candidate SHA-256 | `6bebe5ec95924acbc02f90d14869695ea8d7bc19b331c16c7a34a27b0cbca5a9` |
| R13 candidate Git blob | `677191e393d275ddb1044ccb7369e667f44b22cb` |

```vtrace-author-custody.v1
subject=WP-TST-001-R13
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=6bebe5ec95924acbc02f90d14869695ea8d7bc19b331c16c7a34a27b0cbca5a9
```

R1 through R12 remain immutable, unaccepted review history. R13 is a proposed
successor and does not rewrite their records, findings, or observations.

## R13 consolidated finding disposition

| Finding group | R13 remediation |
|---|---|
| exact process-tree bound | every run requires `JOB_OBJECT_LIMIT_JOB_MEMORY` and exact `JobMemoryLimit=1073741824`; queried extended limits, violation information, accounting, and configuration digest prove the Job aggregate committed-memory ceiling |
| optional process bound | `JOB_OBJECT_LIMIT_PROCESS_MEMORY` is permitted only as an additional exact per-process ceiling, is explicitly bound as null or 1 GiB, and can never substitute for or relax the Job-wide limit |
| termination/result coherence | only a proved Job-wide violation may produce `memory-limit`; it derives `bound-exceeded`, while missing or mismatched mandatory Job configuration derives `binding-mismatch`/`job-identity-lost` |
| execution ordering | `execution_ordinal` is consecutive from 1 across execution origins, independent of evidence versions consumed by review successors; origins, bindings, ledgers, histories, transitions, and set projections bind both values without conflation |
| retained R12 closures | durable named-Job/PID/FILETIME recovery, exhaustive termination tuples, full generated-artifact recomputation, immutable materialization, mutation watches, acyclic receipt/finalization, and closed review/execution successors remain intact |

## Preserved boundary and review

R13 retains R12's exact 18-path implementation allowlist, four fixtures, 16
modes, 123 identities, 148 forward/reverse transpose edges, 38 CR identities,
63 CR edges, and 22 indexed review lanes. Section 7 is byte-identical to
committed R11 and retained R12. Mode evidence advances to
`test-gate-evidence.v10`, set evidence to `test-gate-evidence-set.v9`, and
ledger evidence remains byte-compatible `test-gate-ledger.v2`; its enclosing
mode `ledger_binding` supplies the execution ordinal.

All 22 lanes must independently bind the R13 digest. Any absent/substituted
Job-wide limit, false memory-limit proof, process-only aggregate claim,
execution-ordinal gap/duplicate/review derivation, evidence-version conflation,
or any retained R12 failure condition requires another retained successor.

This pulse does not accept or enter the WP, execute implementation, create
evidence, authorize official use, emit HND/TERM/REL/Taxlane state, publish,
push, or release.

Disposition: **R1–R12 retained; aggregate-memory and independent-execution-
ordinal findings remediated only in proposed R13; independent acceptance review
required; not accepted; not entered**.
