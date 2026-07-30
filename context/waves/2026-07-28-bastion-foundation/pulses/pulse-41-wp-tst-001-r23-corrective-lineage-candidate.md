# Pulse 41 — WP-TST-001-R23 external-subject-dispatch corrective candidate

Date: 2026-07-29

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact retained custody

| Subject | Identity |
|---|---|
| Retained failed R22 candidate commit / parent | `d3a4107f8af41800eff49a40d75e8e1a648e5ffd` / `b6993a9012a6f1dcb3937fced7ae9865a15007f9` |
| R22 WP SHA-256 / Git blob | `bf4bc25d03ce763ae4bf16baf7b13890548db91904ee9d1673970e3f54aa7cff` / `ed83ce0f25c403fe7f839d33a3c7629f17ca1bc5` |
| R22 Pulse 38 SHA-256 / Git blob | `b6b8e95fa8f42d1919d965ba5380c864ab7d95de8236d039abd7fb9b09d3ab8b` / `f030b61670e3f2c4b0489eb6035651f295c60395` |
| R23 WP SHA-256 before candidate commit | `6684f537f957491b232fdb73269dd6c35954f64bdfde5b8963a82566a381d83d` |
| R23 WP prospective Git blob | `df17941e685299e6fd97077d466334756bbf1685` |

R22 is failed, not accepted, not entered, and creates no implementation or
evidence authority. Its sole converged finding was that external dispatch
permitted a null `subject_commit` for failed-governance-stage reviews.

## Candidate author custody

```vtrace-author-custody.v4
subject=WP-TST-001-R23
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=6684f537f957491b232fdb73269dd6c35954f64bdfde5b8963a82566a381d83d
```

## R23 corrective closure

For both failed-amendment reviews and failed-governance-stage reviews,
`ValidatorDispatchMetadataV3.subject_commit` is mandatory non-null. The already
selected lineage parent fixes the exact indexed parent path, exact indexed
review-child path, and exact subject commit before any child byte is parsed.
The child may only confirm equality; it cannot select its schema, validator,
stage, review ID, or subject.

R23 retains the failed R22 amendment beside the six earlier failed amendments,
the valid R21 candidate, and the two failed R21 governance stages. Every
affected current outer or nested contract advances under the R17–R23 matrix.
Future Pulse 42 acceptance must be the sole-parent child of this committed R23
candidate and contain exactly one v6 author-custody fence plus the exact v8
lineage-intent preimage. Future Pulse 43 entry must be the sole-parent child of
that valid acceptance and contain exactly one v6 author-custody fence. Neither
future artifact may self-bind its containing commit.

This candidate changes governance specification only. It does not accept or
enter the WP, authorize implementation, run tests, create evidence, close a
producer hold, publish, push, release, or create official action.

Disposition: **proposed R23 external-subject-dispatch corrective amendment;
not accepted; not entered; no implementation or evidence authority**.
