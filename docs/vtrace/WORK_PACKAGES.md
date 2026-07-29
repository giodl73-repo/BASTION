# BASTION Work Packages

## 1. Register status

This is an implementation-planning register, not implementation authority.
`WP-VV-001` planning and `WP-WS-001` execution are complete. The accepted
empty Rust workspace exists with digest-bound evidence. `WP-REV-001` and
`WP-TST-001` are acceptance-ready; every semantic producer remains
`proposed; entry_blocked`. No hold closure, HND emission, release, official
use, or public authority exists.

`CTRL-HND-EMIT-001` is a deferred non-WP control. It cannot authorize work or
appear as an accepted package. Any future emission requires a new assignment,
successor plan, all 12 applicable non-release holds and mapping decisions
closed, and a newly accepted exact WP. `TBD-REL-001` never becomes an HND-
emission prerequisite; it remains a separate release-blocking hold. The common
gates and exact aliases in
`IMPLEMENTATION_PLAN.md` are normative here.

## 2. Exact 20-WP register and DAG

| WP | Exact boundary/slice | Smallest coherent deliverable | Exact predecessors | Status |
|---|---|---|---|---|
| `WP-VV-001` | VTRACE planning only | Fixed VERIFICATION and VALIDATION plans and registries | frozen baseline | `discovery` |
| `WP-WS-001` | `PB-WS-001` | Empty workspace policy skeleton | `WP-VV-001` | `complete; accepted` |
| `WP-REV-001` | `PB-REV-001` plus `PB-WS-001` membership integration | Empty review/evidence-state scaffold only | `WP-WS-001` | `proposed; acceptance_ready` |
| `WP-TST-001` | `PB-TST-001`, `PB-FIX-001` plus `PB-WS-001` membership integration | Empty isolated harness and assigned fixture-custody scaffold only | `WP-WS-001` | `proposed; acceptance_ready` |
| `WP-CST-001` | `PB-CST-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | SOURCE/security/custody/re-admission gate | `WP-WS-001`, `WP-REV-001`, `WP-TST-001` | `proposed; entry_blocked` |
| `WP-AUT-001` | `PB-AUT-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Civilian mission/authority abstraction | `WP-CST-001` | `proposed; entry_blocked` |
| `WP-ACQ-001` | ACQ slice of `PB-DOM-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Acquisition/industrial-base facets | `WP-AUT-001` | `proposed; entry_blocked` |
| `WP-LOG-001` | LOG slice of `PB-DOM-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Logistics/sustainment lifecycle and tails | `WP-AUT-001`, `WP-ACQ-001` (ACQ creates/registers `PB-DOM-001`) | `proposed; entry_blocked` |
| `WP-ALLY-001` | ALLY slice of `PB-DOM-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Alliance/interoperability and separated burden | `WP-AUT-001`, `WP-ACQ-001` (ACQ creates/registers `PB-DOM-001`) | `proposed; entry_blocked` |
| `WP-RDY-001` | RDY slice of `PB-DOM-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Readiness/safety/resilience facets and floors | `WP-AUT-001`, `WP-LOG-001` | `proposed; entry_blocked` |
| `WP-DST-001` | DST slice of `PB-DOM-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Stakeholder distributions, incidence, burdens, tails | `WP-ACQ-001`, `WP-LOG-001`, `WP-ALLY-001` | `proposed; entry_blocked` |
| `WP-ECO-PRELIM-001` | preliminary ECO slice of `PB-PTH-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Preliminary six-pathway economic envelope | `WP-RDY-001`, `WP-ACQ-001`, `WP-LOG-001`, `WP-ALLY-001`, `WP-DST-001` | `proposed; entry_blocked` |
| `WP-DEL-001` | DEL slice of `PB-PTH-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Mandatory delivery/rollback/realization posture | `WP-ECO-PRELIM-001` | `proposed; entry_blocked` |
| `WP-ECO-FINAL-001` | final ECO slice of `PB-PTH-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Final predecessor-linked envelope | `WP-ECO-PRELIM-001`, `WP-DEL-001` | `proposed; entry_blocked` |
| `WP-ADP-001` | ADP slice of `PB-PTH-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Adaptive successor and later-feedback request | `WP-ECO-FINAL-001` | `proposed; entry_blocked` |
| `WP-HND-001` | `PB-HND-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Held/no-emission proof only | all current accepted product outputs and producer-owned review/test deltas through `WP-ADP-001` | `proposed; entry_blocked` |
| `WP-RUN-001` | `PB-RUN-001`, `PB-HND-001`, plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Fixed-order orchestration without semantics | all current product WPs through `WP-HND-001` | `proposed; entry_blocked` |
| `WP-GEN-001` | `PB-GEN-001`, `PB-FIX-001`, plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Derivative generated/fixture custody | all current product and runner digests | `proposed; entry_blocked` |
| `WP-DOC-001` | `PB-DOC-001` plus producer-owned `PB-REV-001` / `PB-TST-001` deltas | Contract/trace synchronization and REL no-output proof | all current product and runner digests | `proposed; entry_blocked` |
| `WP-INT-001` | final fixed 12-boundary graph | Final integration, support isolation, HND/TERM/REL proof | all current product/runner WPs, `WP-GEN-001`, `WP-DOC-001` | `proposed; entry_blocked` |

REV/TST bootstrap WPs precede CST and create no product meaning or producer
evidence. Each semantic producer later owns its exact sidecar delta inside its
own WP. GEN and DOC precede INT. `CTRL-HND-EMIT-001` is outside this DAG and is
not counted as a WP.

The V&V planning successors and `WP-WS-001` are complete. Separate acceptance
and entry of pure `WP-REV-001` and `WP-TST-001` bootstrap are now eligible. The
TST held pair is a proof input only for those two bootstrap WPs and closes only
through their later executed, independently accepted evidence. No such REV/TST
acceptance or TST closure exists yet; CST and every semantic WP remain blocked.

## 3. Exact controlled-source disposition

Ranges are inclusive. Each of the 298 controlled identities appears in exactly
one source-set row below; multiple WPs in an allocation do not change primary
semantic ownership.

### 3.1 Requirements and functional specifications

| Exact source set | Count | Primary WP | Exact cross-cutting closure |
|---|---:|---|---|
| `BASTION-REQ-SCP-001`–`005`, `BASTION-REQ-SCP-009`–`010`, `BASTION-REQ-SRC-001`–`008` | 15 | `WP-CST-001` | producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-SCP-006`–`008` | 3 | `WP-AUT-001` | `WP-CST-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-RDY-001`–`007` | 7 | `WP-RDY-001` | `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-ACQ-001`–`008` | 8 | `WP-ACQ-001` | `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-LOG-001`–`008` | 8 | `WP-LOG-001` | `WP-RDY-001`, `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-ALLY-001`–`006` | 6 | `WP-ALLY-001` | `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-DST-001`–`005` | 5 | `WP-DST-001` | `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-ECO-001`–`011`, `BASTION-REQ-ECO-014`–`016`, `BASTION-REQ-ECO-018`–`020` | 17 | `WP-ECO-PRELIM-001` | `WP-DEL-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, producer REV/TST deltas, `WP-INT-001` |
| `BASTION-REQ-ECO-012`–`013`, `BASTION-REQ-ECO-017` | 3 | `WP-ADP-001` | `WP-ECO-FINAL-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-TST-001`–`006` | 6 | `WP-REV-001` | `WP-VV-001`, `WP-TST-001`, producer-owned deltas, `WP-DOC-001`, `WP-INT-001` |
| `BASTION-REQ-VTR-001`–`003` | 3 | `WP-REV-001` | `WP-VV-001`, `WP-DOC-001`, `WP-INT-001` |
| `BASTION-REQ-DEL-001`–`007` | 7 | `WP-DEL-001` | `WP-ECO-FINAL-001`, `WP-ADP-001`, producer REV/TST delta, `WP-INT-001` |
| `BASTION-REQ-HND-001`–`007` | 7 | `WP-HND-001` | producer REV/TST delta, `WP-RUN-001`, `WP-INT-001`; `CTRL-HND-EMIT-001` future subset only |
| `BASTION-REQ-REL-001`–`003` | 3 | `WP-DOC-001` | `WP-TST-001`, `WP-INT-001`; no output |
| **Requirements** | **98** | **zero orphan** | |
| `SPEC-SCP-001`–`005`, `SPEC-SCP-009`–`010`, `SPEC-SRC-001`–`008` | 15 | `WP-CST-001` | producer REV/TST delta, `WP-INT-001` |
| `SPEC-SCP-006`–`008` | 3 | `WP-AUT-001` | `WP-CST-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-RDY-001`–`007` | 7 | `WP-RDY-001` | `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-ACQ-001`–`008` | 8 | `WP-ACQ-001` | `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-LOG-001`–`008` | 8 | `WP-LOG-001` | `WP-RDY-001`, `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-ALLY-001`–`006` | 6 | `WP-ALLY-001` | `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-DST-001`–`005` | 5 | `WP-DST-001` | `WP-ECO-PRELIM-001`, `WP-DEL-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-ECO-001`–`011`, `SPEC-ECO-014`–`016`, `SPEC-ECO-018`–`020` | 17 | `WP-ECO-PRELIM-001` | `WP-DEL-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, producer REV/TST deltas, `WP-INT-001` |
| `SPEC-ECO-012`–`013`, `SPEC-ECO-017` | 3 | `WP-ADP-001` | `WP-ECO-FINAL-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-TST-001`–`006` | 6 | `WP-REV-001` | `WP-VV-001`, `WP-TST-001`, producer-owned deltas, `WP-DOC-001`, `WP-INT-001` |
| `SPEC-VTR-001`–`003` | 3 | `WP-REV-001` | `WP-VV-001`, `WP-DOC-001`, `WP-INT-001` |
| `SPEC-DEL-001`–`007` | 7 | `WP-DEL-001` | `WP-ECO-FINAL-001`, `WP-ADP-001`, producer REV/TST delta, `WP-INT-001` |
| `SPEC-HND-001`–`007` | 7 | `WP-HND-001` | producer REV/TST delta, `WP-RUN-001`, `WP-INT-001`; `CTRL-HND-EMIT-001` future subset only |
| `SPEC-REL-001`–`003` | 3 | `WP-DOC-001` | `WP-TST-001`, `WP-INT-001`; no output |
| **Functional specifications** | **98** | **zero orphan** | |

### 3.2 Nonfunctional specifications

| Source | Exact WP allocation |
|---|---|
| `SPEC-NF-001` | `WP-CST-001`, `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-002` | `WP-CST-001`, `WP-AUT-001`, `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-003` | `WP-RDY-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, `WP-HND-001`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-004` | `WP-RDY-001`, `WP-LOG-001`, `WP-DST-001`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-005` | `WP-DST-001`, `WP-ECO-PRELIM-001`, `WP-DEL-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-006` | `WP-ECO-PRELIM-001`, `WP-DEL-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, `WP-HND-001`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-007` | `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-008` | `WP-PRODUCT`, `WP-TST-001`, `WP-GEN-001`, `WP-INT-001` |
| `SPEC-NF-009` | `WP-CST-001`, `WP-REV-001`, `WP-HND-001`, `WP-TST-001`, `WP-INT-001` |
| `SPEC-NF-010` | `WP-PRODUCT`, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |

### 3.3 Exact 13 held pairs

Each pair remains open. A blocker set is a direct/transitive entry block, not a
new dependency edge. A proof-input set permits only fail-closed or no-output
checks while the hold remains open; it never supplies favorable semantics,
entry, emission, or acceptance. `∅` is an exact empty set. `WP-PRODUCT` and
`WP-IMPLEMENTATION` are the frozen exact aliases in `IMPLEMENTATION_PLAN.md`.

### 3.3.1 Hold-to-WP forward allocation

| Exact held pair | Exact blocked current WP set | Exact proof-input WP set | Exact deferred-control block |
|---|---|---|---|
| `SPEC-UNK-SEC-001` / `TBD-SEC-001` | `WP-PRODUCT` + `{WP-GEN-001, WP-DOC-001, WP-INT-001}` | `{WP-TST-001}` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-RDY-001` / `TBD-RDY-001` | `{WP-AUT-001, WP-RDY-001, WP-ACQ-001, WP-LOG-001, WP-ALLY-001, WP-DST-001, WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-SRC-001` / `TBD-SRC-001` | `WP-PRODUCT` + `{WP-GEN-001, WP-DOC-001, WP-INT-001}` | `{WP-TST-001}` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-QNT-001` / `TBD-QNT-001` | `{WP-RDY-001, WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-ACQ-001` / `TBD-ACQ-001` | `{WP-ACQ-001, WP-DST-001, WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-LOG-001` / `TBD-LOG-001` | `{WP-LOG-001, WP-RDY-001, WP-DST-001, WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-ALLY-001` / `TBD-ALLY-001` | `{WP-ALLY-001, WP-DST-001, WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-DST-001` / `TBD-DST-001` | `{WP-DST-001, WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-ECO-001` / `TBD-ECO-001` | `{WP-ECO-PRELIM-001, WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-TST-001` / `TBD-TST-001` | `WP-PRODUCT` + `{WP-GEN-001, WP-DOC-001, WP-INT-001}` | `{WP-REV-001, WP-TST-001}` bootstrap only | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-DEL-001` / `TBD-DEL-001` | `{WP-DEL-001, WP-ECO-FINAL-001, WP-ADP-001, WP-HND-001, WP-RUN-001, WP-GEN-001, WP-DOC-001, WP-INT-001}` | `∅` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-HND-001` / `TBD-HND-001` | `∅` | `{WP-HND-001, WP-DOC-001, WP-INT-001}` | `{CTRL-HND-EMIT-001}` |
| `SPEC-UNK-REL-001` / `TBD-REL-001` | `∅` | `{WP-TST-001, WP-DOC-001, WP-INT-001}` | `∅`; every future release remains blocked outside the current WP/control set |

The deferred-control column is an exact reverse allocation:
`CTRL-HND-EMIT-001` is blocked by the 12 non-release holds `TBD-SEC-001`,
`TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`,
`TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001`,
and `TBD-HND-001`. `TBD-REL-001` is not in that set, never becomes an HND-
emission prerequisite, and remains blocking only for the separate future
release chain.

Accepted historical `CHG-BA-TST-001` permits planning-only VERIFICATION
authoring. Prospective `CHG-BA-TST-BOOT-002` proposes the exact bootstrap
allocation above: the TST pair remains open; it blocks every semantic/product
WP plus GEN, DOC, and INT; it is a proof input only for pure REV/TST bootstrap;
and WS has no TST relationship. Neither change closes either paired identity.

### 3.3.2 WP-to-hold reverse transpose

| WP | Exact blocking holds | Exact proof-input holds |
|---|---|---|
| `WP-VV-001` | `∅` | `∅` |
| `WP-WS-001` | `∅` | `∅` |
| `WP-REV-001` | `∅` | `TBD-TST-001` |
| `WP-TST-001` | `∅` | `TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001`, `TBD-REL-001` |
| `WP-CST-001` | `TBD-SEC-001`, `TBD-SRC-001`, `TBD-TST-001` | `∅` |
| `WP-AUT-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-TST-001` | `∅` |
| `WP-ACQ-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-ACQ-001`, `TBD-TST-001` | `∅` |
| `WP-LOG-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-LOG-001`, `TBD-TST-001` | `∅` |
| `WP-ALLY-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-ALLY-001`, `TBD-TST-001` | `∅` |
| `WP-RDY-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-LOG-001`, `TBD-TST-001` | `∅` |
| `WP-DST-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-TST-001` | `∅` |
| `WP-ECO-PRELIM-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001` | `∅` |
| `WP-DEL-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `∅` |
| `WP-ECO-FINAL-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `∅` |
| `WP-ADP-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `∅` |
| `WP-HND-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `TBD-HND-001` |
| `WP-RUN-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `∅` |
| `WP-GEN-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `∅` |
| `WP-DOC-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `TBD-HND-001`, `TBD-REL-001` |
| `WP-INT-001` | `TBD-SEC-001`, `TBD-RDY-001`, `TBD-SRC-001`, `TBD-QNT-001`, `TBD-ACQ-001`, `TBD-LOG-001`, `TBD-ALLY-001`, `TBD-DST-001`, `TBD-ECO-001`, `TBD-TST-001`, `TBD-DEL-001` | `TBD-HND-001`, `TBD-REL-001` |

The forward blocker pairs and proof-input pairs are authoritative. The reverse
table is their exact set transpose: 134 blocker pairs, 10 proof-input pairs,
and zero forward-only or reverse-only pairs. The only changes from the prior
137/8 fixed-point allocation are the three TST bootstrap relationships fixed
by `CHG-BA-TST-BOOT-002`; every non-TST relationship is unchanged.

### 3.4 Design decisions, contracts, and package boundaries

| Source | Exact WP allocation |
|---|---|
| `DES-SOURCE-001` | `WP-CST-001`, producer deltas, `WP-INT-001` |
| `DES-AUTH-001` | `WP-AUT-001`, `WP-CST-001`, producer delta, `WP-INT-001` |
| `DES-RDY-001` | `WP-RDY-001`, `WP-DEL-001`, producer delta, `WP-INT-001` |
| `DES-ACQ-001` | `WP-ACQ-001`, `WP-DST-001`, `WP-DEL-001`, producer delta, `WP-INT-001` |
| `DES-LOG-001` | `WP-LOG-001`, `WP-RDY-001`, `WP-DST-001`, `WP-DEL-001`, producer delta, `WP-INT-001` |
| `DES-ALLY-001` | `WP-ALLY-001`, `WP-DST-001`, `WP-DEL-001`, producer delta, `WP-INT-001` |
| `DES-DST-001` | `WP-DST-001`, `WP-DEL-001`, producer delta, `WP-INT-001` |
| `DES-ECO-001` | `WP-ECO-PRELIM-001`, `WP-DEL-001`, `WP-ECO-FINAL-001`, producer deltas, `WP-INT-001` |
| `DES-ADP-001` | `WP-ADP-001`, producer delta, `WP-INT-001` |
| `DES-TEST-001` | `WP-REV-001`, `WP-TST-001`, producer-owned deltas, `WP-DOC-001`, `WP-INT-001` |
| `DES-TRACE-001` | `WP-REV-001`, `WP-DOC-001`, `WP-INT-001` |
| `DES-DEL-001` | `WP-DEL-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, producer deltas, `WP-INT-001` |
| `DES-HND-001` | `WP-HND-001`, producer delta, `WP-RUN-001`, `WP-INT-001`; emission remains non-WP control |
| `DES-REL-001` | `WP-DOC-001`, `WP-TST-001`, `WP-INT-001`; no output |
| `CONTRACT-SOURCE-001` | `WP-CST-001`, all producer deltas, `WP-INT-001` |
| `CONTRACT-AUTH-001` | `WP-AUT-001`, producer delta, `WP-INT-001` |
| `CONTRACT-RDY-001` | `WP-RDY-001`, producer delta, `WP-INT-001` |
| `CONTRACT-ACQ-001` | `WP-ACQ-001`, producer delta, `WP-INT-001` |
| `CONTRACT-LOG-001` | `WP-LOG-001`, producer delta, `WP-INT-001` |
| `CONTRACT-ALLY-001` | `WP-ALLY-001`, producer delta, `WP-INT-001` |
| `CONTRACT-DST-001` | `WP-DST-001`, producer delta, `WP-INT-001` |
| `CONTRACT-ECO-001` | `WP-ECO-PRELIM-001`, `WP-ECO-FINAL-001`, `WP-ADP-001`, producer deltas, `WP-INT-001` |
| `CONTRACT-TEST-001` | `WP-REV-001`, `WP-TST-001`, producer-owned deltas, `WP-INT-001`; TERM remains a branch |
| `CONTRACT-DEL-001` | `WP-DEL-001`, producer delta, `WP-INT-001` |
| `CONTRACT-HND-001` | `WP-HND-001`, producer delta, `WP-RUN-001`, `WP-INT-001`; no emission |
| `CONTRACT-REL-001` | `WP-DOC-001`, `WP-TST-001`, `WP-INT-001`; no output |
| `CONTRACT-TRACE-001` | `WP-REV-001`, `WP-DOC-001`, `WP-INT-001` |
### 3.4.1 WP-to-boundary forward touch allocation

A touch is an authored primary slice, first-member workspace integration,
producer-owned REV/TST sidecar delta, or fixed support/integration custody. Reading an accepted predecessor digest does
not by itself create a touch. `∅` is exact. Every `WP-SIDECAR-PRODUCERS`
member has its product/support boundary plus both sidecars.

| WP | Exact touched package-boundary set |
|---|---|
| `WP-VV-001` | `∅` |
| `WP-WS-001` | `{PB-WS-001}` |
| `WP-REV-001` | `{PB-WS-001, PB-REV-001}` |
| `WP-TST-001` | `{PB-WS-001, PB-TST-001, PB-FIX-001}` |
| `WP-CST-001` | `{PB-WS-001, PB-CST-001, PB-REV-001, PB-TST-001}` |
| `WP-AUT-001` | `{PB-WS-001, PB-AUT-001, PB-REV-001, PB-TST-001}` |
| `WP-ACQ-001` | `{PB-WS-001, PB-DOM-001, PB-REV-001, PB-TST-001}` |
| `WP-LOG-001` | `{PB-DOM-001, PB-REV-001, PB-TST-001}` |
| `WP-ALLY-001` | `{PB-DOM-001, PB-REV-001, PB-TST-001}` |
| `WP-RDY-001` | `{PB-DOM-001, PB-REV-001, PB-TST-001}` |
| `WP-DST-001` | `{PB-DOM-001, PB-REV-001, PB-TST-001}` |
| `WP-ECO-PRELIM-001` | `{PB-WS-001, PB-PTH-001, PB-REV-001, PB-TST-001}` |
| `WP-DEL-001` | `{PB-PTH-001, PB-REV-001, PB-TST-001}` |
| `WP-ECO-FINAL-001` | `{PB-PTH-001, PB-REV-001, PB-TST-001}` |
| `WP-ADP-001` | `{PB-PTH-001, PB-REV-001, PB-TST-001}` |
| `WP-HND-001` | `{PB-WS-001, PB-HND-001, PB-REV-001, PB-TST-001}` |
| `WP-RUN-001` | `{PB-WS-001, PB-HND-001, PB-RUN-001, PB-REV-001, PB-TST-001}` |
| `WP-GEN-001` | `{PB-FIX-001, PB-GEN-001, PB-REV-001, PB-TST-001}` |
| `WP-DOC-001` | `{PB-DOC-001, PB-REV-001, PB-TST-001}` |
| `WP-INT-001` | `{PB-WS-001, PB-CST-001, PB-AUT-001, PB-DOM-001, PB-PTH-001, PB-REV-001, PB-HND-001, PB-RUN-001, PB-DOC-001, PB-TST-001, PB-FIX-001, PB-GEN-001}` |

### 3.4.2 Boundary-to-WP reverse touch transpose

| Package boundary | Exact touching WP set |
|---|---|
| `PB-WS-001` | `{WP-WS-001, WP-REV-001, WP-TST-001, WP-CST-001, WP-AUT-001, WP-ACQ-001, WP-ECO-PRELIM-001, WP-HND-001, WP-RUN-001, WP-INT-001}` |
| `PB-CST-001` | `{WP-CST-001, WP-INT-001}` |
| `PB-AUT-001` | `{WP-AUT-001, WP-INT-001}` |
| `PB-DOM-001` | `WP-DOMAIN` + `{WP-INT-001}` |
| `PB-PTH-001` | `WP-PATH` + `{WP-INT-001}` |
| `PB-REV-001` | `{WP-REV-001}` + `WP-SIDECAR-PRODUCERS` + `{WP-INT-001}` |
| `PB-HND-001` | `{WP-HND-001, WP-RUN-001, WP-INT-001}` |
| `PB-RUN-001` | `{WP-RUN-001, WP-INT-001}` |
| `PB-DOC-001` | `{WP-DOC-001, WP-INT-001}` |
| `PB-TST-001` | `{WP-TST-001}` + `WP-SIDECAR-PRODUCERS` + `{WP-INT-001}` |
| `PB-FIX-001` | `{WP-TST-001, WP-GEN-001, WP-INT-001}` |
| `PB-GEN-001` | `{WP-GEN-001, WP-INT-001}` |

The 20 forward rows and 12 reverse rows are authoritative exact set
transposes: 71 WP/PB touch pairs, zero forward-only or reverse-only pairs.

### 3.5 Exact 40 code-rigor dispositions

Every row is blocked from implementation. `WP-VV-001` fixes methods; producers
later own their exact evidence deltas.

| CR set | Exact WP allocation |
|---|---|
| `CR-001` | `WP-VV-001`, `WP-WS-001`, `WP-DOC-001` |
| `CR-002` | `WP-PRODUCT`, `WP-TST-001` |
| `CR-003`–`CR-006` | `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `CR-007` | `WP-WS-001`, `WP-CST-001`, `WP-REV-001`, `WP-HND-001`, `WP-RUN-001`, `WP-INT-001` |
| `CR-008`, `CR-009` | `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `CR-010` | `WP-PRODUCT`, producer-owned REV/TST deltas, `WP-TST-001`, `WP-INT-001`; REL no-output and TERM non-product exceptions preserved |
| `CR-011`–`CR-013` | `WP-PRODUCT`, `WP-TST-001`, `WP-GEN-001`, `WP-INT-001` |
| `CR-014` | `WP-IMPLEMENTATION` |
| `CR-015` | `WP-CST-001`, `WP-AUT-001`, `WP-DOMAIN`, `WP-HND-001`, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-016` | `WP-CST-001`, `WP-LOG-001`, `WP-HND-001`, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-017` | `WP-AUT-001`, `WP-RDY-001`, `WP-LOG-001`, `WP-ALLY-001`, `WP-DST-001`, `WP-PATH`, `WP-HND-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-018` | `WP-DOMAIN`, `WP-TST-001`, `WP-INT-001` |
| `CR-019` | `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `CR-020`, `CR-021` | `WP-DOMAIN`, `WP-PATH`, `WP-HND-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-022` | `WP-PATH`, `WP-HND-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-023` | `WP-REV-001`, producer-owned REV/TST deltas, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-024` | `WP-REV-001`, `WP-HND-001`, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-025`–`CR-030` | `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `CR-031` | `WP-CST-001`, `WP-REV-001`, `WP-HND-001`, `WP-DOC-001`, `WP-TST-001`, `WP-GEN-001`, `WP-INT-001`; parser/fuzz branch conditional |
| `CR-032` | `WP-PRODUCT`, `WP-TST-001`, `WP-GEN-001`, `WP-INT-001` |
| `CR-033` | `WP-IMPLEMENTATION` |
| `CR-034` | `WP-CST-001`, `WP-REV-001`, `WP-DOC-001`, `WP-TST-001`, `WP-GEN-001`, `WP-INT-001` |
| `CR-035` | `WP-VV-001`, `WP-WS-001`, `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `CR-036` | `WP-VV-001`, `WP-WS-001`, `WP-PRODUCT`, `WP-TST-001`, `WP-GEN-001`, `WP-INT-001` |
| `CR-037` | `WP-VV-001`, `WP-PRODUCT`, `WP-TST-001`, `WP-INT-001` |
| `CR-038` | `WP-VV-001`, `WP-WS-001`, `WP-REV-001`, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-039` | `WP-VV-001`, `WP-REV-001`, `WP-DOC-001`, `WP-TST-001`, `WP-INT-001` |
| `CR-040` | `WP-ALL` |

## 4. Per-WP ten-area V-closure matrix

Every row dispositions `VCL-ALL`; accepted evidence is present only for
`WP-VV-001` planning and `WP-WS-001` execution.

| WP | Exact closure | WP-specific focus | Current disposition |
|---|---|---|---|
| `WP-VV-001` | `VCL-ALL` planning methods/owners only; product execution N/A | 298-identity transpose, methods, commands, evidence and decision schemas | `complete; accepted planning fixed point` |
| `WP-WS-001` | `VCL-ALL`; product-only cases reasoned N/A | empty membership, fixed edges, no semantics | `complete; accepted EVT-A` |
| `WP-CST-001` | `VCL-ALL` | SOURCE/security/custody/re-admission, safe failure | `pending` |
| `WP-REV-001` | `VCL-ALL` bootstrap scope only | independence, digest, findings/dissent/evidence state; no producer pass | `pending` |
| `WP-TST-001` | `VCL-ALL` bootstrap scope only | isolation, safe corpus, harness controls; no producer evidence | `pending` |
| `WP-AUT-001` | `VCL-ALL` | civilian authority, mission bounds, SOURCE re-admission | `pending` |
| `WP-ACQ-001` | `VCL-ALL` | acquisition/commonality/capacity/concentration facets | `pending` |
| `WP-LOG-001` | `VCL-ALL` | custody, maintenance/repair tails, lifecycle/degraded recovery | `pending` |
| `WP-ALLY-001` | `VCL-ALL` | sovereignty, compatibility, partner capacity, separated burden | `pending` |
| `WP-RDY-001` | `VCL-ALL` | readiness/safety/resilience/surge/recovery floors | `pending` |
| `WP-DST-001` | `VCL-ALL` | seven lenses, distributions, concentrated effects, tails | `pending` |
| `WP-ECO-PRELIM-001` | `VCL-ALL` | six pathways, checked preliminary accounting, overlap/uncertainty | `pending` |
| `WP-DEL-001` | `VCL-ALL` | owner/capacity/milestones/stop/rollback/realization | `pending` |
| `WP-ECO-FINAL-001` | `VCL-ALL` | predecessor bond, DEL dependence, final reconciliation | `pending` |
| `WP-ADP-001` | `VCL-ALL` | immutable successor, lifecycle, later-feedback restart | `pending` |
| `WP-HND-001` | `VCL-ALL` | held/no emission, bundle identity, TERM finiteness/backflow | `pending`; open holds are proof inputs |
| `WP-RUN-001` | `VCL-ALL` | fixed ordering, typed failures, no semantic ownership | `pending` |
| `WP-GEN-001` | `VCL-ALL` | safe fixtures, deterministic regeneration, no reverse edge | `pending` |
| `WP-DOC-001` | `VCL-ALL` | trace truth, REL no output, no authority/release claim | `pending` |
| `WP-INT-001` | `VCL-ALL` | final post-GEN/DOC source spine, all contracts/invariants/holds/edges | `pending`; every predecessor absent |

## 5. Per-WP parliament, domain, and assurance matrix

`R/P` means required and pending; `PASS` means completed at the accepted exact
WP digest; `N/A` requires the stated fixed reason. Each
`PAR-ALL` cell expands to all eight exact parliament lanes.

| WP | Eight parliament lanes | Exact domain concurrence | Classification & Operational Security | Civilian Control/Law/Safety/Readiness |
|---|---|---|---|---|
| `WP-VV-001` | `PAR-ALL PASS` | all required planning owners PASS | PASS | PASS |
| `WP-WS-001` | `PAR-ALL N/A`: empty coordination only | Maintainer/Scope/independent Review PASS | N/A: no content | N/A: no semantics |
| `WP-CST-001` | `PAR-ALL R/P` | Source/security owners R/P | R/P | R/P |
| `WP-REV-001` | `PAR-ALL R/P` | Review/Test owners R/P | R/P | R/P |
| `WP-TST-001` | `PAR-ALL R/P` | Independent Test/Methodology R/P | R/P | R/P |
| `WP-AUT-001` | `PAR-ALL R/P` | Civilian mission/authority owner R/P | R/P | R/P |
| `WP-ACQ-001` | `PAR-ALL R/P` | Acquisition/industrial-base owner R/P | R/P | R/P |
| `WP-LOG-001` | `PAR-ALL R/P` | Logistics/sustainment owner R/P | R/P | R/P |
| `WP-ALLY-001` | `PAR-ALL R/P` | Alliance/interoperability owner R/P | R/P | R/P |
| `WP-RDY-001` | `PAR-ALL R/P` | Readiness-system owner R/P | R/P | R/P |
| `WP-DST-001` | `PAR-ALL R/P` | Personnel/family/workforce/community owner R/P | R/P | R/P |
| `WP-ECO-PRELIM-001` | `PAR-ALL R/P` | Defense resource/Comptroller owners R/P | R/P | R/P |
| `WP-DEL-001` | `PAR-ALL R/P` | Delivery owner R/P | R/P | R/P |
| `WP-ECO-FINAL-001` | `PAR-ALL R/P` | Defense resource/Comptroller and Delivery owners R/P | R/P | R/P |
| `WP-ADP-001` | `PAR-ALL R/P` | Adaptive/economics and Delivery owners R/P | R/P | R/P |
| `WP-HND-001` | `PAR-ALL R/P` | Taxlane adapter and all contributing owners R/P | R/P | R/P |
| `WP-RUN-001` | `PAR-ALL R/P` | all invoked owners plus Maintainer R/P | R/P | R/P |
| `WP-GEN-001` | `PAR-ALL R/P` | producing owners/Data/Citation R/P | R/P | R/P |
| `WP-DOC-001` | `PAR-ALL R/P` | Maintainer/Citation/Numeracy/Scope R/P | R/P | R/P |
| `WP-INT-001` | `PAR-ALL R/P` | all fixed semantic owners R/P | R/P | R/P |

No lane may compensate for another. Bootstrap review cannot approve a producer.

## 6. Per-WP L0/L1/L2 posture

`WP-WS-001` command execution is complete. Command slots for every later WP
remain symbolic and unavailable until that exact WP is accepted.

| WP | L0 | L1 | L2 |
|---|---|---|---|
| `WP-VV-001` | product execution N/A | product execution N/A | product execution N/A; planning digest/orphan/contradiction/independent review only |
| `WP-WS-001` | complete; accepted | complete; accepted | fixed-edge/no-semantics proof complete; accepted |
| `WP-CST-001` | required; pending | required; pending | SOURCE/security/contract/adversarial closure required; pending |
| `WP-REV-001` | required; pending | required; pending | bootstrap independence/evidence-state only; pending |
| `WP-TST-001` | required; pending | required; pending | bootstrap isolation/safe-corpus only; pending |
| `WP-AUT-001` | required; pending | required; pending | authority/re-admission/adversarial closure required; pending |
| `WP-ACQ-001` | required; pending | required; pending | ACQ contract/facet/property closure required; pending |
| `WP-LOG-001` | required; pending | required; pending | LOG contract/tail/degraded closure required; pending |
| `WP-ALLY-001` | required; pending | required; pending | ALLY contract/sovereignty/burden closure required; pending |
| `WP-RDY-001` | required; pending | required; pending | RDY contract/floor/distribution closure required; pending |
| `WP-DST-001` | required; pending | required; pending | DST lenses/distribution/tail closure required; pending |
| `WP-ECO-PRELIM-001` | required; pending | required; pending | preliminary accounting/pathway/overlap closure required; pending |
| `WP-DEL-001` | required; pending | required; pending | delivery/rollback/realization/successor closure required; pending |
| `WP-ECO-FINAL-001` | required; pending | required; pending | predecessor/final accounting closure required; pending |
| `WP-ADP-001` | required; pending | required; pending | lifecycle/successor/restart closure required; pending |
| `WP-HND-001` | required; pending | required; pending | no-emission/TERM/no-backflow closure required; pending |
| `WP-RUN-001` | required; pending | required; pending | fixed order/typed failure/no-semantics closure required; pending |
| `WP-GEN-001` | required; pending | required; pending | regeneration/fixture/no-reverse-edge closure required; pending |
| `WP-DOC-001` | required; pending | required; pending | trace/REL/no-authority closure required; pending |
| `WP-INT-001` | required; pending | required; pending | all final L2 areas mandatory after GEN/DOC; pending |

## 7. Per-WP entry and exit deltas

All rows inherit the common gates, their exact section 3.3.2 blocking-hold set,
and producer-owned REV/TST closure. “Exact blocker set closed” below always
means that enumerated reverse row, never a prose-derived subset.

| WP | Additional exact entry | Exact exit evidence |
|---|---|---|
| `WP-VV-001` | assignment permits planning only | accepted VERIFICATION/VALIDATION digests; all registries exact; 298 identities zero-orphan; no code diff |
| `WP-WS-001` | representation/toolchain/workspace policy accepted | empty membership/fixed-edge/no-semantics proof |
| `WP-CST-001` | SEC/SRC methods, bounds, safe corpus accepted | SOURCE/security contracts, re-admission, safe failure, aggregate-only evidence |
| `WP-REV-001` | review schema, independence, severity/convergence method accepted | bootstrap digest/conflict/finding/dissent/state proof; no producer pass |
| `WP-TST-001` | isolated test boundary and safe-corpus plan accepted | bootstrap isolation and fixture controls; no producer evidence pre-claimed |
| `WP-AUT-001` | authority method accepted; exact blocker set closed | bounded authority/mission and SOURCE re-admission evidence |
| `WP-ACQ-001` | ACQ method accepted; exact blocker set closed | all acquisition/commonality/capacity/concentration facets and producer delta close |
| `WP-LOG-001` | LOG method accepted; exact blocker set closed | lifecycle/custody/tail/degraded recovery and producer delta close |
| `WP-ALLY-001` | ALLY method accepted; exact blocker set closed | sovereignty/compatibility/partner/separated-ledger and producer delta close |
| `WP-RDY-001` | RDY method accepted; exact blocker set closed | readiness/safety/resilience/surge/recovery floors and producer delta close |
| `WP-DST-001` | DST method accepted; exact blocker set closed | seven lenses/distributions/burdens/tails and producer delta close |
| `WP-ECO-PRELIM-001` | economic method accepted; exact blocker set closed | preliminary six-pathway checked envelope and producer delta close |
| `WP-DEL-001` | delivery method accepted; exact blocker set closed | delivery owner/capacity/stop/rollback/realization and producer delta close |
| `WP-ECO-FINAL-001` | accepted preliminary ECO and DEL exact digests | final predecessor-linked checked envelope and producer delta close |
| `WP-ADP-001` | accepted final ECO and adaptive method | immutable successor/lifecycle/later-feedback restart and producer delta close |
| `WP-HND-001` | plans and exact no-emission representation accepted; exact blocker set closed; section 3.3.2 proof-input set remains open | held/no pack, bundle identity, finite TERM, no backflow/Taxlane/release authority |
| `WP-RUN-001` | all current product digests accepted | fixed order, typed failure propagation, no semantic ownership/new interface |
| `WP-GEN-001` | all current product/runner digests and generator/fixture custody accepted | deterministic regeneration, safe fixture inventory, no hand edit/reverse edge |
| `WP-DOC-001` | all described product/runner digests accepted | exact trace, evidence truth, REL no output, no release/authority claim |
| `WP-INT-001` | every current product/runner plus GEN/DOC complete at accepted digests | final 13-contract, 25-invariant, 18-transition, 13-hold, source spine, support isolation, HND/TERM/REL and forbidden-edge closure |

## 8. Product work versus VTRACE closeout

`WP-VV-001` owns only verification/validation planning and its fixed-point
records. It cannot create implementation or accept a product WP. Product
evidence may update controlled ledgers only inside its later accepted WP; a
ledger update cannot supply meaning, close a hold, self-accept evidence, or
create authority.

## 9. Orphan, partition, and readiness audit

| Controlled class | Count | Allocation result |
|---|---:|---|
| Requirements | 98 | section 3.1; zero orphan |
| Functional specifications | 98 | section 3.1; zero orphan |
| Nonfunctional specifications | 10 | section 3.2; zero orphan |
| Held specification identities | 13 | section 3.3; 134 blocker pairs and 10 proof-input pairs in exact forward/reverse equality; zero orphan; zero closed |
| Design decisions | 14 | section 3.4; zero orphan |
| Contracts | 13 | section 3.4; zero orphan; no fourteenth contract |
| Package boundaries | 12 | section 3.4; 71 exact WP/PB touch pairs in forward/reverse equality; zero orphan; no new boundary |
| Code-rigor constraints | 40 | section 3.5; zero orphan |
| **Controlled identities** | **298** | **298 allocated; zero orphan** |

All 25 invariants and 18 transitions close through their exact CR allocation,
producer-owned deltas, and final INT evidence; none is assumed. The register
contains exactly 20 WPs: completed planning and WS baselines, two
acceptance-ready bootstrap WPs, and 16 blocked later WPs. Future HND emission
remains non-WP deferred control. There are zero accepted waivers or hold
closures. Semantic implementation remains wholly blocked.
