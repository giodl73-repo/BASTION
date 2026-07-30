//! Bounded safe-synthetic readiness remedy comparison.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

const SCHEMA: &str = "bastion.synthetic-readiness-remedies.v1";
const OUTPUT_SCHEMA: &str = "bastion.readiness-remedy-comparison.v1";
const MAX_INPUT_BYTES: usize = 64 * 1024;
const MAX_REMEDIES: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Observation {
    Current(u32),
    Missing,
    Stale(u32),
}

#[derive(Clone, Copy)]
struct FacetSpec {
    id: &'static str,
    threshold_bps: u32,
}

const FACETS: [FacetSpec; 9] = [
    FacetSpec {
        id: "personnel-coverage",
        threshold_bps: 8_500,
    },
    FacetSpec {
        id: "training-currency",
        threshold_bps: 8_500,
    },
    FacetSpec {
        id: "asset-availability",
        threshold_bps: 8_000,
    },
    FacetSpec {
        id: "maintenance-completion",
        threshold_bps: 8_500,
    },
    FacetSpec {
        id: "spares-fill",
        threshold_bps: 8_000,
    },
    FacetSpec {
        id: "logistics-reliability",
        threshold_bps: 8_500,
    },
    FacetSpec {
        id: "supplier-resilience",
        threshold_bps: 7_500,
    },
    FacetSpec {
        id: "interoperability",
        threshold_bps: 8_500,
    },
    FacetSpec {
        id: "safety-compliance",
        threshold_bps: 9_500,
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remedy {
    id: String,
    resource_index: Observation,
    lead_time_days: Observation,
    transition_burden_bps: Observation,
    facets: Vec<Observation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input {
    source_label: String,
    package_id: String,
    mission_label: String,
    baseline: Vec<Observation>,
    remedies: Vec<Remedy>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReadinessResult {
    posture: &'static str,
    floor_bps: Option<u32>,
    bottlenecks: Vec<&'static str>,
    concern_count: usize,
    held_facets: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RemedyResult {
    id: String,
    posture: &'static str,
    floor_bps: Option<u32>,
    floor_change_bps: Option<i64>,
    bottlenecks: Vec<&'static str>,
    concern_count: usize,
    held_facets: Vec<&'static str>,
    resource_index: Observation,
    lead_time_days: Observation,
    transition_burden_bps: Observation,
    safety_regression: bool,
    facets: Vec<Observation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assessment {
    source_label: String,
    package_id: String,
    mission_label: String,
    baseline: ReadinessResult,
    baseline_facets: Vec<Observation>,
    remedies: Vec<RemedyResult>,
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn take<'a>(map: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, String> {
    map.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing field: {key}"))
}

fn parse_observation(raw: &str, maximum: u32) -> Result<Observation, String> {
    if raw == "missing" {
        return Ok(Observation::Missing);
    }
    let (state, value) = raw
        .split_once(':')
        .ok_or_else(|| format!("invalid observation: {raw}"))?;
    let value = value
        .parse::<u32>()
        .map_err(|_| format!("invalid observation value: {raw}"))?;
    if value > maximum {
        return Err(format!("observation exceeds bound: {raw}"));
    }
    match state {
        "current" => Ok(Observation::Current(value)),
        "stale" => Ok(Observation::Stale(value)),
        _ => Err(format!("invalid observation state: {raw}")),
    }
}

/// Parses a closed, aggregate, non-operational fixture.
///
/// # Errors
///
/// Returns an error for malformed, unknown, duplicate, out-of-bound, or
/// incomplete fields.
pub fn parse(input: &str) -> Result<Input, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("input exceeds 65536 bytes".into());
    }
    let mut fields = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {} lacks '='", index + 1))?;
        if fields.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(format!("duplicate field: {key}"));
        }
    }
    if take(&fields, "schema")? != SCHEMA {
        return Err("unsupported schema".into());
    }
    let source_label = take(&fields, "source_label")?.to_owned();
    let package_id = take(&fields, "package_id")?.to_owned();
    let mission_label = take(&fields, "mission_label")?.to_owned();
    if [&source_label, &package_id, &mission_label]
        .iter()
        .any(|value| !valid_id(value))
    {
        return Err("identity fields must be bounded lowercase identifiers".into());
    }
    let remedy_ids: Vec<&str> = take(&fields, "remedy_ids")?.split(',').collect();
    if remedy_ids.len() < 3 || remedy_ids.len() > MAX_REMEDIES {
        return Err("remedy_ids must contain 3 through 8 remedies".into());
    }
    let unique: BTreeSet<&str> = remedy_ids.iter().copied().collect();
    if unique.len() != remedy_ids.len() || remedy_ids.iter().any(|id| !valid_id(id)) {
        return Err("remedy identifiers must be unique bounded lowercase identifiers".into());
    }

    let mut expected: BTreeSet<String> = [
        "schema".to_owned(),
        "source_label".to_owned(),
        "package_id".to_owned(),
        "mission_label".to_owned(),
        "remedy_ids".to_owned(),
    ]
    .into_iter()
    .collect();
    let mut baseline = Vec::with_capacity(FACETS.len());
    for facet in FACETS {
        let key = format!("baseline.{}", facet.id);
        expected.insert(key.clone());
        baseline.push(parse_observation(take(&fields, &key)?, 10_000)?);
    }
    let mut remedies = Vec::with_capacity(remedy_ids.len());
    for id in remedy_ids {
        let resource_key = format!("remedy.{id}.resource-index");
        let lead_key = format!("remedy.{id}.lead-time-days");
        let burden_key = format!("remedy.{id}.transition-burden-bps");
        expected.extend([resource_key.clone(), lead_key.clone(), burden_key.clone()]);
        let resource_index = parse_observation(take(&fields, &resource_key)?, 1_000_000)?;
        let lead_time_days = parse_observation(take(&fields, &lead_key)?, 36_500)?;
        let transition_burden_bps = parse_observation(take(&fields, &burden_key)?, 10_000)?;
        let mut facets = Vec::with_capacity(FACETS.len());
        for facet in FACETS {
            let key = format!("remedy.{id}.{}", facet.id);
            expected.insert(key.clone());
            facets.push(parse_observation(take(&fields, &key)?, 10_000)?);
        }
        remedies.push(Remedy {
            id: id.to_owned(),
            resource_index,
            lead_time_days,
            transition_burden_bps,
            facets,
        });
    }
    if let Some(unknown) = fields.keys().find(|key| !expected.contains(*key)) {
        return Err(format!("unknown field rejected: {unknown}"));
    }
    Ok(Input {
        source_label,
        package_id,
        mission_label,
        baseline,
        remedies,
    })
}

fn evaluate_readiness(facets: &[Observation]) -> ReadinessResult {
    let held_facets: Vec<&'static str> = facets
        .iter()
        .zip(FACETS)
        .filter_map(|(observation, spec)| {
            (!matches!(observation, Observation::Current(_))).then_some(spec.id)
        })
        .collect();
    if !held_facets.is_empty() {
        return ReadinessResult {
            posture: "held",
            floor_bps: None,
            bottlenecks: Vec::new(),
            concern_count: 0,
            held_facets,
        };
    }
    let values: Vec<u32> = facets
        .iter()
        .map(|observation| match observation {
            Observation::Current(value) => *value,
            _ => unreachable!(),
        })
        .collect();
    let floor = values
        .iter()
        .copied()
        .min()
        .expect("fixed non-empty facets");
    let bottlenecks = values
        .iter()
        .zip(FACETS)
        .filter_map(|(value, spec)| (*value == floor).then_some(spec.id))
        .collect();
    let concern_count = values
        .iter()
        .zip(FACETS)
        .filter(|(value, spec)| **value < spec.threshold_bps)
        .count();
    ReadinessResult {
        posture: if concern_count == 0 {
            "assessable"
        } else {
            "needs-attention"
        },
        floor_bps: Some(floor),
        bottlenecks,
        concern_count,
        held_facets,
    }
}

fn current_value(observation: Observation) -> Option<u32> {
    match observation {
        Observation::Current(value) => Some(value),
        _ => None,
    }
}

/// Compares all remedies without choosing or recommending one.
#[must_use]
pub fn assess(input: &Input) -> Assessment {
    let baseline = evaluate_readiness(&input.baseline);
    let baseline_safety = current_value(input.baseline[8]);
    let remedies = input
        .remedies
        .iter()
        .map(|remedy| {
            let readiness = evaluate_readiness(&remedy.facets);
            let metadata_held = [
                remedy.resource_index,
                remedy.lead_time_days,
                remedy.transition_burden_bps,
            ]
            .iter()
            .any(|value| !matches!(value, Observation::Current(_)));
            let safety_regression = match (baseline_safety, current_value(remedy.facets[8])) {
                (Some(baseline_value), Some(remedy_value)) => remedy_value < baseline_value,
                _ => false,
            };
            let held = readiness.posture == "held" || metadata_held || safety_regression;
            let floor_bps = (!held).then_some(readiness.floor_bps).flatten();
            let floor_change_bps = match (baseline.floor_bps, floor_bps) {
                (Some(before), Some(after)) => Some(i64::from(after) - i64::from(before)),
                _ => None,
            };
            RemedyResult {
                id: remedy.id.clone(),
                posture: if held { "held" } else { readiness.posture },
                floor_bps,
                floor_change_bps,
                bottlenecks: if held {
                    Vec::new()
                } else {
                    readiness.bottlenecks
                },
                concern_count: readiness.concern_count,
                held_facets: readiness.held_facets,
                resource_index: remedy.resource_index,
                lead_time_days: remedy.lead_time_days,
                transition_burden_bps: remedy.transition_burden_bps,
                safety_regression,
                facets: remedy.facets.clone(),
            }
        })
        .collect();
    Assessment {
        source_label: input.source_label.clone(),
        package_id: input.package_id.clone(),
        mission_label: input.mission_label.clone(),
        baseline,
        baseline_facets: input.baseline.clone(),
        remedies,
    }
}

fn write_observation(output: &mut String, observation: Observation) {
    match observation {
        Observation::Current(value) => {
            write!(output, "{{\"state\":\"current\",\"value\":{value}}}").unwrap();
        }
        Observation::Stale(value) => {
            write!(output, "{{\"state\":\"stale\",\"value\":{value}}}").unwrap();
        }
        Observation::Missing => output.push_str("{\"state\":\"missing\",\"value\":null}"),
    }
}

fn write_string_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        write!(output, "\"{value}\"").unwrap();
    }
    output.push(']');
}

fn write_facets(output: &mut String, facets: &[Observation]) {
    output.push('[');
    for (index, (observation, spec)) in facets.iter().zip(FACETS).enumerate() {
        if index > 0 {
            output.push(',');
        }
        write!(output, "{{\"id\":\"{}\",\"observation\":", spec.id).unwrap();
        write_observation(output, *observation);
        write!(output, ",\"threshold_bps\":{}}}", spec.threshold_bps).unwrap();
    }
    output.push(']');
}

/// Emits canonical JSON without a selected or recommended remedy field.
#[must_use]
pub fn canonical_json(assessment: &Assessment) -> String {
    let mut output = String::new();
    write!(output, "{{\"schema\":\"{OUTPUT_SCHEMA}\",\"source_label\":\"{}\",\"package_id\":\"{}\",\"mission_label\":\"{}\",\"baseline\":{{\"posture\":\"{}\",\"readiness_floor_bps\":", assessment.source_label, assessment.package_id, assessment.mission_label, assessment.baseline.posture).unwrap();
    match assessment.baseline.floor_bps {
        Some(value) => write!(output, "{value}").unwrap(),
        None => output.push_str("null"),
    }
    output.push_str(",\"bottlenecks\":");
    write_string_array(&mut output, &assessment.baseline.bottlenecks);
    write!(
        output,
        ",\"concern_count\":{},\"held_facets\":",
        assessment.baseline.concern_count
    )
    .unwrap();
    write_string_array(&mut output, &assessment.baseline.held_facets);
    output.push_str(",\"facets\":");
    write_facets(&mut output, &assessment.baseline_facets);
    output.push_str("},\"remedies\":[");
    for (index, remedy) in assessment.remedies.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        write!(
            output,
            "{{\"id\":\"{}\",\"posture\":\"{}\",\"readiness_floor_bps\":",
            remedy.id, remedy.posture
        )
        .unwrap();
        match remedy.floor_bps {
            Some(value) => write!(output, "{value}").unwrap(),
            None => output.push_str("null"),
        }
        output.push_str(",\"floor_change_bps\":");
        match remedy.floor_change_bps {
            Some(value) => write!(output, "{value}").unwrap(),
            None => output.push_str("null"),
        }
        output.push_str(",\"bottlenecks\":");
        write_string_array(&mut output, &remedy.bottlenecks);
        write!(
            output,
            ",\"concern_count\":{},\"held_facets\":",
            remedy.concern_count
        )
        .unwrap();
        write_string_array(&mut output, &remedy.held_facets);
        output.push_str(",\"resource_index\":");
        write_observation(&mut output, remedy.resource_index);
        output.push_str(",\"lead_time_days\":");
        write_observation(&mut output, remedy.lead_time_days);
        output.push_str(",\"transition_burden_bps\":");
        write_observation(&mut output, remedy.transition_burden_bps);
        write!(
            output,
            ",\"safety_regression\":{},\"facets\":",
            remedy.safety_regression
        )
        .unwrap();
        write_facets(&mut output, &remedy.facets);
        output.push('}');
    }
    output.push_str("],\"comparison_rule\":\"no-remedy-selected\",\"authority\":{\"operational_plan\":false,\"deployment_advice\":false,\"targeting\":false,\"procurement_recommendation\":false,\"budget_allocation\":false,\"monetized_savings\":false,\"taxlane_action\":false,\"release\":false}}\n");
    output
}

/// Parses, assesses, and serializes one fixture.
///
/// # Errors
///
/// Returns any parsing error without emitting a partial comparison.
pub fn run(input: &str) -> Result<String, String> {
    let parsed = parse(input)?;
    Ok(canonical_json(&assess(&parsed)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/readiness-remedies.fixture");

    #[test]
    fn inventory_can_leave_the_actual_bottleneck_unchanged() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"id\":\"inventory-expansion\",\"posture\":\"needs-attention\",\"readiness_floor_bps\":6000,\"floor_change_bps\":0,\"bottlenecks\":[\"supplier-resilience\"]"));
    }

    #[test]
    fn compares_three_remedies_without_selecting_one() {
        let output = run(FIXTURE).unwrap();
        assert!(output.contains("\"id\":\"maintenance-spares\",\"posture\":\"needs-attention\",\"readiness_floor_bps\":6500,\"floor_change_bps\":500"));
        assert!(output.contains("\"id\":\"supplier-logistics\",\"posture\":\"assessable\",\"readiness_floor_bps\":8000,\"floor_change_bps\":2000"));
        assert!(output.contains("\"comparison_rule\":\"no-remedy-selected\""));
        assert!(!output.contains("recommended_remedy"));
    }

    #[test]
    fn missing_metadata_and_safety_regression_hold_a_remedy() {
        let missing = FIXTURE.replacen(
            "remedy.inventory-expansion.resource-index=current:130",
            "remedy.inventory-expansion.resource-index=missing",
            1,
        );
        assert!(run(&missing).unwrap().contains(
            "\"id\":\"inventory-expansion\",\"posture\":\"held\",\"readiness_floor_bps\":null"
        ));
        let unsafe_case = FIXTURE.replacen(
            "remedy.maintenance-spares.safety-compliance=current:9700",
            "remedy.maintenance-spares.safety-compliance=current:9600",
            1,
        );
        assert!(
            run(&unsafe_case)
                .unwrap()
                .contains("\"id\":\"maintenance-spares\",\"posture\":\"held\"")
        );
    }

    #[test]
    fn parser_rejects_operational_or_procurement_shaped_fields() {
        for field in [
            "deployment_location=west",
            "target_coordinates=none",
            "selected_vendor=cedar",
            "service_member_name=Alex",
        ] {
            let input = format!("{FIXTURE}{field}\n");
            assert!(
                parse(&input)
                    .unwrap_err()
                    .contains("unknown field rejected")
            );
        }
    }

    #[test]
    fn output_is_deterministic_and_declares_no_authority() {
        let first = run(FIXTURE).unwrap();
        assert_eq!(first, run(FIXTURE).unwrap());
        assert!(first.contains("\"procurement_recommendation\":false"));
        assert!(first.contains("\"monetized_savings\":false"));
        assert!(first.contains("\"release\":false"));
    }
}
