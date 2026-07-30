//! SEM-012: held, non-admitted Taxlane evidence pack.

use std::fmt::Write as _;

use crate::Document;

const KEYS: [&str; 23] = [
    "schema",
    "lane_id",
    "source_digest",
    "scenario_version",
    "need_summary",
    "alternative_count",
    "comparison_rule",
    "lifecycle_cost_low_millions",
    "lifecycle_cost_high_millions",
    "transition_cost_low_millions",
    "transition_cost_high_millions",
    "readiness_floor_bps",
    "safety_floor_bps",
    "supplier_floor_bps",
    "distribution_complete",
    "uncertainty_bps",
    "overlap_millions",
    "dissent_count",
    "unresolved_holds",
    "savings_state",
    "admission_state",
    "taxlane_contract_state",
    "evidence_state",
];

/// Builds a complete held pack that cannot admit or emit itself.
///
/// # Errors
///
/// Rejects incomplete, stale, non-held, self-admitting, false-floor, or
/// reversed range inputs.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    doc.exact(&KEYS)?;
    if doc.text("schema")? != "bastion.held-taxlane-pack.v1"
        || doc.text("evidence_state")? != "current"
    {
        return Err("unsupported or stale pack".into());
    }
    if doc.text("admission_state")? != "held" || doc.text("taxlane_contract_state")? != "unaccepted"
    {
        return Err("pack cannot self-admit or claim an accepted contract".into());
    }
    if doc.text("comparison_rule")? != "no-alternative-selected"
        || doc.text("distribution_complete")? != "true"
        || doc.text("savings_state")? != "held-unrealized"
    {
        return Err("comparison, distribution, or savings evidence is incomplete".into());
    }
    let lifecycle_low = doc.u64("lifecycle_cost_low_millions")?;
    let lifecycle_high = doc.u64("lifecycle_cost_high_millions")?;
    let transition_low = doc.u64("transition_cost_low_millions")?;
    let transition_high = doc.u64("transition_cost_high_millions")?;
    let readiness = doc.u64("readiness_floor_bps")?;
    let safety = doc.u64("safety_floor_bps")?;
    let supplier = doc.u64("supplier_floor_bps")?;
    let uncertainty = doc.u64("uncertainty_bps")?;
    if lifecycle_low > lifecycle_high
        || transition_low > transition_high
        || [readiness, safety, supplier, uncertainty]
            .iter()
            .any(|value| *value > 10_000)
        || safety < 9_500
    {
        return Err("invalid range or non-compensating floor".into());
    }
    if doc.u64("alternative_count")? == 0 || doc.text("unresolved_holds")? == "none" {
        return Err("held pack requires alternatives and unresolved holds".into());
    }
    let mut output = String::new();
    writeln!(output, "{{\"schema\":\"bastion.lane-evidence-pack-candidate.v1\",\"lane_id\":\"{}\",\"source_digest\":\"{}\",\"scenario_version\":\"{}\",\"need_summary\":\"{}\",\"alternative_count\":{},\"comparison_rule\":\"no-alternative-selected\",\"lifecycle_cost_range_millions\":[{lifecycle_low},{lifecycle_high}],\"transition_cost_range_millions\":[{transition_low},{transition_high}],\"readiness_floor_bps\":{readiness},\"safety_floor_bps\":{safety},\"supplier_floor_bps\":{supplier},\"distribution_complete\":true,\"uncertainty_bps\":{uncertainty},\"overlap_millions\":{},\"dissent_count\":{},\"unresolved_holds\":\"{}\",\"savings_state\":\"held-unrealized\",\"admission_state\":\"held\",\"admission_owner\":\"external-taxlane\",\"emitted\":false,\"authority\":{{\"taxlane_admission\":false,\"budget_allocation\":false,\"rate_change\":false,\"procurement_action\":false,\"operational_plan\":false,\"release\":false}}}}", doc.text("lane_id")?, doc.text("source_digest")?, doc.text("scenario_version")?, doc.text("need_summary")?, doc.u64("alternative_count")?, doc.u64("overlap_millions")?, doc.u64("dissent_count")?, doc.text("unresolved_holds")?).unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/held-taxlane-pack.fixture");

    #[test]
    fn packages_need_ranges_floors_distribution_uncertainty_and_holds() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"lifecycle_cost_range_millions\":[100000,120000]"));
        assert!(output.contains("\"savings_state\":\"held-unrealized\""));
        assert!(output.contains("\"admission_state\":\"held\""));
        assert!(output.contains("\"emitted\":false"));
    }

    #[test]
    fn rejects_self_admission_false_savings_or_failed_safety() {
        assert!(run(&FIXTURE.replace("admission_state=held", "admission_state=admitted")).is_err());
        assert!(
            run(&FIXTURE.replace("savings_state=held-unrealized", "savings_state=realized"))
                .is_err()
        );
        assert!(run(&FIXTURE.replace("safety_floor_bps=9700", "safety_floor_bps=9400")).is_err());
    }
}
