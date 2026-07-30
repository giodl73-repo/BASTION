# WP-SEM-011 — Normalized NATO expenditure comparison

Status: `implemented; exit evidence pending final program audit`

SEM-011 compares 2025 estimated defence expenditure shares using NATO's own
definition, preserves the estimate flag, reconciles the US category
distribution within reported rounding, and explicitly forbids treating
spending shares as readiness evidence or targets.

Official source: NATO, *Defence Expenditure of NATO Countries (2014–2025)*,
Tables 3 and 8a.

Command: `cargo +1.95.0 run --locked --offline -q -p bastion-program --bin bastion-program -- sem-011 fixtures/public/nato-defence-expenditure-2025.fixture`.
