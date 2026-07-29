# Pulse 22 — WP-TST-001 retained R10 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R9 governance commit | `69dc2f86783c3bf35cdc1367b2ed787c5da423a9` |
| Retained R9 candidate SHA-256 / blob | `402d4704b5cdc3de593039090478f12ec2ed2f93cf9ce8db222deed6309f82b4` / `1fbe238b42b56b0e3d87590f767e1ce02d89b0cf` |
| Retained R9 pulse SHA-256 / blob | `5e7e69f2b9e22227336b60dbceeb8e3b76ed804bf28f849d11e522788299560a` / `a098cead061bce86ff28b33741d6dfe1fbb081ef` |
| R10 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R10` |
| R10 candidate SHA-256 | `6149e23aba203bb529d13936f872781f98202094984fac7a9c74e0279b700a1b` |
| R10 candidate Git blob | `a6f74dcbc319921a128dda1562b821995b620735` |

```vtrace-author-custody.v1
subject=WP-TST-001-R10
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=6149e23aba203bb529d13936f872781f98202094984fac7a9c74e0279b700a1b
```

R1 through R9 remain immutable, unaccepted review history. R10 is a proposed
successor and does not rewrite their records, findings, or observations.

## Retained R9 VTRACE finding disposition

| Finding | R10 remediation |
|---|---|
| `M01` multi-phase execution | exact ordered phase objects retain every actual argv and tool tuple; L1Static is Cargo per-target then test-gate static assertion, while L1SupplyChain is Cargo metadata then per-target test-gate assertions; no phase can disappear behind target results. |
| `M02` durable run ledger | one exact mode/version/run-attempt namespace, monotonic record filenames, closed run/phase/target start/completion/recovery objects, complete governance/implementation bindings, self-omitting digest chain, write-through create-new durability, and one terminal scan/recovery grammar make partial execution recoverable without retry or overwrite. |
| `M03` raw-path manifests | expected and actual manifests hash canonical JSON lines containing canonical `RawGitPath` objects and explicit digest/null values, so arbitrary raw path bytes cannot collide through delimiter injection. |
| `M04` post-run repository custody | replace objects are disabled and replace refs/grafts are bound; post-execution root, git/common dirs, effective config, index, worktree porcelain, replace refs, common inputs, and implementation tree are rechecked, permitting only the exact current ledger paths before create-new evidence publication. |

## Preserved boundary and review

R10 retains R9's exact 18-path implementation allowlist, four fixtures, 16
modes, 123 identities, 148 forward/reverse transpose edges, 38 CR identities,
63 CR edges, and 22 indexed review lanes. Section 7 is byte-identical to
committed R9. Mode evidence advances to `test-gate-evidence.v7` and set
evidence to `test-gate-evidence-set.v6`.

All 22 lanes must independently bind the R10 digest. Any phase/argv/tool
omission, ledger namespace/record/digest/durability ambiguity, raw-path
manifest ambiguity, replace/graft/config/root/index/worktree drift, unresolved
major, open defer/dissent/conflict, or failed assurance requires another
retained successor.

This pulse does not accept or enter the WP, execute implementation, create
evidence, authorize official use, emit HND/TERM/REL/Taxlane state, publish,
push, or release.

Disposition: **R1–R9 retained; VTRACE findings remediated only in proposed
R10; independent acceptance review required; not accepted; not entered**.
