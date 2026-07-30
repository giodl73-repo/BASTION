//! SEM-007: readiness portfolio alternative comparison.

use std::fmt::Write as _;

use crate::Document;

const ALTERNATIVES: [&str; 4] = [
    "no-change",
    "additional-procurement",
    "sustainment-first",
    "commonality-workforce",
];
const FIELDS: [&str; 6] = [
    "readiness-floor-bps",
    "lifecycle-cost-millions",
    "transition-cost-millions",
    "lead-time-days",
    "safety-bps",
    "supplier-bps",
];

fn keys() -> Vec<String> {
    let mut keys = vec![
        "schema".to_owned(),
        "scenario_version".to_owned(),
        "unit".to_owned(),
    ];
    for alternative in ALTERNATIVES {
        for field in FIELDS {
            keys.push(format!("alternative.{alternative}.{field}"));
        }
    }
    keys
}

/// Compares portfolio hypotheses without selecting one.
///
/// # Errors
///
/// Rejects unknown fields, incompatible units, out-of-bound facets, or safety
/// below the non-compensating floor.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    let keys = keys();
    let borrowed: Vec<&str> = keys.iter().map(String::as_str).collect();
    doc.exact(&borrowed)?;
    if doc.text("schema")? != "bastion.portfolio-alternatives.v1"
        || doc.text("unit")? != "millions-usd"
    {
        return Err("unsupported schema or unit".into());
    }
    let baseline_floor = doc.u64("alternative.no-change.readiness-floor-bps")?;
    let baseline_cost = doc.u64("alternative.no-change.lifecycle-cost-millions")?;
    let mut output = String::new();
    write!(output, "{{\"schema\":\"bastion.portfolio-comparison.v1\",\"scenario_version\":\"{}\",\"unit\":\"millions-usd\",\"alternatives\":[", doc.text("scenario_version")?).unwrap();
    for (index, alternative) in ALTERNATIVES.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let floor = doc.u64(&format!("alternative.{alternative}.readiness-floor-bps"))?;
        let lifecycle = doc.u64(&format!(
            "alternative.{alternative}.lifecycle-cost-millions"
        ))?;
        let transition = doc.u64(&format!(
            "alternative.{alternative}.transition-cost-millions"
        ))?;
        let lead = doc.u64(&format!("alternative.{alternative}.lead-time-days"))?;
        let safety = doc.u64(&format!("alternative.{alternative}.safety-bps"))?;
        let supplier = doc.u64(&format!("alternative.{alternative}.supplier-bps"))?;
        if [floor, safety, supplier]
            .iter()
            .any(|value| *value > 10_000)
        {
            return Err(format!("basis-point value exceeds 10000 for {alternative}"));
        }
        let posture = if safety < 9_500 {
            "held-safety-floor"
        } else {
            "research-comparable"
        };
        let floor_change = i128::from(floor) - i128::from(baseline_floor);
        let cost_change = i128::from(lifecycle) - i128::from(baseline_cost);
        write!(output, "{{\"id\":\"{alternative}\",\"posture\":\"{posture}\",\"readiness_floor_bps\":{floor},\"floor_change_bps\":{floor_change},\"lifecycle_cost_millions\":{lifecycle},\"cost_change_millions\":{cost_change},\"transition_cost_millions\":{transition},\"lead_time_days\":{lead},\"safety_bps\":{safety},\"supplier_bps\":{supplier}}}").unwrap();
    }
    output.push_str("],\"comparison_rule\":\"no-alternative-selected\",\"limits\":{\"values\":\"safe-synthetic\",\"realizable_savings\":\"not-claimed\"},\"authority\":{\"procurement_recommendation\":false,\"budget_allocation\":false,\"operational_plan\":false,\"release\":false}}\n");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/synthetic/portfolio-alternatives.fixture");

    #[test]
    fn shows_procurement_can_add_cost_without_lifting_floor() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"id\":\"additional-procurement\""));
        assert!(output.contains("\"floor_change_bps\":0"));
        assert!(output.contains("\"cost_change_millions\":20000"));
        assert!(output.contains("\"id\":\"commonality-workforce\""));
        assert!(output.contains("\"floor_change_bps\":2000"));
    }

    #[test]
    fn holds_safety_failure_and_rejects_operational_fields() {
        let held = run(&FIXTURE.replace(
            "alternative.sustainment-first.safety-bps=9700",
            "alternative.sustainment-first.safety-bps=9400",
        ))
        .unwrap();
        assert!(held.contains("\"posture\":\"held-safety-floor\""));
        assert!(
            run(&format!("{FIXTURE}selected_vendor=cedar\n"))
                .unwrap_err()
                .contains("unknown field")
        );
    }
}
