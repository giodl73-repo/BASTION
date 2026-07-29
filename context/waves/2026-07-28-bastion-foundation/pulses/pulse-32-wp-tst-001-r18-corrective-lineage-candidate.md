# Pulse 32 — WP-TST-001-R18 commit-lineage closure candidate

Date: 2026-07-29

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate subject

| Subject | Identity |
|---|---|
| R18 `docs/vtrace/WP_TST_001.md` SHA-256 | `785a7ec20b07ba4e52ce6a6ed446c12aa13efc055225f46f82f71d98c6b63a9f` |
| R18 `docs/vtrace/WP_TST_001.md` prospective Git blob | `99e1aee5250052cbf534db9a4f1a66575caaa256` |
| R18 candidate base / failed R17 amendment | `e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e` |
| Failed R17 WP SHA-256 / blob | `21a9260a114477503f8be588a76869a680972226ff9c585414123402f38d73ee` / `84d1ce2f83a4e0ce9804c8b8390006ee8c1b8d32` |
| Failed R17 pulse SHA-256 / blob | `4f2f5e7d2b94687bf120e3b566d4d9ba6df9dc19d2c3030ef3739a3a98f89bc6` / `1a7dd03c9462059cd2e4280ddc54009a5ef750a5` |
| Failed R16 amendment | `be689ae15837b6f5ce0c24a89ee66a6b5939aa75` |
| Failed R15 amendment | `060ddce2e7c0500d162282e928c624fc5f0e0753` |
| Original R14 entry | `6354f5184b97923571dcd397ac9871167833e86e` |
| Retained failed R14 implementation / tree | `7e4591838dfffdc8d1fc35f0e97e77133a56490b` / `53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f` |

```vtrace-author-custody.v1
subject=WP-TST-001-R18
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=785a7ec20b07ba4e52ce6a6ed446c12aa13efc055225f46f82f71d98c6b63a9f
```

## R17 VTRACE finding disposition

R15 through R17 remain immutable failed governance history. R18 preserves every
R17 closure and closes the four new findings only in this successor:

| R17 finding | R18 closure |
|---|---|
| Entry and implementation parent observations were unbound | Exact `git cat-file commit` commands, complete raw stream custody, total exactly-one-parent parsing, and embedded observations now prove acceptance-to-entry and entry-to-corrective-child. |
| Unscoped no-extra diffs were status-filtered | Every no-extra raw command has no diff filter, parses the total Git raw status alphabet, rejects `T` and every other unsupported/unauthorized status, and must be byte-identical to its scoped peer. |
| Set dual-bound identity projection was ambiguous | Failed bindings remain nonauthorizing; overlapping failed+positive IDs are visible in both candidate and failed projections; failed-only IDs are excluded from candidate authority. The same rule is exact for every mode and the set. |
| Commit-parent observation schema was not closed | Acceptance, corrective entry, and corrective implementation all embed the same closed `test-gate-commit-parent-observation.v1` shape and digest preimage; acceptance is its role-specific use without a parallel schema. |

Every other R17 delta, audit, manifest, ledger-v3, evidence, fixture, resource,
review, exit, rollback, hold, and no-authority rule remains normative. R18
creates no implementation or evidence and grants no authority.

Only independent review with zero unresolved critical, major, or actionable
minor finding may recommend a future Pulse 33 R18 acceptance. Its committed
object must have the exact R18 candidate as sole parent. A later Pulse 34 may
separately enter the corrective work, and its committed object must have that
acceptance as sole parent. The corrective implementation must then have the
entry as sole parent. No pulse binds its own future commit.

Disposition: **proposed R18 corrective-lineage amendment; not accepted; not
entered; failed implementation and failed R15/R16/R17 amendments retained; no
evidence, exit, push, publication, or release**.
