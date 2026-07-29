# Pulse 21 — WP-TST-001 retained R9 acceptance candidate

Date: 2026-07-29

Assignment: `WP-TST-001`

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact candidate custody

| Subject | Identity |
|---|---|
| Sole logical WP predecessor: accepted WS exit | `cd1f1d75ec312789fed63a265219d8ad9069a17a` |
| Retained R8 governance commit | `264cff6959c74f4e9430fda3ca9e72b529da318a` |
| Retained R8 candidate SHA-256 / blob | `3165853787462ede6a39c154b060accd8d4ab43e83a36ea1373e5d01aaf86de7` / `41a279ad001ef327c8beb2b76d1ac5d2cb84e540` |
| Retained R8 pulse SHA-256 / blob | `6e7344ec728788dd7df8289466a888ae4f545416fc18b09dd5d693578ce135fe` / `860d5da041418e08b79c4e6b33074f1fdc903292` |
| R9 candidate | `docs/vtrace/WP_TST_001.md` / `WP-TST-001-R9` |
| R9 candidate SHA-256 | `402d4704b5cdc3de593039090478f12ec2ed2f93cf9ce8db222deed6309f82b4` |
| R9 candidate Git blob | `1fbe238b42b56b0e3d87590f767e1ce02d89b0cf` |

```vtrace-author-custody.v1
subject=WP-TST-001-R9
author_id=REV-TST-WP-AUTHOR
controller_id=REV-TST-GOVERNANCE-CONTROLLER
subject_digest=402d4704b5cdc3de593039090478f12ec2ed2f93cf9ce8db222deed6309f82b4
```

R1 through R8 remain immutable, unaccepted review history. R9 is a proposed
successor and does not rewrite their records, findings, or observations.

## Retained R8 cross-review disposition

| Finding | R9 remediation |
|---|---|
| Serial target observability | allocated targets execute one at a time through exact per-target argv, `--exact --test-threads=1`, and durable start/completion records; allocation order, the unique active target, partial prefixes, and all later not-run targets remain observable without parallel/default-libtest text inference. |
| Identity custody | exact committed pulse paths/blocks, raw implementation-commit trailers, and canonical mode/set evidence preimages extract every author/controller into a mandatory candidate-wide registry; exact lane-indexed reviewer-auth records bind reviewer identity, controller, WP, and digest; independence is derived against the complete projection. |
| Repository-root and Git custody | the runner derives one normalized absolute root from its own path; every Git command uses exact `-C`, config, top-root pathspec, and `GIT_OPTIONAL_LOCKS=0`; git-dir/common-dir/index are discovered explicitly for linked worktrees; local config, common info attributes/excludes, index bytes, and porcelain worktree views are bound before/after. |
| Lossless Git paths | every observed Git path is a canonical base64 raw-byte object with decoded uint64 length and SHA-256; ordering and uniqueness use decoded unsigned bytes, retaining invalid UTF-8 and control/TAB/LF/CR bytes without display coercion. |

## Preserved boundary and review

R9 retains R8's exact 18-path implementation allowlist, four fixtures, 16
modes, 123 identities, 148 forward/reverse transpose edges, 38 CR identities,
63 CR edges, and 22 indexed review lanes. Section 7 is byte-identical to
committed R8. Mode evidence advances to `test-gate-evidence.v6` and set
evidence to `test-gate-evidence-set.v5`.

All 22 lanes must independently bind the R9 digest. Any changed digest,
aggregate/parallel target execution, incomplete ledger, caller identity,
omitted author, unauthenticated reviewer, root/pathspec/config drift, linked-
worktree misresolution, Git mutation, lossy path decoding, unresolved major,
open defer/dissent/conflict, or failed assurance requires another successor.

This pulse does not accept or enter the WP, execute implementation, create
evidence, authorize official use, emit HND/TERM/REL/Taxlane state, publish,
push, or release.

Disposition: **R1–R8 retained; cross-review findings remediated only in
proposed R9; independent acceptance review required; not accepted; not
entered**.
