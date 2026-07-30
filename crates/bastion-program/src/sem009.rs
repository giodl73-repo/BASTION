//! SEM-009: non-operational transition-feasibility package.

use std::fmt::Write as _;

use crate::Document;

const KEYS: [&str; 16] = [
    "schema",
    "alternative_id",
    "owner",
    "authority",
    "industrial_capacity_bps",
    "workforce_capacity_bps",
    "milestones_total",
    "milestones_completed",
    "measure_defined",
    "cadence_days",
    "stop_trigger",
    "rollback_action",
    "safety_floor_bps",
    "transition_cost_millions",
    "evidence_state",
    "classification",
];

/// Evaluates whether a fictional alternative has a testable transition package.
///
/// # Errors
///
/// Rejects unknown, stale, non-public, malformed, or impossible fields.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    doc.exact(&KEYS)?;
    if doc.text("schema")? != "bastion.transition-feasibility.v1" {
        return Err("unsupported schema".into());
    }
    if doc.text("evidence_state")? != "current" || doc.text("classification")? != "safe-synthetic" {
        return Err("transition evidence is stale or outside safe-synthetic boundary".into());
    }
    let industrial = doc.u64("industrial_capacity_bps")?;
    let workforce = doc.u64("workforce_capacity_bps")?;
    let total = doc.u64("milestones_total")?;
    let completed = doc.u64("milestones_completed")?;
    let safety = doc.u64("safety_floor_bps")?;
    let cadence = doc.u64("cadence_days")?;
    if [industrial, workforce, safety]
        .iter()
        .any(|value| *value > 10_000)
        || completed > total
        || total == 0
        || cadence == 0
    {
        return Err("invalid transition measure".into());
    }
    let complete_controls = doc.text("owner")? != "unassigned"
        && doc.text("authority")? == "research-only"
        && doc.text("measure_defined")? == "true"
        && doc.text("stop_trigger")? != "missing"
        && doc.text("rollback_action")? != "missing";
    let posture = if !complete_controls {
        "held-incomplete-controls"
    } else if industrial < 7_000 || workforce < 7_000 {
        "held-capacity"
    } else if safety < 9_500 {
        "held-safety-floor"
    } else {
        "transition-testable-research"
    };
    let mut output = String::new();
    writeln!(output, "{{\"schema\":\"bastion.transition-feasibility-result.v1\",\"alternative_id\":\"{}\",\"posture\":\"{posture}\",\"owner\":\"{}\",\"authority_state\":\"research-only\",\"industrial_capacity_bps\":{industrial},\"workforce_capacity_bps\":{workforce},\"milestones\":{{\"total\":{total},\"completed\":{completed}}},\"measure_defined\":{},\"cadence_days\":{cadence},\"stop_trigger\":\"{}\",\"rollback_action\":\"{}\",\"safety_floor_bps\":{safety},\"transition_cost_millions\":{},\"implementation_ready\":false,\"authority\":{{\"procurement_action\":false,\"operational_plan\":false,\"deployment_advice\":false,\"release\":false}}}}", doc.text("alternative_id")?, doc.text("owner")?, doc.text("measure_defined")?, doc.text("stop_trigger")?, doc.text("rollback_action")?, doc.u64("transition_cost_millions")?).unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/synthetic/transition-feasibility.fixture");

    #[test]
    fn makes_complete_research_transition_testable_not_implementation_ready() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"posture\":\"transition-testable-research\""));
        assert!(output.contains("\"implementation_ready\":false"));
        assert!(output.contains("\"rollback_action\":\"restore-prior-support-package\""));
    }

    #[test]
    fn holds_missing_controls_capacity_or_safety() {
        assert!(
            run(&FIXTURE.replace("owner=portfolio-transition-office", "owner=unassigned"))
                .unwrap()
                .contains("held-incomplete-controls")
        );
        assert!(
            run(&FIXTURE.replace(
                "industrial_capacity_bps=7600",
                "industrial_capacity_bps=6500"
            ))
            .unwrap()
            .contains("held-capacity")
        );
        assert!(
            run(&FIXTURE.replace("safety_floor_bps=9700", "safety_floor_bps=9400"))
                .unwrap()
                .contains("held-safety-floor")
        );
    }
}
