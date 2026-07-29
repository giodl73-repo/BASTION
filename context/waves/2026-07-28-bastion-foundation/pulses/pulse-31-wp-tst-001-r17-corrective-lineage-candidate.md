# Pulse 31 — WP-TST-001-R17 corrective-lineage candidate

Date: 2026-07-29

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate subject

| Subject | Identity |
|---|---|
| R17 `docs/vtrace/WP_TST_001.md` SHA-256 | `21a9260a114477503f8be588a76869a680972226ff9c585414123402f38d73ee` |
| R17 `docs/vtrace/WP_TST_001.md` prospective Git blob | `84d1ce2f83a4e0ce9804c8b8390006ee8c1b8d32` |
| R17 candidate base / failed R16 amendment | `be689ae15837b6f5ce0c24a89ee66a6b5939aa75` |
| Failed R16 WP SHA-256 / blob | `f276da805c02d8ac17a8eb8c2f3f11bc56b04191bdc4705234dd9118e2b97477` / `b85cedfc6fd0290038ecfeebb25ac61f97166ca2` |
| Failed R16 pulse SHA-256 / blob | `d8c709214f4597274d929a45bd0ea0975987c27d0358243c3b31f7c97eee28a2` / `5b31c06a55fc35c383b6690b310b420bdbaaaeba` |
| Failed R15 amendment | `060ddce2e7c0500d162282e928c624fc5f0e0753` |
| Original R14 entry / pulse SHA-256 | `6354f5184b97923571dcd397ac9871167833e86e` / `84bdc53fed341919db253d2799a2d7119a965fce5b505dd28df0fec840f5c035` |
| Retained failed R14 implementation / tree | `7e4591838dfffdc8d1fc35f0e97e77133a56490b` / `53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f` |
| Failed R14 implementation-manifest digest | `c238e0e4d0661afec9f7a9c91b883ffadf886e15349ab08982ba1076f91ee9f7` |

```vtrace-author-custody.v1
subject=WP-TST-001-R17
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=21a9260a114477503f8be588a76869a680972226ff9c585414123402f38d73ee
```

## R16 VTRACE finding disposition

R15 and R16 remain immutable failed governance history. R17 closes the six R16
findings only in this successor candidate:

| R16 finding | R17 closure |
|---|---|
| ObservedPreflight confused the complete implementation and corrective domains | Every normative implementation row and manifest now comes only from the original-R14-entry-to-corrective-child 18-literal-path complete projection. One separately retained unscoped R17-entry-to-child observation proves only no-extra/subset equality and cannot supply a manifest. |
| Failed-author exclusion erased independently positive identities | Failed bindings confer no authority, but their IDs remain visible and an ID with any independent positive binding remains in the candidate-author projection. Execution successors replace one mode binding and retain the other six of seven. |
| Acceptance first parent was not exact or bound | AcceptanceBindingV3 binds raw acceptance commit custody and requires its sole first parent to equal the exact R17 candidate; exact Git command, parse, digest, and negative rules are fixed. |
| Lineage-intent fields/preimages were incomplete | Every literal v2 intent key/value LF row is specified, including R14 entry/pulse, failed commit/tree/manifest, both failed amendment commits, R17 candidate, and both delta kinds. |
| Failed implementation audit was substituted | The immutable audit now binds the original six exact identities, severities, titles, affected paths, behaviors, order, and dispositions, including the critical Measure-Object `combined_bytes` runner failure. |
| Ledger schema reference was stale | Every current ledger declaration and cross-reference names exact `test-gate-ledger.v3`. |

Every other R16 schema, fixture, command, resource, evidence lifecycle, review,
exit, rollback, hold, and no-authority restriction remains normative. R17
creates no implementation or evidence and grants no authority.

Only independent review with zero unresolved critical, major, or actionable
minor finding may recommend an R17 acceptance pulse. That future Pulse 32 must
be committed as a direct child of the exact committed R17 candidate, and its
AcceptanceBindingV3 must prove that sole first-parent identity. A later Pulse
33 may separately enter the corrective work. No pulse binds its own future
commit.

Disposition: **proposed R17 corrective-lineage amendment; not accepted; not
entered; failed implementation and failed R15/R16 amendments retained; no
evidence, exit, push, publication, or release**.
