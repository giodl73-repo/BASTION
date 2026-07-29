# Pulse 24 — WP-TST-001 retained R12 acceptance candidate

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
| R12 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R12` |
| R12 candidate SHA-256 | `b6654694983513c99730ceb0a900f44a288f26a845db41bf4ec1a7395bb193aa` |
| R12 candidate Git blob | `64c051acc6b5724661b7e7181a35deca219d5ac7` |

```vtrace-author-custody.v1
subject=WP-TST-001-R12
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=b6654694983513c99730ceb0a900f44a288f26a845db41bf4ec1a7395bb193aa
```

R1 through R11 remain immutable, unaccepted review history. R12 is a proposed
successor and does not rewrite their records, findings, or observations.

## R12 consolidated finding disposition

| Finding group | R12 remediation |
|---|---|
| durable recovery identity | a run-derived named Windows Job, lossless process creation FILETIME, queried kill-on-close/memory configuration, exact Create/Open/Query/Terminate/wait proof, and full termination event/exit/stream tuple make recovery safe against PID reuse and supervisor loss |
| generated and review custody | preflight and postrun freshly parse every ledger, mode/set evidence, review-auth, receipt, and finalization path with exact tag/raw hash/schema/record digest/predecessor chain; postrun permits only current-ledger additions |
| execution successors | origin/review/execution discriminants, predecessor-execution binding, execution history, stable/mutable/reset field sets, fresh run root, and immutable prior artifacts distinguish rerun/reproduction from review overlays |
| immutable materialization | watches and denied handles begin before the first commit-tree read; blob object IDs and copied bytes reverify; workers use only the external immutable materialization and Cargo target root |
| acyclic publication | post-worker evidence, receipt, and self-excluding finalization use exact create-new order, full snapshots, precommitted final watch/snapshot projections, and no rewrite or self-digest cycle |
| retained lifecycle | R11's exact phases, native exits, paired start/terminal schemas, leaf-to-root recovery, Git custody, sequential generated-artifact allowance, and byte-identical trace remain closed |

## Preserved boundary and review

R12 retains R11's exact 18-path implementation allowlist, four fixtures, 16
modes, 123 identities, 148 forward/reverse transpose edges, 38 CR identities,
63 CR edges, and 22 indexed review lanes. Section 7 is byte-identical to
committed R11. Mode evidence advances to `test-gate-evidence.v9`, set evidence
to `test-gate-evidence-set.v8`, and ledger evidence remains
`test-gate-ledger.v2`.

All 22 lanes must independently bind the R12 digest. Any Job/PID/FILETIME or
termination-tuple ambiguity, generated-artifact omission, execution/review
transition confusion, worktree-sourced execution, mutation-watch gap,
publication/finalization cycle or mismatch, unresolved major, open defer/
dissent/conflict, or failed assurance requires another retained successor.

This pulse does not accept or enter the WP, execute implementation, create
evidence, authorize official use, emit HND/TERM/REL/Taxlane state, publish,
push, or release.

Disposition: **R1–R11 retained; consolidated findings remediated only in
proposed R12; independent acceptance review required; not accepted; not
entered**.
