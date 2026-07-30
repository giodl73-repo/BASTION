//! SEM-010: observed trigger to immutable readiness successor cycle.

use std::fmt::Write as _;

use crate::Document;

const KEYS: [&str; 16] = [
    "schema",
    "current_version",
    "successor_version",
    "current_rank",
    "successor_rank",
    "owner",
    "observed_readiness_floor_bps",
    "readiness_trigger_bps",
    "observed_lifecycle_cost_millions",
    "cost_ceiling_millions",
    "observed_safety_bps",
    "safety_trigger_bps",
    "observed_supplier_bps",
    "supplier_trigger_bps",
    "action",
    "evidence_state",
];

/// Converts current aggregate observations into at most one successor action.
///
/// # Errors
///
/// Rejects stale evidence, in-place mutation, non-monotone rank, invalid
/// measures, or an action inconsistent with trigger state.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    doc.exact(&KEYS)?;
    if doc.text("schema")? != "bastion.adaptive-readiness-cycle.v1"
        || doc.text("evidence_state")? != "current"
    {
        return Err("unsupported or stale adaptive cycle".into());
    }
    if doc.text("current_version")? == doc.text("successor_version")? {
        return Err("successor cannot mutate current version".into());
    }
    let current_rank = doc.u64("current_rank")?;
    let successor_rank = doc.u64("successor_rank")?;
    if successor_rank
        != current_rank
            .checked_add(1)
            .ok_or_else(|| "rank overflow".to_owned())?
    {
        return Err("successor rank must increment exactly once".into());
    }
    let readiness = doc.u64("observed_readiness_floor_bps")?;
    let readiness_trigger = doc.u64("readiness_trigger_bps")?;
    let cost = doc.u64("observed_lifecycle_cost_millions")?;
    let cost_ceiling = doc.u64("cost_ceiling_millions")?;
    let safety = doc.u64("observed_safety_bps")?;
    let safety_trigger = doc.u64("safety_trigger_bps")?;
    let supplier = doc.u64("observed_supplier_bps")?;
    let supplier_trigger = doc.u64("supplier_trigger_bps")?;
    if [
        readiness,
        readiness_trigger,
        safety,
        safety_trigger,
        supplier,
        supplier_trigger,
    ]
    .iter()
    .any(|value| *value > 10_000)
    {
        return Err("adaptive basis-point value exceeds 10000".into());
    }
    let mut triggers = Vec::new();
    if readiness < readiness_trigger {
        triggers.push("readiness-floor");
    }
    if cost > cost_ceiling {
        triggers.push("lifecycle-cost");
    }
    if safety < safety_trigger {
        triggers.push("safety-floor");
    }
    if supplier < supplier_trigger {
        triggers.push("supplier-floor");
    }
    let action = doc.text("action")?;
    if triggers.is_empty() && action != "retain" {
        return Err("action requires an observed trigger".into());
    }
    if !triggers.is_empty() && action == "retain" {
        return Err("triggered cycle cannot silently retain".into());
    }
    let mut output = String::new();
    write!(output, "{{\"schema\":\"bastion.adaptive-successor.v1\",\"current_version\":\"{}\",\"successor_version\":\"{}\",\"current_rank\":{current_rank},\"successor_rank\":{successor_rank},\"owner\":\"{}\",\"triggers\":[", doc.text("current_version")?, doc.text("successor_version")?, doc.text("owner")?).unwrap();
    for (index, trigger) in triggers.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        write!(output, "\"{trigger}\"").unwrap();
    }
    writeln!(output, "],\"action\":\"{action}\",\"current_mutated\":false,\"same_invocation_retry\":false,\"authority\":{{\"procurement_action\":false,\"operational_plan\":false,\"deployment_advice\":false,\"release\":false}}}}").unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/synthetic/adaptive-readiness-cycle.fixture");

    #[test]
    fn creates_one_successor_from_readiness_cost_and_supplier_triggers() {
        let output = run(FIXTURE).unwrap();
        assert!(
            output.contains(
                "\"triggers\":[\"readiness-floor\",\"lifecycle-cost\",\"supplier-floor\"]"
            )
        );
        assert!(output.contains("\"successor_rank\":3"));
        assert!(output.contains("\"current_mutated\":false"));
    }

    #[test]
    fn rejects_in_place_nonmonotone_or_inconsistent_action() {
        assert!(
            run(&FIXTURE.replace(
                "successor_version=bastion-cycle-v3",
                "successor_version=bastion-cycle-v2"
            ))
            .is_err()
        );
        assert!(run(&FIXTURE.replace("successor_rank=3", "successor_rank=4")).is_err());
        assert!(
            run(&FIXTURE.replace("action=revise-commonality-workforce", "action=retain")).is_err()
        );
    }
}
