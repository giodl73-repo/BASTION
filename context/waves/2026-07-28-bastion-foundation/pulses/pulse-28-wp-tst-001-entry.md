# Pulse 28 — WP-TST-001 entry

Date: 2026-07-29

Exact accepted WP SHA-256: `0c909cb0aab010d4b936c93ae770ebf98fdabc421b5c4883ba967ef6a5c6955b`

Acceptance commit: `8bb1140925688f32a4997926567919997e9bf3f9`

Acceptance-pulse SHA-256: `2624c41bc933753f303c845061d177ed79ab9398d3d3d85e5c7a8f78a0709d8f`

Decision: **enter** the exact implementation allowlist on branch
`codex/wp-tst-001` from this entry commit in the isolated child worktree.

Once this pulse is committed, its containing commit is the immutable
`entry_commit` and `implementation_delta_base`. The first implementation commit
must be its direct non-merge child and may change only the exact accepted R14
paths, dependencies, fixtures, runner, and workspace membership.

Stop on any path, dependency, command, bound, schema, trace, history, content,
review, hold, or authority deviation. Entry accepts no implementation result,
evidence, producer pass, held-pair closure, HND/TERM/REL/Taxlane action,
operational or fiscal decision, external action, push, publication, or release.
