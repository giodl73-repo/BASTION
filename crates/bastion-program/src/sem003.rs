//! SEM-003: admitted public unclassified aggregate mini-corpus.

use std::fmt::Write as _;

use crate::Document;

const KEYS: [&str; 16] = [
    "schema",
    "source_id",
    "source_url",
    "publication_date",
    "data_period",
    "evidence_state",
    "classification",
    "programs_assessed",
    "mta_programs",
    "mta_investment_millions",
    "average_capability_months_lower_bound",
    "mta_entry_cohort",
    "mta_immature_entry_count",
    "current_mta_reviewed",
    "current_mta_immature_count",
    "total_program_investment_millions_lower_bound",
];

/// Admits and summarizes one official public aggregate source record.
///
/// # Errors
///
/// Rejects stale, non-public, non-official, incomplete, unknown, or
/// arithmetically inconsistent records.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    doc.exact(&KEYS)?;
    if doc.text("schema")? != "bastion.public-unclassified-mini-corpus.v1" {
        return Err("unsupported schema".into());
    }
    if doc.text("evidence_state")? != "current"
        || doc.text("classification")? != "public-unclassified"
    {
        return Err("evidence is not current public-unclassified material".into());
    }
    let source_url = doc.text("source_url")?;
    if !source_url.starts_with("https://www.gao.gov/") {
        return Err("source_url is outside the admitted official domain".into());
    }
    let programs = doc.u64("programs_assessed")?;
    let mta_programs = doc.u64("mta_programs")?;
    let entry_cohort = doc.u64("mta_entry_cohort")?;
    let immature_entry = doc.u64("mta_immature_entry_count")?;
    let current_reviewed = doc.u64("current_mta_reviewed")?;
    let current_immature = doc.u64("current_mta_immature_count")?;
    if mta_programs > programs
        || immature_entry > entry_cohort
        || current_immature > current_reviewed
    {
        return Err("aggregate program subsets exceed their totals".into());
    }
    let mut output = String::new();
    writeln!(
        output,
        "{{\"schema\":\"bastion.admitted-public-unclassified-aggregate.v1\",\"source_id\":\"{}\",\"source_url\":\"{}\",\"publication_date\":\"{}\",\"data_period\":\"{}\",\"evidence_state\":\"current\",\"classification\":\"public-unclassified\",\"aggregates\":{{\"programs_assessed\":{programs},\"mta_programs\":{mta_programs},\"mta_investment_millions\":{},\"average_capability_months_lower_bound\":{},\"mta_entry_cohort\":{entry_cohort},\"mta_immature_entry_count\":{immature_entry},\"current_mta_reviewed\":{current_reviewed},\"current_mta_immature_count\":{current_immature},\"total_program_investment_millions_lower_bound\":{}}},\"limits\":{{\"readiness_inference\":\"not-permitted\",\"operational_detail\":\"absent\",\"lower_bounds_are_exact_totals\":false}},\"authority\":{{\"procurement_recommendation\":false,\"budget_allocation\":false,\"taxlane_action\":false,\"release\":false}}}}",
        doc.text("source_id")?,
        source_url,
        doc.text("publication_date")?,
        doc.text("data_period")?,
        doc.u64("mta_investment_millions")?,
        doc.u64("average_capability_months_lower_bound")?,
        doc.u64("total_program_investment_millions_lower_bound")?
    )
    .unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../../fixtures/public/gao-26-108457.fixture");

    #[test]
    fn admits_official_aggregate_and_reconciles_program_subsets() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"programs_assessed\":104"));
        assert!(output.contains("\"mta_immature_entry_count\":18"));
        assert!(output.contains("\"current_mta_immature_count\":7"));
        assert!(output.contains("\"total_program_investment_millions_lower_bound\":2400000"));
    }

    #[test]
    fn rejects_nonofficial_stale_or_operational_shaped_content() {
        assert!(run(&FIXTURE.replace("https://www.gao.gov/", "https://example.com/")).is_err());
        assert!(run(&FIXTURE.replace("evidence_state=current", "evidence_state=stale")).is_err());
        assert!(
            run(&format!("{FIXTURE}deployment_location=west\n"))
                .unwrap_err()
                .contains("unknown field")
        );
    }

    #[test]
    fn output_is_deterministic_and_non_authoritative() {
        let first = run(FIXTURE).unwrap();
        assert_eq!(first, run(FIXTURE).unwrap());
        assert!(first.contains("\"readiness_inference\":\"not-permitted\""));
        assert!(first.contains("\"procurement_recommendation\":false"));
    }
}
