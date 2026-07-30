//! SEM-006: acquisition and lifecycle resource accounting.

use std::fmt::Write as _;

use crate::Document;

const CATEGORIES: [&str; 8] = [
    "acquisition",
    "personnel",
    "operations",
    "maintenance",
    "spares",
    "infrastructure",
    "transition",
    "unallocated",
];

fn keys() -> Vec<String> {
    let mut keys = vec![
        "schema".to_owned(),
        "scenario_version".to_owned(),
        "unit".to_owned(),
        "price_basis".to_owned(),
        "horizon_years".to_owned(),
        "authorized".to_owned(),
    ];
    for category in CATEGORIES {
        keys.push(format!("category.{category}"));
    }
    keys
}

/// Reconciles a fictional lifecycle resource envelope.
///
/// # Errors
///
/// Rejects unknown fields, incompatible units, overflow, or a non-zero
/// allocation residual.
pub fn run(input: &str) -> Result<String, String> {
    let doc = Document::parse(input)?;
    let keys = keys();
    let borrowed: Vec<&str> = keys.iter().map(String::as_str).collect();
    doc.exact(&borrowed)?;
    if doc.text("schema")? != "bastion.lifecycle-accounting.v1" {
        return Err("unsupported schema".into());
    }
    if doc.text("unit")? != "millions-usd" || doc.text("price_basis")? != "constant-illustrative" {
        return Err("incompatible accounting dimension".into());
    }
    let horizon = doc.u64("horizon_years")?;
    if !(1..=50).contains(&horizon) {
        return Err("horizon_years is outside 1..=50".into());
    }
    let authorized = doc.u64("authorized")?;
    let mut allocated = 0_u64;
    let mut output = String::new();
    write!(output, "{{\"schema\":\"bastion.reconciled-lifecycle-accounting.v1\",\"scenario_version\":\"{}\",\"unit\":\"millions-usd\",\"price_basis\":\"constant-illustrative\",\"horizon_years\":{horizon},\"authorized\":{authorized},\"categories\":[", doc.text("scenario_version")?).unwrap();
    for (index, category) in CATEGORIES.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let value = doc.u64(&format!("category.{category}"))?;
        allocated = allocated
            .checked_add(value)
            .ok_or_else(|| "allocation overflow".to_owned())?;
        write!(output, "{{\"id\":\"{category}\",\"amount\":{value}}}").unwrap();
    }
    if allocated != authorized {
        return Err("non-zero lifecycle allocation residual".into());
    }
    writeln!(output, "],\"allocated\":{allocated},\"residual\":0,\"limits\":{{\"values\":\"safe-synthetic-accounting\",\"savings\":\"not-calculated\"}},\"authority\":{{\"procurement_action\":false,\"budget_allocation\":false,\"taxlane_action\":false,\"release\":false}}}}").unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/lifecycle-accounting.fixture");

    #[test]
    fn reconciles_every_lifecycle_category() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"authorized\":100000"));
        assert!(output.contains("\"id\":\"maintenance\",\"amount\":13000"));
        assert!(output.contains("\"id\":\"transition\",\"amount\":4000"));
        assert!(output.contains("\"residual\":0"));
    }

    #[test]
    fn rejects_residuals_and_false_dimension() {
        assert!(
            run(&FIXTURE.replace("category.unallocated=5000", "category.unallocated=5001"))
                .is_err()
        );
        assert!(run(&FIXTURE.replace("unit=millions-usd", "unit=readiness-bps")).is_err());
    }
}
