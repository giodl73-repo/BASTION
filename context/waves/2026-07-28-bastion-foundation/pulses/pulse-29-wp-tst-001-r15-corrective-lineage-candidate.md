# Pulse 29 — WP-TST-001-R15 corrective-lineage candidate

Date: 2026-07-29

Status: `proposed; acceptance_candidate; not_accepted; not_entered`

## Exact retained subject

| Subject | Identity |
|---|---|
| Accepted R14 candidate commit | `0705a2b42228e865c92a2da7ea2bfc82489bf49e` |
| Accepted R14 WP SHA-256 / blob | `0c909cb0aab010d4b936c93ae770ebf98fdabc421b5c4883ba967ef6a5c6955b` / `182b36ffba985c7e8d432bb5a3b18aa0b76a557a` |
| R14 acceptance commit / pulse SHA-256 | `8bb1140925688f32a4997926567919997e9bf3f9` / `2624c41bc933753f303c845061d177ed79ab9398d3d3d85e5c7a8f78a0709d8f` |
| R14 entry commit / pulse SHA-256 | `6354f5184b97923571dcd397ac9871167833e86e` / `84bdc53fed341919db253d2799a2d7119a965fce5b505dd28df0fec840f5c035` |
| Retained failed implementation commit / tree | `7e4591838dfffdc8d1fc35f0e97e77133a56490b` / `53b06c8c083c6f2cf4a3a1bf5320b7a819bca76f` |
| R15 `docs/vtrace/WP_TST_001.md` SHA-256 | `b38dd43763cc705771ef4bf2ff7c838c81b4c16ae58f79a319ac8f92fe95b6cb` |
| R15 `docs/vtrace/WP_TST_001.md` Git blob | `99ddada77364e46cd82ab9b37a1eeac68c9dfb4a` |

## Finding disposition

Independent implementation review found four critical and two major defects in
the R14 implementation. The commit is retained as failed history. It produced
no accepted mode evidence, review set, hold closure, exit, or authority.

R14 required its sole promotable implementation commit to be the direct child
of the original entry, so no later corrective child could satisfy the accepted
lineage rule. R15 repairs only that governance impossibility. A separately
accepted R15 entry may authorize one corrective direct child; complete
implementation custody remains the exact 18-path delta from the original R14
entry, while a second bounded delta records the correction from the R15 entry.

Every R14 implementation path, dependency, fixture, command, resource bound,
schema, trace, review lane, evidence lifecycle, exit rule, rollback, hold, and
no-authority rule remains normative. No path or behavior is broadened.

Only independent review with zero unresolved critical, major, or actionable
minor finding may recommend an R15 acceptance pulse. Acceptance would authorize
only a separate corrective-entry pulse; neither pulse binds its own future
commit or digest.

Disposition: **proposed corrective-lineage amendment; not accepted; not
entered; failed implementation retained; no evidence, exit, push, or release**.
