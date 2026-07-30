//! SEM-005: aggregate purchase-to-usable-readiness realization.

use std::fmt::Write as _;

use crate::Document;

const STAGES: [&str; 8] = [
    "funded",
    "contracted",
    "delivered",
    "staffed",
    "trained",
    "maintained",
    "supplied",
    "usable",
];

fn keys() -> Vec<String> {
    let mut keys = vec![
        "schema".to_owned(),
        "scenario_version".to_owned(),
        "package_id".to_owned(),
    ];
    for stage in STAGES {
        keys.push(format!("capacity.{stage}"));
    }
    keys
}

/// Traces a fictional aggregate capacity index to usable readiness.
///
/// # Errors
///
/// Rejects unknown fields, zero baselines, or non-conserving stages.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    let keys = keys();
    let borrowed: Vec<&str> = keys.iter().map(String::as_str).collect();
    doc.exact(&borrowed)?;
    if doc.text("schema")? != "bastion.aggregate-capacity-realization.v1" {
        return Err("unsupported schema".into());
    }
    let mut values = Vec::new();
    for stage in STAGES {
        values.push(doc.u64(&format!("capacity.{stage}"))?);
    }
    if values[0] == 0 || values.windows(2).any(|pair| pair[1] > pair[0]) {
        return Err("non-conserving capacity realization funnel".into());
    }
    let realized_bps = values[7]
        .checked_mul(10_000)
        .ok_or_else(|| "realization overflow".to_owned())?
        / values[0];
    let mut largest_loss = 0_u64;
    let mut limiting_handoff = "none";
    for (index, pair) in values.windows(2).enumerate() {
        let loss = pair[0] - pair[1];
        if loss > largest_loss {
            largest_loss = loss;
            limiting_handoff = STAGES[index + 1];
        }
    }
    let mut output = String::new();
    write!(output, "{{\"schema\":\"bastion.capacity-realization.v1\",\"scenario_version\":\"{}\",\"package_id\":\"{}\",\"stages\":[", doc.text("scenario_version")?, doc.text("package_id")?).unwrap();
    for (index, stage) in STAGES.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        write!(
            output,
            "{{\"id\":\"{stage}\",\"capacity_index\":{}}}",
            values[index]
        )
        .unwrap();
    }
    writeln!(output, "],\"realization_bps\":{realized_bps},\"largest_handoff_loss\":{largest_loss},\"limiting_handoff\":\"{limiting_handoff}\",\"limits\":{{\"capacity_index\":\"synthetic-not-units\",\"unused_is_savings\":false,\"mission_success_probability\":\"not-inferred\"}},\"authority\":{{\"operational_plan\":false,\"procurement_action\":false,\"deployment_advice\":false,\"release\":false}}}}").unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/capacity-realization.fixture");

    #[test]
    fn distinguishes_purchase_from_usable_readiness() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"realization_bps\":6000"));
        assert!(output.contains("\"largest_handoff_loss\":1200"));
        assert!(output.contains("\"limiting_handoff\":\"maintained\""));
        assert!(output.contains("\"unused_is_savings\":false"));
    }

    #[test]
    fn rejects_nonconserving_or_operational_input() {
        assert!(
            run(&FIXTURE.replace("capacity.delivered=9200", "capacity.delivered=11000")).is_err()
        );
        assert!(
            run(&format!("{FIXTURE}target_coordinates=none\n"))
                .unwrap_err()
                .contains("unknown field")
        );
    }
}
