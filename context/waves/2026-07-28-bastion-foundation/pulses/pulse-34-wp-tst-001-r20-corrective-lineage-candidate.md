# Pulse 34 — WP-TST-001-R20 external schema-dispatch candidate

Date: 2026-07-29

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate subject

| Subject | Identity |
|---|---|
| R20 `docs/vtrace/WP_TST_001.md` SHA-256 | `f5e230de66b9ef41be7ac84817610cb7c966757db2aa119fe9a345cd1bdbb287` |
| R20 `docs/vtrace/WP_TST_001.md` prospective Git blob | `5c246e8e518bd27d24aaf536cc351f2b03370b6d` |
| R20 candidate base / failed R19 amendment | `4602ced667aa1188133429c2011d57736d203a72` |
| Failed R19 WP SHA-256 / blob | `33763165576e450132a98547f8d23d5746c96c7c1f95b8688ccf8b5a6766b8a4` / `daac24b6eaa19e22979b77f6f2b7fbab215a3507` |
| Failed R19 pulse SHA-256 / blob | `a5edc0c0f217581fe8d46329eea6c7c7ed967df75e70d54a0b52743c26e4e3fe` / `9126b40d140c6b9c272f3440ff31b3a574f9a7e4` |
| Failed R18 amendment | `97716b9165cc4b8f0e6a51010376b0a8e46cc169` |
| Failed R17 amendment | `e7eb6be0adcdb5afc069f5cd3e45cf62dbc31a6e` |
| Failed R16 amendment | `be689ae15837b6f5ce0c24a89ee66a6b5939aa75` |
| Failed R15 amendment | `060ddce2e7c0500d162282e928c624fc5f0e0753` |
| Original R14 entry | `6354f5184b97923571dcd397ac9871167833e86e` |
| Retained failed R14 implementation / tree | `7e4591838dfffdc8d1fc35f0e97e77133a56490b` / `53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f` |

```vtrace-author-custody.v1
subject=WP-TST-001-R20
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=f5e230de66b9ef41be7ac84817610cb7c966757db2aa119fe9a345cd1bdbb287
```

## R19 combined-review disposition

R15 through R19 remain immutable failed governance history. R20 preserves every
substantive R19 execution contract and closes the five combined findings only
in this successor:

| R19 finding | R20 closure |
|---|---|
| Frozen R17 and R18 schema matrices were collapsed | The exact matrix now keeps R17 intent/governance-delta/failed-amendment/corrective-lineage v2 and acceptance/entry/implementation v3 distinct from R18 v3/v4 values, all bound to exact committed WP bytes. |
| Validator selection could trust nested candidate identity | Before any nested parse, external artifact kind, outer schema, WP revision, and candidate commit resolve the committed WP at commit:path; blob ID and raw-byte SHA must match the frozen table. Nested or caller candidate identity cannot select a validator. |
| Ledger wording was stale | Every current runtime reference is exact retained `test-gate-ledger.v4`; schemas are not advanced solely because the WP revision advanced. |
| Expanded failed-amendment review reused v1 | R15–R18 remain frozen v1; the new R19 review is exact `test-gate-failed-amendment-review.v2`, and the expanded governance envelopes are independently versioned. |
| Historical negatives omitted failed R15 | R20 names failed R15 through R19 and binds each exact commit:path, blob, and raw-byte SHA before revision-specific validation. |

Unchanged R19 runtime/outer wire contracts retain their R19 schema versions.
Only governance/review envelopes whose ordered shape or value algebra expands
advance. No execution, resource, evidence, review, exit, rollback, or authority
rule changes. R20 creates no implementation or evidence and grants no authority.

Only independent review with zero unresolved critical, major, or actionable
minor finding may recommend a future Pulse 35 R20 acceptance. Its committed
object must have the exact externally selected R20 candidate as sole parent. A
later Pulse 36 may separately enter the corrective work, and its committed
object must have that acceptance as sole parent. The corrective implementation
must then have the entry as sole parent. No pulse binds its own future commit.

Disposition: **proposed R20 externally dispatched schema-version corrective
amendment; not accepted; not entered; failed implementation and failed
R15/R16/R17/R18/R19 amendments retained; no evidence, exit, push, publication,
or release**.
