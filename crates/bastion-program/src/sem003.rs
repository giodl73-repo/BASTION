//! SEM-003: admitted public unclassified aggregate mini-corpus.

use std::fmt::Write as _;

use crate::Document;

const KEYS: [&str; 14] = [
    "schema",
    "source_id",
    "source_url",
    "publication_date",
    "data_period",
    "evidence_state",
    "classification",
    "programs_assessed",
    "recurring_mdaps",
    "portfolio_cost_increase_millions",
    "dominant_program_increase_millions",
    "mta_programs",
    "mta_investment_millions",
    "average_initial_capability_months",
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
    let recurring = doc.u64("recurring_mdaps")?;
    let increase = doc.u64("portfolio_cost_increase_millions")?;
    let dominant = doc.u64("dominant_program_increase_millions")?;
    let mta_programs = doc.u64("mta_programs")?;
    if recurring > programs || mta_programs > programs || dominant > increase {
        return Err("aggregate program or cost subsets exceed their totals".into());
    }
    let dominant_share_bps = dominant
        .checked_mul(10_000)
        .ok_or_else(|| "cost share overflow".to_owned())?
        / increase;
    let mut output = String::new();
    writeln!(
        output,
        "{{\"schema\":\"bastion.admitted-public-unclassified-aggregate.v1\",\"source_id\":\"{}\",\"source_url\":\"{}\",\"publication_date\":\"{}\",\"data_period\":\"{}\",\"evidence_state\":\"current\",\"classification\":\"public-unclassified\",\"aggregates\":{{\"programs_assessed\":{programs},\"recurring_mdaps\":{recurring},\"portfolio_cost_increase_millions\":{increase},\"dominant_program_increase_millions\":{dominant},\"dominant_share_bps\":{dominant_share_bps},\"mta_programs\":{mta_programs},\"mta_investment_millions\":{},\"average_initial_capability_months\":{}}},\"limits\":{{\"readiness_inference\":\"not-permitted\",\"operational_detail\":\"absent\"}},\"authority\":{{\"procurement_recommendation\":false,\"budget_allocation\":false,\"taxlane_action\":false,\"release\":false}}}}",
        doc.text("source_id")?,
        source_url,
        doc.text("publication_date")?,
        doc.text("data_period")?,
        doc.u64("mta_investment_millions")?,
        doc.u64("average_initial_capability_months")?
    )
    .unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../../fixtures/public/gao-25-107569.fixture");

    #[test]
    fn admits_official_aggregate_and_reconciles_cost_concentration() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"programs_assessed\":106"));
        assert!(output.contains("\"portfolio_cost_increase_millions\":49300"));
        assert!(output.contains("\"dominant_share_bps\":7302"));
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
