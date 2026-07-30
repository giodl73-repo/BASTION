//! SEM-004: aggregate supply, workforce, maintenance, and recovery scenarios.

use std::fmt::Write as _;

use crate::Document;

const SCENARIOS: [&str; 3] = ["baseline", "disruption", "recovery"];
const FACETS: [&str; 4] = [
    "supplier-bps",
    "workforce-bps",
    "maintenance-bps",
    "spares-bps",
];

fn keys() -> Vec<String> {
    let mut keys = vec![
        "schema".to_owned(),
        "source_id".to_owned(),
        "baseline_version".to_owned(),
        "successor_version".to_owned(),
        "recovery_days".to_owned(),
    ];
    for scenario in SCENARIOS {
        for facet in FACETS {
            keys.push(format!("scenario.{scenario}.{facet}"));
        }
    }
    keys
}

/// Compares public-aggregate-derived, non-operational stress scenarios.
///
/// # Errors
///
/// Rejects unknown fields, invalid versions, out-of-bound facets, or recovery
/// that remains below the disruption floor.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    let keys = keys();
    let borrowed: Vec<&str> = keys.iter().map(String::as_str).collect();
    doc.exact(&borrowed)?;
    if doc.text("schema")? != "bastion.aggregate-stress-recovery.v1" {
        return Err("unsupported schema".into());
    }
    if doc.text("baseline_version")? == doc.text("successor_version")? {
        return Err("successor scenario must have a new immutable version".into());
    }
    let recovery_days = doc.u64("recovery_days")?;
    if recovery_days == 0 || recovery_days > 3650 {
        return Err("recovery_days is outside 1..=3650".into());
    }
    let mut floors = Vec::new();
    let mut output = String::new();
    write!(output, "{{\"schema\":\"bastion.stress-recovery-comparison.v1\",\"source_id\":\"{}\",\"baseline_version\":\"{}\",\"successor_version\":\"{}\",\"recovery_days\":{recovery_days},\"scenarios\":[", doc.text("source_id")?, doc.text("baseline_version")?, doc.text("successor_version")?).unwrap();
    for (index, scenario) in SCENARIOS.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let mut values = Vec::new();
        for facet in FACETS {
            let value = doc.u64(&format!("scenario.{scenario}.{facet}"))?;
            if value > 10_000 {
                return Err(format!("scenario facet exceeds 10000: {scenario}.{facet}"));
            }
            values.push(value);
        }
        let floor = values
            .iter()
            .copied()
            .min()
            .ok_or_else(|| "scenario contains no support facets".to_owned())?;
        floors.push(floor);
        write!(output, "{{\"id\":\"{scenario}\",\"supplier_bps\":{},\"workforce_bps\":{},\"maintenance_bps\":{},\"spares_bps\":{},\"support_floor_bps\":{floor}}}", values[0], values[1], values[2], values[3]).unwrap();
    }
    if floors[2] < floors[1] {
        return Err("recovery support floor remains below disruption floor".into());
    }
    output.push_str("],\"limits\":{\"mission_success_probability\":\"not-inferred\",\"operational_detail\":\"absent\"},\"authority\":{\"operational_plan\":false,\"deployment_advice\":false,\"procurement_recommendation\":false,\"release\":false}}\n");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/stress-recovery.fixture");

    #[test]
    fn exposes_disruption_and_bounded_recovery() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"id\":\"baseline\""));
        assert!(output.contains("\"support_floor_bps\":6000"));
        assert!(output.contains("\"support_floor_bps\":4200"));
        assert!(output.contains("\"support_floor_bps\":7200"));
    }

    #[test]
    fn rejects_in_place_version_mutation_and_operational_fields() {
        assert!(
            run(&FIXTURE.replace(
                "successor_version=bastion-scenario-v2",
                "successor_version=bastion-scenario-v1"
            ))
            .is_err()
        );
        assert!(
            run(&format!("{FIXTURE}deployment_location=west\n"))
                .unwrap_err()
                .contains("unknown field")
        );
    }
}
