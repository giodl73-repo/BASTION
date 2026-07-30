# Pulse 44 — WP-TST-001-R24 ledger-version-language corrective candidate

Date: 2026-07-29

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact retained custody

| Subject | Identity |
|---|---|
| Retained failed R23 candidate commit / parent | `beb9d946961b1bb13ade8c1cf0a28f79cda5f3b1` / `d3a4107f8af41800eff49a40d75e8e1a648e5ffd` |
| R23 WP SHA-256 / Git blob | `6684f537f957491b232fdb73269dd6c35954f64bdfde5b8963a82566a381d83d` / `df17941e685299e6fd97077d466334756bbf1685` |
| R23 Pulse 41 SHA-256 / Git blob | `fd290e6a58260167c1a9e24f541c2ce6eeb485bc745513f66fd9400925e65b19` / `88ec5c83200822a4eee25eb58de5d1832e96eee0` |
| R24 WP SHA-256 before candidate commit | `a07f4fe578d9563a6a339261dd3d28dc02ab7a9684540acefded63bf0b97811d` |
| R24 WP prospective Git blob | `9ee19d257f80fc842826b8ca0b78ded678f0bdd8` |

R23 is failed, not accepted, not entered, and creates no implementation or
evidence authority. Its sole finding was the stale statement that the current
v7 ledger admitted the positive acceptance/entry field “by v6.”

## Candidate author custody

```vtrace-author-custody.v5
subject=WP-TST-001-R24
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=a07f4fe578d9563a6a339261dd3d28dc02ab7a9684540acefded63bf0b97811d
```

## R24 corrective closure

The current ledger is exactly `test-gate-ledger.v8` everywhere: its R17–R24
matrix cell, kind-to-schema map, schema definition, downstream references, and
the rule stating that no R14 positive acceptance/entry field is admitted by
v8. R23 v7 remains frozen failed history and is never current or promotable.

R24 retains all prior failed amendments and stages. Every revision-bound
contract advances consistently. Future Pulse 45 acceptance must be the
sole-parent child of this committed candidate and contain exactly one v7
author-custody fence plus the exact v9 lineage-intent preimage. Future Pulse 46
entry must be the sole-parent child of that acceptance and contain exactly one
v7 author-custody fence. Neither future artifact self-binds its commit.

This candidate changes governance specification only. It does not accept or
enter the WP, authorize implementation, run tests, create evidence, close a
producer hold, publish, push, release, or create official action.

Disposition: **proposed R24 ledger-version-language corrective amendment; not
accepted; not entered; no implementation or evidence authority**.
