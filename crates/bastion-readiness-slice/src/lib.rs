//! One safe-synthetic BASTION semantic slice: readiness-package assessment.
//!
//! The slice is public/synthetic, aggregate, unclassified, and non-operational.
//! It cannot plan deployments, select forces, target, procure, or allocate funds.

use std::collections::BTreeMap;
use std::fmt::{Display, Write as _};

pub const INPUT_SCHEMA: &str = "bastion.synthetic-readiness-package.v1";
pub const OUTPUT_SCHEMA: &str = "bastion.readiness-assessment.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceState {
    Current,
    Missing,
    Stale,
    NotApplicable,
}

impl EvidenceState {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Missing => "missing",
            Self::Stale => "stale",
            Self::NotApplicable => "not-applicable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Observation {
    pub state: EvidenceState,
    pub value_bps: Option<u16>,
}

#[derive(Clone, Copy, Debug)]
struct FacetSpec {
    id: &'static str,
    threshold_bps: u16,
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
pub struct ReadinessPackage {
    pub package_id: String,
    pub mission_label: String,
    pub mission_days: u16,
    pub source_label: String,
    baseline: BTreeMap<&'static str, Observation>,
    alternative: BTreeMap<&'static str, Observation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FacetPosture {
    Pass,
    Concern,
    Held,
    NotApplicable,
}

impl FacetPosture {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Concern => "concern",
            Self::Held => "held",
            Self::NotApplicable => "not-applicable",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FacetAssessment {
    pub id: &'static str,
    pub evidence_state: EvidenceState,
    pub observed_bps: Option<u16>,
    pub threshold_bps: u16,
    pub posture: FacetPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackagePosture {
    Assessable,
    NeedsAttention,
    Held,
}

impl PackagePosture {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Assessable => "assessable",
            Self::NeedsAttention => "needs-attention",
            Self::Held => "held",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageAssessment {
    pub posture: PackagePosture,
    pub readiness_floor_bps: Option<u16>,
    pub bottleneck_facets: Vec<&'static str>,
    pub pass_count: usize,
    pub concern_count: usize,
    pub held_count: usize,
    pub facets: Vec<FacetAssessment>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadinessComparison {
    pub improved_facets: Vec<&'static str>,
    pub worsened_facets: Vec<&'static str>,
    pub held_resolved_facets: Vec<&'static str>,
    pub baseline_floor_bps: Option<u16>,
    pub alternative_floor_bps: Option<u16>,
    pub floor_change_bps: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadinessAssessment {
    pub package_id: String,
    pub mission_label: String,
    pub mission_days: u16,
    pub source_label: String,
    pub baseline: PackageAssessment,
    pub alternative: PackageAssessment,
    pub comparison: ReadinessComparison,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SliceError {
    DuplicateKey(String),
    InvalidLine(usize),
    MissingKey(String),
    InvalidValue { key: String, value: String },
    UnexpectedKey(String),
    WrongSchema(String),
    UnsafeIdentifier(String),
    MissionDaysOutOfRange,
}

impl Display for SliceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateKey(key) => write!(formatter, "duplicate key: {key}"),
            Self::InvalidLine(line) => write!(formatter, "invalid line: {line}"),
            Self::MissingKey(key) => write!(formatter, "missing key: {key}"),
            Self::InvalidValue { key, value } => {
                write!(formatter, "invalid value for {key}: {value}")
            }
            Self::UnexpectedKey(key) => write!(formatter, "unexpected key: {key}"),
            Self::WrongSchema(schema) => write!(formatter, "unsupported schema: {schema}"),
            Self::UnsafeIdentifier(value) => write!(formatter, "unsafe identifier: {value}"),
            Self::MissionDaysOutOfRange => {
                formatter.write_str("mission_days must be between 1 and 365")
            }
        }
    }
}

impl std::error::Error for SliceError {}

fn safe_text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
}

fn take(map: &mut BTreeMap<String, String>, key: &str) -> Result<String, SliceError> {
    map.remove(key)
        .ok_or_else(|| SliceError::MissingKey(key.to_owned()))
}

fn parse_observation(key: &str, value: &str) -> Result<Observation, SliceError> {
    let invalid = || SliceError::InvalidValue {
        key: key.to_owned(),
        value: value.to_owned(),
    };
    if value == "missing" {
        return Ok(Observation {
            state: EvidenceState::Missing,
            value_bps: None,
        });
    }
    if value == "not-applicable" {
        return Ok(Observation {
            state: EvidenceState::NotApplicable,
            value_bps: None,
        });
    }
    let (state, raw) = value.split_once(':').ok_or_else(invalid)?;
    let state = match state {
        "current" => EvidenceState::Current,
        "stale" => EvidenceState::Stale,
        _ => return Err(invalid()),
    };
    let parsed = raw.parse::<u16>().map_err(|_| invalid())?;
    if parsed > 10_000 {
        return Err(invalid());
    }
    Ok(Observation {
        state,
        value_bps: Some(parsed),
    })
}

/// Parses the closed safe-synthetic readiness-package fixture.
///
/// # Errors
///
/// Returns [`SliceError`] when the schema, key set, identifier, mission
/// horizon, or evidence representation is invalid.
pub fn parse_package(input: &str) -> Result<ReadinessPackage, SliceError> {
    let mut values = BTreeMap::new();
    for (index, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or(SliceError::InvalidLine(index + 1))?;
        let key = key.trim().to_owned();
        let value = value.trim().to_owned();
        if values.insert(key.clone(), value).is_some() {
            return Err(SliceError::DuplicateKey(key));
        }
    }
    let schema = take(&mut values, "schema")?;
    if schema != INPUT_SCHEMA {
        return Err(SliceError::WrongSchema(schema));
    }
    let package_id = take(&mut values, "package_id")?;
    let mission_label = take(&mut values, "mission_label")?;
    let source_label = take(&mut values, "source_label")?;
    if !safe_text(&package_id) || !safe_text(&mission_label) || !safe_text(&source_label) {
        return Err(SliceError::UnsafeIdentifier(format!(
            "{package_id}/{mission_label}/{source_label}"
        )));
    }
    let days_text = take(&mut values, "mission_days")?;
    let mission_days = days_text
        .parse::<u16>()
        .map_err(|_| SliceError::InvalidValue {
            key: "mission_days".to_owned(),
            value: days_text.clone(),
        })?;
    if !(1..=365).contains(&mission_days) {
        return Err(SliceError::MissionDaysOutOfRange);
    }
    let mut baseline = BTreeMap::new();
    let mut alternative = BTreeMap::new();
    for spec in FACETS {
        for (prefix, destination) in [
            ("baseline", &mut baseline),
            ("alternative", &mut alternative),
        ] {
            let key = format!("{prefix}.{}_bps", spec.id.replace('-', "_"));
            let raw = take(&mut values, &key)?;
            destination.insert(spec.id, parse_observation(&key, &raw)?);
        }
    }
    if let Some(key) = values.keys().next() {
        return Err(SliceError::UnexpectedKey(key.clone()));
    }
    Ok(ReadinessPackage {
        package_id,
        mission_label,
        mission_days,
        source_label,
        baseline,
        alternative,
    })
}

fn assess_package(values: &BTreeMap<&'static str, Observation>) -> PackageAssessment {
    let facets = FACETS
        .iter()
        .map(|spec| {
            let observation = values[spec.id];
            let posture = match observation.state {
                EvidenceState::Current => {
                    if observation
                        .value_bps
                        .expect("current observation has value")
                        >= spec.threshold_bps
                    {
                        FacetPosture::Pass
                    } else {
                        FacetPosture::Concern
                    }
                }
                EvidenceState::Missing | EvidenceState::Stale => FacetPosture::Held,
                EvidenceState::NotApplicable => FacetPosture::NotApplicable,
            };
            FacetAssessment {
                id: spec.id,
                evidence_state: observation.state,
                observed_bps: observation.value_bps,
                threshold_bps: spec.threshold_bps,
                posture,
            }
        })
        .collect::<Vec<_>>();
    let pass_count = facets
        .iter()
        .filter(|facet| facet.posture == FacetPosture::Pass)
        .count();
    let concern_count = facets
        .iter()
        .filter(|facet| facet.posture == FacetPosture::Concern)
        .count();
    let held_count = facets
        .iter()
        .filter(|facet| facet.posture == FacetPosture::Held)
        .count();
    let posture = if held_count > 0 {
        PackagePosture::Held
    } else if concern_count > 0 {
        PackagePosture::NeedsAttention
    } else {
        PackagePosture::Assessable
    };
    let readiness_floor_bps = if held_count > 0 {
        None
    } else {
        facets.iter().filter_map(|facet| facet.observed_bps).min()
    };
    let bottleneck_facets = readiness_floor_bps.map_or_else(Vec::new, |floor| {
        facets
            .iter()
            .filter(|facet| facet.observed_bps == Some(floor))
            .map(|facet| facet.id)
            .collect()
    });
    PackageAssessment {
        posture,
        readiness_floor_bps,
        bottleneck_facets,
        pass_count,
        concern_count,
        held_count,
        facets,
    }
}

#[must_use]
pub fn assess(package: &ReadinessPackage) -> ReadinessAssessment {
    let baseline = assess_package(&package.baseline);
    let alternative = assess_package(&package.alternative);
    let mut improved_facets = Vec::new();
    let mut worsened_facets = Vec::new();
    let mut held_resolved_facets = Vec::new();
    for (before, after) in baseline.facets.iter().zip(&alternative.facets) {
        let rank = |posture| match posture {
            FacetPosture::Pass | FacetPosture::NotApplicable => 0,
            FacetPosture::Concern => 1,
            FacetPosture::Held => 2,
        };
        if before.posture == FacetPosture::Held
            && matches!(after.posture, FacetPosture::Pass | FacetPosture::Concern)
        {
            held_resolved_facets.push(before.id);
        }
        if rank(after.posture) < rank(before.posture) {
            improved_facets.push(before.id);
        } else if rank(after.posture) > rank(before.posture) {
            worsened_facets.push(before.id);
        }
    }
    let floor_change_bps = baseline
        .readiness_floor_bps
        .zip(alternative.readiness_floor_bps)
        .map(|(before, after)| i32::from(after) - i32::from(before));
    ReadinessAssessment {
        package_id: package.package_id.clone(),
        mission_label: package.mission_label.clone(),
        mission_days: package.mission_days,
        source_label: package.source_label.clone(),
        comparison: ReadinessComparison {
            improved_facets,
            worsened_facets,
            held_resolved_facets,
            baseline_floor_bps: baseline.readiness_floor_bps,
            alternative_floor_bps: alternative.readiness_floor_bps,
            floor_change_bps,
        },
        baseline,
        alternative,
    }
}

fn json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            value if value.is_control() => {
                let _ = write!(output, "\\u{:04x}", value as u32);
            }
            value => output.push(value),
        }
    }
    output.push('"');
}

fn write_string_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        json_string(output, value);
    }
    output.push(']');
}

fn write_optional_u16(output: &mut String, value: Option<u16>) {
    if let Some(value) = value {
        let _ = write!(output, "{value}");
    } else {
        output.push_str("null");
    }
}

fn write_package(output: &mut String, package: &PackageAssessment) {
    let _ = write!(
        output,
        "{{\"posture\":\"{}\",\"readiness_floor_bps\":",
        package.posture.as_str()
    );
    write_optional_u16(output, package.readiness_floor_bps);
    output.push_str(",\"bottleneck_facets\":");
    write_string_array(output, &package.bottleneck_facets);
    let _ = write!(
        output,
        ",\"pass_count\":{},\"concern_count\":{},\"held_count\":{},\"facets\":[",
        package.pass_count, package.concern_count, package.held_count
    );
    for (index, facet) in package.facets.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let _ = write!(
            output,
            "{{\"id\":\"{}\",\"evidence_state\":\"{}\",\"observed_bps\":",
            facet.id,
            facet.evidence_state.as_str()
        );
        write_optional_u16(output, facet.observed_bps);
        let _ = write!(
            output,
            ",\"threshold_bps\":{},\"posture\":\"{}\"}}",
            facet.threshold_bps,
            facet.posture.as_str()
        );
    }
    output.push_str("]}");
}

impl ReadinessAssessment {
    #[must_use]
    pub fn to_canonical_json(&self) -> String {
        let mut output = String::new();
        output.push_str("{\"schema\":");
        json_string(&mut output, OUTPUT_SCHEMA);
        output.push_str(",\"package_id\":");
        json_string(&mut output, &self.package_id);
        output.push_str(",\"mission_label\":");
        json_string(&mut output, &self.mission_label);
        let _ = write!(output, ",\"mission_days\":{}", self.mission_days);
        output.push_str(",\"source_label\":");
        json_string(&mut output, &self.source_label);
        output.push_str(",\"baseline\":");
        write_package(&mut output, &self.baseline);
        output.push_str(",\"alternative\":");
        write_package(&mut output, &self.alternative);
        output.push_str(",\"comparison\":{");
        output.push_str("\"improved_facets\":");
        write_string_array(&mut output, &self.comparison.improved_facets);
        output.push_str(",\"worsened_facets\":");
        write_string_array(&mut output, &self.comparison.worsened_facets);
        output.push_str(",\"held_resolved_facets\":");
        write_string_array(&mut output, &self.comparison.held_resolved_facets);
        output.push_str(",\"baseline_floor_bps\":");
        write_optional_u16(&mut output, self.comparison.baseline_floor_bps);
        output.push_str(",\"alternative_floor_bps\":");
        write_optional_u16(&mut output, self.comparison.alternative_floor_bps);
        output.push_str(",\"floor_change_bps\":");
        if let Some(value) = self.comparison.floor_change_bps {
            let _ = write!(output, "{value}");
        } else {
            output.push_str("null");
        }
        output.push_str("},\"authority\":{\"operational_plan\":false,\"deployment_advice\":false,\"targeting\":false,\"procurement\":false,\"budget_allocation\":false,\"taxlane_action\":false}}\n");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/readiness-package.fixture");

    #[test]
    fn synthetic_slice_identifies_bottleneck_and_support_improvement() {
        let result = assess(&parse_package(FIXTURE).expect("fixture parses"));
        assert_eq!(result.baseline.posture, PackagePosture::NeedsAttention);
        assert_eq!(result.baseline.readiness_floor_bps, Some(6_000));
        assert_eq!(
            result.baseline.bottleneck_facets,
            vec!["supplier-resilience"]
        );
        assert_eq!(result.alternative.posture, PackagePosture::Assessable);
        assert_eq!(result.alternative.readiness_floor_bps, Some(7_800));
        assert_eq!(result.comparison.floor_change_bps, Some(1_800));
        assert_eq!(result.comparison.improved_facets.len(), 5);
    }

    #[test]
    fn missing_or_stale_facet_holds_and_suppresses_floor() {
        let changed = FIXTURE
            .replace(
                "baseline.spares_fill_bps=current:6800",
                "baseline.spares_fill_bps=missing",
            )
            .replace(
                "alternative.spares_fill_bps=current:8500",
                "alternative.spares_fill_bps=stale:8500",
            );
        let result = assess(&parse_package(&changed).expect("held fixture parses"));
        assert_eq!(result.baseline.posture, PackagePosture::Held);
        assert_eq!(result.baseline.readiness_floor_bps, None);
        assert_eq!(result.alternative.posture, PackagePosture::Held);
        assert_eq!(result.comparison.floor_change_bps, None);
    }

    #[test]
    fn parser_rejects_operational_or_person_shaped_extra_fields() {
        for field in [
            "deployment_location",
            "target_coordinates",
            "service_member_name",
        ] {
            let extra = format!("{FIXTURE}{field}=forbidden\n");
            assert!(
                matches!(parse_package(&extra), Err(SliceError::UnexpectedKey(key)) if key == field)
            );
        }
    }

    #[test]
    fn all_readiness_facets_remain_visible() {
        let result = assess(&parse_package(FIXTURE).expect("fixture parses"));
        assert_eq!(result.baseline.facets.len(), FACETS.len());
        assert!(
            result
                .baseline
                .facets
                .iter()
                .any(|facet| facet.id == "maintenance-completion")
        );
        assert!(
            result
                .baseline
                .facets
                .iter()
                .any(|facet| facet.id == "interoperability")
        );
        assert!(
            result
                .baseline
                .facets
                .iter()
                .any(|facet| facet.id == "safety-compliance")
        );
    }

    #[test]
    fn canonical_output_is_stable_and_non_operational() {
        let result = assess(&parse_package(FIXTURE).expect("fixture parses"));
        let first = result.to_canonical_json();
        assert_eq!(first, result.to_canonical_json());
        assert!(first.starts_with("{\"schema\":\"bastion.readiness-assessment.v1\""));
        assert!(first.contains("\"operational_plan\":false"));
        assert!(first.ends_with('\n'));
    }
}
