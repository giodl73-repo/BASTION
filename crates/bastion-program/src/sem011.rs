//! SEM-011: definition-normalized NATO expenditure comparison.

use std::fmt::Write as _;

use crate::Document;

const KEYS: [&str; 20] = [
    "schema",
    "source_id",
    "source_url",
    "publication_date",
    "data_period",
    "evidence_state",
    "estimate_state",
    "definition",
    "unit",
    "peer_use",
    "gdp.us-bps",
    "gdp.france-bps",
    "gdp.united-kingdom-bps",
    "gdp.canada-bps",
    "gdp.nato-europe-canada-bps",
    "gdp.nato-total-bps",
    "us.equipment-bps",
    "us.personnel-bps",
    "us.infrastructure-bps",
    "us.other-bps",
];

/// Produces a normalized NATO diagnostic comparison without a spending target.
///
/// # Errors
///
/// Rejects stale/non-NATO evidence, mismatched definitions, unknown fields,
/// out-of-bound shares, or an unreconciled US category distribution.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    doc.exact(&KEYS)?;
    if doc.text("schema")? != "bastion.nato-peer-comparison.v1"
        || doc.text("evidence_state")? != "current"
        || doc.text("estimate_state")? != "2025-estimate"
        || !doc.text("source_url")?.starts_with("https://www.nato.int/")
        || doc.text("definition")? != "nato-defence-expenditure"
        || doc.text("unit")? != "basis-points-of-gdp"
        || doc.text("peer_use")? != "diagnostic-not-target"
    {
        return Err("incompatible peer evidence or definition".into());
    }
    let us = doc.u64("gdp.us-bps")?;
    let peers = [
        ("france", doc.u64("gdp.france-bps")?),
        ("united-kingdom", doc.u64("gdp.united-kingdom-bps")?),
        ("canada", doc.u64("gdp.canada-bps")?),
        ("nato-europe-canada", doc.u64("gdp.nato-europe-canada-bps")?),
        ("nato-total", doc.u64("gdp.nato-total-bps")?),
    ];
    if us > 10_000 || peers.iter().any(|(_, value)| *value > 10_000) {
        return Err("GDP share exceeds 10000 basis points".into());
    }
    let categories = [
        doc.u64("us.equipment-bps")?,
        doc.u64("us.personnel-bps")?,
        doc.u64("us.infrastructure-bps")?,
        doc.u64("us.other-bps")?,
    ];
    let category_total: u64 = categories.iter().sum();
    if category_total.abs_diff(10_000) > 2 {
        return Err("US category distribution does not reconcile within rounding tolerance".into());
    }
    let mut output = String::new();
    write!(output, "{{\"schema\":\"bastion.normalized-peer-comparison.v1\",\"source_id\":\"{}\",\"source_url\":\"{}\",\"data_period\":\"{}\",\"estimate_state\":\"2025-estimate\",\"definition\":\"nato-defence-expenditure\",\"us_gdp_bps\":{us},\"peers\":[", doc.text("source_id")?, doc.text("source_url")?, doc.text("data_period")?).unwrap();
    for (index, (id, value)) in peers.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let gap = i128::from(us) - i128::from(*value);
        write!(
            output,
            "{{\"id\":\"{id}\",\"gdp_bps\":{value},\"us_gap_bps\":{gap}}}"
        )
        .unwrap();
    }
    writeln!(output, "],\"us_category_distribution\":{{\"equipment_bps\":{},\"personnel_bps\":{},\"infrastructure_bps\":{},\"other_bps\":{},\"rounding_residual_bps\":{}}},\"peer_use\":\"diagnostic-not-target\",\"limits\":{{\"readiness_comparison\":\"not-supported-by-spending-share\"}},\"authority\":{{\"spending_target\":false,\"procurement_recommendation\":false,\"operational_plan\":false,\"release\":false}}}}", categories[0], categories[1], categories[2], categories[3], i128::from(category_total) - 10_000).unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/public/nato-defence-expenditure-2025.fixture");

    #[test]
    fn compares_one_definition_and_preserves_estimate_and_category_limits() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"us_gdp_bps\":322"));
        assert!(output.contains("\"id\":\"nato-europe-canada\",\"gdp_bps\":227,\"us_gap_bps\":95"));
        assert!(output.contains("\"rounding_residual_bps\":-1"));
        assert!(output.contains("\"readiness_comparison\":\"not-supported-by-spending-share\""));
    }

    #[test]
    fn rejects_peer_target_conversion_and_wrong_domain() {
        assert!(
            run(&FIXTURE.replace("peer_use=diagnostic-not-target", "peer_use=target")).is_err()
        );
        assert!(run(&FIXTURE.replace("https://www.nato.int/", "https://example.com/")).is_err());
    }
}
