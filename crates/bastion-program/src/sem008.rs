//! SEM-008: cross-stakeholder defense burden and incidence map.

use std::fmt::Write as _;

use crate::Document;

const GROUPS: [&str; 7] = [
    "personnel",
    "families",
    "maintainers",
    "suppliers",
    "communities",
    "partners",
    "taxpayers",
];
const FIELDS: [&str; 4] = [
    "population-index",
    "workload-change-bps",
    "transition-burden-bps",
    "cost-incidence-millions",
];

fn keys() -> Vec<String> {
    let mut keys = vec![
        "schema".to_owned(),
        "alternative_id".to_owned(),
        "unit".to_owned(),
    ];
    for group in GROUPS {
        for field in FIELDS {
            keys.push(format!("group.{group}.{field}"));
        }
    }
    keys
}

/// Maps separated stakeholder burden without ranking human worth.
///
/// # Errors
///
/// Rejects unknown fields, invalid dimensions, zero population indices,
/// out-of-bound burdens, or non-reconciling cost incidence.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    let keys = keys();
    let borrowed: Vec<&str> = keys.iter().map(String::as_str).collect();
    doc.exact(&borrowed)?;
    if doc.text("schema")? != "bastion.stakeholder-incidence.v1"
        || doc.text("unit")? != "millions-usd"
    {
        return Err("unsupported schema or unit".into());
    }
    let mut incidence_total = 0_i128;
    let mut output = String::new();
    write!(
        output,
        "{{\"schema\":\"bastion.distribution-map.v1\",\"alternative_id\":\"{}\",\"groups\":[",
        doc.text("alternative_id")?
    )
    .unwrap();
    for (index, group) in GROUPS.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let population = doc.u64(&format!("group.{group}.population-index"))?;
        let workload = doc.i64(&format!("group.{group}.workload-change-bps"))?;
        let burden = doc.u64(&format!("group.{group}.transition-burden-bps"))?;
        let incidence = doc.i64(&format!("group.{group}.cost-incidence-millions"))?;
        if population == 0 || workload.unsigned_abs() > 10_000 || burden > 10_000 {
            return Err(format!("invalid distribution value for {group}"));
        }
        incidence_total += i128::from(incidence);
        write!(output, "{{\"id\":\"{group}\",\"population_index\":{population},\"workload_change_bps\":{workload},\"transition_burden_bps\":{burden},\"cost_incidence_millions\":{incidence}}}").unwrap();
    }
    if incidence_total != 0 {
        return Err("cost incidence does not reconcile to zero".into());
    }
    output.push_str("],\"cost_incidence_residual_millions\":0,\"aggregation_rule\":\"no-composite-priority\",\"limits\":{\"population_index\":\"synthetic-not-headcount\"},\"authority\":{\"human_worth_ranking\":false,\"procurement_recommendation\":false,\"operational_plan\":false,\"release\":false}}\n");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/synthetic/defense-stakeholder-incidence.fixture");

    #[test]
    fn exposes_every_group_and_reconciles_shifted_cost() {
        let output = run(FIXTURE).unwrap();
        for group in GROUPS {
            assert!(output.contains(&format!("\"id\":\"{group}\"")));
        }
        assert!(output.contains("\"cost_incidence_residual_millions\":0"));
        assert!(output.contains("\"aggregation_rule\":\"no-composite-priority\""));
    }

    #[test]
    fn rejects_hidden_cost_shift_and_operational_input() {
        assert!(
            run(&FIXTURE.replace(
                "group.taxpayers.cost-incidence-millions=6000",
                "group.taxpayers.cost-incidence-millions=6001"
            ))
            .is_err()
        );
        assert!(
            run(&format!("{FIXTURE}unit_location=west\n"))
                .unwrap_err()
                .contains("unknown field")
        );
    }
}
