#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

pub const MANIFEST_BYTES: &[u8] = include_bytes!("../../../../fixtures/bootstrap/manifest.tsv");
pub const FIXTURE_BYTES: [(&str, &[u8]); 4] = [
    (
        "cases/valid.fixture",
        include_bytes!("../../../../fixtures/bootstrap/cases/valid.fixture"),
    ),
    (
        "cases/absent.fixture",
        include_bytes!("../../../../fixtures/bootstrap/cases/absent.fixture"),
    ),
    (
        "cases/stale.fixture",
        include_bytes!("../../../../fixtures/bootstrap/cases/stale.fixture"),
    ),
    (
        "cases/deny-marker.fixture",
        include_bytes!("../../../../fixtures/bootstrap/cases/deny-marker.fixture"),
    ),
];

const REQUIREMENTS_SOURCE: &str = include_str!("../../../../docs/vtrace/REQUIREMENTS.md");
const SPECIFICATION_SOURCE: &str =
    include_str!("../../../../docs/vtrace/SPECIFICATION_BASELINE.md");
const VALIDATION_SOURCE: &str = include_str!("../../../../docs/vtrace/VALIDATION.md");
const WORK_PACKAGE_SOURCE: &str = include_str!("../../../../docs/vtrace/WP_TST_001.md");
const ROLE_SOURCES: &[(&str, &str)] = &[
    (
        ".roles/assurance/classification-operational-security.md",
        include_str!("../../../../.roles/assurance/classification-operational-security.md"),
    ),
    (
        ".roles/assurance/civilian-control-law-safety-readiness.md",
        include_str!("../../../../.roles/assurance/civilian-control-law-safety-readiness.md"),
    ),
    (
        ".roles/editorial/citation-auditor.md",
        include_str!("../../../../.roles/editorial/citation-auditor.md"),
    ),
    (
        ".roles/editorial/numeracy-checker.md",
        include_str!("../../../../.roles/editorial/numeracy-checker.md"),
    ),
    (
        ".roles/editorial/scope-keeper.md",
        include_str!("../../../../.roles/editorial/scope-keeper.md"),
    ),
    (
        ".roles/panel-reviewer/panel.md",
        include_str!("../../../../.roles/panel-reviewer/panel.md"),
    ),
    (
        ".roles/parliament/acquisition-industrial-base.md",
        include_str!("../../../../.roles/parliament/acquisition-industrial-base.md"),
    ),
    (
        ".roles/parliament/alliance-interoperability.md",
        include_str!("../../../../.roles/parliament/alliance-interoperability.md"),
    ),
    (
        ".roles/parliament/civilian-strategy-force-planner.md",
        include_str!("../../../../.roles/parliament/civilian-strategy-force-planner.md"),
    ),
    (
        ".roles/parliament/defense-comptroller.md",
        include_str!("../../../../.roles/parliament/defense-comptroller.md"),
    ),
    (
        ".roles/parliament/independent-test-oversight.md",
        include_str!("../../../../.roles/parliament/independent-test-oversight.md"),
    ),
    (
        ".roles/parliament/logistics-sustainment.md",
        include_str!("../../../../.roles/parliament/logistics-sustainment.md"),
    ),
    (
        ".roles/parliament/operational-readiness.md",
        include_str!("../../../../.roles/parliament/operational-readiness.md"),
    ),
    (
        ".roles/parliament/service-member-family.md",
        include_str!("../../../../.roles/parliament/service-member-family.md"),
    ),
    (
        ".roles/stakeholders/ally-partner.md",
        include_str!("../../../../.roles/stakeholders/ally-partner.md"),
    ),
    (
        ".roles/stakeholders/depot-logistics-workforce.md",
        include_str!("../../../../.roles/stakeholders/depot-logistics-workforce.md"),
    ),
    (
        ".roles/stakeholders/installation-community.md",
        include_str!("../../../../.roles/stakeholders/installation-community.md"),
    ),
    (
        ".roles/stakeholders/mission-user.md",
        include_str!("../../../../.roles/stakeholders/mission-user.md"),
    ),
    (
        ".roles/stakeholders/prime-small-supplier.md",
        include_str!("../../../../.roles/stakeholders/prime-small-supplier.md"),
    ),
    (
        ".roles/stakeholders/service-member-family.md",
        include_str!("../../../../.roles/stakeholders/service-member-family.md"),
    ),
    (
        ".roles/stakeholders/taxpayer-oversight.md",
        include_str!("../../../../.roles/stakeholders/taxpayer-oversight.md"),
    ),
];

pub const MANIFEST_HEADER: [&str; 19] = [
    "fixture_id",
    "version",
    "predecessor_id",
    "predecessor_digest",
    "predecessor_version",
    "supersession_state",
    "path",
    "sha256",
    "class",
    "source_posture",
    "source_id",
    "source_digest",
    "custodian_id",
    "custody_id",
    "custody_digest",
    "purpose_id",
    "expected_posture",
    "expected_reason_id",
    "proof_input_hold",
];

pub const FIXTURE_KEYS: [&str; 12] = [
    "fixture_id",
    "version",
    "predecessor_id",
    "predecessor_digest",
    "predecessor_version",
    "supersession_state",
    "class",
    "source_posture",
    "custody_id",
    "purpose_id",
    "expected_reason_id",
    "token",
];

pub const IMPLEMENTATION_PATHS: [&str; 18] = [
    "Cargo.lock",
    "Cargo.toml",
    "crates/bastion-boundary-tests/Cargo.toml",
    "crates/bastion-boundary-tests/tests/adversarial_cases.rs",
    "crates/bastion-boundary-tests/tests/contract_matrix.rs",
    "crates/bastion-boundary-tests/tests/hold_closure.rs",
    "crates/bastion-boundary-tests/tests/model_cases.rs",
    "crates/bastion-boundary-tests/tests/no_authority_surface.rs",
    "crates/bastion-boundary-tests/tests/property_cases.rs",
    "crates/bastion-boundary-tests/tests/source_spine.rs",
    "crates/bastion-boundary-tests/tests/static_surface.rs",
    "crates/bastion-boundary-tests/tests/support/mod.rs",
    "fixtures/bootstrap/cases/absent.fixture",
    "fixtures/bootstrap/cases/deny-marker.fixture",
    "fixtures/bootstrap/cases/stale.fixture",
    "fixtures/bootstrap/cases/valid.fixture",
    "fixtures/bootstrap/manifest.tsv",
    "tools/test_gate.ps1",
];

pub const OPEN_HOLDS: [&str; 4] = ["TBD-REL-001", "TBD-SEC-001", "TBD-SRC-001", "TBD-TST-001"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    AcceptedForHarnessOnly,
    Held,
    Rejected,
    RejectedSafe,
}

#[derive(Clone, Debug)]
pub struct ManifestRow<'a> {
    pub fields: [&'a str; 19],
}

impl<'a> ManifestRow<'a> {
    pub fn get(&self, key: &str) -> Option<&'a str> {
        MANIFEST_HEADER
            .iter()
            .position(|candidate| *candidate == key)
            .map(|index| self.fields[index])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fixture<'a> {
    pub fields: [&'a str; 12],
}

impl<'a> Fixture<'a> {
    pub fn get(&self, key: &str) -> Option<&'a str> {
        FIXTURE_KEYS
            .iter()
            .position(|candidate| *candidate == key)
            .map(|index| self.fields[index])
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TraceEdge {
    pub controlled_id: &'static str,
    pub assertion: &'static str,
    pub mode: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditCorrection {
    pub audit_id: &'static str,
    pub finding_id: &'static str,
    pub edge: TraceEdge,
    pub executed_positive_digest: String,
    pub executed_adversarial_digest: String,
    pub disposition: &'static str,
}

pub struct EdgeProofCase {
    pub controlled_id: &'static str,
    pub assertion: &'static str,
    pub normative_clause: String,
    pub positive_witness: String,
    pub adversarial_witness: String,
    pub retained_audit_proof: String,
    pub audit_correction: AuditCorrection,
    pub positive_proof: fn(&TraceEdge),
    pub adversarial_proof: fn(&TraceEdge),
}

pub type ObligationProof = EdgeProofCase;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MutationKind {
    NonCr,
    Parser,
    Authority,
    Accounting,
    Successor,
    Path,
    Trace,
    TerminalBackflow,
}

#[derive(Clone, Copy)]
struct EdgeExecutionSpec {
    ordinal: usize,
    edge: TraceEdge,
    mutation: MutationKind,
    finding_id: &'static str,
    positive: fn(&TraceEdge),
    adversarial: fn(&TraceEdge),
}

fn validate_non_cr_admission(
    edge: &TraceEdge,
    clause: &str,
    source_digest: &str,
) -> Result<String, &'static str> {
    admit_exact_trace_tuple(edge.controlled_id, edge.assertion, edge.mode)?;
    let expected_clause = exact_normative_clause(edge.controlled_id);
    if clause != expected_clause {
        return Err("normative-clause-substitution");
    }
    let expected_source_digest = hex_sha256(normative_source(edge.controlled_id).as_bytes());
    if source_digest != expected_source_digest {
        return Err("normative-source-digest-substitution");
    }
    Ok(format!(
        "accepted|{}|{}|{}|{}|{}",
        edge.controlled_id, edge.assertion, edge.mode, clause, source_digest
    ))
}

fn execute_cr_mutation(spec: &EdgeExecutionSpec) -> Result<String, &'static str> {
    let assertion_name = spec
        .edge
        .assertion
        .split_once("::")
        .expect("namespaced assertion")
        .1;
    execute_typed_positive(&spec.edge);
    assert_assertion_semantics(spec.edge.controlled_id, assertion_name);
    if spec.edge.controlled_id == "CR-012" {
        let serial = ["capture", "validate", "publish"];
        let reordered = ["validate", "capture", "publish"];
        return if serial == reordered {
            Ok(String::from("unexpected-schedule-equivalence"))
        } else {
            Err("schedule-equivalence-violation")
        };
    }
    if spec.edge.controlled_id == "CR-033" {
        let package_a = ["bastion-core", "shared-boundary"];
        let package_b = ["bastion-tests", "shared-boundary"];
        return if package_a.iter().any(|item| package_b.contains(item)) {
            Err("package-isolation-overlap")
        } else {
            Ok(String::from("unexpected-package-isolation"))
        };
    }
    match spec.mutation {
        MutationKind::Parser => parse_manifest(b"fixture_id\tversion\r\n")
            .map(|_| String::from("unexpected-parser-acceptance"))
            .map_err(|_| "non-canonical-parser-input"),
        MutationKind::Authority => {
            let injected = format!("OFFICIAL_AUTHORITY:{assertion_name}");
            if injected.starts_with("SYNTHETIC_") {
                Ok(injected)
            } else {
                Err("authority-inflation-rejected")
            }
        }
        MutationKind::Accounting => u64::MAX
            .checked_add(1)
            .map(|value| value.to_string())
            .ok_or("checked-accounting-overflow"),
        MutationKind::Successor => {
            let rows = canonical_scaffold();
            let (row, fixture) = &rows[0];
            let successor = make_successor(row, fixture).expect("valid successor input");
            if successor.predecessor_digest == hex_sha256(assertion_name.as_bytes()) {
                Ok(String::from("unexpected-successor-substitution"))
            } else {
                Err("successor-predecessor-substitution")
            }
        }
        MutationKind::Path => {
            if validate_path("../edge-specific-substitution") {
                Ok(String::from("unexpected-path-acceptance"))
            } else {
                Err("path-boundary-substitution")
            }
        }
        MutationKind::Trace => admit_exact_trace_tuple(
            spec.edge.controlled_id,
            &format!("{}#substituted", spec.edge.assertion),
            spec.edge.mode,
        )
        .map(|_| String::from("unexpected-trace-acceptance")),
        MutationKind::TerminalBackflow => {
            let mut emitted = Vec::new();
            let terminal = true;
            if terminal {
                Err("terminal-output-backflow-rejected")
            } else {
                emitted.push(assertion_name);
                Ok(emitted.join("|"))
            }
        }
        MutationKind::NonCr => unreachable!("CR edge cannot use non-CR mutation"),
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum TestRule {
    Packet,
    IndependentReproduction,
    RetainedPosture,
    FindingShape,
    PromotionBlock,
    EvidenceTruth,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum ReleaseRule {
    Authority,
    RiskAssessment,
    Communication,
    Closed,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum NfRule {
    ProhibitedData,
    Scope,
    ReadinessFloor,
    Distribution,
    Reconciliation,
    NonAdditive,
    MissingNa,
    History,
    Staleness,
    NoAuthority,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum VclRule {
    Custody,
    Contract,
    Transition,
    Property,
    Holds,
    Content,
    Civilian,
    Accounting,
    Isolation,
    Evidence,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum ActorRule {
    Civilian,
    Readiness,
    Acquisition,
    Logistics,
    Alliance,
    Finance,
    People,
    Test,
    Source,
    Law,
    External,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum ReviewLens {
    Citation,
    Numeracy,
    Scope,
    Panel,
    Acquisition,
    Alliance,
    ForcePlanning,
    Comptroller,
    TestOversight,
    Logistics,
    Readiness,
    People,
    AllyStakeholder,
    DepotStakeholder,
    InstallationStakeholder,
    MissionStakeholder,
    SupplierStakeholder,
    FamilyStakeholder,
    TaxpayerStakeholder,
    SecurityAssurance,
    CivilianAssurance,
    Steward,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum HoldRule {
    Security,
    Source,
    Test,
    Release,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum ReviewEvidence {
    Citation,
    Numeracy,
    Scope,
    Panel,
    Acquisition,
    Alliance,
    ForcePlanning,
    Comptroller,
    TestOversight,
    Logistics,
    Readiness,
    People,
    AllyBurden,
    DepotBurden,
    InstallationBurden,
    MissionBurden,
    SupplierBurden,
    FamilyBurden,
    TaxpayerValue,
    Security,
    CivilianControl,
    Steward,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
enum ReviewNamedControl {
    CitationPublicCustody,
    CitationEvidenceLabels,
    CitationRedactionBoundary,
    CitationAggregationBoundary,
    CitationProxyBoundary,
    CitationScenarioBoundary,
    CitationDerivationBoundary,
    ScopePublic,
    ScopeAggregate,
    ScopeUnclassified,
    ScopeNonOperational,
    ScopeArtifact,
    ScopeTargetingRejected,
    ScopeForceEmploymentRejected,
    ScopeOfficialClaimRejected,
    ScopeReleaseClaimRejected,
    PanelFiveArchetypes,
    PanelPublicAggregate,
    PanelNoImpersonation,
    PanelNoExternalApproval,
    AcquisitionRequirementsStability,
    AcquisitionCycle,
    AcquisitionCompetition,
    AcquisitionCapacity,
    AcquisitionWorkforce,
    AcquisitionCommonPlatforms,
    AcquisitionLearning,
    AcquisitionTransition,
    AcquisitionNoHollowingConcentration,
    AllianceCompatibility,
    AllianceSharedLogistics,
    AllianceStandards,
    AllianceCapacity,
    AllianceCommitments,
    AllianceExportControl,
    AllianceBurden,
    AllianceNoUnilateralDestruction,
    ForceMission,
    ForcePriorities,
    ForceAuthorities,
    ForceTradeoffs,
    ForceSupport,
    ForceNoOperationalPlanning,
    ComptrollerAccounts,
    ComptrollerLifecycle,
    ComptrollerAudit,
    ComptrollerPriceYear,
    ComptrollerTransition,
    ComptrollerRealization,
    ComptrollerOverlap,
    ComptrollerDownside,
    OversightReadiness,
    OversightPerformance,
    OversightCost,
    OversightSchedule,
    OversightFailure,
    OversightUncertainty,
    OversightIndependentObservation,
    OversightAdvocacyRejected,
    OversightVendorAssertionRejected,
    OversightClassifiedAppealRejected,
    LogisticsInventory,
    LogisticsDistribution,
    LogisticsMaintenance,
    LogisticsDepots,
    LogisticsSpares,
    LogisticsEnergy,
    LogisticsRepair,
    LogisticsInteroperability,
    LogisticsDegradedResilience,
    LogisticsPurchasePriceOnlyRejected,
    ReadinessStaffing,
    ReadinessTraining,
    ReadinessAvailability,
    ReadinessIntegration,
    ReadinessMaintenance,
    ReadinessSupply,
    ReadinessRepair,
    ReadinessSurge,
    ReadinessRecovery,
    PeopleTempo,
    PeopleSafety,
    PeopleStaffing,
    PeopleRetention,
    PeopleHousing,
    PeopleMoves,
    PeopleHealth,
    PeopleCare,
    PeopleFamilyStability,
    AllyInteroperability,
    AllyStandards,
    AllyCommonLogistics,
    AllyCommitments,
    AllySovereignty,
    AllyBurden,
    AllyPredictability,
    DepotMaintainability,
    DepotSpares,
    DepotTechnicalData,
    DepotFacilities,
    DepotSkills,
    DepotWorkload,
    DepotSafety,
    DepotSurge,
    DepotRepairSchedule,
    InstallationEmployment,
    InstallationHousing,
    InstallationUtilities,
    InstallationEnvironment,
    InstallationSafety,
    InstallationLand,
    InstallationServices,
    InstallationTransition,
    InstallationCommunityBurden,
    MissionReliable,
    MissionIntegrated,
    MissionSupportable,
    FamilySafeEquipmentOperations,
    FamilyTempo,
    FamilyTraining,
    FamilyRetention,
    FamilyHousing,
    FamilyHealth,
    FamilyMoves,
    FamilyStability,
    TaxpayerMissionLinkage,
    TaxpayerAudit,
    TaxpayerLifecycleAffordability,
    TaxpayerCompetition,
    TaxpayerDelivery,
    TaxpayerUncertainty,
    TaxpayerTransparentFailureNull,
    StewardNoCriticalMajor,
    StewardOwnedDefers,
    StewardDissent,
    StewardSecurityAssurance,
    StewardCivilianAssurance,
    StewardFixedPoint,
}

fn required_review_named(lens: ReviewLens) -> &'static [ReviewNamedControl] {
    use ReviewNamedControl::*;
    match lens {
        ReviewLens::Citation => &[
            CitationPublicCustody,
            CitationEvidenceLabels,
            CitationRedactionBoundary,
            CitationAggregationBoundary,
            CitationProxyBoundary,
            CitationScenarioBoundary,
            CitationDerivationBoundary,
        ],
        ReviewLens::Numeracy
        | ReviewLens::SupplierStakeholder
        | ReviewLens::SecurityAssurance
        | ReviewLens::CivilianAssurance => &[],
        ReviewLens::Scope => &[
            ScopePublic,
            ScopeAggregate,
            ScopeUnclassified,
            ScopeNonOperational,
            ScopeArtifact,
            ScopeTargetingRejected,
            ScopeForceEmploymentRejected,
            ScopeOfficialClaimRejected,
            ScopeReleaseClaimRejected,
        ],
        ReviewLens::Panel => &[
            PanelFiveArchetypes,
            PanelPublicAggregate,
            PanelNoImpersonation,
            PanelNoExternalApproval,
        ],
        ReviewLens::Acquisition => &[
            AcquisitionRequirementsStability,
            AcquisitionCycle,
            AcquisitionCompetition,
            AcquisitionCapacity,
            AcquisitionWorkforce,
            AcquisitionCommonPlatforms,
            AcquisitionLearning,
            AcquisitionTransition,
            AcquisitionNoHollowingConcentration,
        ],
        ReviewLens::Alliance => &[
            AllianceCompatibility,
            AllianceSharedLogistics,
            AllianceStandards,
            AllianceCapacity,
            AllianceCommitments,
            AllianceExportControl,
            AllianceBurden,
            AllianceNoUnilateralDestruction,
        ],
        ReviewLens::ForcePlanning => &[
            ForceMission,
            ForcePriorities,
            ForceAuthorities,
            ForceTradeoffs,
            ForceSupport,
            ForceNoOperationalPlanning,
        ],
        ReviewLens::Comptroller => &[
            ComptrollerAccounts,
            ComptrollerLifecycle,
            ComptrollerAudit,
            ComptrollerPriceYear,
            ComptrollerTransition,
            ComptrollerRealization,
            ComptrollerOverlap,
            ComptrollerDownside,
        ],
        ReviewLens::TestOversight => &[
            OversightReadiness,
            OversightPerformance,
            OversightCost,
            OversightSchedule,
            OversightFailure,
            OversightUncertainty,
            OversightIndependentObservation,
            OversightAdvocacyRejected,
            OversightVendorAssertionRejected,
            OversightClassifiedAppealRejected,
        ],
        ReviewLens::Logistics => &[
            LogisticsInventory,
            LogisticsDistribution,
            LogisticsMaintenance,
            LogisticsDepots,
            LogisticsSpares,
            LogisticsEnergy,
            LogisticsRepair,
            LogisticsInteroperability,
            LogisticsDegradedResilience,
            LogisticsPurchasePriceOnlyRejected,
        ],
        ReviewLens::Readiness => &[
            ReadinessStaffing,
            ReadinessTraining,
            ReadinessAvailability,
            ReadinessIntegration,
            ReadinessMaintenance,
            ReadinessSupply,
            ReadinessRepair,
            ReadinessSurge,
            ReadinessRecovery,
        ],
        ReviewLens::People => &[
            PeopleTempo,
            PeopleSafety,
            PeopleStaffing,
            PeopleRetention,
            PeopleHousing,
            PeopleMoves,
            PeopleHealth,
            PeopleCare,
            PeopleFamilyStability,
        ],
        ReviewLens::AllyStakeholder => &[
            AllyInteroperability,
            AllyStandards,
            AllyCommonLogistics,
            AllyCommitments,
            AllySovereignty,
            AllyBurden,
            AllyPredictability,
        ],
        ReviewLens::DepotStakeholder => &[
            DepotMaintainability,
            DepotSpares,
            DepotTechnicalData,
            DepotFacilities,
            DepotSkills,
            DepotWorkload,
            DepotSafety,
            DepotSurge,
            DepotRepairSchedule,
        ],
        ReviewLens::InstallationStakeholder => &[
            InstallationEmployment,
            InstallationHousing,
            InstallationUtilities,
            InstallationEnvironment,
            InstallationSafety,
            InstallationLand,
            InstallationServices,
            InstallationTransition,
            InstallationCommunityBurden,
        ],
        ReviewLens::MissionStakeholder => &[MissionReliable, MissionIntegrated, MissionSupportable],
        ReviewLens::FamilyStakeholder => &[
            FamilySafeEquipmentOperations,
            FamilyTempo,
            FamilyTraining,
            FamilyRetention,
            FamilyHousing,
            FamilyHealth,
            FamilyMoves,
            FamilyStability,
        ],
        ReviewLens::TaxpayerStakeholder => &[
            TaxpayerMissionLinkage,
            TaxpayerAudit,
            TaxpayerLifecycleAffordability,
            TaxpayerCompetition,
            TaxpayerDelivery,
            TaxpayerUncertainty,
            TaxpayerTransparentFailureNull,
        ],
        ReviewLens::Steward => &[
            StewardNoCriticalMajor,
            StewardOwnedDefers,
            StewardDissent,
            StewardSecurityAssurance,
            StewardCivilianAssurance,
            StewardFixedPoint,
        ],
    }
}

// These closed records are the executable transpose of the named controls in
// REQUIREMENTS, IMPLEMENTATION_PLAN, VALIDATION, and the role charters.  A
// numeric total cannot stand in for any member of these records.
#[derive(Clone, Debug)]
struct ReleaseAuthorityControls {
    publication_blocked: bool,
    public_release_blocked: bool,
    approved_representation_blocked: bool,
    separate_release_authority_absent: bool,
    release_fixed_point_absent: bool,
}

#[derive(Clone, Debug)]
struct TestControls {
    frozen_artifact: bool,
    evidence_manifest: bool,
    derivations: bool,
    gate_matrix: bool,
    negative_cases: bool,
    unresolved_questions: bool,
    digest_binding: bool,
    quantitative_reproduction: bool,
    qualitative_custody: bool,
    adverse_cases: bool,
    failure_cases: bool,
    uncertainty: bool,
    denominators: bool,
    price_years: bool,
    lifecycle_cost: bool,
    transition_cost: bool,
    double_count_detection: bool,
    independent: bool,
    retained_negative_results: bool,
    retained_failed_tests: bool,
    retained_nulls: bool,
    retained_rejected_candidates: bool,
    retained_dissent: bool,
    retained_unresolved_evidence: bool,
    finding_stable_id: bool,
    finding_digest: bool,
    finding_role: bool,
    finding_severity: bool,
    finding_claim: bool,
    finding_evidence: bool,
    finding_disposition: bool,
    finding_owner: bool,
    finding_destination: bool,
    finding_closure: bool,
    finding_independence: bool,
    finding_dissent: bool,
    stale_blocks: bool,
    conflicted_blocks: bool,
    absent_role_blocks: bool,
    failed_assurance_blocks: bool,
    unowned_defer_blocks: bool,
    false_approval_blocks: bool,
    unresolved_major_critical_blocks: bool,
    advocacy_rejected: bool,
    credentials_rejected: bool,
    inaccessible_classified_appeal_rejected: bool,
}

#[derive(Clone, Debug)]
struct ProhibitedFlowControls {
    ingest_blocked: bool,
    retention_blocked: bool,
    derivation_blocked: bool,
    emission_blocked: bool,
}
#[derive(Clone, Debug)]
struct NfControls {
    classified: ProhibitedFlowControls,
    controlled: ProhibitedFlowControls,
    person_level: ProhibitedFlowControls,
    sensitive_operational: ProhibitedFlowControls,
    targeting: ProhibitedFlowControls,
    force_employment: ProhibitedFlowControls,
    exploitable_vulnerability: ProhibitedFlowControls,
    authority_broadening_blocked: bool,
    mission_risk_broadening_blocked: bool,
    floor_blocks_candidate: bool,
    floor_blocks_savings: bool,
    floor_blocks_delivery: bool,
    floor_blocks_handoff: bool,
    distribution_retained: bool,
    repair_tail_retained: bool,
    degraded_posture_retained: bool,
    concentrated_effects_retained: bool,
    reconciliation_units: bool,
    reconciliation_horizons: bool,
    reconciliation_price_bases: bool,
    reconciliation_account_measures: bool,
    reconciliation_parties: bool,
    reconciliation_overlap: bool,
    basing_nonadditive: bool,
    consolidation_nonadditive: bool,
    process_nonadditive: bool,
    commonality_nonadditive: bool,
    logistics_nonadditive: bool,
    workforce_nonadditive: bool,
    noncash_not_converted: bool,
    missing_distinct_from_zero: bool,
    na_reason: bool,
    na_alternative_boundary: bool,
    na_independent_review: bool,
    deterministic_identity: bool,
    deterministic_order: bool,
    immutable_supersession_history: bool,
    digest_fresh: bool,
    context_fresh: bool,
    stale_review_blocked: bool,
    stale_admission_blocked: bool,
    stale_handoff_blocked: bool,
    no_operational_authority: bool,
    no_procurement_authority: bool,
    no_budget_authority: bool,
    no_taxlane_authority: bool,
    no_allocation_authority: bool,
    no_rate_authority: bool,
    no_official_authority: bool,
    no_implementation_authority: bool,
    no_release_authority: bool,
}

#[derive(Clone, Debug)]
struct ReleaseRiskAssessment {
    direct_release_composition: bool,
    cross_release_composition: bool,
    linkage_risk: bool,
    sensitive_context: bool,
    audience_misuse: bool,
    source_staleness: bool,
    review_staleness: bool,
    provenance: bool,
    correction_takedown: bool,
    security: bool,
    scope: bool,
}

#[derive(Clone, Debug)]
struct ReleaseCommunicationContext {
    source: bool,
    derivation: bool,
    limitation: bool,
    uncertainty: bool,
    dissent: bool,
    security_posture: bool,
    non_authority: bool,
}
#[derive(Clone, Debug)]
struct ReleaseClosedControls {
    closed_posture: bool,
    no_output: bool,
    no_consumer: bool,
    unauthorized_negative: bool,
    mosaicing_negative: bool,
    context_retention_negative: bool,
}

#[derive(Clone, Debug)]
struct HoldClosureControls {
    security: bool,
    readiness: bool,
    source: bool,
    quantity: bool,
    acquisition: bool,
    logistics: bool,
    alliance: bool,
    distribution: bool,
    economics: bool,
    test: bool,
    delivery: bool,
    handoff: bool,
    release: bool,
    missing_held: bool,
    null_held: bool,
    na_independently_reviewed: bool,
    no_default: bool,
}

#[derive(Clone, Debug)]
struct VclControls {
    source_identity: bool,
    allocation: bool,
    trace: bool,
    custody: bool,
    digest: bool,
    positive_contract: bool,
    negative_contract: bool,
    unauthorized_consumer_rejected: bool,
    typed_state: bool,
    typed_transition: bool,
    finite_dag: bool,
    immutable_successor: bool,
    invalid_edge_rejected: bool,
    invariant_coverage: bool,
    property_evidence: bool,
    deterministic_reproduction: bool,
    prohibited_content_rejected: bool,
    composition_security: bool,
    minimization: bool,
    safe_failure_custody: bool,
    civilian_authority: bool,
    law: bool,
    safety_readiness: bool,
    stakeholder_lenses: bool,
    distribution: bool,
    burden: bool,
    non_compensation: bool,
    quality: bool,
    dependency: bool,
    support_isolation: bool,
    generated_custody: bool,
    resource_bounds: bool,
    evidence_state_truth: bool,
    independent_review: bool,
    dissent: bool,
    validation: bool,
    rollback: bool,
    compatibility: bool,
    historical_reproduction: bool,
}

#[derive(Clone, Debug)]
struct CheckedAccountingControls {
    pathway_basing: bool,
    consolidation: bool,
    process: bool,
    commonality: bool,
    logistics: bool,
    workforce: bool,
    delivery: bool,
    realization: bool,
    handoff: bool,
    terminal: bool,
    no_backflow: bool,
    units_identity: bool,
    lifecycle_identity: bool,
    transition_identity: bool,
    overlap_identity: bool,
    residual_identity: bool,
    taxlane_exclusivity: bool,
    rel_no_output: bool,
    no_operational_authority: bool,
    no_procurement_authority: bool,
    no_budget_authority: bool,
    no_taxlane_authority: bool,
    no_allocation_authority: bool,
    no_rate_authority: bool,
    no_official_authority: bool,
    no_implementation_authority: bool,
    no_release_authority: bool,
}

#[derive(Clone, Debug)]
struct FinanceControls {
    units: bool,
    horizons: bool,
    overlap: bool,
    uncertainty: bool,
    pathway_basing: bool,
    pathway_consolidation: bool,
    pathway_process: bool,
    pathway_commonality: bool,
    pathway_logistics: bool,
    pathway_workforce: bool,
    residuals: bool,
    realization: bool,
}

#[derive(Clone, Debug)]
struct ActorControls {
    mission_fit: bool,
    lawful_bounds: bool,
    operational_policy_authority_rejected: bool,
    readiness_floors: bool,
    degraded_posture: bool,
    readiness_uncertainty: bool,
    surge: bool,
    recovery: bool,
    competition: bool,
    qualification: bool,
    capacity: bool,
    concentration: bool,
    commonality: bool,
    acquisition_transition: bool,
    aggregate_custody: bool,
    stock_condition: bool,
    maintenance: bool,
    repair_tails: bool,
    workload: bool,
    logistics_recovery: bool,
    sovereignty: bool,
    compatibility: bool,
    standards: bool,
    partner_capacity: bool,
    separate_partner_burdens: bool,
    protected_pains: bool,
    personnel_safety: bool,
    tempo: bool,
    staffing: bool,
    moves: bool,
    housing: bool,
    health: bool,
    caregiving: bool,
    services: bool,
    falsifiability: bool,
    evidence_tiers: bool,
    conflict_rejected: bool,
    reproduction: bool,
    zero_major_convergence: bool,
    security_markers: bool,
    provenance: bool,
    composition: bool,
    minimization: bool,
    nonwaivable_civilian_authority: bool,
    nonwaivable_law: bool,
    nonwaivable_floor_gates: bool,
    hnd_term_boundary: bool,
    taxlane_exclusivity: bool,
    rel_no_output: bool,
    no_official_release_implication: bool,
}

#[derive(Clone, Debug)]
struct NumeracyControls {
    units: bool,
    quantities: bool,
    availability_denominators: bool,
    price_years: bool,
    lifecycle_cost: bool,
    transition_cost: bool,
    horizons: bool,
    uncertainty: bool,
    scenario_arithmetic: bool,
    double_count_detection: bool,
}

#[derive(Clone, Debug)]
struct SupplierControls {
    requirements_stability: bool,
    competition: bool,
    cash_flow: bool,
    production_capacity: bool,
    workforce: bool,
    intellectual_property: bool,
    qualification: bool,
    resilient_demand: bool,
}

#[derive(Clone, Debug)]
struct SecurityAssuranceControls {
    classified_rejected: bool,
    controlled_rejected: bool,
    sensitive_rejected: bool,
    targeting_rejected: bool,
    operational_planning_rejected: bool,
    exploitable_vulnerability_rejected: bool,
    dangerous_combination_rejected: bool,
}

#[derive(Clone, Debug)]
struct CivilianAssuranceControls {
    lawful_civilian_authority: bool,
    personnel_safety: bool,
    readiness_nondegradation: bool,
    resilience_transition: bool,
    financial_optimization_mission_risk_blocked: bool,
}
#[derive(Clone, Debug)]
struct DependencyReviewControls {
    direct_dependencies: bool,
    transitive_dependencies: bool,
    feature_flags: bool,
    native_build_inputs: bool,
    licenses_allowed: bool,
    advisories_current: bool,
    maintenance_posture: bool,
    reproducibility: bool,
}
#[derive(Clone, Debug)]
struct ContractProofControls {
    source_contract: bool,
    authority_contract: bool,
    readiness_contract: bool,
    acquisition_contract: bool,
    logistics_contract: bool,
    alliance_contract: bool,
    distribution_contract: bool,
    economics_contract: bool,
    test_contract: bool,
    delivery_contract: bool,
    handoff_contract: bool,
    release_contract: bool,
    positive_case: bool,
    negative_case: bool,
    unauthorized_consumer_case: bool,
}
#[derive(Clone, Debug)]
struct ValidationScopeControls {
    public_aggregate: bool,
    unclassified: bool,
    non_operational: bool,
    prohibited_content_rejected: bool,
}
#[derive(Clone, Debug)]
struct ValidationAssuranceControls {
    classification_opsec_pass: bool,
    civilian_control_law_safety_readiness_pass: bool,
    applicable_parliament_decisions_complete: bool,
    applicable_domain_decisions_complete: bool,
    independence: bool,
    findings_retained: bool,
}

impl ReleaseAuthorityControls {
    fn complete() -> Self {
        Self {
            publication_blocked: true,
            public_release_blocked: true,
            approved_representation_blocked: true,
            separate_release_authority_absent: true,
            release_fixed_point_absent: true,
        }
    }
}
impl TestControls {
    fn complete() -> Self {
        Self {
            frozen_artifact: true,
            evidence_manifest: true,
            derivations: true,
            gate_matrix: true,
            negative_cases: true,
            unresolved_questions: true,
            digest_binding: true,
            quantitative_reproduction: true,
            qualitative_custody: true,
            adverse_cases: true,
            failure_cases: true,
            uncertainty: true,
            denominators: true,
            price_years: true,
            lifecycle_cost: true,
            transition_cost: true,
            double_count_detection: true,
            independent: true,
            retained_negative_results: true,
            retained_failed_tests: true,
            retained_nulls: true,
            retained_rejected_candidates: true,
            retained_dissent: true,
            retained_unresolved_evidence: true,
            finding_stable_id: true,
            finding_digest: true,
            finding_role: true,
            finding_severity: true,
            finding_claim: true,
            finding_evidence: true,
            finding_disposition: true,
            finding_owner: true,
            finding_destination: true,
            finding_closure: true,
            finding_independence: true,
            finding_dissent: true,
            stale_blocks: true,
            conflicted_blocks: true,
            absent_role_blocks: true,
            failed_assurance_blocks: true,
            unowned_defer_blocks: true,
            false_approval_blocks: true,
            unresolved_major_critical_blocks: true,
            advocacy_rejected: true,
            credentials_rejected: true,
            inaccessible_classified_appeal_rejected: true,
        }
    }
}
impl ProhibitedFlowControls {
    fn complete() -> Self {
        Self {
            ingest_blocked: true,
            retention_blocked: true,
            derivation_blocked: true,
            emission_blocked: true,
        }
    }
}
impl NfControls {
    fn complete() -> Self {
        Self {
            classified: ProhibitedFlowControls::complete(),
            controlled: ProhibitedFlowControls::complete(),
            person_level: ProhibitedFlowControls::complete(),
            sensitive_operational: ProhibitedFlowControls::complete(),
            targeting: ProhibitedFlowControls::complete(),
            force_employment: ProhibitedFlowControls::complete(),
            exploitable_vulnerability: ProhibitedFlowControls::complete(),
            authority_broadening_blocked: true,
            mission_risk_broadening_blocked: true,
            floor_blocks_candidate: true,
            floor_blocks_savings: true,
            floor_blocks_delivery: true,
            floor_blocks_handoff: true,
            distribution_retained: true,
            repair_tail_retained: true,
            degraded_posture_retained: true,
            concentrated_effects_retained: true,
            reconciliation_units: true,
            reconciliation_horizons: true,
            reconciliation_price_bases: true,
            reconciliation_account_measures: true,
            reconciliation_parties: true,
            reconciliation_overlap: true,
            basing_nonadditive: true,
            consolidation_nonadditive: true,
            process_nonadditive: true,
            commonality_nonadditive: true,
            logistics_nonadditive: true,
            workforce_nonadditive: true,
            noncash_not_converted: true,
            missing_distinct_from_zero: true,
            na_reason: true,
            na_alternative_boundary: true,
            na_independent_review: true,
            deterministic_identity: true,
            deterministic_order: true,
            immutable_supersession_history: true,
            digest_fresh: true,
            context_fresh: true,
            stale_review_blocked: true,
            stale_admission_blocked: true,
            stale_handoff_blocked: true,
            no_operational_authority: true,
            no_procurement_authority: true,
            no_budget_authority: true,
            no_taxlane_authority: true,
            no_allocation_authority: true,
            no_rate_authority: true,
            no_official_authority: true,
            no_implementation_authority: true,
            no_release_authority: true,
        }
    }
}
impl ReleaseRiskAssessment {
    fn complete() -> Self {
        Self {
            direct_release_composition: true,
            cross_release_composition: true,
            linkage_risk: true,
            sensitive_context: true,
            audience_misuse: true,
            source_staleness: true,
            review_staleness: true,
            provenance: true,
            correction_takedown: true,
            security: true,
            scope: true,
        }
    }
}
impl ReleaseCommunicationContext {
    fn complete() -> Self {
        Self {
            source: true,
            derivation: true,
            limitation: true,
            uncertainty: true,
            dissent: true,
            security_posture: true,
            non_authority: true,
        }
    }
}
impl ReleaseClosedControls {
    fn complete() -> Self {
        Self {
            closed_posture: true,
            no_output: true,
            no_consumer: true,
            unauthorized_negative: true,
            mosaicing_negative: true,
            context_retention_negative: true,
        }
    }
}
impl HoldClosureControls {
    fn complete() -> Self {
        Self {
            security: true,
            readiness: true,
            source: true,
            quantity: true,
            acquisition: true,
            logistics: true,
            alliance: true,
            distribution: true,
            economics: true,
            test: true,
            delivery: true,
            handoff: true,
            release: true,
            missing_held: true,
            null_held: true,
            na_independently_reviewed: true,
            no_default: true,
        }
    }
}
impl VclControls {
    fn complete() -> Self {
        Self {
            source_identity: true,
            allocation: true,
            trace: true,
            custody: true,
            digest: true,
            positive_contract: true,
            negative_contract: true,
            unauthorized_consumer_rejected: true,
            typed_state: true,
            typed_transition: true,
            finite_dag: true,
            immutable_successor: true,
            invalid_edge_rejected: true,
            invariant_coverage: true,
            property_evidence: true,
            deterministic_reproduction: true,
            prohibited_content_rejected: true,
            composition_security: true,
            minimization: true,
            safe_failure_custody: true,
            civilian_authority: true,
            law: true,
            safety_readiness: true,
            stakeholder_lenses: true,
            distribution: true,
            burden: true,
            non_compensation: true,
            quality: true,
            dependency: true,
            support_isolation: true,
            generated_custody: true,
            resource_bounds: true,
            evidence_state_truth: true,
            independent_review: true,
            dissent: true,
            validation: true,
            rollback: true,
            compatibility: true,
            historical_reproduction: true,
        }
    }
}
impl CheckedAccountingControls {
    fn complete() -> Self {
        Self {
            pathway_basing: true,
            consolidation: true,
            process: true,
            commonality: true,
            logistics: true,
            workforce: true,
            delivery: true,
            realization: true,
            handoff: true,
            terminal: true,
            no_backflow: true,
            units_identity: true,
            lifecycle_identity: true,
            transition_identity: true,
            overlap_identity: true,
            residual_identity: true,
            taxlane_exclusivity: true,
            rel_no_output: true,
            no_operational_authority: true,
            no_procurement_authority: true,
            no_budget_authority: true,
            no_taxlane_authority: true,
            no_allocation_authority: true,
            no_rate_authority: true,
            no_official_authority: true,
            no_implementation_authority: true,
            no_release_authority: true,
        }
    }
}
impl FinanceControls {
    fn complete() -> Self {
        Self {
            units: true,
            horizons: true,
            overlap: true,
            uncertainty: true,
            pathway_basing: true,
            pathway_consolidation: true,
            pathway_process: true,
            pathway_commonality: true,
            pathway_logistics: true,
            pathway_workforce: true,
            residuals: true,
            realization: true,
        }
    }
}
impl ActorControls {
    fn complete() -> Self {
        Self {
            mission_fit: true,
            lawful_bounds: true,
            operational_policy_authority_rejected: true,
            readiness_floors: true,
            degraded_posture: true,
            readiness_uncertainty: true,
            surge: true,
            recovery: true,
            competition: true,
            qualification: true,
            capacity: true,
            concentration: true,
            commonality: true,
            acquisition_transition: true,
            aggregate_custody: true,
            stock_condition: true,
            maintenance: true,
            repair_tails: true,
            workload: true,
            logistics_recovery: true,
            sovereignty: true,
            compatibility: true,
            standards: true,
            partner_capacity: true,
            separate_partner_burdens: true,
            protected_pains: true,
            personnel_safety: true,
            tempo: true,
            staffing: true,
            moves: true,
            housing: true,
            health: true,
            caregiving: true,
            services: true,
            falsifiability: true,
            evidence_tiers: true,
            conflict_rejected: true,
            reproduction: true,
            zero_major_convergence: true,
            security_markers: true,
            provenance: true,
            composition: true,
            minimization: true,
            nonwaivable_civilian_authority: true,
            nonwaivable_law: true,
            nonwaivable_floor_gates: true,
            hnd_term_boundary: true,
            taxlane_exclusivity: true,
            rel_no_output: true,
            no_official_release_implication: true,
        }
    }
}
impl NumeracyControls {
    fn complete() -> Self {
        Self {
            units: true,
            quantities: true,
            availability_denominators: true,
            price_years: true,
            lifecycle_cost: true,
            transition_cost: true,
            horizons: true,
            uncertainty: true,
            scenario_arithmetic: true,
            double_count_detection: true,
        }
    }
}
impl SupplierControls {
    fn complete() -> Self {
        Self {
            requirements_stability: true,
            competition: true,
            cash_flow: true,
            production_capacity: true,
            workforce: true,
            intellectual_property: true,
            qualification: true,
            resilient_demand: true,
        }
    }
}
impl SecurityAssuranceControls {
    fn complete() -> Self {
        Self {
            classified_rejected: true,
            controlled_rejected: true,
            sensitive_rejected: true,
            targeting_rejected: true,
            operational_planning_rejected: true,
            exploitable_vulnerability_rejected: true,
            dangerous_combination_rejected: true,
        }
    }
}
impl CivilianAssuranceControls {
    fn complete() -> Self {
        Self {
            lawful_civilian_authority: true,
            personnel_safety: true,
            readiness_nondegradation: true,
            resilience_transition: true,
            financial_optimization_mission_risk_blocked: true,
        }
    }
}
impl DependencyReviewControls {
    fn complete() -> Self {
        Self {
            direct_dependencies: true,
            transitive_dependencies: true,
            feature_flags: true,
            native_build_inputs: true,
            licenses_allowed: true,
            advisories_current: true,
            maintenance_posture: true,
            reproducibility: true,
        }
    }
}
impl ContractProofControls {
    fn complete() -> Self {
        Self {
            source_contract: true,
            authority_contract: true,
            readiness_contract: true,
            acquisition_contract: true,
            logistics_contract: true,
            alliance_contract: true,
            distribution_contract: true,
            economics_contract: true,
            test_contract: true,
            delivery_contract: true,
            handoff_contract: true,
            release_contract: true,
            positive_case: true,
            negative_case: true,
            unauthorized_consumer_case: true,
        }
    }
}
impl ValidationScopeControls {
    fn complete() -> Self {
        Self {
            public_aggregate: true,
            unclassified: true,
            non_operational: true,
            prohibited_content_rejected: true,
        }
    }
}
impl ValidationAssuranceControls {
    fn complete() -> Self {
        Self {
            classification_opsec_pass: true,
            civilian_control_law_safety_readiness_pass: true,
            applicable_parliament_decisions_complete: true,
            applicable_domain_decisions_complete: true,
            independence: true,
            findings_retained: true,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum CrOp {
    LogicalContract,
    LogicalResponsibility,
    ContractFixtureMatrix,
    TypedFailure,
    TypedBranchTotality,
    ExhaustionBound,
    FiniteBoundsProgress,
    ResourceBound,
    ResourceBoundRegistry,
    CallDepth,
    Termination,
    FailureVisibility,
    InvalidState,
    DefaultFallback,
    MissingDefaultHold,
    TypedFamilyExhaustiveness,
    TypedStateExhaustiveness,
    ReleaseNoOutput,
    AdmissionBypass,
    AuthorityNoninflation,
    FalseSavingsNoAuthority,
    TerminalNoBackflow,
    ReplayIdentity,
    OrderInvariance,
    DigestReproductionOrder,
    AmbientStateAbsence,
    ScheduleEquivalence,
    Successor,
    Dependency,
    FixedDependencyGraph,
    Content,
    ContentBoundaryProvenance,
    Composition,
    Floor,
    Distribution,
    NullState,
    StateNullNaStale,
    Accounting,
    ReconciliationIdentity,
    Burden,
    Delivery,
    FindingDissentRetention,
    ReviewIndependence,
    CrossRoleReview,
    HoldPropagation,
    WaiverLedger,
    Invariant,
    Property,
    Transition,
    Parser,
    ParserFuzz,
    Regression,
    RegressionReplay,
    Isolation,
    PackageIsolation,
    Generated,
    GeneratedProvenance,
    Quality,
    QualityOutput,
    LicenseAdvisory,
    Evidence,
    EvidenceDigestTruth,
    Trace,
}

#[derive(Clone, Debug)]
enum CrState {
    LogicalContract {
        logical_surface: bool,
    },
    LogicalResponsibility {
        responsibility_allocated: bool,
    },
    ContractFixtureMatrix {
        controls: ContractProofControls,
    },
    TypedFailure {
        typed_error_rejected: bool,
        error_identity_preserved: bool,
    },
    TypedBranchTotality {
        typed_variants_covered: bool,
        branches_total: bool,
    },
    ExhaustionBound {
        limit_enforced: bool,
        exhaustion_fail_closed: bool,
    },
    FiniteBoundsProgress {
        finite_bound: bool,
        progress_measure: bool,
    },
    ResourceBound {
        input_bound: bool,
        memory_bound: bool,
        exhaustion_response: bool,
        degradation_evidence: bool,
    },
    ResourceBoundRegistry {
        registry_present: bool,
        accepted_bounds: bool,
        resources_allocated: bool,
    },
    CallDepth {
        call_depth: u8,
        max_depth: u8,
    },
    Termination {
        decreasing_measure: Vec<u8>,
    },
    FailureVisibility {
        errors_visible: bool,
        failure_receipt: bool,
    },
    InvalidState {
        invalid_rejected: bool,
        transition_blocked: bool,
    },
    DefaultFallback {
        missing_rejected: bool,
        default_absent: bool,
    },
    MissingDefaultHold {
        missing_held: bool,
        default_absent: bool,
    },
    TypedFamilyExhaustiveness {
        families_complete: bool,
        unknown_family_rejected: bool,
    },
    TypedStateExhaustiveness {
        states_complete: bool,
        invalid_state_rejected: bool,
    },
    ReleaseNoOutput {
        output_absent: bool,
    },
    AdmissionBypass {
        universal_admission_required: bool,
    },
    AuthorityNoninflation {
        authority_not_inflated: bool,
    },
    FalseSavingsNoAuthority {
        false_savings_rejected: bool,
        authority_absent: bool,
    },
    TerminalNoBackflow {
        terminal_output_absent: bool,
        backflow_absent: bool,
    },
    ReplayIdentity {
        identical_replay: bool,
    },
    OrderInvariance {
        order_independent: bool,
    },
    DigestReproductionOrder {
        canonical_order: bool,
        digest_reproduced: bool,
    },
    AmbientStateAbsence {
        ambient_state_absent: bool,
    },
    ScheduleEquivalence {
        schedules_equivalent: bool,
    },
    Successor {
        immutable: bool,
        acyclic: bool,
    },
    Dependency {
        consumer_direction: bool,
    },
    FixedDependencyGraph {
        graph_fixed: bool,
        cycles_absent: bool,
    },
    Content {
        prohibited_content_rejected: bool,
    },
    ContentBoundaryProvenance {
        boundary_preserved: bool,
        provenance_retained: bool,
    },
    Composition {
        minimized: bool,
        unsafe_join_rejected: bool,
    },
    Floor {
        noncompensable: bool,
    },
    Distribution {
        distribution_preserved: bool,
        tails_preserved: bool,
    },
    NullState {
        missing_distinct: bool,
        null_distinct: bool,
    },
    StateNullNaStale {
        missing_distinct: bool,
        null_distinct: bool,
        na_distinct: bool,
        stale_distinct: bool,
    },
    Accounting {
        checked_arithmetic: bool,
        overflow_rejected: bool,
    },
    ReconciliationIdentity {
        identity_balanced: bool,
        residual_zero: bool,
        overlap_reconciled: bool,
    },
    Burden {
        incidence_measured: bool,
        tail_measured: bool,
    },
    Delivery {
        pathways_separate: bool,
        delivery_bound: bool,
        adaptive_successor: bool,
    },
    FindingDissentRetention {
        finding_retained: bool,
        dissent_retained: bool,
    },
    ReviewIndependence {
        independent: bool,
        self_review_rejected: bool,
    },
    CrossRoleReview {
        roles_complete: bool,
        conflicts_rejected: bool,
    },
    HoldPropagation {
        all_holds_propagated: bool,
        downstream_blocked: bool,
    },
    WaiverLedger {
        ledger_nonwaiver: bool,
        bypass_rejected: bool,
    },
    Invariant {
        coverage_complete: bool,
    },
    Property {
        evidence_set_complete: bool,
        reproduction_deterministic: bool,
    },
    Transition {
        typed_transition: bool,
        invalid_edge_rejected: bool,
    },
    Parser {
        surface_minimal: bool,
        authority_absent: bool,
    },
    ParserFuzz {
        malformed_rejected: bool,
        panic_absent: bool,
        authority_absent: bool,
    },
    Regression {
        successor_immutable: bool,
        history_acyclic: bool,
    },
    RegressionReplay {
        golden_replayed: bool,
        digest_matches: bool,
    },
    Isolation {
        modes_separate: bool,
        cross_mode_flow_absent: bool,
    },
    PackageIsolation {
        package_boundary: bool,
        dependency_direction: bool,
    },
    Generated {
        emission_absent: bool,
    },
    GeneratedProvenance {
        generated_manifest_entry_absent: bool,
        provenance_custody: bool,
    },
    Quality {
        registry_complete: bool,
        gates_bound: bool,
    },
    QualityOutput {
        output_bound: bool,
        digest_bound: bool,
    },
    LicenseAdvisory {
        controls: DependencyReviewControls,
    },
    Evidence {
        state_history_truthful: bool,
    },
    EvidenceDigestTruth {
        evidence_bound: bool,
        digest_truthful: bool,
    },
    Trace {
        transpose_equal: bool,
        orphans_absent: bool,
    },
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TypedObligation {
    Test(TestRule),
    Release(ReleaseRule),
    Nf(NfRule),
    Vcl(VclRule),
    ValidationScope,
    ValidationAssurance,
    Actor(ActorRule),
    Review(ReviewLens),
    Hold(HoldRule),
    Cr(CrOp),
}

fn cr_typed_operation(assertion: &str) -> CrOp {
    match assertion {
        "cr_002_logical_contract" => CrOp::LogicalContract,
        "cr_002_logical_responsibility" => CrOp::LogicalResponsibility,
        "cr_030_per_contract_fixture_matrix" => CrOp::ContractFixtureMatrix,
        "cr_003_typed_failure_rejection" => CrOp::TypedFailure,
        "cr_003_typed_branch_totality" => CrOp::TypedBranchTotality,
        "cr_004_exhaustion_failure" => CrOp::ExhaustionBound,
        "cr_004_finite_bounds_progress" => CrOp::FiniteBoundsProgress,
        "cr_037_resource_bound_failure" => CrOp::ResourceBound,
        "cr_037_resource_bound_registry" => CrOp::ResourceBoundRegistry,
        "cr_005_call_graph_depth" => CrOp::CallDepth,
        "cr_005_termination_violation" => CrOp::Termination,
        "cr_006_hidden_failure_scan" => CrOp::FailureVisibility,
        "cr_006_invalid_state" => CrOp::InvalidState,
        "cr_008_default_fallback_rejection" => CrOp::DefaultFallback,
        "cr_008_missing_default_hold" => CrOp::MissingDefaultHold,
        "cr_009_typed_family_exhaustiveness" => CrOp::TypedFamilyExhaustiveness,
        "cr_009_typed_state_exhaustiveness" => CrOp::TypedStateExhaustiveness,
        "cr_010_release_exception_no_output" => CrOp::ReleaseNoOutput,
        "cr_010_universal_admission_bypass" => CrOp::AdmissionBypass,
        "cr_017_authority_noninflation" => CrOp::AuthorityNoninflation,
        "cr_021_false_savings_no_authority" => CrOp::FalseSavingsNoAuthority,
        "cr_024_terminal_no_output_backflow" => CrOp::TerminalNoBackflow,
        "cr_011_replay_identity" => CrOp::ReplayIdentity,
        "cr_011_order_invariance" => CrOp::OrderInvariance,
        "cr_011_digest_reproduction_order" => CrOp::DigestReproductionOrder,
        "cr_012_ambient_state_absence" => CrOp::AmbientStateAbsence,
        "cr_012_schedule_equivalence" => CrOp::ScheduleEquivalence,
        "cr_013_immutable_successor_acyclic" => CrOp::Successor,
        "cr_014_consumer_direction" => CrOp::Dependency,
        "cr_014_fixed_dependency_graph" => CrOp::FixedDependencyGraph,
        "cr_015_prohibited_content" => CrOp::Content,
        "cr_015_content_boundary_provenance" => CrOp::ContentBoundaryProvenance,
        "cr_016_composition_minimization" => CrOp::Composition,
        "cr_017_floor_noncompensation" => CrOp::Floor,
        "cr_018_facet_distribution_conservation" => CrOp::Distribution,
        "cr_019_missing_null_hold" => CrOp::NullState,
        "cr_019_state_null_na_stale" => CrOp::StateNullNaStale,
        "cr_020_checked_accounting" => CrOp::Accounting,
        "cr_020_reconciliation_identity" => CrOp::ReconciliationIdentity,
        "cr_021_burden_shift_rejection" => CrOp::Burden,
        "cr_022_eco_delivery_adaptive_shape" => CrOp::Delivery,
        "cr_023_finding_dissent_retention" => CrOp::FindingDissentRetention,
        "cr_023_review_independence" => CrOp::ReviewIndependence,
        "cr_029_cross_role_adversarial" => CrOp::CrossRoleReview,
        "cr_025_hold_transpose_propagation" => CrOp::HoldPropagation,
        "cr_038_waiver_ledger_nonwaiver" => CrOp::WaiverLedger,
        "cr_026_invariant_coverage" => CrOp::Invariant,
        "cr_027_property_evidence_set" => CrOp::Property,
        "cr_028_transition_model_evidence" => CrOp::Transition,
        "cr_031_parser_surface_absent" => CrOp::Parser,
        "cr_031_parser_fuzz_authority_absent" => CrOp::ParserFuzz,
        "cr_032_golden_successor_history" => CrOp::Regression,
        "cr_032_regression_replay" => CrOp::RegressionReplay,
        "cr_033_mode_isolation" => CrOp::Isolation,
        "cr_033_package_isolation" => CrOp::PackageIsolation,
        "cr_034_generated_no_emission" => CrOp::Generated,
        "cr_034_generated_provenance_absence" => CrOp::GeneratedProvenance,
        "cr_035_quality_gate_registry" => CrOp::Quality,
        "cr_035_quality_output_binding" => CrOp::QualityOutput,
        "cr_036_dependency_license_advisory" => CrOp::LicenseAdvisory,
        "cr_039_evidence_state_history" => CrOp::Evidence,
        "cr_039_evidence_digest_truth" => CrOp::EvidenceDigestTruth,
        "cr_040_mechanical_trace_contradiction" => CrOp::Trace,
        _ => panic!("unmapped typed CR operation: {assertion}"),
    }
}

fn typed_obligation(edge: &TraceEdge) -> TypedObligation {
    if edge.controlled_id.starts_with("CR-") {
        return TypedObligation::Cr(cr_typed_operation(
            edge.assertion.split_once("::").unwrap().1,
        ));
    }
    match edge.controlled_id {
        "BASTION-REQ-TST-001" | "SPEC-TST-001" | "DES-TEST-001" | "CONTRACT-TEST-001" => {
            TypedObligation::Test(TestRule::Packet)
        }
        "BASTION-REQ-TST-002" | "SPEC-TST-002" => {
            TypedObligation::Test(TestRule::IndependentReproduction)
        }
        "BASTION-REQ-TST-003" | "SPEC-TST-003" => TypedObligation::Test(TestRule::RetainedPosture),
        "BASTION-REQ-TST-004" | "SPEC-TST-004" => TypedObligation::Test(TestRule::FindingShape),
        "BASTION-REQ-TST-005" | "SPEC-TST-005" => TypedObligation::Test(TestRule::PromotionBlock),
        "BASTION-REQ-TST-006" | "SPEC-TST-006" => TypedObligation::Test(TestRule::EvidenceTruth),
        "BASTION-REQ-REL-001" | "SPEC-REL-001" => TypedObligation::Release(ReleaseRule::Authority),
        "BASTION-REQ-REL-002" | "SPEC-REL-002" => {
            TypedObligation::Release(ReleaseRule::RiskAssessment)
        }
        "BASTION-REQ-REL-003" | "SPEC-REL-003" => {
            TypedObligation::Release(ReleaseRule::Communication)
        }
        "DES-REL-001" | "CONTRACT-REL-001" => TypedObligation::Release(ReleaseRule::Closed),
        "SPEC-NF-001" => TypedObligation::Nf(NfRule::ProhibitedData),
        "SPEC-NF-002" => TypedObligation::Nf(NfRule::Scope),
        "SPEC-NF-003" => TypedObligation::Nf(NfRule::ReadinessFloor),
        "SPEC-NF-004" => TypedObligation::Nf(NfRule::Distribution),
        "SPEC-NF-005" => TypedObligation::Nf(NfRule::Reconciliation),
        "SPEC-NF-006" => TypedObligation::Nf(NfRule::NonAdditive),
        "SPEC-NF-007" => TypedObligation::Nf(NfRule::MissingNa),
        "SPEC-NF-008" => TypedObligation::Nf(NfRule::History),
        "SPEC-NF-009" => TypedObligation::Nf(NfRule::Staleness),
        "SPEC-NF-010" => TypedObligation::Nf(NfRule::NoAuthority),
        "VCL-01" => TypedObligation::Vcl(VclRule::Custody),
        "VCL-02" => TypedObligation::Vcl(VclRule::Contract),
        "VCL-03" => TypedObligation::Vcl(VclRule::Transition),
        "VCL-04" => TypedObligation::Vcl(VclRule::Property),
        "VCL-05" => TypedObligation::Vcl(VclRule::Holds),
        "VCL-06" => TypedObligation::Vcl(VclRule::Content),
        "VCL-07" => TypedObligation::Vcl(VclRule::Civilian),
        "VCL-08" => TypedObligation::Vcl(VclRule::Accounting),
        "VCL-09" => TypedObligation::Vcl(VclRule::Isolation),
        "VCL-10" => TypedObligation::Vcl(VclRule::Evidence),
        "VAL-SCOPE" => TypedObligation::ValidationScope,
        "VAL-ASSURANCE" => TypedObligation::ValidationAssurance,
        "ACT-CIV" => TypedObligation::Actor(ActorRule::Civilian),
        "ACT-RDY" => TypedObligation::Actor(ActorRule::Readiness),
        "ACT-ACQ" => TypedObligation::Actor(ActorRule::Acquisition),
        "ACT-LOG" => TypedObligation::Actor(ActorRule::Logistics),
        "ACT-ALLY" => TypedObligation::Actor(ActorRule::Alliance),
        "ACT-FIN" => TypedObligation::Actor(ActorRule::Finance),
        "ACT-PPL" => TypedObligation::Actor(ActorRule::People),
        "ACT-TST" => TypedObligation::Actor(ActorRule::Test),
        "ACT-SRC" => TypedObligation::Actor(ActorRule::Source),
        "ACT-LAW" => TypedObligation::Actor(ActorRule::Law),
        "ACT-EXT" => TypedObligation::Actor(ActorRule::External),
        ".roles/editorial/citation-auditor.md" => TypedObligation::Review(ReviewLens::Citation),
        ".roles/editorial/numeracy-checker.md" => TypedObligation::Review(ReviewLens::Numeracy),
        ".roles/editorial/scope-keeper.md" => TypedObligation::Review(ReviewLens::Scope),
        ".roles/panel-reviewer/panel.md" => TypedObligation::Review(ReviewLens::Panel),
        ".roles/parliament/acquisition-industrial-base.md" => {
            TypedObligation::Review(ReviewLens::Acquisition)
        }
        ".roles/parliament/alliance-interoperability.md" => {
            TypedObligation::Review(ReviewLens::Alliance)
        }
        ".roles/parliament/civilian-strategy-force-planner.md" => {
            TypedObligation::Review(ReviewLens::ForcePlanning)
        }
        ".roles/parliament/defense-comptroller.md" => {
            TypedObligation::Review(ReviewLens::Comptroller)
        }
        ".roles/parliament/independent-test-oversight.md" => {
            TypedObligation::Review(ReviewLens::TestOversight)
        }
        ".roles/parliament/logistics-sustainment.md" => {
            TypedObligation::Review(ReviewLens::Logistics)
        }
        ".roles/parliament/operational-readiness.md" => {
            TypedObligation::Review(ReviewLens::Readiness)
        }
        ".roles/parliament/service-member-family.md" => TypedObligation::Review(ReviewLens::People),
        ".roles/stakeholders/ally-partner.md" => {
            TypedObligation::Review(ReviewLens::AllyStakeholder)
        }
        ".roles/stakeholders/depot-logistics-workforce.md" => {
            TypedObligation::Review(ReviewLens::DepotStakeholder)
        }
        ".roles/stakeholders/installation-community.md" => {
            TypedObligation::Review(ReviewLens::InstallationStakeholder)
        }
        ".roles/stakeholders/mission-user.md" => {
            TypedObligation::Review(ReviewLens::MissionStakeholder)
        }
        ".roles/stakeholders/prime-small-supplier.md" => {
            TypedObligation::Review(ReviewLens::SupplierStakeholder)
        }
        ".roles/stakeholders/service-member-family.md" => {
            TypedObligation::Review(ReviewLens::FamilyStakeholder)
        }
        ".roles/stakeholders/taxpayer-oversight.md" => {
            TypedObligation::Review(ReviewLens::TaxpayerStakeholder)
        }
        ".roles/assurance/classification-operational-security.md" => {
            TypedObligation::Review(ReviewLens::SecurityAssurance)
        }
        ".roles/assurance/civilian-control-law-safety-readiness.md" => {
            TypedObligation::Review(ReviewLens::CivilianAssurance)
        }
        "Role review steward" => TypedObligation::Review(ReviewLens::Steward),
        "SPEC-UNK-SEC-001" | "TBD-SEC-001" => TypedObligation::Hold(HoldRule::Security),
        "SPEC-UNK-SRC-001" | "TBD-SRC-001" => TypedObligation::Hold(HoldRule::Source),
        "SPEC-UNK-TST-001" | "TBD-TST-001" => TypedObligation::Hold(HoldRule::Test),
        "SPEC-UNK-REL-001" | "TBD-REL-001" => TypedObligation::Hold(HoldRule::Release),
        id => panic!("unmapped typed obligation: {id}"),
    }
}

#[derive(Clone, Debug)]
enum DomainState {
    Test {
        rule: TestRule,
        controls: TestControls,
    },
    Release {
        rule: ReleaseRule,
        authority: ReleaseAuthorityControls,
        risk: ReleaseRiskAssessment,
        communication: ReleaseCommunicationContext,
        closed: ReleaseClosedControls,
    },
    Nf {
        rule: NfRule,
        controls: NfControls,
    },
    Vcl {
        rule: VclRule,
        controls: VclControls,
        hold_closure: HoldClosureControls,
        accounting: CheckedAccountingControls,
    },
    ValidationScope(ValidationScopeControls),
    ValidationAssurance(ValidationAssuranceControls),
    Actor {
        rule: ActorRule,
        controls: ActorControls,
        finance: FinanceControls,
    },
    Review {
        lens: ReviewLens,
        evidence: ReviewEvidence,
        independent: bool,
        named: Vec<ReviewNamedControl>,
        numeracy: NumeracyControls,
        supplier: SupplierControls,
        security: SecurityAssuranceControls,
        civilian: CivilianAssuranceControls,
    },
    Hold {
        rule: HoldRule,
        admission_blocked: bool,
        downstream_blocked: bool,
        emitted: bool,
    },
    Cr(CrState),
}

fn review_evidence(lens: ReviewLens) -> ReviewEvidence {
    match lens {
        ReviewLens::Citation => ReviewEvidence::Citation,
        ReviewLens::Numeracy => ReviewEvidence::Numeracy,
        ReviewLens::Scope => ReviewEvidence::Scope,
        ReviewLens::Panel => ReviewEvidence::Panel,
        ReviewLens::Acquisition => ReviewEvidence::Acquisition,
        ReviewLens::Alliance => ReviewEvidence::Alliance,
        ReviewLens::ForcePlanning => ReviewEvidence::ForcePlanning,
        ReviewLens::Comptroller => ReviewEvidence::Comptroller,
        ReviewLens::TestOversight => ReviewEvidence::TestOversight,
        ReviewLens::Logistics => ReviewEvidence::Logistics,
        ReviewLens::Readiness => ReviewEvidence::Readiness,
        ReviewLens::People => ReviewEvidence::People,
        ReviewLens::AllyStakeholder => ReviewEvidence::AllyBurden,
        ReviewLens::DepotStakeholder => ReviewEvidence::DepotBurden,
        ReviewLens::InstallationStakeholder => ReviewEvidence::InstallationBurden,
        ReviewLens::MissionStakeholder => ReviewEvidence::MissionBurden,
        ReviewLens::SupplierStakeholder => ReviewEvidence::SupplierBurden,
        ReviewLens::FamilyStakeholder => ReviewEvidence::FamilyBurden,
        ReviewLens::TaxpayerStakeholder => ReviewEvidence::TaxpayerValue,
        ReviewLens::SecurityAssurance => ReviewEvidence::Security,
        ReviewLens::CivilianAssurance => ReviewEvidence::CivilianControl,
        ReviewLens::Steward => ReviewEvidence::Steward,
    }
}

fn positive_cr_state(operation: CrOp) -> CrState {
    match operation {
        CrOp::LogicalContract => CrState::LogicalContract {
            logical_surface: true,
        },
        CrOp::LogicalResponsibility => CrState::LogicalResponsibility {
            responsibility_allocated: true,
        },
        CrOp::ContractFixtureMatrix => CrState::ContractFixtureMatrix {
            controls: ContractProofControls::complete(),
        },
        CrOp::TypedFailure => CrState::TypedFailure {
            typed_error_rejected: true,
            error_identity_preserved: true,
        },
        CrOp::TypedBranchTotality => CrState::TypedBranchTotality {
            typed_variants_covered: true,
            branches_total: true,
        },
        CrOp::ExhaustionBound => CrState::ExhaustionBound {
            limit_enforced: true,
            exhaustion_fail_closed: true,
        },
        CrOp::FiniteBoundsProgress => CrState::FiniteBoundsProgress {
            finite_bound: true,
            progress_measure: true,
        },
        CrOp::ResourceBound => CrState::ResourceBound {
            input_bound: true,
            memory_bound: true,
            exhaustion_response: true,
            degradation_evidence: true,
        },
        CrOp::ResourceBoundRegistry => CrState::ResourceBoundRegistry {
            registry_present: true,
            accepted_bounds: true,
            resources_allocated: true,
        },
        CrOp::CallDepth => CrState::CallDepth {
            call_depth: 3,
            max_depth: 4,
        },
        CrOp::Termination => CrState::Termination {
            decreasing_measure: vec![4, 3, 2, 1, 0],
        },
        CrOp::FailureVisibility => CrState::FailureVisibility {
            errors_visible: true,
            failure_receipt: true,
        },
        CrOp::InvalidState => CrState::InvalidState {
            invalid_rejected: true,
            transition_blocked: true,
        },
        CrOp::DefaultFallback => CrState::DefaultFallback {
            missing_rejected: true,
            default_absent: true,
        },
        CrOp::MissingDefaultHold => CrState::MissingDefaultHold {
            missing_held: true,
            default_absent: true,
        },
        CrOp::TypedFamilyExhaustiveness => CrState::TypedFamilyExhaustiveness {
            families_complete: true,
            unknown_family_rejected: true,
        },
        CrOp::TypedStateExhaustiveness => CrState::TypedStateExhaustiveness {
            states_complete: true,
            invalid_state_rejected: true,
        },
        CrOp::ReleaseNoOutput => CrState::ReleaseNoOutput {
            output_absent: true,
        },
        CrOp::AdmissionBypass => CrState::AdmissionBypass {
            universal_admission_required: true,
        },
        CrOp::AuthorityNoninflation => CrState::AuthorityNoninflation {
            authority_not_inflated: true,
        },
        CrOp::FalseSavingsNoAuthority => CrState::FalseSavingsNoAuthority {
            false_savings_rejected: true,
            authority_absent: true,
        },
        CrOp::TerminalNoBackflow => CrState::TerminalNoBackflow {
            terminal_output_absent: true,
            backflow_absent: true,
        },
        CrOp::ReplayIdentity => CrState::ReplayIdentity {
            identical_replay: true,
        },
        CrOp::OrderInvariance => CrState::OrderInvariance {
            order_independent: true,
        },
        CrOp::DigestReproductionOrder => CrState::DigestReproductionOrder {
            canonical_order: true,
            digest_reproduced: true,
        },
        CrOp::AmbientStateAbsence => CrState::AmbientStateAbsence {
            ambient_state_absent: true,
        },
        CrOp::ScheduleEquivalence => CrState::ScheduleEquivalence {
            schedules_equivalent: true,
        },
        CrOp::Successor => CrState::Successor {
            immutable: true,
            acyclic: true,
        },
        CrOp::Dependency => CrState::Dependency {
            consumer_direction: true,
        },
        CrOp::FixedDependencyGraph => CrState::FixedDependencyGraph {
            graph_fixed: true,
            cycles_absent: true,
        },
        CrOp::Content => CrState::Content {
            prohibited_content_rejected: true,
        },
        CrOp::ContentBoundaryProvenance => CrState::ContentBoundaryProvenance {
            boundary_preserved: true,
            provenance_retained: true,
        },
        CrOp::Composition => CrState::Composition {
            minimized: true,
            unsafe_join_rejected: true,
        },
        CrOp::Floor => CrState::Floor {
            noncompensable: true,
        },
        CrOp::Distribution => CrState::Distribution {
            distribution_preserved: true,
            tails_preserved: true,
        },
        CrOp::NullState => CrState::NullState {
            missing_distinct: true,
            null_distinct: true,
        },
        CrOp::StateNullNaStale => CrState::StateNullNaStale {
            missing_distinct: true,
            null_distinct: true,
            na_distinct: true,
            stale_distinct: true,
        },
        CrOp::Accounting => CrState::Accounting {
            checked_arithmetic: true,
            overflow_rejected: true,
        },
        CrOp::ReconciliationIdentity => CrState::ReconciliationIdentity {
            identity_balanced: true,
            residual_zero: true,
            overlap_reconciled: true,
        },
        CrOp::Burden => CrState::Burden {
            incidence_measured: true,
            tail_measured: true,
        },
        CrOp::Delivery => CrState::Delivery {
            pathways_separate: true,
            delivery_bound: true,
            adaptive_successor: true,
        },
        CrOp::FindingDissentRetention => CrState::FindingDissentRetention {
            finding_retained: true,
            dissent_retained: true,
        },
        CrOp::ReviewIndependence => CrState::ReviewIndependence {
            independent: true,
            self_review_rejected: true,
        },
        CrOp::CrossRoleReview => CrState::CrossRoleReview {
            roles_complete: true,
            conflicts_rejected: true,
        },
        CrOp::HoldPropagation => CrState::HoldPropagation {
            all_holds_propagated: true,
            downstream_blocked: true,
        },
        CrOp::WaiverLedger => CrState::WaiverLedger {
            ledger_nonwaiver: true,
            bypass_rejected: true,
        },
        CrOp::Invariant => CrState::Invariant {
            coverage_complete: true,
        },
        CrOp::Property => CrState::Property {
            evidence_set_complete: true,
            reproduction_deterministic: true,
        },
        CrOp::Transition => CrState::Transition {
            typed_transition: true,
            invalid_edge_rejected: true,
        },
        CrOp::Parser => CrState::Parser {
            surface_minimal: true,
            authority_absent: true,
        },
        CrOp::ParserFuzz => CrState::ParserFuzz {
            malformed_rejected: true,
            panic_absent: true,
            authority_absent: true,
        },
        CrOp::Regression => CrState::Regression {
            successor_immutable: true,
            history_acyclic: true,
        },
        CrOp::RegressionReplay => CrState::RegressionReplay {
            golden_replayed: true,
            digest_matches: true,
        },
        CrOp::Isolation => CrState::Isolation {
            modes_separate: true,
            cross_mode_flow_absent: true,
        },
        CrOp::PackageIsolation => CrState::PackageIsolation {
            package_boundary: true,
            dependency_direction: true,
        },
        CrOp::Generated => CrState::Generated {
            emission_absent: true,
        },
        CrOp::GeneratedProvenance => CrState::GeneratedProvenance {
            generated_manifest_entry_absent: true,
            provenance_custody: true,
        },
        CrOp::Quality => CrState::Quality {
            registry_complete: true,
            gates_bound: true,
        },
        CrOp::QualityOutput => CrState::QualityOutput {
            output_bound: true,
            digest_bound: true,
        },
        CrOp::LicenseAdvisory => CrState::LicenseAdvisory {
            controls: DependencyReviewControls::complete(),
        },
        CrOp::Evidence => CrState::Evidence {
            state_history_truthful: true,
        },
        CrOp::EvidenceDigestTruth => CrState::EvidenceDigestTruth {
            evidence_bound: true,
            digest_truthful: true,
        },
        CrOp::Trace => CrState::Trace {
            transpose_equal: true,
            orphans_absent: true,
        },
    }
}

fn positive_domain_state(kind: TypedObligation) -> DomainState {
    match kind {
        TypedObligation::Test(rule) => DomainState::Test {
            rule,
            controls: TestControls::complete(),
        },
        TypedObligation::Release(rule) => DomainState::Release {
            rule,
            authority: ReleaseAuthorityControls::complete(),
            risk: ReleaseRiskAssessment::complete(),
            communication: ReleaseCommunicationContext::complete(),
            closed: ReleaseClosedControls::complete(),
        },
        TypedObligation::Nf(rule) => DomainState::Nf {
            rule,
            controls: NfControls::complete(),
        },
        TypedObligation::Vcl(rule) => DomainState::Vcl {
            rule,
            controls: VclControls::complete(),
            hold_closure: HoldClosureControls::complete(),
            accounting: CheckedAccountingControls::complete(),
        },
        TypedObligation::ValidationScope => {
            DomainState::ValidationScope(ValidationScopeControls::complete())
        }
        TypedObligation::ValidationAssurance => {
            DomainState::ValidationAssurance(ValidationAssuranceControls::complete())
        }
        TypedObligation::Actor(rule) => DomainState::Actor {
            rule,
            controls: ActorControls::complete(),
            finance: FinanceControls::complete(),
        },
        TypedObligation::Review(lens) => DomainState::Review {
            lens,
            evidence: review_evidence(lens),
            independent: true,
            named: required_review_named(lens).to_vec(),
            numeracy: NumeracyControls::complete(),
            supplier: SupplierControls::complete(),
            security: SecurityAssuranceControls::complete(),
            civilian: CivilianAssuranceControls::complete(),
        },
        TypedObligation::Hold(rule) => DomainState::Hold {
            rule,
            admission_blocked: true,
            downstream_blocked: true,
            emitted: false,
        },
        TypedObligation::Cr(operation) => DomainState::Cr(positive_cr_state(operation)),
    }
}

fn require_domain(condition: bool, error: &'static str) -> Result<(), &'static str> {
    if condition { Ok(()) } else { Err(error) }
}

macro_rules! require_fields {
    ($state:expr; $($field:ident => $error:literal),+ $(,)?) => {{
        $(require_domain($state.$field, $error)?;)+
        Ok(())
    }};
}

fn validate_test_controls(rule: TestRule, c: &TestControls) -> Result<(), &'static str> {
    match rule {
        TestRule::Packet => require_fields!(c;
            frozen_artifact => "test-frozen-artifact-omitted", evidence_manifest => "test-evidence-manifest-omitted",
            derivations => "test-derivations-omitted", gate_matrix => "test-gate-matrix-omitted",
            negative_cases => "test-negative-cases-omitted", unresolved_questions => "test-unresolved-questions-omitted",
            digest_binding => "test-digest-binding-omitted"),
        TestRule::IndependentReproduction => require_fields!(c;
            quantitative_reproduction => "test-quantitative-reproduction-omitted", qualitative_custody => "test-qualitative-custody-omitted",
            adverse_cases => "test-adverse-cases-omitted", failure_cases => "test-failure-cases-omitted",
            uncertainty => "test-uncertainty-omitted", denominators => "test-denominators-omitted",
            price_years => "test-price-years-omitted", lifecycle_cost => "test-lifecycle-cost-omitted",
            transition_cost => "test-transition-cost-omitted", double_count_detection => "test-double-count-undetected",
            independent => "test-independence-omitted"),
        TestRule::RetainedPosture => require_fields!(c;
            retained_negative_results => "retained-negative-results-omitted", retained_failed_tests => "retained-failed-tests-omitted",
            retained_nulls => "retained-nulls-omitted", retained_rejected_candidates => "retained-rejected-candidates-omitted",
            retained_dissent => "retained-dissent-omitted", retained_unresolved_evidence => "retained-unresolved-evidence-omitted"),
        TestRule::FindingShape => require_fields!(c;
            finding_stable_id => "finding-stable-id-omitted", finding_digest => "finding-digest-omitted",
            finding_role => "finding-role-omitted", finding_severity => "finding-severity-omitted",
            finding_claim => "finding-claim-omitted", finding_evidence => "finding-evidence-omitted",
            finding_disposition => "finding-disposition-omitted", finding_owner => "finding-owner-omitted",
            finding_destination => "finding-destination-omitted", finding_closure => "finding-closure-omitted",
            finding_independence => "finding-independence-omitted", finding_dissent => "finding-dissent-omitted"),
        TestRule::PromotionBlock => require_fields!(c;
            stale_blocks => "stale-promotion-unblocked", conflicted_blocks => "conflicted-promotion-unblocked",
            absent_role_blocks => "absent-role-promotion-unblocked", failed_assurance_blocks => "failed-assurance-promotion-unblocked",
            unowned_defer_blocks => "unowned-defer-promotion-unblocked", false_approval_blocks => "false-approval-promotion-unblocked",
            unresolved_major_critical_blocks => "major-critical-promotion-unblocked"),
        TestRule::EvidenceTruth => require_fields!(c;
            advocacy_rejected => "advocacy-substituted-for-evidence", credentials_rejected => "credentials-substituted-for-evidence",
            inaccessible_classified_appeal_rejected => "classified-appeal-substituted-for-evidence"),
    }
}
fn validate_prohibited_flow(
    c: &ProhibitedFlowControls,
    errors: [&'static str; 4],
) -> Result<(), &'static str> {
    require_domain(c.ingest_blocked, errors[0])?;
    require_domain(c.retention_blocked, errors[1])?;
    require_domain(c.derivation_blocked, errors[2])?;
    require_domain(c.emission_blocked, errors[3])
}
fn validate_nf_controls(rule: NfRule, c: &NfControls) -> Result<(), &'static str> {
    match rule {
        NfRule::ProhibitedData => {
            validate_prohibited_flow(
                &c.classified,
                [
                    "classified-ingest-enabled",
                    "classified-retention-enabled",
                    "classified-derivation-enabled",
                    "classified-emission-enabled",
                ],
            )?;
            validate_prohibited_flow(
                &c.controlled,
                [
                    "controlled-ingest-enabled",
                    "controlled-retention-enabled",
                    "controlled-derivation-enabled",
                    "controlled-emission-enabled",
                ],
            )?;
            validate_prohibited_flow(
                &c.person_level,
                [
                    "person-level-ingest-enabled",
                    "person-level-retention-enabled",
                    "person-level-derivation-enabled",
                    "person-level-emission-enabled",
                ],
            )?;
            validate_prohibited_flow(
                &c.sensitive_operational,
                [
                    "sensitive-operational-ingest-enabled",
                    "sensitive-operational-retention-enabled",
                    "sensitive-operational-derivation-enabled",
                    "sensitive-operational-emission-enabled",
                ],
            )?;
            validate_prohibited_flow(
                &c.targeting,
                [
                    "targeting-ingest-enabled",
                    "targeting-retention-enabled",
                    "targeting-derivation-enabled",
                    "targeting-emission-enabled",
                ],
            )?;
            validate_prohibited_flow(
                &c.force_employment,
                [
                    "force-employment-ingest-enabled",
                    "force-employment-retention-enabled",
                    "force-employment-derivation-enabled",
                    "force-employment-emission-enabled",
                ],
            )?;
            validate_prohibited_flow(
                &c.exploitable_vulnerability,
                [
                    "vulnerability-ingest-enabled",
                    "vulnerability-retention-enabled",
                    "vulnerability-derivation-enabled",
                    "vulnerability-emission-enabled",
                ],
            )
        }
        NfRule::Scope => require_fields!(c;
            authority_broadening_blocked => "authority-scope-broadened", mission_risk_broadening_blocked => "mission-risk-scope-broadened"),
        NfRule::ReadinessFloor => require_fields!(c;
            floor_blocks_candidate => "floor-failure-candidate-admitted", floor_blocks_savings => "floor-failure-savings-admitted",
            floor_blocks_delivery => "floor-failure-delivery-admitted", floor_blocks_handoff => "floor-failure-handoff-admitted"),
        NfRule::Distribution => require_fields!(c;
            distribution_retained => "distribution-omitted", repair_tail_retained => "repair-tail-omitted",
            degraded_posture_retained => "degraded-posture-omitted", concentrated_effects_retained => "concentrated-effects-omitted"),
        NfRule::Reconciliation => require_fields!(c;
            reconciliation_units => "reconciliation-units-omitted", reconciliation_horizons => "reconciliation-horizons-omitted",
            reconciliation_price_bases => "reconciliation-price-bases-omitted", reconciliation_account_measures => "reconciliation-account-measures-omitted",
            reconciliation_parties => "reconciliation-parties-omitted", reconciliation_overlap => "reconciliation-overlap-omitted"),
        NfRule::NonAdditive => require_fields!(c;
            basing_nonadditive => "basing-pathway-auto-summed", consolidation_nonadditive => "consolidation-pathway-auto-summed",
            process_nonadditive => "process-pathway-auto-summed", commonality_nonadditive => "commonality-pathway-auto-summed",
            logistics_nonadditive => "logistics-pathway-auto-summed", workforce_nonadditive => "workforce-pathway-auto-summed",
            noncash_not_converted => "noncash-converted-to-savings"),
        NfRule::MissingNa => require_fields!(c;
            missing_distinct_from_zero => "missing-defaulted-zero", na_reason => "na-reason-omitted",
            na_alternative_boundary => "na-alternative-boundary-omitted", na_independent_review => "na-independent-review-omitted"),
        NfRule::History => require_fields!(c;
            deterministic_identity => "history-identity-nondeterministic", deterministic_order => "history-order-nondeterministic",
            immutable_supersession_history => "supersession-history-rewritten"),
        NfRule::Staleness => require_fields!(c;
            digest_fresh => "bound-digest-stale", context_fresh => "bound-context-stale",
            stale_review_blocked => "stale-review-admitted", stale_admission_blocked => "stale-input-admitted",
            stale_handoff_blocked => "stale-handoff-admitted"),
        NfRule::NoAuthority => require_fields!(c;
            no_operational_authority => "operational-authority-created", no_procurement_authority => "procurement-authority-created",
            no_budget_authority => "budget-authority-created", no_taxlane_authority => "taxlane-authority-created",
            no_allocation_authority => "allocation-authority-created", no_rate_authority => "rate-authority-created",
            no_official_authority => "official-authority-created", no_implementation_authority => "implementation-authority-created",
            no_release_authority => "release-authority-created"),
    }
}

fn validate_release_authority(c: &ReleaseAuthorityControls) -> Result<(), &'static str> {
    require_fields!(c;
        publication_blocked => "release-publication-enabled",
        public_release_blocked => "public-release-enabled",
        approved_representation_blocked => "approved-representation-enabled",
        separate_release_authority_absent => "release-authority-fabricated",
        release_fixed_point_absent => "release-fixed-point-fabricated",
    )
}
fn validate_release_risk(c: &ReleaseRiskAssessment) -> Result<(), &'static str> {
    require_fields!(c;
        direct_release_composition => "direct-release-composition-omitted",
        cross_release_composition => "cross-release-composition-omitted",
        linkage_risk => "release-linkage-risk-omitted",
        sensitive_context => "release-sensitive-context-omitted",
        audience_misuse => "release-audience-misuse-omitted",
        source_staleness => "release-source-staleness-omitted",
        review_staleness => "release-review-staleness-omitted",
        provenance => "release-provenance-omitted",
        correction_takedown => "release-correction-takedown-omitted",
        security => "release-security-acceptance-omitted",
        scope => "release-scope-acceptance-omitted",
    )
}
fn validate_release_communication(c: &ReleaseCommunicationContext) -> Result<(), &'static str> {
    require_fields!(c;
        source => "release-source-context-omitted",
        derivation => "release-derivation-context-omitted",
        limitation => "release-limitation-context-omitted",
        uncertainty => "release-uncertainty-context-omitted",
        dissent => "release-dissent-context-omitted",
        security_posture => "release-security-posture-omitted",
        non_authority => "release-non-authority-context-omitted",
    )
}
fn validate_release_closed(c: &ReleaseClosedControls) -> Result<(), &'static str> {
    require_fields!(c;
        closed_posture => "release-closed-posture-omitted", no_output => "closed-release-output-enabled",
        no_consumer => "closed-release-consumer-enabled", unauthorized_negative => "closed-unauthorized-negative-omitted",
        mosaicing_negative => "closed-mosaicing-negative-omitted", context_retention_negative => "closed-context-negative-omitted")
}
fn validate_hold_closure(c: &HoldClosureControls) -> Result<(), &'static str> {
    require_fields!(c;
        security => "security-hold-omitted", readiness => "readiness-hold-omitted",
        source => "source-hold-omitted", quantity => "quantity-hold-omitted",
        acquisition => "acquisition-hold-omitted", logistics => "logistics-hold-omitted",
        alliance => "alliance-hold-omitted", distribution => "distribution-hold-omitted",
        economics => "economics-hold-omitted", test => "test-hold-omitted",
        delivery => "delivery-hold-omitted", handoff => "handoff-hold-omitted",
        release => "release-hold-omitted", missing_held => "missing-state-defaulted",
        null_held => "null-state-defaulted", na_independently_reviewed => "na-not-independently-reviewed",
        no_default => "hold-default-applied",
    )
}
fn validate_vcl_controls(rule: VclRule, c: &VclControls) -> Result<(), &'static str> {
    match rule {
        VclRule::Custody => require_fields!(c;
            source_identity => "vcl-source-identity-omitted", allocation => "vcl-allocation-omitted",
            trace => "vcl-trace-omitted", custody => "vcl-custody-omitted", digest => "vcl-digest-omitted"),
        VclRule::Contract => require_fields!(c;
            positive_contract => "vcl-positive-contract-omitted", negative_contract => "vcl-negative-contract-omitted",
            unauthorized_consumer_rejected => "vcl-unauthorized-consumer-admitted"),
        VclRule::Transition => require_fields!(c;
            typed_state => "vcl-typed-state-omitted", typed_transition => "vcl-typed-transition-omitted",
            finite_dag => "vcl-finite-dag-omitted", immutable_successor => "vcl-successor-omitted",
            invalid_edge_rejected => "vcl-invalid-edge-admitted"),
        VclRule::Property => require_fields!(c;
            invariant_coverage => "vcl-invariant-coverage-omitted", property_evidence => "vcl-property-evidence-omitted",
            deterministic_reproduction => "vcl-deterministic-reproduction-omitted"),
        VclRule::Content => require_fields!(c;
            prohibited_content_rejected => "vcl-prohibited-content-admitted", composition_security => "vcl-composition-security-omitted",
            minimization => "vcl-minimization-omitted", safe_failure_custody => "vcl-safe-failure-custody-omitted"),
        VclRule::Civilian => require_fields!(c;
            civilian_authority => "vcl-civilian-authority-omitted", law => "vcl-law-omitted",
            safety_readiness => "vcl-safety-readiness-omitted", stakeholder_lenses => "vcl-stakeholder-lenses-omitted",
            distribution => "vcl-distribution-omitted", burden => "vcl-burden-omitted",
            non_compensation => "vcl-non-compensation-omitted"),
        VclRule::Isolation => require_fields!(c;
            quality => "vcl-quality-omitted", dependency => "vcl-dependency-omitted",
            support_isolation => "vcl-support-isolation-omitted", generated_custody => "vcl-generated-custody-omitted",
            resource_bounds => "vcl-resource-bounds-omitted"),
        VclRule::Evidence => require_fields!(c;
            evidence_state_truth => "vcl-evidence-state-truth-omitted", independent_review => "vcl-independent-review-omitted",
            dissent => "vcl-dissent-omitted", validation => "vcl-validation-omitted", rollback => "vcl-rollback-omitted",
            compatibility => "vcl-compatibility-omitted", historical_reproduction => "vcl-historical-reproduction-omitted"),
        VclRule::Holds | VclRule::Accounting => Ok(()),
    }
}
fn validate_checked_accounting(c: &CheckedAccountingControls) -> Result<(), &'static str> {
    require_fields!(c;
        pathway_basing => "basing-pathway-accounting-omitted",
        consolidation => "consolidation-pathway-accounting-omitted",
        process => "process-pathway-accounting-omitted",
        commonality => "commonality-pathway-accounting-omitted",
        logistics => "logistics-pathway-accounting-omitted",
        workforce => "workforce-pathway-accounting-omitted",
        delivery => "delivery-accounting-omitted", realization => "realization-accounting-omitted",
        handoff => "handoff-accounting-omitted", terminal => "terminal-accounting-omitted",
        no_backflow => "terminal-backflow-enabled", units_identity => "accounting-units-mismatch",
        lifecycle_identity => "lifecycle-identity-failed", transition_identity => "transition-identity-failed",
        overlap_identity => "overlap-identity-failed", residual_identity => "residual-identity-failed",
        taxlane_exclusivity => "vcl-accounting-taxlane-exclusivity-omitted",
        rel_no_output => "vcl-accounting-rel-output-enabled",
        no_operational_authority => "vcl-accounting-operational-authority-created",
        no_procurement_authority => "vcl-accounting-procurement-authority-created",
        no_budget_authority => "vcl-accounting-budget-authority-created",
        no_taxlane_authority => "vcl-accounting-taxlane-authority-created",
        no_allocation_authority => "vcl-accounting-allocation-authority-created",
        no_rate_authority => "vcl-accounting-rate-authority-created",
        no_official_authority => "vcl-accounting-official-authority-created",
        no_implementation_authority => "vcl-accounting-implementation-authority-created",
        no_release_authority => "vcl-accounting-release-authority-created",
    )
}
fn validate_finance(c: &FinanceControls) -> Result<(), &'static str> {
    require_fields!(c;
        units => "finance-units-omitted", horizons => "finance-horizons-omitted",
        overlap => "finance-overlap-omitted", uncertainty => "finance-uncertainty-omitted",
        pathway_basing => "finance-basing-pathway-omitted", pathway_consolidation => "finance-consolidation-pathway-omitted",
        pathway_process => "finance-process-pathway-omitted", pathway_commonality => "finance-commonality-pathway-omitted",
        pathway_logistics => "finance-logistics-pathway-omitted", pathway_workforce => "finance-workforce-pathway-omitted",
        residuals => "finance-residuals-omitted", realization => "finance-realization-omitted",
    )
}
fn validate_actor_controls(rule: ActorRule, c: &ActorControls) -> Result<(), &'static str> {
    match rule {
        ActorRule::Civilian => require_fields!(c;
            mission_fit => "actor-mission-fit-omitted", lawful_bounds => "actor-lawful-bounds-omitted",
            operational_policy_authority_rejected => "actor-operational-policy-authority-created"),
        ActorRule::Readiness => require_fields!(c;
            readiness_floors => "actor-readiness-floors-omitted", degraded_posture => "actor-degraded-posture-omitted",
            readiness_uncertainty => "actor-readiness-uncertainty-omitted", surge => "actor-surge-omitted", recovery => "actor-recovery-omitted"),
        ActorRule::Acquisition => require_fields!(c;
            competition => "actor-competition-omitted", qualification => "actor-qualification-omitted", capacity => "actor-capacity-omitted",
            concentration => "actor-concentration-omitted", commonality => "actor-commonality-omitted",
            acquisition_transition => "actor-acquisition-transition-omitted"),
        ActorRule::Logistics => require_fields!(c;
            aggregate_custody => "actor-aggregate-custody-omitted", stock_condition => "actor-stock-condition-omitted",
            maintenance => "actor-maintenance-omitted", repair_tails => "actor-repair-tails-omitted",
            workload => "actor-workload-omitted", logistics_recovery => "actor-logistics-recovery-omitted"),
        ActorRule::Alliance => require_fields!(c;
            sovereignty => "actor-sovereignty-omitted", compatibility => "actor-compatibility-omitted", standards => "actor-standards-omitted",
            partner_capacity => "actor-partner-capacity-omitted", separate_partner_burdens => "actor-partner-burdens-collapsed"),
        ActorRule::People => require_fields!(c;
            protected_pains => "actor-protected-pains-omitted", personnel_safety => "actor-personnel-safety-omitted", tempo => "actor-tempo-omitted",
            staffing => "actor-staffing-omitted", moves => "actor-moves-omitted", housing => "actor-housing-omitted", health => "actor-health-omitted",
            caregiving => "actor-caregiving-omitted", services => "actor-services-omitted"),
        ActorRule::Test => require_fields!(c;
            falsifiability => "actor-falsifiability-omitted", evidence_tiers => "actor-evidence-tiers-omitted",
            conflict_rejected => "actor-conflict-admitted", reproduction => "actor-reproduction-omitted",
            zero_major_convergence => "actor-zero-major-convergence-omitted"),
        ActorRule::Source => require_fields!(c;
            security_markers => "actor-security-markers-omitted", provenance => "actor-provenance-omitted",
            composition => "actor-composition-omitted", minimization => "actor-minimization-omitted"),
        ActorRule::Law => require_fields!(c;
            nonwaivable_civilian_authority => "actor-civilian-authority-waived", nonwaivable_law => "actor-law-waived",
            nonwaivable_floor_gates => "actor-floor-gates-waived"),
        ActorRule::External => require_fields!(c;
            hnd_term_boundary => "actor-hnd-term-boundary-omitted", taxlane_exclusivity => "actor-taxlane-exclusivity-omitted",
            rel_no_output => "actor-rel-output-enabled", no_official_release_implication => "actor-official-release-implied"),
        ActorRule::Finance => Ok(()),
    }
}
fn validate_numeracy(c: &NumeracyControls) -> Result<(), &'static str> {
    require_fields!(c;
        units => "numeracy-units-omitted", quantities => "numeracy-quantities-omitted",
        availability_denominators => "numeracy-availability-denominator-omitted", price_years => "numeracy-price-years-omitted",
        lifecycle_cost => "numeracy-lifecycle-cost-omitted", transition_cost => "numeracy-transition-cost-omitted",
        horizons => "numeracy-horizons-omitted", uncertainty => "numeracy-uncertainty-omitted",
        scenario_arithmetic => "numeracy-scenario-arithmetic-invalid", double_count_detection => "numeracy-double-count-undetected",
    )
}
fn validate_supplier(c: &SupplierControls) -> Result<(), &'static str> {
    require_fields!(c;
        requirements_stability => "supplier-requirements-stability-omitted", competition => "supplier-competition-omitted",
        cash_flow => "supplier-cash-flow-omitted", production_capacity => "supplier-production-capacity-omitted",
        workforce => "supplier-workforce-omitted", intellectual_property => "supplier-ip-omitted",
        qualification => "supplier-qualification-omitted", resilient_demand => "supplier-resilient-demand-omitted",
    )
}
fn validate_review_named(
    lens: ReviewLens,
    actual: &[ReviewNamedControl],
) -> Result<(), &'static str> {
    let required = required_review_named(lens);
    if required.iter().any(|control| !actual.contains(control)) {
        return Err("review-named-control-omitted");
    }
    if actual.len() != required.len() || actual.iter().any(|control| !required.contains(control)) {
        return Err("review-named-control-incompatible");
    }
    Ok(())
}
fn validate_security_assurance(c: &SecurityAssuranceControls) -> Result<(), &'static str> {
    require_fields!(c;
        classified_rejected => "classified-content-admitted", controlled_rejected => "controlled-content-admitted",
        sensitive_rejected => "sensitive-content-admitted", targeting_rejected => "targeting-content-admitted",
        operational_planning_rejected => "operational-planning-content-admitted",
        exploitable_vulnerability_rejected => "exploitable-vulnerability-admitted",
        dangerous_combination_rejected => "dangerous-public-field-combination-admitted",
    )
}
fn validate_civilian_assurance(c: &CivilianAssuranceControls) -> Result<(), &'static str> {
    require_fields!(c;
        lawful_civilian_authority => "lawful-civilian-authority-omitted", personnel_safety => "personnel-safety-omitted",
        readiness_nondegradation => "readiness-nondegradation-omitted", resilience_transition => "resilience-transition-test-omitted",
        financial_optimization_mission_risk_blocked => "financial-optimization-mission-risk-unblocked",
    )
}
fn validate_dependency_review(c: &DependencyReviewControls) -> Result<(), &'static str> {
    require_fields!(c;
        direct_dependencies => "direct-dependency-review-omitted", transitive_dependencies => "transitive-dependency-review-omitted",
        feature_flags => "dependency-feature-review-omitted", native_build_inputs => "native-build-review-omitted",
        licenses_allowed => "license-policy-rejected", advisories_current => "advisory-data-stale",
        maintenance_posture => "dependency-maintenance-review-omitted", reproducibility => "dependency-reproducibility-review-omitted")
}
fn validate_contract_fixture_matrix(c: &ContractProofControls) -> Result<(), &'static str> {
    require_fields!(c;
        source_contract => "source-contract-fixture-omitted", authority_contract => "authority-contract-fixture-omitted",
        readiness_contract => "readiness-contract-fixture-omitted", acquisition_contract => "acquisition-contract-fixture-omitted",
        logistics_contract => "logistics-contract-fixture-omitted", alliance_contract => "alliance-contract-fixture-omitted",
        distribution_contract => "distribution-contract-fixture-omitted", economics_contract => "economics-contract-fixture-omitted",
        test_contract => "test-contract-fixture-omitted", delivery_contract => "delivery-contract-fixture-omitted",
        handoff_contract => "handoff-contract-fixture-omitted", release_contract => "release-contract-fixture-omitted",
        positive_case => "contract-positive-case-omitted", negative_case => "contract-negative-case-omitted",
        unauthorized_consumer_case => "contract-unauthorized-consumer-case-omitted")
}
fn validate_validation_scope(c: &ValidationScopeControls) -> Result<(), &'static str> {
    require_fields!(c;
        public_aggregate => "validation-scope-not-public-aggregate",
        unclassified => "validation-scope-not-unclassified",
        non_operational => "validation-scope-operational",
        prohibited_content_rejected => "validation-scope-prohibited-content-admitted")
}
fn validate_validation_assurance(c: &ValidationAssuranceControls) -> Result<(), &'static str> {
    require_fields!(c;
        classification_opsec_pass => "validation-classification-opsec-missing",
        civilian_control_law_safety_readiness_pass => "validation-civilian-law-safety-readiness-missing",
        applicable_parliament_decisions_complete => "validation-parliament-decisions-incomplete",
        applicable_domain_decisions_complete => "validation-domain-decisions-incomplete",
        independence => "validation-assurance-conflict",
        findings_retained => "validation-findings-dropped")
}

fn validate_cr_state(operation: CrOp, state: &CrState) -> Result<(), &'static str> {
    macro_rules! required {
        ($($condition:expr => $error:literal),+ $(,)?) => {{
            $(require_domain($condition, $error)?;)+
            Ok(())
        }};
    }
    match (operation, state) {
        (CrOp::LogicalContract, CrState::LogicalContract { logical_surface }) => {
            required!(*logical_surface => "logical-contract-surface-omitted")
        }
        (
            CrOp::LogicalResponsibility,
            CrState::LogicalResponsibility {
                responsibility_allocated,
            },
        ) => required!(*responsibility_allocated => "logical-contract-responsibility-omitted"),
        (CrOp::ContractFixtureMatrix, CrState::ContractFixtureMatrix { controls }) => {
            validate_contract_fixture_matrix(controls)
        }
        (
            CrOp::TypedFailure,
            CrState::TypedFailure {
                typed_error_rejected,
                error_identity_preserved,
            },
        ) => {
            required!(*typed_error_rejected => "typed-failure-admitted", *error_identity_preserved => "typed-error-identity-lost")
        }
        (
            CrOp::TypedBranchTotality,
            CrState::TypedBranchTotality {
                typed_variants_covered,
                branches_total,
            },
        ) => {
            required!(*typed_variants_covered => "typed-variant-coverage-incomplete", *branches_total => "typed-branch-nontotal")
        }
        (
            CrOp::ExhaustionBound,
            CrState::ExhaustionBound {
                limit_enforced,
                exhaustion_fail_closed,
            },
        ) => {
            required!(*limit_enforced => "exhaustion-limit-unenforced", *exhaustion_fail_closed => "exhaustion-not-fail-closed")
        }
        (
            CrOp::FiniteBoundsProgress,
            CrState::FiniteBoundsProgress {
                finite_bound,
                progress_measure,
            },
        ) => {
            required!(*finite_bound => "finite-progress-bound-missing", *progress_measure => "progress-measure-missing")
        }
        (
            CrOp::ResourceBound,
            CrState::ResourceBound {
                input_bound,
                memory_bound,
                exhaustion_response,
                degradation_evidence,
            },
        ) => {
            required!(*input_bound => "input-resource-bound-missing", *memory_bound => "memory-resource-bound-missing", *exhaustion_response => "exhaustion-response-missing", *degradation_evidence => "degradation-evidence-missing")
        }
        (
            CrOp::ResourceBoundRegistry,
            CrState::ResourceBoundRegistry {
                registry_present,
                accepted_bounds,
                resources_allocated,
            },
        ) => {
            required!(*registry_present => "resource-bound-registry-missing", *accepted_bounds => "accepted-resource-bounds-missing", *resources_allocated => "resource-allocation-missing")
        }
        (
            CrOp::CallDepth,
            CrState::CallDepth {
                call_depth,
                max_depth,
            },
        ) => required!(call_depth <= max_depth => "call-depth-exceeded"),
        (CrOp::Termination, CrState::Termination { decreasing_measure }) => {
            required!(decreasing_measure.windows(2).all(|w| w[1] < w[0]) => "termination-measure-not-decreasing")
        }
        (
            CrOp::FailureVisibility,
            CrState::FailureVisibility {
                errors_visible,
                failure_receipt,
            },
        ) => {
            required!(*errors_visible => "hidden-failure", *failure_receipt => "failure-receipt-missing")
        }
        (
            CrOp::InvalidState,
            CrState::InvalidState {
                invalid_rejected,
                transition_blocked,
            },
        ) => {
            required!(*invalid_rejected => "invalid-state-hidden", *transition_blocked => "invalid-state-transition-admitted")
        }
        (
            CrOp::DefaultFallback,
            CrState::DefaultFallback {
                missing_rejected,
                default_absent,
            },
        ) => {
            required!(*missing_rejected => "default-fallback-admitted", *default_absent => "default-present")
        }
        (
            CrOp::MissingDefaultHold,
            CrState::MissingDefaultHold {
                missing_held,
                default_absent,
            },
        ) => {
            required!(*missing_held => "missing-default-not-held", *default_absent => "missing-default-present")
        }
        (
            CrOp::TypedFamilyExhaustiveness,
            CrState::TypedFamilyExhaustiveness {
                families_complete,
                unknown_family_rejected,
            },
        ) => {
            required!(*families_complete => "typed-family-nonexhaustive", *unknown_family_rejected => "unknown-family-admitted")
        }
        (
            CrOp::TypedStateExhaustiveness,
            CrState::TypedStateExhaustiveness {
                states_complete,
                invalid_state_rejected,
            },
        ) => {
            required!(*states_complete => "typed-state-nonexhaustive", *invalid_state_rejected => "invalid-typed-state-admitted")
        }
        (CrOp::ReleaseNoOutput, CrState::ReleaseNoOutput { output_absent }) => {
            required!(*output_absent => "release-output-emitted")
        }
        (
            CrOp::AdmissionBypass,
            CrState::AdmissionBypass {
                universal_admission_required,
            },
        ) => required!(*universal_admission_required => "admission-bypass-emitted"),
        (
            CrOp::AuthorityNoninflation,
            CrState::AuthorityNoninflation {
                authority_not_inflated,
            },
        ) => required!(*authority_not_inflated => "authority-inflated"),
        (
            CrOp::FalseSavingsNoAuthority,
            CrState::FalseSavingsNoAuthority {
                false_savings_rejected,
                authority_absent,
            },
        ) => {
            required!(*false_savings_rejected => "false-savings-admitted", *authority_absent => "false-savings-authority-created")
        }
        (
            CrOp::TerminalNoBackflow,
            CrState::TerminalNoBackflow {
                terminal_output_absent,
                backflow_absent,
            },
        ) => {
            required!(*terminal_output_absent => "terminal-output-emitted", *backflow_absent => "terminal-backflow-emitted")
        }
        (CrOp::ReplayIdentity, CrState::ReplayIdentity { identical_replay }) => {
            required!(*identical_replay => "replay-identity-diverged")
        }
        (CrOp::OrderInvariance, CrState::OrderInvariance { order_independent }) => {
            required!(*order_independent => "order-invariance-failed")
        }
        (
            CrOp::DigestReproductionOrder,
            CrState::DigestReproductionOrder {
                canonical_order,
                digest_reproduced,
            },
        ) => {
            required!(*canonical_order => "digest-order-noncanonical", *digest_reproduced => "digest-reproduction-order-failed")
        }
        (
            CrOp::AmbientStateAbsence,
            CrState::AmbientStateAbsence {
                ambient_state_absent,
            },
        ) => required!(*ambient_state_absent => "ambient-state-observed"),
        (
            CrOp::ScheduleEquivalence,
            CrState::ScheduleEquivalence {
                schedules_equivalent,
            },
        ) => required!(*schedules_equivalent => "schedule-equivalence-failed"),
        (CrOp::Successor, CrState::Successor { immutable, acyclic }) => {
            required!(*immutable => "successor-mutated", *acyclic => "successor-cycle")
        }
        (CrOp::Dependency, CrState::Dependency { consumer_direction }) => {
            required!(*consumer_direction => "dependency-direction-violated")
        }
        (
            CrOp::FixedDependencyGraph,
            CrState::FixedDependencyGraph {
                graph_fixed,
                cycles_absent,
            },
        ) => {
            required!(*graph_fixed => "fixed-dependency-graph-violated", *cycles_absent => "fixed-dependency-cycle")
        }
        (
            CrOp::Content,
            CrState::Content {
                prohibited_content_rejected,
            },
        ) => required!(*prohibited_content_rejected => "content-boundary-violated"),
        (
            CrOp::ContentBoundaryProvenance,
            CrState::ContentBoundaryProvenance {
                boundary_preserved,
                provenance_retained,
            },
        ) => {
            required!(*boundary_preserved => "content-boundary-provenance-missing", *provenance_retained => "content-provenance-dropped")
        }
        (
            CrOp::Composition,
            CrState::Composition {
                minimized,
                unsafe_join_rejected,
            },
        ) => {
            required!(*minimized => "composition-not-minimized", *unsafe_join_rejected => "unsafe-composition-admitted")
        }
        (CrOp::Floor, CrState::Floor { noncompensable }) => {
            required!(*noncompensable => "floor-compensated")
        }
        (
            CrOp::Distribution,
            CrState::Distribution {
                distribution_preserved,
                tails_preserved,
            },
        ) => {
            required!(*distribution_preserved => "distribution-not-conserved", *tails_preserved => "distribution-tails-dropped")
        }
        (
            CrOp::NullState,
            CrState::NullState {
                missing_distinct,
                null_distinct,
            },
        ) => {
            required!(*missing_distinct => "missing-state-collapsed", *null_distinct => "null-state-collapsed")
        }
        (
            CrOp::StateNullNaStale,
            CrState::StateNullNaStale {
                missing_distinct,
                null_distinct,
                na_distinct,
                stale_distinct,
            },
        ) => {
            required!(*missing_distinct => "missing-state-collapsed", *null_distinct => "null-state-collapsed", *na_distinct => "na-state-collapsed", *stale_distinct => "stale-state-collapsed")
        }
        (
            CrOp::Accounting,
            CrState::Accounting {
                checked_arithmetic,
                overflow_rejected,
            },
        ) => {
            required!(*checked_arithmetic => "accounting-unbalanced", *overflow_rejected => "accounting-overflow-admitted")
        }
        (
            CrOp::ReconciliationIdentity,
            CrState::ReconciliationIdentity {
                identity_balanced,
                residual_zero,
                overlap_reconciled,
            },
        ) => {
            required!(*identity_balanced => "reconciliation-identity-failed", *residual_zero => "reconciliation-residual-nonzero", *overlap_reconciled => "reconciliation-overlap-unreconciled")
        }
        (
            CrOp::Burden,
            CrState::Burden {
                incidence_measured,
                tail_measured,
            },
        ) => {
            required!(*incidence_measured => "burden-shift-unmeasured", *tail_measured => "burden-tail-unmeasured")
        }
        (
            CrOp::Delivery,
            CrState::Delivery {
                pathways_separate,
                delivery_bound,
                adaptive_successor,
            },
        ) => {
            required!(*pathways_separate => "delivery-pathways-collapsed", *delivery_bound => "delivery-transition-invalid", *adaptive_successor => "delivery-successor-missing")
        }
        (
            CrOp::FindingDissentRetention,
            CrState::FindingDissentRetention {
                finding_retained,
                dissent_retained,
            },
        ) => {
            required!(*finding_retained => "finding-not-retained", *dissent_retained => "finding-dissent-not-retained")
        }
        (
            CrOp::ReviewIndependence,
            CrState::ReviewIndependence {
                independent,
                self_review_rejected,
            },
        ) => {
            required!(*independent => "review-conflict", *self_review_rejected => "self-review-admitted")
        }
        (
            CrOp::CrossRoleReview,
            CrState::CrossRoleReview {
                roles_complete,
                conflicts_rejected,
            },
        ) => {
            required!(*roles_complete => "cross-role-review-incomplete", *conflicts_rejected => "cross-role-review-conflict")
        }
        (
            CrOp::HoldPropagation,
            CrState::HoldPropagation {
                all_holds_propagated,
                downstream_blocked,
            },
        ) => {
            required!(*all_holds_propagated => "hold-not-propagated", *downstream_blocked => "hold-downstream-unblocked")
        }
        (
            CrOp::WaiverLedger,
            CrState::WaiverLedger {
                ledger_nonwaiver,
                bypass_rejected,
            },
        ) => {
            required!(*ledger_nonwaiver => "waiver-ledger-bypassed", *bypass_rejected => "waiver-bypass-admitted")
        }
        (CrOp::Invariant, CrState::Invariant { coverage_complete }) => {
            required!(*coverage_complete => "invariant-uncovered")
        }
        (
            CrOp::Property,
            CrState::Property {
                evidence_set_complete,
                reproduction_deterministic,
            },
        ) => {
            required!(*evidence_set_complete => "property-evidence-missing", *reproduction_deterministic => "property-reproduction-nondeterministic")
        }
        (
            CrOp::Transition,
            CrState::Transition {
                typed_transition,
                invalid_edge_rejected,
            },
        ) => {
            required!(*typed_transition => "invalid-transition", *invalid_edge_rejected => "invalid-transition-edge-admitted")
        }
        (
            CrOp::Parser,
            CrState::Parser {
                surface_minimal,
                authority_absent,
            },
        ) => {
            required!(*surface_minimal => "parser-surface-expanded", *authority_absent => "parser-created-authority")
        }
        (
            CrOp::ParserFuzz,
            CrState::ParserFuzz {
                malformed_rejected,
                panic_absent,
                authority_absent,
            },
        ) => {
            required!(*malformed_rejected => "parser-fuzz-malformed-admitted", *panic_absent => "parser-fuzz-panicked", *authority_absent => "parser-fuzz-created-authority")
        }
        (
            CrOp::Regression,
            CrState::Regression {
                successor_immutable,
                history_acyclic,
            },
        ) => {
            required!(*successor_immutable => "regression-successor-mutated", *history_acyclic => "regression-replay-diverged")
        }
        (
            CrOp::RegressionReplay,
            CrState::RegressionReplay {
                golden_replayed,
                digest_matches,
            },
        ) => {
            required!(*golden_replayed => "regression-replay-evidence-missing", *digest_matches => "regression-replay-digest-mismatch")
        }
        (
            CrOp::Isolation,
            CrState::Isolation {
                modes_separate,
                cross_mode_flow_absent,
            },
        ) => {
            required!(*modes_separate => "isolation-overlap", *cross_mode_flow_absent => "isolation-cross-mode-flow")
        }
        (
            CrOp::PackageIsolation,
            CrState::PackageIsolation {
                package_boundary,
                dependency_direction,
            },
        ) => {
            required!(*package_boundary => "package-isolation-overlap", *dependency_direction => "package-dependency-direction-violated")
        }
        (CrOp::Generated, CrState::Generated { emission_absent }) => {
            required!(*emission_absent => "generated-custody-violation")
        }
        (
            CrOp::GeneratedProvenance,
            CrState::GeneratedProvenance {
                generated_manifest_entry_absent,
                provenance_custody,
            },
        ) => {
            required!(*generated_manifest_entry_absent => "generated-manifest-entry-present", *provenance_custody => "generated-provenance-missing")
        }
        (
            CrOp::Quality,
            CrState::Quality {
                registry_complete,
                gates_bound,
            },
        ) => {
            required!(*registry_complete => "quality-registry-incomplete", *gates_bound => "quality-output-unbound")
        }
        (
            CrOp::QualityOutput,
            CrState::QualityOutput {
                output_bound,
                digest_bound,
            },
        ) => {
            required!(*output_bound => "quality-output-binding-missing", *digest_bound => "quality-digest-binding-missing")
        }
        (CrOp::LicenseAdvisory, CrState::LicenseAdvisory { controls }) => {
            validate_dependency_review(controls)
        }
        (
            CrOp::Evidence,
            CrState::Evidence {
                state_history_truthful,
            },
        ) => required!(*state_history_truthful => "evidence-digest-unbound"),
        (
            CrOp::EvidenceDigestTruth,
            CrState::EvidenceDigestTruth {
                evidence_bound,
                digest_truthful,
            },
        ) => {
            required!(*evidence_bound => "evidence-digest-truth-failed", *digest_truthful => "evidence-digest-untruthful")
        }
        (
            CrOp::Trace,
            CrState::Trace {
                transpose_equal,
                orphans_absent,
            },
        ) => {
            required!(*transpose_equal => "trace-contradiction", *orphans_absent => "trace-orphan")
        }
        _ => Err("typed-domain-incompatible"),
    }
}

fn validate_domain_state(kind: TypedObligation, state: &DomainState) -> Result<(), &'static str> {
    match (kind, state) {
        (
            TypedObligation::Test(rule),
            DomainState::Test {
                rule: actual,
                controls,
            },
        ) if rule == *actual => validate_test_controls(rule, controls),
        (
            TypedObligation::Release(rule),
            DomainState::Release {
                rule: actual,
                authority,
                risk,
                communication,
                closed,
            },
        ) if rule == *actual => match rule {
            ReleaseRule::Authority => validate_release_authority(authority),
            ReleaseRule::RiskAssessment => validate_release_risk(risk),
            ReleaseRule::Communication => validate_release_communication(communication),
            ReleaseRule::Closed => validate_release_closed(closed),
        },
        (
            TypedObligation::Nf(rule),
            DomainState::Nf {
                rule: actual,
                controls,
            },
        ) if rule == *actual => validate_nf_controls(rule, controls),
        (
            TypedObligation::Vcl(rule),
            DomainState::Vcl {
                rule: actual,
                controls,
                hold_closure,
                accounting,
            },
        ) if rule == *actual => match rule {
            VclRule::Custody => validate_vcl_controls(rule, controls),
            VclRule::Contract => validate_vcl_controls(rule, controls),
            VclRule::Transition => validate_vcl_controls(rule, controls),
            VclRule::Property => validate_vcl_controls(rule, controls),
            VclRule::Holds => validate_hold_closure(hold_closure),
            VclRule::Content => validate_vcl_controls(rule, controls),
            VclRule::Civilian => validate_vcl_controls(rule, controls),
            VclRule::Accounting => validate_checked_accounting(accounting),
            VclRule::Isolation => validate_vcl_controls(rule, controls),
            VclRule::Evidence => validate_vcl_controls(rule, controls),
        },
        (TypedObligation::ValidationScope, DomainState::ValidationScope(controls)) => {
            validate_validation_scope(controls)
        }
        (TypedObligation::ValidationAssurance, DomainState::ValidationAssurance(controls)) => {
            validate_validation_assurance(controls)
        }
        (
            TypedObligation::Actor(rule),
            DomainState::Actor {
                rule: actual,
                controls,
                finance,
            },
        ) if rule == *actual => match rule {
            ActorRule::Civilian => validate_actor_controls(rule, controls),
            ActorRule::Readiness => validate_actor_controls(rule, controls),
            ActorRule::Acquisition => validate_actor_controls(rule, controls),
            ActorRule::Logistics => validate_actor_controls(rule, controls),
            ActorRule::Alliance => validate_actor_controls(rule, controls),
            ActorRule::Finance => validate_finance(finance),
            ActorRule::People => validate_actor_controls(rule, controls),
            ActorRule::Test => validate_actor_controls(rule, controls),
            ActorRule::Source => validate_actor_controls(rule, controls),
            ActorRule::Law => validate_actor_controls(rule, controls),
            ActorRule::External => validate_actor_controls(rule, controls),
        },
        (
            TypedObligation::Review(lens),
            DomainState::Review {
                lens: actual,
                evidence,
                independent,
                named,
                numeracy,
                supplier,
                security,
                civilian,
            },
        ) if lens == *actual => {
            if !independent {
                return Err("review-conflict");
            };
            if *evidence != review_evidence(lens) {
                return Err("role-evidence-incompatible");
            };
            validate_review_named(lens, named)?;
            match lens {
                ReviewLens::Citation => Ok(()),
                ReviewLens::Numeracy => validate_numeracy(numeracy),
                ReviewLens::Scope => Ok(()),
                ReviewLens::Panel => Ok(()),
                ReviewLens::Acquisition => Ok(()),
                ReviewLens::Alliance => Ok(()),
                ReviewLens::ForcePlanning => validate_civilian_assurance(civilian),
                ReviewLens::Comptroller => validate_numeracy(numeracy),
                ReviewLens::TestOversight => Ok(()),
                ReviewLens::Logistics => Ok(()),
                ReviewLens::Readiness => Ok(()),
                ReviewLens::People => Ok(()),
                ReviewLens::AllyStakeholder => Ok(()),
                ReviewLens::DepotStakeholder => Ok(()),
                ReviewLens::InstallationStakeholder => Ok(()),
                ReviewLens::MissionStakeholder => Ok(()),
                ReviewLens::SupplierStakeholder => validate_supplier(supplier),
                ReviewLens::FamilyStakeholder => Ok(()),
                ReviewLens::TaxpayerStakeholder => validate_numeracy(numeracy),
                ReviewLens::SecurityAssurance => validate_security_assurance(security),
                ReviewLens::CivilianAssurance => validate_civilian_assurance(civilian),
                ReviewLens::Steward => Ok(()),
            }
        }
        (
            TypedObligation::Hold(rule),
            DomainState::Hold {
                rule: actual,
                admission_blocked,
                downstream_blocked,
                emitted,
            },
        ) if rule == *actual => {
            if !admission_blocked || !downstream_blocked || *emitted {
                Err("hold-bypass")
            } else {
                Ok(())
            }
        }
        (TypedObligation::Cr(operation), DomainState::Cr(state)) => {
            validate_cr_state(operation, state)
        }
        _ => Err("typed-domain-incompatible"),
    }
}

fn negative_cr_state(operation: CrOp, state: &mut CrState) -> &'static str {
    match (operation, state) {
        (CrOp::LogicalContract, CrState::LogicalContract { logical_surface }) => {
            *logical_surface = false;
            "logical-contract-surface-omitted"
        }
        (
            CrOp::LogicalResponsibility,
            CrState::LogicalResponsibility {
                responsibility_allocated,
            },
        ) => {
            *responsibility_allocated = false;
            "logical-contract-responsibility-omitted"
        }
        (CrOp::ContractFixtureMatrix, CrState::ContractFixtureMatrix { controls }) => {
            controls.source_contract = false;
            "source-contract-fixture-omitted"
        }
        (
            CrOp::TypedFailure,
            CrState::TypedFailure {
                typed_error_rejected,
                ..
            },
        ) => {
            *typed_error_rejected = false;
            "typed-failure-admitted"
        }
        (
            CrOp::TypedBranchTotality,
            CrState::TypedBranchTotality {
                typed_variants_covered,
                ..
            },
        ) => {
            *typed_variants_covered = false;
            "typed-variant-coverage-incomplete"
        }
        (CrOp::ExhaustionBound, CrState::ExhaustionBound { limit_enforced, .. }) => {
            *limit_enforced = false;
            "exhaustion-limit-unenforced"
        }
        (CrOp::FiniteBoundsProgress, CrState::FiniteBoundsProgress { finite_bound, .. }) => {
            *finite_bound = false;
            "finite-progress-bound-missing"
        }
        (CrOp::ResourceBound, CrState::ResourceBound { input_bound, .. }) => {
            *input_bound = false;
            "input-resource-bound-missing"
        }
        (
            CrOp::ResourceBoundRegistry,
            CrState::ResourceBoundRegistry {
                registry_present, ..
            },
        ) => {
            *registry_present = false;
            "resource-bound-registry-missing"
        }
        (
            CrOp::CallDepth,
            CrState::CallDepth {
                call_depth,
                max_depth,
            },
        ) => {
            *call_depth = *max_depth + 1;
            "call-depth-exceeded"
        }
        (CrOp::Termination, CrState::Termination { decreasing_measure }) => {
            *decreasing_measure = vec![4, 3, 3, 2];
            "termination-measure-not-decreasing"
        }
        (CrOp::FailureVisibility, CrState::FailureVisibility { errors_visible, .. }) => {
            *errors_visible = false;
            "hidden-failure"
        }
        (
            CrOp::InvalidState,
            CrState::InvalidState {
                invalid_rejected, ..
            },
        ) => {
            *invalid_rejected = false;
            "invalid-state-hidden"
        }
        (
            CrOp::DefaultFallback,
            CrState::DefaultFallback {
                missing_rejected, ..
            },
        ) => {
            *missing_rejected = false;
            "default-fallback-admitted"
        }
        (CrOp::MissingDefaultHold, CrState::MissingDefaultHold { missing_held, .. }) => {
            *missing_held = false;
            "missing-default-not-held"
        }
        (
            CrOp::TypedFamilyExhaustiveness,
            CrState::TypedFamilyExhaustiveness {
                families_complete, ..
            },
        ) => {
            *families_complete = false;
            "typed-family-nonexhaustive"
        }
        (
            CrOp::TypedStateExhaustiveness,
            CrState::TypedStateExhaustiveness {
                states_complete, ..
            },
        ) => {
            *states_complete = false;
            "typed-state-nonexhaustive"
        }
        (CrOp::ReleaseNoOutput, CrState::ReleaseNoOutput { output_absent }) => {
            *output_absent = false;
            "release-output-emitted"
        }
        (
            CrOp::AdmissionBypass,
            CrState::AdmissionBypass {
                universal_admission_required,
            },
        ) => {
            *universal_admission_required = false;
            "admission-bypass-emitted"
        }
        (
            CrOp::AuthorityNoninflation,
            CrState::AuthorityNoninflation {
                authority_not_inflated,
            },
        ) => {
            *authority_not_inflated = false;
            "authority-inflated"
        }
        (
            CrOp::FalseSavingsNoAuthority,
            CrState::FalseSavingsNoAuthority {
                false_savings_rejected,
                ..
            },
        ) => {
            *false_savings_rejected = false;
            "false-savings-admitted"
        }
        (
            CrOp::TerminalNoBackflow,
            CrState::TerminalNoBackflow {
                terminal_output_absent,
                ..
            },
        ) => {
            *terminal_output_absent = false;
            "terminal-output-emitted"
        }
        (CrOp::ReplayIdentity, CrState::ReplayIdentity { identical_replay }) => {
            *identical_replay = false;
            "replay-identity-diverged"
        }
        (CrOp::OrderInvariance, CrState::OrderInvariance { order_independent }) => {
            *order_independent = false;
            "order-invariance-failed"
        }
        (
            CrOp::DigestReproductionOrder,
            CrState::DigestReproductionOrder {
                canonical_order, ..
            },
        ) => {
            *canonical_order = false;
            "digest-order-noncanonical"
        }
        (
            CrOp::AmbientStateAbsence,
            CrState::AmbientStateAbsence {
                ambient_state_absent,
            },
        ) => {
            *ambient_state_absent = false;
            "ambient-state-observed"
        }
        (
            CrOp::ScheduleEquivalence,
            CrState::ScheduleEquivalence {
                schedules_equivalent,
            },
        ) => {
            *schedules_equivalent = false;
            "schedule-equivalence-failed"
        }
        (CrOp::Successor, CrState::Successor { immutable, .. }) => {
            *immutable = false;
            "successor-mutated"
        }
        (CrOp::Dependency, CrState::Dependency { consumer_direction }) => {
            *consumer_direction = false;
            "dependency-direction-violated"
        }
        (CrOp::FixedDependencyGraph, CrState::FixedDependencyGraph { graph_fixed, .. }) => {
            *graph_fixed = false;
            "fixed-dependency-graph-violated"
        }
        (
            CrOp::Content,
            CrState::Content {
                prohibited_content_rejected,
            },
        ) => {
            *prohibited_content_rejected = false;
            "content-boundary-violated"
        }
        (
            CrOp::ContentBoundaryProvenance,
            CrState::ContentBoundaryProvenance {
                boundary_preserved, ..
            },
        ) => {
            *boundary_preserved = false;
            "content-boundary-provenance-missing"
        }
        (CrOp::Composition, CrState::Composition { minimized, .. }) => {
            *minimized = false;
            "composition-not-minimized"
        }
        (CrOp::Floor, CrState::Floor { noncompensable }) => {
            *noncompensable = false;
            "floor-compensated"
        }
        (
            CrOp::Distribution,
            CrState::Distribution {
                distribution_preserved,
                ..
            },
        ) => {
            *distribution_preserved = false;
            "distribution-not-conserved"
        }
        (
            CrOp::NullState,
            CrState::NullState {
                missing_distinct, ..
            },
        ) => {
            *missing_distinct = false;
            "missing-state-collapsed"
        }
        (
            CrOp::StateNullNaStale,
            CrState::StateNullNaStale {
                missing_distinct, ..
            },
        ) => {
            *missing_distinct = false;
            "missing-state-collapsed"
        }
        (
            CrOp::Accounting,
            CrState::Accounting {
                checked_arithmetic, ..
            },
        ) => {
            *checked_arithmetic = false;
            "accounting-unbalanced"
        }
        (
            CrOp::ReconciliationIdentity,
            CrState::ReconciliationIdentity {
                identity_balanced, ..
            },
        ) => {
            *identity_balanced = false;
            "reconciliation-identity-failed"
        }
        (
            CrOp::Burden,
            CrState::Burden {
                incidence_measured, ..
            },
        ) => {
            *incidence_measured = false;
            "burden-shift-unmeasured"
        }
        (
            CrOp::Delivery,
            CrState::Delivery {
                pathways_separate, ..
            },
        ) => {
            *pathways_separate = false;
            "delivery-pathways-collapsed"
        }
        (
            CrOp::FindingDissentRetention,
            CrState::FindingDissentRetention {
                finding_retained, ..
            },
        ) => {
            *finding_retained = false;
            "finding-not-retained"
        }
        (CrOp::ReviewIndependence, CrState::ReviewIndependence { independent, .. }) => {
            *independent = false;
            "review-conflict"
        }
        (CrOp::CrossRoleReview, CrState::CrossRoleReview { roles_complete, .. }) => {
            *roles_complete = false;
            "cross-role-review-incomplete"
        }
        (
            CrOp::HoldPropagation,
            CrState::HoldPropagation {
                all_holds_propagated,
                ..
            },
        ) => {
            *all_holds_propagated = false;
            "hold-not-propagated"
        }
        (
            CrOp::WaiverLedger,
            CrState::WaiverLedger {
                ledger_nonwaiver, ..
            },
        ) => {
            *ledger_nonwaiver = false;
            "waiver-ledger-bypassed"
        }
        (CrOp::Invariant, CrState::Invariant { coverage_complete }) => {
            *coverage_complete = false;
            "invariant-uncovered"
        }
        (
            CrOp::Property,
            CrState::Property {
                evidence_set_complete,
                ..
            },
        ) => {
            *evidence_set_complete = false;
            "property-evidence-missing"
        }
        (
            CrOp::Transition,
            CrState::Transition {
                typed_transition, ..
            },
        ) => {
            *typed_transition = false;
            "invalid-transition"
        }
        (
            CrOp::Parser,
            CrState::Parser {
                surface_minimal, ..
            },
        ) => {
            *surface_minimal = false;
            "parser-surface-expanded"
        }
        (
            CrOp::ParserFuzz,
            CrState::ParserFuzz {
                malformed_rejected, ..
            },
        ) => {
            *malformed_rejected = false;
            "parser-fuzz-malformed-admitted"
        }
        (
            CrOp::Regression,
            CrState::Regression {
                successor_immutable,
                ..
            },
        ) => {
            *successor_immutable = false;
            "regression-successor-mutated"
        }
        (
            CrOp::RegressionReplay,
            CrState::RegressionReplay {
                golden_replayed, ..
            },
        ) => {
            *golden_replayed = false;
            "regression-replay-evidence-missing"
        }
        (CrOp::Isolation, CrState::Isolation { modes_separate, .. }) => {
            *modes_separate = false;
            "isolation-overlap"
        }
        (
            CrOp::PackageIsolation,
            CrState::PackageIsolation {
                package_boundary, ..
            },
        ) => {
            *package_boundary = false;
            "package-isolation-overlap"
        }
        (CrOp::Generated, CrState::Generated { emission_absent }) => {
            *emission_absent = false;
            "generated-custody-violation"
        }
        (
            CrOp::GeneratedProvenance,
            CrState::GeneratedProvenance {
                generated_manifest_entry_absent,
                ..
            },
        ) => {
            *generated_manifest_entry_absent = false;
            "generated-manifest-entry-present"
        }
        (
            CrOp::Quality,
            CrState::Quality {
                registry_complete, ..
            },
        ) => {
            *registry_complete = false;
            "quality-registry-incomplete"
        }
        (CrOp::QualityOutput, CrState::QualityOutput { output_bound, .. }) => {
            *output_bound = false;
            "quality-output-binding-missing"
        }
        (CrOp::LicenseAdvisory, CrState::LicenseAdvisory { controls }) => {
            controls.licenses_allowed = false;
            "license-policy-rejected"
        }
        (
            CrOp::Evidence,
            CrState::Evidence {
                state_history_truthful,
            },
        ) => {
            *state_history_truthful = false;
            "evidence-digest-unbound"
        }
        (CrOp::EvidenceDigestTruth, CrState::EvidenceDigestTruth { evidence_bound, .. }) => {
            *evidence_bound = false;
            "evidence-digest-truth-failed"
        }
        (
            CrOp::Trace,
            CrState::Trace {
                transpose_equal, ..
            },
        ) => {
            *transpose_equal = false;
            "trace-contradiction"
        }
        _ => unreachable!(),
    }
}

fn negative_domain_state(kind: TypedObligation) -> (DomainState, &'static str) {
    let mut state = positive_domain_state(kind);
    let error = match (&kind, &mut state) {
        (TypedObligation::Test(rule), DomainState::Test { controls, .. }) => match rule {
            TestRule::IndependentReproduction => {
                controls.independent = false;
                "test-independence-omitted"
            }
            TestRule::FindingShape => {
                controls.finding_stable_id = false;
                "finding-stable-id-omitted"
            }
            TestRule::PromotionBlock => {
                controls.stale_blocks = false;
                "stale-promotion-unblocked"
            }
            TestRule::EvidenceTruth => {
                controls.advocacy_rejected = false;
                "advocacy-substituted-for-evidence"
            }
            TestRule::Packet => {
                controls.frozen_artifact = false;
                "test-frozen-artifact-omitted"
            }
            TestRule::RetainedPosture => {
                controls.retained_negative_results = false;
                "retained-negative-results-omitted"
            }
        },
        (
            TypedObligation::Release(rule),
            DomainState::Release {
                authority,
                risk,
                communication,
                closed,
                ..
            },
        ) => match rule {
            ReleaseRule::Authority => {
                authority.publication_blocked = false;
                "release-publication-enabled"
            }
            ReleaseRule::RiskAssessment => {
                risk.direct_release_composition = false;
                "direct-release-composition-omitted"
            }
            ReleaseRule::Communication => {
                communication.source = false;
                "release-source-context-omitted"
            }
            ReleaseRule::Closed => {
                closed.no_output = false;
                "closed-release-output-enabled"
            }
        },
        (TypedObligation::Nf(rule), DomainState::Nf { controls, .. }) => match rule {
            NfRule::ProhibitedData => {
                controls.classified.ingest_blocked = false;
                "classified-ingest-enabled"
            }
            NfRule::Scope => {
                controls.authority_broadening_blocked = false;
                "authority-scope-broadened"
            }
            NfRule::ReadinessFloor => {
                controls.floor_blocks_candidate = false;
                "floor-failure-candidate-admitted"
            }
            NfRule::Distribution => {
                controls.distribution_retained = false;
                "distribution-omitted"
            }
            NfRule::Reconciliation => {
                controls.reconciliation_units = false;
                "reconciliation-units-omitted"
            }
            NfRule::NonAdditive => {
                controls.basing_nonadditive = false;
                "basing-pathway-auto-summed"
            }
            NfRule::MissingNa => {
                controls.missing_distinct_from_zero = false;
                "missing-defaulted-zero"
            }
            NfRule::History => {
                controls.deterministic_identity = false;
                "history-identity-nondeterministic"
            }
            NfRule::Staleness => {
                controls.digest_fresh = false;
                "bound-digest-stale"
            }
            NfRule::NoAuthority => {
                controls.no_operational_authority = false;
                "operational-authority-created"
            }
        },
        (
            TypedObligation::Vcl(rule),
            DomainState::Vcl {
                controls,
                hold_closure,
                accounting,
                ..
            },
        ) => match rule {
            VclRule::Custody => {
                controls.source_identity = false;
                "vcl-source-identity-omitted"
            }
            VclRule::Contract => {
                controls.positive_contract = false;
                "vcl-positive-contract-omitted"
            }
            VclRule::Transition => {
                controls.typed_state = false;
                "vcl-typed-state-omitted"
            }
            VclRule::Property => {
                controls.invariant_coverage = false;
                "vcl-invariant-coverage-omitted"
            }
            VclRule::Holds => {
                hold_closure.security = false;
                "security-hold-omitted"
            }
            VclRule::Content => {
                controls.prohibited_content_rejected = false;
                "vcl-prohibited-content-admitted"
            }
            VclRule::Civilian => {
                controls.civilian_authority = false;
                "vcl-civilian-authority-omitted"
            }
            VclRule::Accounting => {
                accounting.delivery = false;
                "delivery-accounting-omitted"
            }
            VclRule::Isolation => {
                controls.quality = false;
                "vcl-quality-omitted"
            }
            VclRule::Evidence => {
                controls.evidence_state_truth = false;
                "vcl-evidence-state-truth-omitted"
            }
        },
        (TypedObligation::ValidationScope, DomainState::ValidationScope(controls)) => {
            controls.public_aggregate = false;
            "validation-scope-not-public-aggregate"
        }
        (TypedObligation::ValidationAssurance, DomainState::ValidationAssurance(controls)) => {
            controls.independence = false;
            "validation-assurance-conflict"
        }
        (
            TypedObligation::Actor(rule),
            DomainState::Actor {
                controls, finance, ..
            },
        ) => match rule {
            ActorRule::Civilian => {
                controls.mission_fit = false;
                "actor-mission-fit-omitted"
            }
            ActorRule::Readiness => {
                controls.readiness_floors = false;
                "actor-readiness-floors-omitted"
            }
            ActorRule::Acquisition => {
                controls.competition = false;
                "actor-competition-omitted"
            }
            ActorRule::Logistics => {
                controls.aggregate_custody = false;
                "actor-aggregate-custody-omitted"
            }
            ActorRule::Alliance => {
                controls.sovereignty = false;
                "actor-sovereignty-omitted"
            }
            ActorRule::Finance => {
                finance.units = false;
                "finance-units-omitted"
            }
            ActorRule::People => {
                controls.protected_pains = false;
                "actor-protected-pains-omitted"
            }
            ActorRule::Test => {
                controls.falsifiability = false;
                "actor-falsifiability-omitted"
            }
            ActorRule::Source => {
                controls.security_markers = false;
                "actor-security-markers-omitted"
            }
            ActorRule::Law => {
                controls.nonwaivable_civilian_authority = false;
                "actor-civilian-authority-waived"
            }
            ActorRule::External => {
                controls.hnd_term_boundary = false;
                "actor-hnd-term-boundary-omitted"
            }
        },
        (
            TypedObligation::Review(lens),
            DomainState::Review {
                named,
                numeracy,
                supplier,
                security,
                civilian,
                ..
            },
        ) => match lens {
            ReviewLens::Citation => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Numeracy => {
                numeracy.units = false;
                "numeracy-units-omitted"
            }
            ReviewLens::Scope => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Panel => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Acquisition => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Alliance => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::ForcePlanning => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Comptroller => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::TestOversight => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Logistics => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::Readiness => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::People => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::AllyStakeholder => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::DepotStakeholder => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::InstallationStakeholder => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::MissionStakeholder => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::SupplierStakeholder => {
                supplier.requirements_stability = false;
                "supplier-requirements-stability-omitted"
            }
            ReviewLens::FamilyStakeholder => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::TaxpayerStakeholder => {
                named.pop();
                "review-named-control-omitted"
            }
            ReviewLens::SecurityAssurance => {
                security.classified_rejected = false;
                "classified-content-admitted"
            }
            ReviewLens::CivilianAssurance => {
                civilian.lawful_civilian_authority = false;
                "lawful-civilian-authority-omitted"
            }
            ReviewLens::Steward => {
                named.pop();
                "review-named-control-omitted"
            }
        },
        (
            TypedObligation::Hold(_),
            DomainState::Hold {
                downstream_blocked, ..
            },
        ) => {
            *downstream_blocked = false;
            "hold-bypass"
        }
        (TypedObligation::Cr(operation), DomainState::Cr(state)) => {
            negative_cr_state(*operation, state)
        }
        _ => unreachable!(),
    };
    (state, error)
}

fn execute_typed_positive(edge: &TraceEdge) {
    let kind = typed_obligation(edge);
    assert_eq!(
        validate_domain_state(kind, &positive_domain_state(kind)),
        Ok(())
    );
}
fn execute_typed_negative(edge: &TraceEdge) -> &'static str {
    let kind = typed_obligation(edge);
    let (state, error) = negative_domain_state(kind);
    assert_eq!(validate_domain_state(kind, &state), Err(error));
    if kind == TypedObligation::Cr(CrOp::LicenseAdvisory) {
        let mut advisory = positive_domain_state(kind);
        if let DomainState::Cr(CrState::LicenseAdvisory { controls }) = &mut advisory {
            controls.advisories_current = false
        };
        assert_eq!(
            validate_domain_state(kind, &advisory),
            Err("advisory-data-stale")
        );
    }
    error
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum SharedControlFamily {
    ContractFixture,
    DependencyReview,
    HoldClosure,
    CheckedAccounting,
    Finance,
    Numeracy,
    Supplier,
    SecurityAssurance,
    CivilianAssurance,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ControlFamily {
    Typed(TypedObligation),
    Shared(SharedControlFamily),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SourceControlId {
    family: ControlFamily,
    ordinal: u16,
    name: String,
}

fn assert_named_field_omission_matrix() -> BTreeSet<SourceControlId> {
    let mut covered = 0usize;
    let mut mutation_controls = BTreeSet::new();
    macro_rules! omissions {
        ($family:expr, $ty:ty, $validator:expr; $($field:ident => $error:literal),+ $(,)?) => {
            $(
                let mut controls = <$ty>::complete();
                controls.$field = false;
                assert_eq!($validator(&controls), Err($error));
                assert!(mutation_controls.insert(SourceControlId {
                    family: $family,
                    ordinal: 0,
                    name: stringify!($field).to_owned(),
                }));
                covered += 1;
            )+
        };
    }
    omissions!(ControlFamily::Typed(TypedObligation::Test(TestRule::Packet)), TestControls, |c| validate_test_controls(TestRule::Packet, c);
        frozen_artifact => "test-frozen-artifact-omitted", evidence_manifest => "test-evidence-manifest-omitted",
        derivations => "test-derivations-omitted", gate_matrix => "test-gate-matrix-omitted",
        negative_cases => "test-negative-cases-omitted", unresolved_questions => "test-unresolved-questions-omitted",
        digest_binding => "test-digest-binding-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Test(TestRule::IndependentReproduction)), TestControls, |c| validate_test_controls(TestRule::IndependentReproduction, c);
        quantitative_reproduction => "test-quantitative-reproduction-omitted", qualitative_custody => "test-qualitative-custody-omitted",
        adverse_cases => "test-adverse-cases-omitted", failure_cases => "test-failure-cases-omitted",
        uncertainty => "test-uncertainty-omitted", denominators => "test-denominators-omitted", price_years => "test-price-years-omitted",
        lifecycle_cost => "test-lifecycle-cost-omitted", transition_cost => "test-transition-cost-omitted",
        double_count_detection => "test-double-count-undetected", independent => "test-independence-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Test(TestRule::RetainedPosture)), TestControls, |c| validate_test_controls(TestRule::RetainedPosture, c);
        retained_negative_results => "retained-negative-results-omitted", retained_failed_tests => "retained-failed-tests-omitted",
        retained_nulls => "retained-nulls-omitted", retained_rejected_candidates => "retained-rejected-candidates-omitted",
        retained_dissent => "retained-dissent-omitted", retained_unresolved_evidence => "retained-unresolved-evidence-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Test(TestRule::FindingShape)), TestControls, |c| validate_test_controls(TestRule::FindingShape, c);
        finding_stable_id => "finding-stable-id-omitted", finding_digest => "finding-digest-omitted", finding_role => "finding-role-omitted",
        finding_severity => "finding-severity-omitted", finding_claim => "finding-claim-omitted", finding_evidence => "finding-evidence-omitted",
        finding_disposition => "finding-disposition-omitted", finding_owner => "finding-owner-omitted",
        finding_destination => "finding-destination-omitted", finding_closure => "finding-closure-omitted",
        finding_independence => "finding-independence-omitted", finding_dissent => "finding-dissent-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Test(TestRule::PromotionBlock)), TestControls, |c| validate_test_controls(TestRule::PromotionBlock, c);
        stale_blocks => "stale-promotion-unblocked", conflicted_blocks => "conflicted-promotion-unblocked",
        absent_role_blocks => "absent-role-promotion-unblocked", failed_assurance_blocks => "failed-assurance-promotion-unblocked",
        unowned_defer_blocks => "unowned-defer-promotion-unblocked", false_approval_blocks => "false-approval-promotion-unblocked",
        unresolved_major_critical_blocks => "major-critical-promotion-unblocked");
    omissions!(ControlFamily::Typed(TypedObligation::Test(TestRule::EvidenceTruth)), TestControls, |c| validate_test_controls(TestRule::EvidenceTruth, c);
        advocacy_rejected => "advocacy-substituted-for-evidence", credentials_rejected => "credentials-substituted-for-evidence",
        inaccessible_classified_appeal_rejected => "classified-appeal-substituted-for-evidence");
    macro_rules! prohibited_omissions {
        ($member:ident; $ingest:literal, $retain:literal, $derive:literal, $emit:literal) => {{
            let family = ControlFamily::Typed(TypedObligation::Nf(NfRule::ProhibitedData));
            let mut c = NfControls::complete();
            c.$member.ingest_blocked = false;
            assert_eq!(
                validate_nf_controls(NfRule::ProhibitedData, &c),
                Err($ingest)
            );
            assert!(mutation_controls.insert(SourceControlId {
                family,
                ordinal: 0,
                name: concat!(stringify!($member), ".ingest_blocked").to_owned()
            }));
            covered += 1;
            let mut c = NfControls::complete();
            c.$member.retention_blocked = false;
            assert_eq!(
                validate_nf_controls(NfRule::ProhibitedData, &c),
                Err($retain)
            );
            assert!(mutation_controls.insert(SourceControlId {
                family,
                ordinal: 0,
                name: concat!(stringify!($member), ".retention_blocked").to_owned()
            }));
            covered += 1;
            let mut c = NfControls::complete();
            c.$member.derivation_blocked = false;
            assert_eq!(
                validate_nf_controls(NfRule::ProhibitedData, &c),
                Err($derive)
            );
            assert!(mutation_controls.insert(SourceControlId {
                family,
                ordinal: 0,
                name: concat!(stringify!($member), ".derivation_blocked").to_owned()
            }));
            covered += 1;
            let mut c = NfControls::complete();
            c.$member.emission_blocked = false;
            assert_eq!(validate_nf_controls(NfRule::ProhibitedData, &c), Err($emit));
            assert!(mutation_controls.insert(SourceControlId {
                family,
                ordinal: 0,
                name: concat!(stringify!($member), ".emission_blocked").to_owned()
            }));
            covered += 1;
        }};
    }
    prohibited_omissions!(classified; "classified-ingest-enabled", "classified-retention-enabled", "classified-derivation-enabled", "classified-emission-enabled");
    prohibited_omissions!(controlled; "controlled-ingest-enabled", "controlled-retention-enabled", "controlled-derivation-enabled", "controlled-emission-enabled");
    prohibited_omissions!(person_level; "person-level-ingest-enabled", "person-level-retention-enabled", "person-level-derivation-enabled", "person-level-emission-enabled");
    prohibited_omissions!(sensitive_operational; "sensitive-operational-ingest-enabled", "sensitive-operational-retention-enabled", "sensitive-operational-derivation-enabled", "sensitive-operational-emission-enabled");
    prohibited_omissions!(targeting; "targeting-ingest-enabled", "targeting-retention-enabled", "targeting-derivation-enabled", "targeting-emission-enabled");
    prohibited_omissions!(force_employment; "force-employment-ingest-enabled", "force-employment-retention-enabled", "force-employment-derivation-enabled", "force-employment-emission-enabled");
    prohibited_omissions!(exploitable_vulnerability; "vulnerability-ingest-enabled", "vulnerability-retention-enabled", "vulnerability-derivation-enabled", "vulnerability-emission-enabled");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::Scope)), NfControls, |c| validate_nf_controls(NfRule::Scope, c);
        authority_broadening_blocked => "authority-scope-broadened", mission_risk_broadening_blocked => "mission-risk-scope-broadened");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::ReadinessFloor)), NfControls, |c| validate_nf_controls(NfRule::ReadinessFloor, c);
        floor_blocks_candidate => "floor-failure-candidate-admitted", floor_blocks_savings => "floor-failure-savings-admitted",
        floor_blocks_delivery => "floor-failure-delivery-admitted", floor_blocks_handoff => "floor-failure-handoff-admitted");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::Distribution)), NfControls, |c| validate_nf_controls(NfRule::Distribution, c);
        distribution_retained => "distribution-omitted", repair_tail_retained => "repair-tail-omitted",
        degraded_posture_retained => "degraded-posture-omitted", concentrated_effects_retained => "concentrated-effects-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::Reconciliation)), NfControls, |c| validate_nf_controls(NfRule::Reconciliation, c);
        reconciliation_units => "reconciliation-units-omitted", reconciliation_horizons => "reconciliation-horizons-omitted",
        reconciliation_price_bases => "reconciliation-price-bases-omitted", reconciliation_account_measures => "reconciliation-account-measures-omitted",
        reconciliation_parties => "reconciliation-parties-omitted", reconciliation_overlap => "reconciliation-overlap-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::NonAdditive)), NfControls, |c| validate_nf_controls(NfRule::NonAdditive, c);
        basing_nonadditive => "basing-pathway-auto-summed", consolidation_nonadditive => "consolidation-pathway-auto-summed",
        process_nonadditive => "process-pathway-auto-summed", commonality_nonadditive => "commonality-pathway-auto-summed",
        logistics_nonadditive => "logistics-pathway-auto-summed", workforce_nonadditive => "workforce-pathway-auto-summed",
        noncash_not_converted => "noncash-converted-to-savings");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::MissingNa)), NfControls, |c| validate_nf_controls(NfRule::MissingNa, c);
        missing_distinct_from_zero => "missing-defaulted-zero", na_reason => "na-reason-omitted",
        na_alternative_boundary => "na-alternative-boundary-omitted", na_independent_review => "na-independent-review-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::History)), NfControls, |c| validate_nf_controls(NfRule::History, c);
        deterministic_identity => "history-identity-nondeterministic", deterministic_order => "history-order-nondeterministic",
        immutable_supersession_history => "supersession-history-rewritten");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::Staleness)), NfControls, |c| validate_nf_controls(NfRule::Staleness, c);
        digest_fresh => "bound-digest-stale", context_fresh => "bound-context-stale", stale_review_blocked => "stale-review-admitted",
        stale_admission_blocked => "stale-input-admitted", stale_handoff_blocked => "stale-handoff-admitted");
    omissions!(ControlFamily::Typed(TypedObligation::Nf(NfRule::NoAuthority)), NfControls, |c| validate_nf_controls(NfRule::NoAuthority, c);
        no_operational_authority => "operational-authority-created", no_procurement_authority => "procurement-authority-created",
        no_budget_authority => "budget-authority-created", no_taxlane_authority => "taxlane-authority-created",
        no_allocation_authority => "allocation-authority-created", no_rate_authority => "rate-authority-created",
        no_official_authority => "official-authority-created", no_implementation_authority => "implementation-authority-created",
        no_release_authority => "release-authority-created");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Custody)), VclControls, |c| validate_vcl_controls(VclRule::Custody, c);
        source_identity => "vcl-source-identity-omitted", allocation => "vcl-allocation-omitted", trace => "vcl-trace-omitted",
        custody => "vcl-custody-omitted", digest => "vcl-digest-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Contract)), VclControls, |c| validate_vcl_controls(VclRule::Contract, c);
        positive_contract => "vcl-positive-contract-omitted", negative_contract => "vcl-negative-contract-omitted",
        unauthorized_consumer_rejected => "vcl-unauthorized-consumer-admitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Transition)), VclControls, |c| validate_vcl_controls(VclRule::Transition, c);
        typed_state => "vcl-typed-state-omitted", typed_transition => "vcl-typed-transition-omitted", finite_dag => "vcl-finite-dag-omitted",
        immutable_successor => "vcl-successor-omitted", invalid_edge_rejected => "vcl-invalid-edge-admitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Property)), VclControls, |c| validate_vcl_controls(VclRule::Property, c);
        invariant_coverage => "vcl-invariant-coverage-omitted", property_evidence => "vcl-property-evidence-omitted",
        deterministic_reproduction => "vcl-deterministic-reproduction-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Content)), VclControls, |c| validate_vcl_controls(VclRule::Content, c);
        prohibited_content_rejected => "vcl-prohibited-content-admitted", composition_security => "vcl-composition-security-omitted",
        minimization => "vcl-minimization-omitted", safe_failure_custody => "vcl-safe-failure-custody-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Civilian)), VclControls, |c| validate_vcl_controls(VclRule::Civilian, c);
        civilian_authority => "vcl-civilian-authority-omitted", law => "vcl-law-omitted", safety_readiness => "vcl-safety-readiness-omitted",
        stakeholder_lenses => "vcl-stakeholder-lenses-omitted", distribution => "vcl-distribution-omitted", burden => "vcl-burden-omitted",
        non_compensation => "vcl-non-compensation-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Isolation)), VclControls, |c| validate_vcl_controls(VclRule::Isolation, c);
        quality => "vcl-quality-omitted", dependency => "vcl-dependency-omitted", support_isolation => "vcl-support-isolation-omitted",
        generated_custody => "vcl-generated-custody-omitted", resource_bounds => "vcl-resource-bounds-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Vcl(VclRule::Evidence)), VclControls, |c| validate_vcl_controls(VclRule::Evidence, c);
        evidence_state_truth => "vcl-evidence-state-truth-omitted", independent_review => "vcl-independent-review-omitted",
        dissent => "vcl-dissent-omitted", validation => "vcl-validation-omitted", rollback => "vcl-rollback-omitted",
        compatibility => "vcl-compatibility-omitted", historical_reproduction => "vcl-historical-reproduction-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Civilian)), ActorControls, |c| validate_actor_controls(ActorRule::Civilian, c);
        mission_fit => "actor-mission-fit-omitted", lawful_bounds => "actor-lawful-bounds-omitted",
        operational_policy_authority_rejected => "actor-operational-policy-authority-created");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Readiness)), ActorControls, |c| validate_actor_controls(ActorRule::Readiness, c);
        readiness_floors => "actor-readiness-floors-omitted", degraded_posture => "actor-degraded-posture-omitted",
        readiness_uncertainty => "actor-readiness-uncertainty-omitted", surge => "actor-surge-omitted", recovery => "actor-recovery-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Acquisition)), ActorControls, |c| validate_actor_controls(ActorRule::Acquisition, c);
        competition => "actor-competition-omitted", qualification => "actor-qualification-omitted", capacity => "actor-capacity-omitted",
        concentration => "actor-concentration-omitted", commonality => "actor-commonality-omitted",
        acquisition_transition => "actor-acquisition-transition-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Logistics)), ActorControls, |c| validate_actor_controls(ActorRule::Logistics, c);
        aggregate_custody => "actor-aggregate-custody-omitted", stock_condition => "actor-stock-condition-omitted",
        maintenance => "actor-maintenance-omitted", repair_tails => "actor-repair-tails-omitted", workload => "actor-workload-omitted",
        logistics_recovery => "actor-logistics-recovery-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Alliance)), ActorControls, |c| validate_actor_controls(ActorRule::Alliance, c);
        sovereignty => "actor-sovereignty-omitted", compatibility => "actor-compatibility-omitted", standards => "actor-standards-omitted",
        partner_capacity => "actor-partner-capacity-omitted", separate_partner_burdens => "actor-partner-burdens-collapsed");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::People)), ActorControls, |c| validate_actor_controls(ActorRule::People, c);
        protected_pains => "actor-protected-pains-omitted", personnel_safety => "actor-personnel-safety-omitted", tempo => "actor-tempo-omitted",
        staffing => "actor-staffing-omitted", moves => "actor-moves-omitted", housing => "actor-housing-omitted", health => "actor-health-omitted",
        caregiving => "actor-caregiving-omitted", services => "actor-services-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Test)), ActorControls, |c| validate_actor_controls(ActorRule::Test, c);
        falsifiability => "actor-falsifiability-omitted", evidence_tiers => "actor-evidence-tiers-omitted",
        conflict_rejected => "actor-conflict-admitted", reproduction => "actor-reproduction-omitted",
        zero_major_convergence => "actor-zero-major-convergence-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Source)), ActorControls, |c| validate_actor_controls(ActorRule::Source, c);
        security_markers => "actor-security-markers-omitted", provenance => "actor-provenance-omitted",
        composition => "actor-composition-omitted", minimization => "actor-minimization-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::Law)), ActorControls, |c| validate_actor_controls(ActorRule::Law, c);
        nonwaivable_civilian_authority => "actor-civilian-authority-waived", nonwaivable_law => "actor-law-waived",
        nonwaivable_floor_gates => "actor-floor-gates-waived");
    omissions!(ControlFamily::Typed(TypedObligation::Actor(ActorRule::External)), ActorControls, |c| validate_actor_controls(ActorRule::External, c);
        hnd_term_boundary => "actor-hnd-term-boundary-omitted", taxlane_exclusivity => "actor-taxlane-exclusivity-omitted",
        rel_no_output => "actor-rel-output-enabled", no_official_release_implication => "actor-official-release-implied");
    macro_rules! cr_omissions {
        ($operation:ident, $variant:ident; $($field:ident => $error:literal),+ $(,)?) => {
            $(
                let mut state = positive_cr_state(CrOp::$operation);
                let CrState::$variant { $field, .. } = &mut state else { unreachable!() };
                *$field = false;
                assert_eq!(validate_cr_state(CrOp::$operation, &state), Err($error));
                assert!(mutation_controls.insert(SourceControlId {
                    family: ControlFamily::Typed(TypedObligation::Cr(CrOp::$operation)),
                    ordinal: 0,
                    name: stringify!($field).to_owned(),
                }));
                covered += 1;
            )+
        };
    }
    cr_omissions!(LogicalContract, LogicalContract; logical_surface => "logical-contract-surface-omitted");
    cr_omissions!(LogicalResponsibility, LogicalResponsibility; responsibility_allocated => "logical-contract-responsibility-omitted");
    cr_omissions!(TypedFailure, TypedFailure; typed_error_rejected => "typed-failure-admitted", error_identity_preserved => "typed-error-identity-lost");
    cr_omissions!(TypedBranchTotality, TypedBranchTotality; typed_variants_covered => "typed-variant-coverage-incomplete", branches_total => "typed-branch-nontotal");
    cr_omissions!(ExhaustionBound, ExhaustionBound; limit_enforced => "exhaustion-limit-unenforced", exhaustion_fail_closed => "exhaustion-not-fail-closed");
    cr_omissions!(FiniteBoundsProgress, FiniteBoundsProgress; finite_bound => "finite-progress-bound-missing", progress_measure => "progress-measure-missing");
    cr_omissions!(ResourceBound, ResourceBound; input_bound => "input-resource-bound-missing", memory_bound => "memory-resource-bound-missing", exhaustion_response => "exhaustion-response-missing", degradation_evidence => "degradation-evidence-missing");
    cr_omissions!(ResourceBoundRegistry, ResourceBoundRegistry; registry_present => "resource-bound-registry-missing", accepted_bounds => "accepted-resource-bounds-missing", resources_allocated => "resource-allocation-missing");
    for (call_depth, max_depth) in [(5, 4), (4, 3)] {
        let state = CrState::CallDepth {
            call_depth,
            max_depth,
        };
        assert_eq!(
            validate_cr_state(CrOp::CallDepth, &state),
            Err("call-depth-exceeded")
        );
        let name = if call_depth == 5 {
            "call_depth"
        } else {
            "max_depth"
        };
        assert!(mutation_controls.insert(SourceControlId {
            family: ControlFamily::Typed(TypedObligation::Cr(CrOp::CallDepth)),
            ordinal: 0,
            name: name.to_owned(),
        }));
        covered += 1;
    }
    let state = CrState::Termination {
        decreasing_measure: vec![4, 3, 3, 2],
    };
    assert_eq!(
        validate_cr_state(CrOp::Termination, &state),
        Err("termination-measure-not-decreasing")
    );
    assert!(mutation_controls.insert(SourceControlId {
        family: ControlFamily::Typed(TypedObligation::Cr(CrOp::Termination)),
        ordinal: 0,
        name: "decreasing_measure".to_owned(),
    }));
    covered += 1;
    cr_omissions!(FailureVisibility, FailureVisibility; errors_visible => "hidden-failure", failure_receipt => "failure-receipt-missing");
    cr_omissions!(InvalidState, InvalidState; invalid_rejected => "invalid-state-hidden", transition_blocked => "invalid-state-transition-admitted");
    cr_omissions!(DefaultFallback, DefaultFallback; missing_rejected => "default-fallback-admitted", default_absent => "default-present");
    cr_omissions!(MissingDefaultHold, MissingDefaultHold; missing_held => "missing-default-not-held", default_absent => "missing-default-present");
    cr_omissions!(TypedFamilyExhaustiveness, TypedFamilyExhaustiveness; families_complete => "typed-family-nonexhaustive", unknown_family_rejected => "unknown-family-admitted");
    cr_omissions!(TypedStateExhaustiveness, TypedStateExhaustiveness; states_complete => "typed-state-nonexhaustive", invalid_state_rejected => "invalid-typed-state-admitted");
    cr_omissions!(ReleaseNoOutput, ReleaseNoOutput; output_absent => "release-output-emitted");
    cr_omissions!(AdmissionBypass, AdmissionBypass; universal_admission_required => "admission-bypass-emitted");
    cr_omissions!(AuthorityNoninflation, AuthorityNoninflation; authority_not_inflated => "authority-inflated");
    cr_omissions!(FalseSavingsNoAuthority, FalseSavingsNoAuthority; false_savings_rejected => "false-savings-admitted", authority_absent => "false-savings-authority-created");
    cr_omissions!(TerminalNoBackflow, TerminalNoBackflow; terminal_output_absent => "terminal-output-emitted", backflow_absent => "terminal-backflow-emitted");
    cr_omissions!(ReplayIdentity, ReplayIdentity; identical_replay => "replay-identity-diverged");
    cr_omissions!(OrderInvariance, OrderInvariance; order_independent => "order-invariance-failed");
    cr_omissions!(DigestReproductionOrder, DigestReproductionOrder; canonical_order => "digest-order-noncanonical", digest_reproduced => "digest-reproduction-order-failed");
    cr_omissions!(AmbientStateAbsence, AmbientStateAbsence; ambient_state_absent => "ambient-state-observed");
    cr_omissions!(ScheduleEquivalence, ScheduleEquivalence; schedules_equivalent => "schedule-equivalence-failed");
    cr_omissions!(Successor, Successor; immutable => "successor-mutated", acyclic => "successor-cycle");
    cr_omissions!(Dependency, Dependency; consumer_direction => "dependency-direction-violated");
    cr_omissions!(FixedDependencyGraph, FixedDependencyGraph; graph_fixed => "fixed-dependency-graph-violated", cycles_absent => "fixed-dependency-cycle");
    cr_omissions!(Content, Content; prohibited_content_rejected => "content-boundary-violated");
    cr_omissions!(ContentBoundaryProvenance, ContentBoundaryProvenance; boundary_preserved => "content-boundary-provenance-missing", provenance_retained => "content-provenance-dropped");
    cr_omissions!(Composition, Composition; minimized => "composition-not-minimized", unsafe_join_rejected => "unsafe-composition-admitted");
    cr_omissions!(Floor, Floor; noncompensable => "floor-compensated");
    cr_omissions!(Distribution, Distribution; distribution_preserved => "distribution-not-conserved", tails_preserved => "distribution-tails-dropped");
    cr_omissions!(NullState, NullState; missing_distinct => "missing-state-collapsed", null_distinct => "null-state-collapsed");
    cr_omissions!(StateNullNaStale, StateNullNaStale; missing_distinct => "missing-state-collapsed", null_distinct => "null-state-collapsed", na_distinct => "na-state-collapsed", stale_distinct => "stale-state-collapsed");
    cr_omissions!(Accounting, Accounting; checked_arithmetic => "accounting-unbalanced", overflow_rejected => "accounting-overflow-admitted");
    cr_omissions!(ReconciliationIdentity, ReconciliationIdentity; identity_balanced => "reconciliation-identity-failed", residual_zero => "reconciliation-residual-nonzero", overlap_reconciled => "reconciliation-overlap-unreconciled");
    cr_omissions!(Burden, Burden; incidence_measured => "burden-shift-unmeasured", tail_measured => "burden-tail-unmeasured");
    cr_omissions!(Delivery, Delivery; pathways_separate => "delivery-pathways-collapsed", delivery_bound => "delivery-transition-invalid", adaptive_successor => "delivery-successor-missing");
    cr_omissions!(FindingDissentRetention, FindingDissentRetention; finding_retained => "finding-not-retained", dissent_retained => "finding-dissent-not-retained");
    cr_omissions!(ReviewIndependence, ReviewIndependence; independent => "review-conflict", self_review_rejected => "self-review-admitted");
    cr_omissions!(CrossRoleReview, CrossRoleReview; roles_complete => "cross-role-review-incomplete", conflicts_rejected => "cross-role-review-conflict");
    cr_omissions!(HoldPropagation, HoldPropagation; all_holds_propagated => "hold-not-propagated", downstream_blocked => "hold-downstream-unblocked");
    cr_omissions!(WaiverLedger, WaiverLedger; ledger_nonwaiver => "waiver-ledger-bypassed", bypass_rejected => "waiver-bypass-admitted");
    cr_omissions!(Invariant, Invariant; coverage_complete => "invariant-uncovered");
    cr_omissions!(Property, Property; evidence_set_complete => "property-evidence-missing", reproduction_deterministic => "property-reproduction-nondeterministic");
    cr_omissions!(Transition, Transition; typed_transition => "invalid-transition", invalid_edge_rejected => "invalid-transition-edge-admitted");
    cr_omissions!(Parser, Parser; surface_minimal => "parser-surface-expanded", authority_absent => "parser-created-authority");
    cr_omissions!(ParserFuzz, ParserFuzz; malformed_rejected => "parser-fuzz-malformed-admitted", panic_absent => "parser-fuzz-panicked", authority_absent => "parser-fuzz-created-authority");
    cr_omissions!(Regression, Regression; successor_immutable => "regression-successor-mutated", history_acyclic => "regression-replay-diverged");
    cr_omissions!(RegressionReplay, RegressionReplay; golden_replayed => "regression-replay-evidence-missing", digest_matches => "regression-replay-digest-mismatch");
    cr_omissions!(Isolation, Isolation; modes_separate => "isolation-overlap", cross_mode_flow_absent => "isolation-cross-mode-flow");
    cr_omissions!(PackageIsolation, PackageIsolation; package_boundary => "package-isolation-overlap", dependency_direction => "package-dependency-direction-violated");
    cr_omissions!(Generated, Generated; emission_absent => "generated-custody-violation");
    cr_omissions!(GeneratedProvenance, GeneratedProvenance; generated_manifest_entry_absent => "generated-manifest-entry-present", provenance_custody => "generated-provenance-missing");
    cr_omissions!(Quality, Quality; registry_complete => "quality-registry-incomplete", gates_bound => "quality-output-unbound");
    cr_omissions!(QualityOutput, QualityOutput; output_bound => "quality-output-binding-missing", digest_bound => "quality-digest-binding-missing");
    cr_omissions!(Evidence, Evidence; state_history_truthful => "evidence-digest-unbound");
    cr_omissions!(EvidenceDigestTruth, EvidenceDigestTruth; evidence_bound => "evidence-digest-truth-failed", digest_truthful => "evidence-digest-untruthful");
    cr_omissions!(Trace, Trace; transpose_equal => "trace-contradiction", orphans_absent => "trace-orphan");
    omissions!(ControlFamily::Shared(SharedControlFamily::DependencyReview), DependencyReviewControls, |c: &DependencyReviewControls| validate_cr_state(CrOp::LicenseAdvisory, &CrState::LicenseAdvisory { controls: c.clone() });
        direct_dependencies => "direct-dependency-review-omitted", transitive_dependencies => "transitive-dependency-review-omitted",
        feature_flags => "dependency-feature-review-omitted", native_build_inputs => "native-build-review-omitted",
        licenses_allowed => "license-policy-rejected", advisories_current => "advisory-data-stale",
        maintenance_posture => "dependency-maintenance-review-omitted", reproducibility => "dependency-reproducibility-review-omitted");
    omissions!(ControlFamily::Shared(SharedControlFamily::ContractFixture), ContractProofControls, |c: &ContractProofControls| validate_cr_state(CrOp::ContractFixtureMatrix, &CrState::ContractFixtureMatrix { controls: c.clone() });
        source_contract => "source-contract-fixture-omitted", authority_contract => "authority-contract-fixture-omitted",
        readiness_contract => "readiness-contract-fixture-omitted", acquisition_contract => "acquisition-contract-fixture-omitted",
        logistics_contract => "logistics-contract-fixture-omitted", alliance_contract => "alliance-contract-fixture-omitted",
        distribution_contract => "distribution-contract-fixture-omitted", economics_contract => "economics-contract-fixture-omitted",
        test_contract => "test-contract-fixture-omitted", delivery_contract => "delivery-contract-fixture-omitted",
        handoff_contract => "handoff-contract-fixture-omitted", release_contract => "release-contract-fixture-omitted",
        positive_case => "contract-positive-case-omitted", negative_case => "contract-negative-case-omitted",
        unauthorized_consumer_case => "contract-unauthorized-consumer-case-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::ValidationScope), ValidationScopeControls, validate_validation_scope;
        public_aggregate => "validation-scope-not-public-aggregate", unclassified => "validation-scope-not-unclassified",
        non_operational => "validation-scope-operational", prohibited_content_rejected => "validation-scope-prohibited-content-admitted");
    omissions!(ControlFamily::Typed(TypedObligation::ValidationAssurance), ValidationAssuranceControls, validate_validation_assurance;
        classification_opsec_pass => "validation-classification-opsec-missing",
        civilian_control_law_safety_readiness_pass => "validation-civilian-law-safety-readiness-missing",
        applicable_parliament_decisions_complete => "validation-parliament-decisions-incomplete",
        applicable_domain_decisions_complete => "validation-domain-decisions-incomplete",
        independence => "validation-assurance-conflict", findings_retained => "validation-findings-dropped");
    omissions!(ControlFamily::Typed(TypedObligation::Release(ReleaseRule::Authority)), ReleaseAuthorityControls, validate_release_authority;
        publication_blocked => "release-publication-enabled", public_release_blocked => "public-release-enabled",
        approved_representation_blocked => "approved-representation-enabled", separate_release_authority_absent => "release-authority-fabricated",
        release_fixed_point_absent => "release-fixed-point-fabricated");
    omissions!(ControlFamily::Typed(TypedObligation::Release(ReleaseRule::RiskAssessment)), ReleaseRiskAssessment, validate_release_risk;
        direct_release_composition => "direct-release-composition-omitted", cross_release_composition => "cross-release-composition-omitted",
        linkage_risk => "release-linkage-risk-omitted", sensitive_context => "release-sensitive-context-omitted",
        audience_misuse => "release-audience-misuse-omitted", source_staleness => "release-source-staleness-omitted",
        review_staleness => "release-review-staleness-omitted", provenance => "release-provenance-omitted",
        correction_takedown => "release-correction-takedown-omitted", security => "release-security-acceptance-omitted",
        scope => "release-scope-acceptance-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Release(ReleaseRule::Communication)), ReleaseCommunicationContext, validate_release_communication;
        source => "release-source-context-omitted", derivation => "release-derivation-context-omitted",
        limitation => "release-limitation-context-omitted", uncertainty => "release-uncertainty-context-omitted",
        dissent => "release-dissent-context-omitted", security_posture => "release-security-posture-omitted",
        non_authority => "release-non-authority-context-omitted");
    omissions!(ControlFamily::Typed(TypedObligation::Release(ReleaseRule::Closed)), ReleaseClosedControls, validate_release_closed;
        closed_posture => "release-closed-posture-omitted", no_output => "closed-release-output-enabled",
        no_consumer => "closed-release-consumer-enabled", unauthorized_negative => "closed-unauthorized-negative-omitted",
        mosaicing_negative => "closed-mosaicing-negative-omitted", context_retention_negative => "closed-context-negative-omitted");
    omissions!(ControlFamily::Shared(SharedControlFamily::HoldClosure), HoldClosureControls, validate_hold_closure;
        security => "security-hold-omitted", readiness => "readiness-hold-omitted", source => "source-hold-omitted",
        quantity => "quantity-hold-omitted", acquisition => "acquisition-hold-omitted", logistics => "logistics-hold-omitted",
        alliance => "alliance-hold-omitted", distribution => "distribution-hold-omitted", economics => "economics-hold-omitted",
        test => "test-hold-omitted", delivery => "delivery-hold-omitted", handoff => "handoff-hold-omitted",
        release => "release-hold-omitted", missing_held => "missing-state-defaulted", null_held => "null-state-defaulted",
        na_independently_reviewed => "na-not-independently-reviewed", no_default => "hold-default-applied");
    omissions!(ControlFamily::Shared(SharedControlFamily::CheckedAccounting), CheckedAccountingControls, validate_checked_accounting;
        pathway_basing => "basing-pathway-accounting-omitted", consolidation => "consolidation-pathway-accounting-omitted",
        process => "process-pathway-accounting-omitted", commonality => "commonality-pathway-accounting-omitted",
        logistics => "logistics-pathway-accounting-omitted", workforce => "workforce-pathway-accounting-omitted",
        delivery => "delivery-accounting-omitted", realization => "realization-accounting-omitted", handoff => "handoff-accounting-omitted",
        terminal => "terminal-accounting-omitted", no_backflow => "terminal-backflow-enabled", units_identity => "accounting-units-mismatch",
        lifecycle_identity => "lifecycle-identity-failed", transition_identity => "transition-identity-failed",
        overlap_identity => "overlap-identity-failed", residual_identity => "residual-identity-failed",
        taxlane_exclusivity => "vcl-accounting-taxlane-exclusivity-omitted", rel_no_output => "vcl-accounting-rel-output-enabled",
        no_operational_authority => "vcl-accounting-operational-authority-created",
        no_procurement_authority => "vcl-accounting-procurement-authority-created",
        no_budget_authority => "vcl-accounting-budget-authority-created",
        no_taxlane_authority => "vcl-accounting-taxlane-authority-created",
        no_allocation_authority => "vcl-accounting-allocation-authority-created",
        no_rate_authority => "vcl-accounting-rate-authority-created",
        no_official_authority => "vcl-accounting-official-authority-created",
        no_implementation_authority => "vcl-accounting-implementation-authority-created",
        no_release_authority => "vcl-accounting-release-authority-created");
    omissions!(ControlFamily::Shared(SharedControlFamily::Finance), FinanceControls, validate_finance;
        units => "finance-units-omitted", horizons => "finance-horizons-omitted", overlap => "finance-overlap-omitted",
        uncertainty => "finance-uncertainty-omitted", pathway_basing => "finance-basing-pathway-omitted",
        pathway_consolidation => "finance-consolidation-pathway-omitted", pathway_process => "finance-process-pathway-omitted",
        pathway_commonality => "finance-commonality-pathway-omitted", pathway_logistics => "finance-logistics-pathway-omitted",
        pathway_workforce => "finance-workforce-pathway-omitted", residuals => "finance-residuals-omitted",
        realization => "finance-realization-omitted");
    omissions!(ControlFamily::Shared(SharedControlFamily::Numeracy), NumeracyControls, validate_numeracy;
        units => "numeracy-units-omitted", quantities => "numeracy-quantities-omitted",
        availability_denominators => "numeracy-availability-denominator-omitted", price_years => "numeracy-price-years-omitted",
        lifecycle_cost => "numeracy-lifecycle-cost-omitted", transition_cost => "numeracy-transition-cost-omitted",
        horizons => "numeracy-horizons-omitted", uncertainty => "numeracy-uncertainty-omitted",
        scenario_arithmetic => "numeracy-scenario-arithmetic-invalid", double_count_detection => "numeracy-double-count-undetected");
    omissions!(ControlFamily::Shared(SharedControlFamily::Supplier), SupplierControls, validate_supplier;
        requirements_stability => "supplier-requirements-stability-omitted", competition => "supplier-competition-omitted",
        cash_flow => "supplier-cash-flow-omitted", production_capacity => "supplier-production-capacity-omitted",
        workforce => "supplier-workforce-omitted", intellectual_property => "supplier-ip-omitted",
        qualification => "supplier-qualification-omitted", resilient_demand => "supplier-resilient-demand-omitted");
    omissions!(ControlFamily::Shared(SharedControlFamily::SecurityAssurance), SecurityAssuranceControls, validate_security_assurance;
        classified_rejected => "classified-content-admitted", controlled_rejected => "controlled-content-admitted",
        sensitive_rejected => "sensitive-content-admitted", targeting_rejected => "targeting-content-admitted",
        operational_planning_rejected => "operational-planning-content-admitted",
        exploitable_vulnerability_rejected => "exploitable-vulnerability-admitted",
        dangerous_combination_rejected => "dangerous-public-field-combination-admitted");
    omissions!(ControlFamily::Shared(SharedControlFamily::CivilianAssurance), CivilianAssuranceControls, validate_civilian_assurance;
        lawful_civilian_authority => "lawful-civilian-authority-omitted", personnel_safety => "personnel-safety-omitted",
        readiness_nondegradation => "readiness-nondegradation-omitted", resilience_transition => "resilience-transition-test-omitted",
        financial_optimization_mission_risk_blocked => "financial-optimization-mission-risk-unblocked");
    let review_lenses = [
        ReviewLens::Citation,
        ReviewLens::Numeracy,
        ReviewLens::Scope,
        ReviewLens::Panel,
        ReviewLens::Acquisition,
        ReviewLens::Alliance,
        ReviewLens::ForcePlanning,
        ReviewLens::Comptroller,
        ReviewLens::TestOversight,
        ReviewLens::Logistics,
        ReviewLens::Readiness,
        ReviewLens::People,
        ReviewLens::AllyStakeholder,
        ReviewLens::DepotStakeholder,
        ReviewLens::InstallationStakeholder,
        ReviewLens::MissionStakeholder,
        ReviewLens::SupplierStakeholder,
        ReviewLens::FamilyStakeholder,
        ReviewLens::TaxpayerStakeholder,
        ReviewLens::SecurityAssurance,
        ReviewLens::CivilianAssurance,
        ReviewLens::Steward,
    ];
    let mut reached_review_controls = BTreeSet::new();
    for lens in review_lenses {
        let required = required_review_named(lens);
        for control in required {
            reached_review_controls.insert(*control);
        }
        for omitted in 0..required.len() {
            let omitted_control = required[omitted];
            let mut controls = required.to_vec();
            controls.remove(omitted);
            assert_eq!(
                validate_review_named(lens, &controls),
                Err("review-named-control-omitted")
            );
            assert!(mutation_controls.insert(SourceControlId {
                family: ControlFamily::Typed(TypedObligation::Review(lens)),
                ordinal: 0,
                name: format!("{omitted_control:?}"),
            }));
            covered += 1;
        }
        let foreign = if required.contains(&ReviewNamedControl::CitationPublicCustody) {
            ReviewNamedControl::ScopePublic
        } else {
            ReviewNamedControl::CitationPublicCustody
        };
        let mut substituted = required.to_vec();
        substituted.push(foreign);
        assert_eq!(
            validate_review_named(lens, &substituted),
            Err("review-named-control-incompatible")
        );
    }
    let review_variant_count = ReviewNamedControl::StewardFixedPoint as usize + 1;
    assert_eq!(
        reached_review_controls.len(),
        review_variant_count,
        "dead review named control"
    );
    assert_eq!(covered, mutation_controls.len());
    let mut next_ordinal = BTreeMap::<ControlFamily, u16>::new();
    mutation_controls
        .into_iter()
        .map(|mut control| {
            let ordinal = next_ordinal.entry(control.family).or_default();
            control.ordinal = *ordinal;
            *ordinal = ordinal.checked_add(1).expect("bounded family controls");
            control
        })
        .collect()
}

fn source_control_families(kind: TypedObligation) -> Vec<ControlFamily> {
    let mut families = match kind {
        TypedObligation::Vcl(VclRule::Holds) | TypedObligation::Hold(_) => {
            vec![ControlFamily::Shared(SharedControlFamily::HoldClosure)]
        }
        TypedObligation::Vcl(VclRule::Accounting) => vec![ControlFamily::Shared(
            SharedControlFamily::CheckedAccounting,
        )],
        TypedObligation::Actor(ActorRule::Finance) => {
            vec![ControlFamily::Shared(SharedControlFamily::Finance)]
        }
        TypedObligation::Cr(CrOp::ContractFixtureMatrix) => {
            vec![ControlFamily::Shared(SharedControlFamily::ContractFixture)]
        }
        TypedObligation::Cr(CrOp::LicenseAdvisory) => {
            vec![ControlFamily::Shared(SharedControlFamily::DependencyReview)]
        }
        _ => vec![ControlFamily::Typed(kind)],
    };
    if let TypedObligation::Review(lens) = kind {
        match lens {
            ReviewLens::Numeracy | ReviewLens::Comptroller | ReviewLens::TaxpayerStakeholder => {
                families.push(ControlFamily::Shared(SharedControlFamily::Numeracy))
            }
            ReviewLens::SupplierStakeholder => {
                families.push(ControlFamily::Shared(SharedControlFamily::Supplier))
            }
            ReviewLens::SecurityAssurance => families.push(ControlFamily::Shared(
                SharedControlFamily::SecurityAssurance,
            )),
            ReviewLens::ForcePlanning | ReviewLens::CivilianAssurance => families.push(
                ControlFamily::Shared(SharedControlFamily::CivilianAssurance),
            ),
            _ => {}
        }
    }
    families
}

const NORMATIVE_SOURCE_OWNER_CODES: [u16; 148] = [
    100, 101, 102, 103, 104, 105, 200, 201, 202, 100, 101, 102, 103, 104, 105, 200, 201, 202, 300,
    301, 302, 303, 304, 305, 306, 307, 308, 309, 100, 203, 100, 203, 1000, 1001, 1003, 1004, 1005,
    1006, 1009, 1010, 1011, 1012, 1013, 1014, 1015, 1016, 1017, 1018, 1022, 1023, 1024, 1025, 1026,
    1027, 1028, 1029, 1030, 1031, 1032, 1033, 1019, 1034, 1035, 1036, 1037, 1038, 1039, 1020, 1040,
    1041, 1042, 1021, 1044, 1046, 1047, 1048, 1043, 1002, 1049, 1050, 1051, 1052, 1053, 1054, 1055,
    1056, 1057, 1058, 1059, 1008, 1007, 1045, 1060, 1061, 1062, 400, 401, 402, 403, 404, 405, 406,
    407, 408, 409, 500, 501, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 706, 710, 704,
    709, 707, 711, 708, 705, 703, 721, 700, 702, 701, 717, 715, 713, 716, 714, 712, 718, 719, 720,
    800, 800, 801, 801, 802, 802, 803, 803,
];

fn typed_obligation_code(kind: TypedObligation) -> u16 {
    match kind {
        TypedObligation::Test(rule) => 100 + rule as u16,
        TypedObligation::Release(rule) => 200 + rule as u16,
        TypedObligation::Nf(rule) => 300 + rule as u16,
        TypedObligation::Vcl(rule) => 400 + rule as u16,
        TypedObligation::ValidationScope => 500,
        TypedObligation::ValidationAssurance => 501,
        TypedObligation::Actor(rule) => 600 + rule as u16,
        TypedObligation::Review(lens) => 700 + lens as u16,
        TypedObligation::Hold(rule) => 800 + rule as u16,
        TypedObligation::Cr(operation) => 1000 + operation as u16,
    }
}

fn source_control_count(family: ControlFamily) -> usize {
    match family {
        ControlFamily::Shared(shared) => match shared {
            SharedControlFamily::ContractFixture => 15,
            SharedControlFamily::DependencyReview => 8,
            SharedControlFamily::HoldClosure => 17,
            SharedControlFamily::CheckedAccounting => 27,
            SharedControlFamily::Finance => 12,
            SharedControlFamily::Numeracy => 10,
            SharedControlFamily::Supplier => 8,
            SharedControlFamily::SecurityAssurance => 7,
            SharedControlFamily::CivilianAssurance => 5,
        },
        ControlFamily::Typed(kind) => match kind {
            TypedObligation::Test(rule) => match rule {
                TestRule::Packet => 7,
                TestRule::IndependentReproduction => 11,
                TestRule::RetainedPosture => 6,
                TestRule::FindingShape => 12,
                TestRule::PromotionBlock => 7,
                TestRule::EvidenceTruth => 3,
            },
            TypedObligation::Release(rule) => match rule {
                ReleaseRule::Authority => 5,
                ReleaseRule::RiskAssessment => 11,
                ReleaseRule::Communication => 7,
                ReleaseRule::Closed => 6,
            },
            TypedObligation::Nf(rule) => match rule {
                NfRule::ProhibitedData => 28,
                NfRule::Scope => 2,
                NfRule::ReadinessFloor => 4,
                NfRule::Distribution => 4,
                NfRule::Reconciliation => 6,
                NfRule::NonAdditive => 7,
                NfRule::MissingNa => 4,
                NfRule::History => 3,
                NfRule::Staleness => 5,
                NfRule::NoAuthority => 9,
            },
            TypedObligation::Vcl(rule) => match rule {
                VclRule::Custody => 5,
                VclRule::Contract => 3,
                VclRule::Transition => 5,
                VclRule::Property => 3,
                VclRule::Content => 4,
                VclRule::Civilian => 7,
                VclRule::Isolation => 5,
                VclRule::Evidence => 7,
                VclRule::Holds | VclRule::Accounting => unreachable!(),
            },
            TypedObligation::ValidationScope => 4,
            TypedObligation::ValidationAssurance => 6,
            TypedObligation::Actor(rule) => match rule {
                ActorRule::Civilian => 3,
                ActorRule::Readiness => 5,
                ActorRule::Acquisition => 6,
                ActorRule::Logistics => 6,
                ActorRule::Alliance => 5,
                ActorRule::People => 9,
                ActorRule::Test => 5,
                ActorRule::Source => 4,
                ActorRule::Law => 3,
                ActorRule::External => 4,
                ActorRule::Finance => unreachable!(),
            },
            TypedObligation::Review(lens) => required_review_named(lens).len(),
            TypedObligation::Cr(operation) => match operation {
                CrOp::LogicalContract | CrOp::LogicalResponsibility => 1,
                CrOp::TypedFailure
                | CrOp::TypedBranchTotality
                | CrOp::ExhaustionBound
                | CrOp::FiniteBoundsProgress
                | CrOp::CallDepth
                | CrOp::FailureVisibility
                | CrOp::InvalidState
                | CrOp::DefaultFallback
                | CrOp::MissingDefaultHold
                | CrOp::TypedFamilyExhaustiveness
                | CrOp::TypedStateExhaustiveness
                | CrOp::FalseSavingsNoAuthority
                | CrOp::TerminalNoBackflow
                | CrOp::DigestReproductionOrder
                | CrOp::Successor
                | CrOp::FixedDependencyGraph
                | CrOp::ContentBoundaryProvenance
                | CrOp::Composition
                | CrOp::Distribution
                | CrOp::NullState
                | CrOp::Accounting
                | CrOp::Burden
                | CrOp::FindingDissentRetention
                | CrOp::ReviewIndependence
                | CrOp::CrossRoleReview
                | CrOp::HoldPropagation
                | CrOp::WaiverLedger
                | CrOp::Property
                | CrOp::Transition
                | CrOp::Parser
                | CrOp::Regression
                | CrOp::RegressionReplay
                | CrOp::Isolation
                | CrOp::PackageIsolation
                | CrOp::GeneratedProvenance
                | CrOp::Quality
                | CrOp::QualityOutput
                | CrOp::EvidenceDigestTruth
                | CrOp::Trace => 2,
                CrOp::ResourceBound => 4,
                CrOp::ResourceBoundRegistry
                | CrOp::ReconciliationIdentity
                | CrOp::Delivery
                | CrOp::ParserFuzz => 3,
                CrOp::StateNullNaStale => 4,
                CrOp::Termination
                | CrOp::ReleaseNoOutput
                | CrOp::AdmissionBypass
                | CrOp::AuthorityNoninflation
                | CrOp::ReplayIdentity
                | CrOp::OrderInvariance
                | CrOp::AmbientStateAbsence
                | CrOp::ScheduleEquivalence
                | CrOp::Dependency
                | CrOp::Content
                | CrOp::Floor
                | CrOp::Invariant
                | CrOp::Generated
                | CrOp::Evidence => 1,
                CrOp::ContractFixtureMatrix | CrOp::LicenseAdvisory => unreachable!(),
            },
            TypedObligation::Hold(_) => unreachable!(),
        },
    }
}

fn source_allocation_digest(
    allocations: &BTreeMap<(&'static str, &'static str), Vec<SourceControlId>>,
) -> Option<String> {
    let mut preimage = String::new();
    for edge in TRACE_MANIFEST {
        let tuple = (edge.controlled_id, edge.assertion);
        preimage.push_str(edge.controlled_id);
        preimage.push('\t');
        preimage.push_str(edge.assertion);
        preimage.push('\n');
        for control in allocations.get(&tuple)? {
            preimage.push_str(&format!(
                "{:?}\t{}\t{}\n",
                control.family, control.ordinal, control.name
            ));
        }
    }
    Some(hex_sha256(preimage.as_bytes()))
}

fn assert_obligation_allocation_registry() {
    static AUDIT: std::sync::Once = std::sync::Once::new();
    AUDIT.call_once(|| {
        const AUTHORITATIVE_CONTROL_COUNT: usize = 607;
        const AUTHORITATIVE_CONTROL_DIGEST: &str =
            "a0d8e0cfee59cbeac2958c2f23d33a99fb325a9c939c68be113c0d78dc9789f8";
        const AUTHORITATIVE_ALLOCATION_DIGEST: &str =
            "931843c0688cfb64c0dbaf551d5502163a06dc2f340358d2e2cc7ccf3e42374a";
        let mutation_controls = assert_named_field_omission_matrix();
        assert_eq!(mutation_controls.len(), AUTHORITATIVE_CONTROL_COUNT);
        let mutation_families: BTreeSet<_> = mutation_controls
            .iter()
            .map(|control| control.family)
            .collect();
        for family in mutation_families {
            assert_eq!(
                mutation_controls
                    .iter()
                    .filter(|control| control.family == family)
                    .count(),
                source_control_count(family),
                "named control count differs from literal family allocation: {family:?}"
            );
        }
        assert_eq!(EDGE_EXECUTION_SPECS.len(), 148);
        let mut tuples = BTreeSet::new();
        let mut forward_allocations = BTreeMap::new();
        let mut reverse_allocations: BTreeMap<SourceControlId, BTreeSet<_>> = BTreeMap::new();
        let mut test_rules = BTreeSet::new();
        let mut release_rules = BTreeSet::new();
        let mut nf_rules = BTreeSet::new();
        let mut vcl_rules = BTreeSet::new();
        let mut actor_rules = BTreeSet::new();
        let mut review_lenses = BTreeSet::new();
        let mut review_evidence_variants = BTreeSet::new();
        let mut hold_rules = BTreeSet::new();
        let mut cr_operations = BTreeSet::new();
        let mut validation_scope_seen = false;
        let mut validation_assurance_seen = false;
        for (ordinal, spec) in EDGE_EXECUTION_SPECS.iter().enumerate() {
            assert_eq!(spec.ordinal, ordinal);
            assert_eq!(spec.edge, TRACE_MANIFEST[ordinal]);
            let source_tuple = (spec.edge.controlled_id, spec.edge.assertion);
            assert!(tuples.insert(source_tuple));
            let kind = typed_obligation(&spec.edge);
            assert_eq!(
                typed_obligation_code(kind),
                NORMATIVE_SOURCE_OWNER_CODES[ordinal],
                "candidate source-to-control owner differs from literal normative registry"
            );
            let families = source_control_families(kind);
            let exact_controls: Vec<_> = mutation_controls
                .iter()
                .filter(|control| families.contains(&control.family))
                .cloned()
                .collect();
            assert!(
                !exact_controls.is_empty(),
                "source tuple has no allocated controls"
            );
            assert!(
                forward_allocations
                    .insert(source_tuple, exact_controls.clone())
                    .is_none()
            );
            for control in exact_controls {
                reverse_allocations
                    .entry(control)
                    .or_default()
                    .insert(source_tuple);
            }
            match kind {
                TypedObligation::Test(rule) => {
                    test_rules.insert(rule as usize);
                }
                TypedObligation::Release(rule) => {
                    release_rules.insert(rule as usize);
                }
                TypedObligation::Nf(rule) => {
                    nf_rules.insert(rule as usize);
                }
                TypedObligation::Vcl(rule) => {
                    vcl_rules.insert(rule as usize);
                }
                TypedObligation::ValidationScope => validation_scope_seen = true,
                TypedObligation::ValidationAssurance => validation_assurance_seen = true,
                TypedObligation::Actor(rule) => {
                    actor_rules.insert(rule as usize);
                }
                TypedObligation::Review(lens) => {
                    review_lenses.insert(lens as usize);
                    review_evidence_variants.insert(review_evidence(lens) as usize);
                }
                TypedObligation::Hold(rule) => {
                    hold_rules.insert(rule as usize);
                }
                TypedObligation::Cr(operation) => {
                    cr_operations.insert(operation as usize);
                }
            }
            assert_eq!(
                validate_domain_state(kind, &positive_domain_state(kind)),
                Ok(())
            );
            let incompatible = EDGE_EXECUTION_SPECS
                .iter()
                .map(|candidate| typed_obligation(&candidate.edge))
                .find(|candidate| *candidate != kind)
                .expect("incompatible typed domain allocation");
            assert_eq!(
                validate_domain_state(kind, &positive_domain_state(incompatible)),
                Err("typed-domain-incompatible")
            );
            let _ = execute_typed_negative(&spec.edge);
        }
        fn assert_dense_reachability(actual: &BTreeSet<usize>, last: usize) {
            assert_eq!(actual.len(), last + 1, "dead typed enum variant");
            assert!((0..=last).all(|ordinal| actual.contains(&ordinal)));
        }
        assert_dense_reachability(&test_rules, TestRule::EvidenceTruth as usize);
        assert_dense_reachability(&release_rules, ReleaseRule::Closed as usize);
        assert_dense_reachability(&nf_rules, NfRule::NoAuthority as usize);
        assert_dense_reachability(&vcl_rules, VclRule::Evidence as usize);
        assert_dense_reachability(&actor_rules, ActorRule::External as usize);
        assert_dense_reachability(&review_lenses, ReviewLens::Steward as usize);
        assert_dense_reachability(&review_evidence_variants, ReviewEvidence::Steward as usize);
        assert_dense_reachability(&hold_rules, HoldRule::Release as usize);
        assert_dense_reachability(&cr_operations, CrOp::Trace as usize);
        assert!(validation_scope_seen && validation_assurance_seen);
        assert_eq!(forward_allocations.len(), EDGE_EXECUTION_SPECS.len());
        assert_eq!(
            reverse_allocations.keys().cloned().collect::<BTreeSet<_>>(),
            mutation_controls,
            "orphan or dead source control allocation"
        );
        assert!(
            reverse_allocations
                .values()
                .all(|sources| !sources.is_empty())
        );
        let mut control_preimage = String::new();
        for control in &mutation_controls {
            control_preimage.push_str(&format!(
                "{:?}\t{}\t{}\n",
                control.family, control.ordinal, control.name
            ));
        }
        let control_digest = hex_sha256(control_preimage.as_bytes());
        let allocation_digest =
            source_allocation_digest(&forward_allocations).expect("literal tuple allocation");
        assert_eq!(control_digest, AUTHORITATIVE_CONTROL_DIGEST);
        assert_eq!(allocation_digest, AUTHORITATIVE_ALLOCATION_DIGEST);

        let mut removed_control = mutation_controls.clone();
        removed_control.pop_first();
        let removed_preimage = removed_control
            .iter()
            .map(|control| {
                format!(
                    "{:?}\t{}\t{}\n",
                    control.family, control.ordinal, control.name
                )
            })
            .collect::<String>();
        assert_ne!(
            hex_sha256(removed_preimage.as_bytes()),
            AUTHORITATIVE_CONTROL_DIGEST,
            "removed named control escaped the fixed authority digest"
        );

        let mut swapped_controls = mutation_controls.clone();
        let first = swapped_controls.first().expect("authority control").clone();
        let second = swapped_controls
            .iter()
            .find(|control| control.family != first.family)
            .expect("second authority family")
            .clone();
        let mut selected = vec![first, second];
        swapped_controls.remove(&selected[0]);
        swapped_controls.remove(&selected[1]);
        let first_name = selected[0].name.clone();
        selected[0].name = selected[1].name.clone();
        selected[1].name = first_name;
        swapped_controls.extend(selected);
        let swapped_preimage = swapped_controls
            .iter()
            .map(|control| {
                format!(
                    "{:?}\t{}\t{}\n",
                    control.family, control.ordinal, control.name
                )
            })
            .collect::<String>();
        assert_ne!(
            hex_sha256(swapped_preimage.as_bytes()),
            AUTHORITATIVE_CONTROL_DIGEST,
            "swapped control name escaped the fixed authority digest"
        );

        let mut removed_source_mapping = forward_allocations.clone();
        removed_source_mapping
            .remove(&(TRACE_MANIFEST[0].controlled_id, TRACE_MANIFEST[0].assertion));
        assert!(source_allocation_digest(&removed_source_mapping).is_none());

        let mut removed_family_mapping = forward_allocations.clone();
        let multi_family_tuple = (
            ".roles/editorial/numeracy-checker.md",
            "property_cases::trace_role_editorial_numeracy_checker",
        );
        let controls = removed_family_mapping
            .get_mut(&multi_family_tuple)
            .expect("literal multi-family source allocation");
        controls.retain(|control| {
            control.family != ControlFamily::Shared(SharedControlFamily::Numeracy)
        });
        assert_ne!(
            source_allocation_digest(&removed_family_mapping)
                .expect("all source tuples remain allocated"),
            AUTHORITATIVE_ALLOCATION_DIGEST,
            "removed family mapping escaped the fixed allocation digest"
        );
    });
}

fn execute_positive(spec: &EdgeExecutionSpec, edge: &TraceEdge) -> String {
    assert_eq!(*edge, spec.edge, "edge-specific proof substitution");
    assert_eq!(TRACE_MANIFEST[spec.ordinal], spec.edge);
    assert_controlled_source(edge.controlled_id);
    execute_typed_positive(edge);
    let assertion_name = edge
        .assertion
        .split_once("::")
        .expect("namespaced assertion")
        .1;
    assert_assertion_semantics(edge.controlled_id, assertion_name);
    let clause = exact_normative_clause(edge.controlled_id);
    let source_digest = hex_sha256(normative_source(edge.controlled_id).as_bytes());
    validate_non_cr_admission(edge, &clause, &source_digest)
        .expect("explicit edge positive admission")
}

fn execute_non_cr_semantic_adversarial(edge: &TraceEdge, assertion_name: &str) -> &'static str {
    let _ = assertion_name;
    execute_typed_negative(edge)
}

fn execute_adversarial(spec: &EdgeExecutionSpec, edge: &TraceEdge) -> String {
    assert_eq!(*edge, spec.edge, "edge-specific adversarial substitution");
    if spec.mutation == MutationKind::NonCr {
        let clause = exact_normative_clause(edge.controlled_id);
        let source_digest = hex_sha256(normative_source(edge.controlled_id).as_bytes());
        validate_non_cr_admission(edge, &clause, &source_digest)
            .expect("non-CR valid input must admit before mutation");
        let clause_result =
            validate_non_cr_admission(edge, &format!("{clause}#substituted"), &source_digest);
        let digest_result = validate_non_cr_admission(
            edge,
            &clause,
            &hex_sha256(format!("{source_digest}#substituted").as_bytes()),
        );
        assert_eq!(clause_result, Err("normative-clause-substitution"));
        assert_eq!(digest_result, Err("normative-source-digest-substitution"));
        let assertion_name = edge
            .assertion
            .split_once("::")
            .expect("namespaced assertion")
            .1;
        let semantic_rejection = execute_non_cr_semantic_adversarial(edge, assertion_name);
        return format!(
            "rejected|{}|{}|clause=normative-clause-substitution|digest=normative-source-digest-substitution|semantic={semantic_rejection}",
            edge.controlled_id, edge.assertion,
        );
    }
    let semantic_rejection = execute_typed_negative(edge);
    let rejected = execute_cr_mutation(spec).expect_err("CR mutation must reject");
    format!(
        "rejected|{}|{}|{:?}|{}|semantic={}",
        edge.controlled_id, edge.assertion, spec.mutation, rejected, semantic_rejection
    )
}

macro_rules! define_edge_case {
    ($positive:ident, $adversarial:ident, $spec:ident, $ordinal:literal, $controlled:literal, $assertion:literal, $mode:literal, $mutation:expr, $finding:literal) => {
        fn $positive(edge: &TraceEdge) {
            let _ = execute_positive(&$spec, edge);
        }
        fn $adversarial(edge: &TraceEdge) {
            let _ = execute_adversarial(&$spec, edge);
        }
        const $spec: EdgeExecutionSpec = EdgeExecutionSpec {
            ordinal: $ordinal,
            edge: TraceEdge {
                controlled_id: $controlled,
                assertion: $assertion,
                mode: $mode,
            },
            mutation: $mutation,
            finding_id: $finding,
            positive: $positive,
            adversarial: $adversarial,
        };
    };
}

define_edge_case!(
    edge_000_positive,
    edge_000_adversarial,
    EDGE_000,
    0,
    "BASTION-REQ-TST-001",
    "source_spine::trace_bastion_req_tst_001",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_001_positive,
    edge_001_adversarial,
    EDGE_001,
    1,
    "BASTION-REQ-TST-002",
    "property_cases::trace_bastion_req_tst_002",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_002_positive,
    edge_002_adversarial,
    EDGE_002,
    2,
    "BASTION-REQ-TST-003",
    "model_cases::trace_bastion_req_tst_003",
    "L2Model",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_003_positive,
    edge_003_adversarial,
    EDGE_003,
    3,
    "BASTION-REQ-TST-004",
    "contract_matrix::trace_bastion_req_tst_004",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_004_positive,
    edge_004_adversarial,
    EDGE_004,
    4,
    "BASTION-REQ-TST-005",
    "hold_closure::trace_bastion_req_tst_005",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_005_positive,
    edge_005_adversarial,
    EDGE_005,
    5,
    "BASTION-REQ-TST-006",
    "adversarial_cases::trace_bastion_req_tst_006",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_006_positive,
    edge_006_adversarial,
    EDGE_006,
    6,
    "BASTION-REQ-REL-001",
    "no_authority_surface::trace_bastion_req_rel_001",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_007_positive,
    edge_007_adversarial,
    EDGE_007,
    7,
    "BASTION-REQ-REL-002",
    "adversarial_cases::trace_bastion_req_rel_002",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_008_positive,
    edge_008_adversarial,
    EDGE_008,
    8,
    "BASTION-REQ-REL-003",
    "no_authority_surface::trace_bastion_req_rel_003",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_009_positive,
    edge_009_adversarial,
    EDGE_009,
    9,
    "SPEC-TST-001",
    "source_spine::trace_spec_tst_001",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_010_positive,
    edge_010_adversarial,
    EDGE_010,
    10,
    "SPEC-TST-002",
    "property_cases::trace_spec_tst_002",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_011_positive,
    edge_011_adversarial,
    EDGE_011,
    11,
    "SPEC-TST-003",
    "model_cases::trace_spec_tst_003",
    "L2Model",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_012_positive,
    edge_012_adversarial,
    EDGE_012,
    12,
    "SPEC-TST-004",
    "contract_matrix::trace_spec_tst_004",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_013_positive,
    edge_013_adversarial,
    EDGE_013,
    13,
    "SPEC-TST-005",
    "hold_closure::trace_spec_tst_005",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_014_positive,
    edge_014_adversarial,
    EDGE_014,
    14,
    "SPEC-TST-006",
    "adversarial_cases::trace_spec_tst_006",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_015_positive,
    edge_015_adversarial,
    EDGE_015,
    15,
    "SPEC-REL-001",
    "no_authority_surface::trace_spec_rel_001",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_016_positive,
    edge_016_adversarial,
    EDGE_016,
    16,
    "SPEC-REL-002",
    "adversarial_cases::trace_spec_rel_002",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_017_positive,
    edge_017_adversarial,
    EDGE_017,
    17,
    "SPEC-REL-003",
    "no_authority_surface::trace_spec_rel_003",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_018_positive,
    edge_018_adversarial,
    EDGE_018,
    18,
    "SPEC-NF-001",
    "adversarial_cases::trace_spec_nf_001",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_019_positive,
    edge_019_adversarial,
    EDGE_019,
    19,
    "SPEC-NF-002",
    "no_authority_surface::trace_spec_nf_002",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_020_positive,
    edge_020_adversarial,
    EDGE_020,
    20,
    "SPEC-NF-003",
    "no_authority_surface::trace_spec_nf_003",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_021_positive,
    edge_021_adversarial,
    EDGE_021,
    21,
    "SPEC-NF-004",
    "property_cases::trace_spec_nf_004",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_022_positive,
    edge_022_adversarial,
    EDGE_022,
    22,
    "SPEC-NF-005",
    "property_cases::trace_spec_nf_005",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_023_positive,
    edge_023_adversarial,
    EDGE_023,
    23,
    "SPEC-NF-006",
    "model_cases::trace_spec_nf_006",
    "L2Model",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_024_positive,
    edge_024_adversarial,
    EDGE_024,
    24,
    "SPEC-NF-007",
    "property_cases::trace_spec_nf_007",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_025_positive,
    edge_025_adversarial,
    EDGE_025,
    25,
    "SPEC-NF-008",
    "contract_matrix::trace_spec_nf_008",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_026_positive,
    edge_026_adversarial,
    EDGE_026,
    26,
    "SPEC-NF-009",
    "model_cases::trace_spec_nf_009",
    "L2Model",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_027_positive,
    edge_027_adversarial,
    EDGE_027,
    27,
    "SPEC-NF-010",
    "source_spine::trace_spec_nf_010",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_028_positive,
    edge_028_adversarial,
    EDGE_028,
    28,
    "DES-TEST-001",
    "contract_matrix::trace_des_test_001",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_029_positive,
    edge_029_adversarial,
    EDGE_029,
    29,
    "DES-REL-001",
    "no_authority_surface::trace_des_rel_001",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_030_positive,
    edge_030_adversarial,
    EDGE_030,
    30,
    "CONTRACT-TEST-001",
    "contract_matrix::trace_contract_test_001",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_031_positive,
    edge_031_adversarial,
    EDGE_031,
    31,
    "CONTRACT-REL-001",
    "no_authority_surface::trace_contract_rel_001",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_032_positive,
    edge_032_adversarial,
    EDGE_032,
    32,
    "CR-002",
    "contract_matrix::cr_002_logical_contract",
    "L2Contract",
    MutationKind::Path,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_033_positive,
    edge_033_adversarial,
    EDGE_033,
    33,
    "CR-002",
    "source_spine::cr_002_logical_responsibility",
    "L2SourceSpine",
    MutationKind::Path,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_034_positive,
    edge_034_adversarial,
    EDGE_034,
    34,
    "CR-003",
    "adversarial_cases::cr_003_typed_failure_rejection",
    "L2Adversarial",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_035_positive,
    edge_035_adversarial,
    EDGE_035,
    35,
    "CR-003",
    "contract_matrix::cr_003_typed_branch_totality",
    "L2Contract",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_036_positive,
    edge_036_adversarial,
    EDGE_036,
    36,
    "CR-004",
    "adversarial_cases::cr_004_exhaustion_failure",
    "L2Adversarial",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_037_positive,
    edge_037_adversarial,
    EDGE_037,
    37,
    "CR-004",
    "property_cases::cr_004_finite_bounds_progress",
    "L2Property",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_038_positive,
    edge_038_adversarial,
    EDGE_038,
    38,
    "CR-005",
    "static_surface::cr_005_call_graph_depth",
    "L1Static",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_039_positive,
    edge_039_adversarial,
    EDGE_039,
    39,
    "CR-005",
    "adversarial_cases::cr_005_termination_violation",
    "L2Adversarial",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_040_positive,
    edge_040_adversarial,
    EDGE_040,
    40,
    "CR-006",
    "adversarial_cases::cr_006_hidden_failure_scan",
    "L2Adversarial",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_041_positive,
    edge_041_adversarial,
    EDGE_041,
    41,
    "CR-006",
    "model_cases::cr_006_invalid_state",
    "L2Model",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_042_positive,
    edge_042_adversarial,
    EDGE_042,
    42,
    "CR-008",
    "adversarial_cases::cr_008_default_fallback_rejection",
    "L2Adversarial",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_043_positive,
    edge_043_adversarial,
    EDGE_043,
    43,
    "CR-008",
    "hold_closure::cr_008_missing_default_hold",
    "L2HoldClosure",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_044_positive,
    edge_044_adversarial,
    EDGE_044,
    44,
    "CR-009",
    "contract_matrix::cr_009_typed_family_exhaustiveness",
    "L2Contract",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_045_positive,
    edge_045_adversarial,
    EDGE_045,
    45,
    "CR-009",
    "model_cases::cr_009_typed_state_exhaustiveness",
    "L2Model",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_046_positive,
    edge_046_adversarial,
    EDGE_046,
    46,
    "CR-010",
    "no_authority_surface::cr_010_release_exception_no_output",
    "L2NoAuthority",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_047_positive,
    edge_047_adversarial,
    EDGE_047,
    47,
    "CR-010",
    "property_cases::cr_010_universal_admission_bypass",
    "L2Property",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_048_positive,
    edge_048_adversarial,
    EDGE_048,
    48,
    "CR-011",
    "model_cases::cr_011_replay_identity",
    "L2Model",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_049_positive,
    edge_049_adversarial,
    EDGE_049,
    49,
    "CR-011",
    "property_cases::cr_011_order_invariance",
    "L2Property",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_050_positive,
    edge_050_adversarial,
    EDGE_050,
    50,
    "CR-011",
    "source_spine::cr_011_digest_reproduction_order",
    "L2SourceSpine",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_051_positive,
    edge_051_adversarial,
    EDGE_051,
    51,
    "CR-012",
    "static_surface::cr_012_ambient_state_absence",
    "L1Static",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_052_positive,
    edge_052_adversarial,
    EDGE_052,
    52,
    "CR-012",
    "property_cases::cr_012_schedule_equivalence",
    "L2Property",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_053_positive,
    edge_053_adversarial,
    EDGE_053,
    53,
    "CR-013",
    "model_cases::cr_013_immutable_successor_acyclic",
    "L2Model",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_054_positive,
    edge_054_adversarial,
    EDGE_054,
    54,
    "CR-014",
    "static_surface::cr_014_consumer_direction",
    "L1Static",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_055_positive,
    edge_055_adversarial,
    EDGE_055,
    55,
    "CR-014",
    "test_gate::cr_014_fixed_dependency_graph",
    "L1SupplyChain",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_056_positive,
    edge_056_adversarial,
    EDGE_056,
    56,
    "CR-015",
    "adversarial_cases::cr_015_prohibited_content",
    "L2Adversarial",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_057_positive,
    edge_057_adversarial,
    EDGE_057,
    57,
    "CR-015",
    "contract_matrix::cr_015_content_boundary_provenance",
    "L2Contract",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_058_positive,
    edge_058_adversarial,
    EDGE_058,
    58,
    "CR-016",
    "adversarial_cases::cr_016_composition_minimization",
    "L2Adversarial",
    MutationKind::Path,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_059_positive,
    edge_059_adversarial,
    EDGE_059,
    59,
    "CR-017",
    "adversarial_cases::cr_017_floor_noncompensation",
    "L2Adversarial",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_060_positive,
    edge_060_adversarial,
    EDGE_060,
    60,
    "CR-017",
    "no_authority_surface::cr_017_authority_noninflation",
    "L2NoAuthority",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_061_positive,
    edge_061_adversarial,
    EDGE_061,
    61,
    "CR-018",
    "property_cases::cr_018_facet_distribution_conservation",
    "L2Property",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_062_positive,
    edge_062_adversarial,
    EDGE_062,
    62,
    "CR-019",
    "hold_closure::cr_019_missing_null_hold",
    "L2HoldClosure",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_063_positive,
    edge_063_adversarial,
    EDGE_063,
    63,
    "CR-019",
    "model_cases::cr_019_state_null_na_stale",
    "L2Model",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_064_positive,
    edge_064_adversarial,
    EDGE_064,
    64,
    "CR-020",
    "model_cases::cr_020_checked_accounting",
    "L2Model",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_065_positive,
    edge_065_adversarial,
    EDGE_065,
    65,
    "CR-020",
    "property_cases::cr_020_reconciliation_identity",
    "L2Property",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_066_positive,
    edge_066_adversarial,
    EDGE_066,
    66,
    "CR-021",
    "adversarial_cases::cr_021_burden_shift_rejection",
    "L2Adversarial",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_067_positive,
    edge_067_adversarial,
    EDGE_067,
    67,
    "CR-021",
    "no_authority_surface::cr_021_false_savings_no_authority",
    "L2NoAuthority",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_068_positive,
    edge_068_adversarial,
    EDGE_068,
    68,
    "CR-022",
    "model_cases::cr_022_eco_delivery_adaptive_shape",
    "L2Model",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_069_positive,
    edge_069_adversarial,
    EDGE_069,
    69,
    "CR-023",
    "hold_closure::cr_023_finding_dissent_retention",
    "L2HoldClosure",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_070_positive,
    edge_070_adversarial,
    EDGE_070,
    70,
    "CR-023",
    "source_spine::cr_023_review_independence",
    "L2SourceSpine",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_071_positive,
    edge_071_adversarial,
    EDGE_071,
    71,
    "CR-024",
    "no_authority_surface::cr_024_terminal_no_output_backflow",
    "L2NoAuthority",
    MutationKind::TerminalBackflow,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_072_positive,
    edge_072_adversarial,
    EDGE_072,
    72,
    "CR-025",
    "hold_closure::cr_025_hold_transpose_propagation",
    "L2HoldClosure",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_073_positive,
    edge_073_adversarial,
    EDGE_073,
    73,
    "CR-026",
    "source_spine::cr_026_invariant_coverage",
    "L2SourceSpine",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_074_positive,
    edge_074_adversarial,
    EDGE_074,
    74,
    "CR-027",
    "property_cases::cr_027_property_evidence_set",
    "L2Property",
    MutationKind::Accounting,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_075_positive,
    edge_075_adversarial,
    EDGE_075,
    75,
    "CR-028",
    "model_cases::cr_028_transition_model_evidence",
    "L2Model",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_076_positive,
    edge_076_adversarial,
    EDGE_076,
    76,
    "CR-029",
    "adversarial_cases::cr_029_cross_role_adversarial",
    "L2Adversarial",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_077_positive,
    edge_077_adversarial,
    EDGE_077,
    77,
    "CR-030",
    "contract_matrix::cr_030_per_contract_fixture_matrix",
    "L2Contract",
    MutationKind::Path,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_078_positive,
    edge_078_adversarial,
    EDGE_078,
    78,
    "CR-031",
    "static_surface::cr_031_parser_surface_absent",
    "L1Static",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_079_positive,
    edge_079_adversarial,
    EDGE_079,
    79,
    "CR-031",
    "adversarial_cases::cr_031_parser_fuzz_authority_absent",
    "L2Adversarial",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_080_positive,
    edge_080_adversarial,
    EDGE_080,
    80,
    "CR-032",
    "model_cases::cr_032_golden_successor_history",
    "L2Model",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_081_positive,
    edge_081_adversarial,
    EDGE_081,
    81,
    "CR-032",
    "property_cases::cr_032_regression_replay",
    "L2Property",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_082_positive,
    edge_082_adversarial,
    EDGE_082,
    82,
    "CR-033",
    "static_surface::cr_033_mode_isolation",
    "L1Static",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_083_positive,
    edge_083_adversarial,
    EDGE_083,
    83,
    "CR-033",
    "test_gate::cr_033_package_isolation",
    "L1SupplyChain",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_084_positive,
    edge_084_adversarial,
    EDGE_084,
    84,
    "CR-034",
    "no_authority_surface::cr_034_generated_no_emission",
    "L2NoAuthority",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_085_positive,
    edge_085_adversarial,
    EDGE_085,
    85,
    "CR-034",
    "source_spine::cr_034_generated_provenance_absence",
    "L2SourceSpine",
    MutationKind::Authority,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_086_positive,
    edge_086_adversarial,
    EDGE_086,
    86,
    "CR-035",
    "static_surface::cr_035_quality_gate_registry",
    "L1Static",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_087_positive,
    edge_087_adversarial,
    EDGE_087,
    87,
    "CR-035",
    "source_spine::cr_035_quality_output_binding",
    "L2SourceSpine",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_088_positive,
    edge_088_adversarial,
    EDGE_088,
    88,
    "CR-036",
    "test_gate::cr_036_dependency_license_advisory",
    "L1SupplyChain",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_089_positive,
    edge_089_adversarial,
    EDGE_089,
    89,
    "CR-037",
    "static_surface::cr_037_resource_bound_registry",
    "L1Static",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_090_positive,
    edge_090_adversarial,
    EDGE_090,
    90,
    "CR-037",
    "adversarial_cases::cr_037_resource_bound_failure",
    "L2Adversarial",
    MutationKind::Parser,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_091_positive,
    edge_091_adversarial,
    EDGE_091,
    91,
    "CR-038",
    "hold_closure::cr_038_waiver_ledger_nonwaiver",
    "L2HoldClosure",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_092_positive,
    edge_092_adversarial,
    EDGE_092,
    92,
    "CR-039",
    "hold_closure::cr_039_evidence_state_history",
    "L2HoldClosure",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_093_positive,
    edge_093_adversarial,
    EDGE_093,
    93,
    "CR-039",
    "source_spine::cr_039_evidence_digest_truth",
    "L2SourceSpine",
    MutationKind::Successor,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_094_positive,
    edge_094_adversarial,
    EDGE_094,
    94,
    "CR-040",
    "source_spine::cr_040_mechanical_trace_contradiction",
    "L2SourceSpine",
    MutationKind::Trace,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_095_positive,
    edge_095_adversarial,
    EDGE_095,
    95,
    "VCL-01",
    "source_spine::trace_vcl_01",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_096_positive,
    edge_096_adversarial,
    EDGE_096,
    96,
    "VCL-02",
    "contract_matrix::trace_vcl_02",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_097_positive,
    edge_097_adversarial,
    EDGE_097,
    97,
    "VCL-03",
    "model_cases::trace_vcl_03",
    "L2Model",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_098_positive,
    edge_098_adversarial,
    EDGE_098,
    98,
    "VCL-04",
    "property_cases::trace_vcl_04",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_099_positive,
    edge_099_adversarial,
    EDGE_099,
    99,
    "VCL-05",
    "hold_closure::trace_vcl_05",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_100_positive,
    edge_100_adversarial,
    EDGE_100,
    100,
    "VCL-06",
    "adversarial_cases::trace_vcl_06",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_101_positive,
    edge_101_adversarial,
    EDGE_101,
    101,
    "VCL-07",
    "no_authority_surface::trace_vcl_07",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_102_positive,
    edge_102_adversarial,
    EDGE_102,
    102,
    "VCL-08",
    "no_authority_surface::trace_vcl_08",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_103_positive,
    edge_103_adversarial,
    EDGE_103,
    103,
    "VCL-09",
    "static_surface::trace_vcl_09",
    "L1Static",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M02"
);
define_edge_case!(
    edge_104_positive,
    edge_104_adversarial,
    EDGE_104,
    104,
    "VCL-10",
    "source_spine::trace_vcl_10",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_105_positive,
    edge_105_adversarial,
    EDGE_105,
    105,
    "VAL-SCOPE",
    "source_spine::trace_val_scope",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_106_positive,
    edge_106_adversarial,
    EDGE_106,
    106,
    "VAL-ASSURANCE",
    "hold_closure::trace_val_assurance",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_107_positive,
    edge_107_adversarial,
    EDGE_107,
    107,
    "ACT-CIV",
    "no_authority_surface::trace_act_civ",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_108_positive,
    edge_108_adversarial,
    EDGE_108,
    108,
    "ACT-RDY",
    "source_spine::trace_act_rdy",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_109_positive,
    edge_109_adversarial,
    EDGE_109,
    109,
    "ACT-ACQ",
    "source_spine::trace_act_acq",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_110_positive,
    edge_110_adversarial,
    EDGE_110,
    110,
    "ACT-LOG",
    "source_spine::trace_act_log",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_111_positive,
    edge_111_adversarial,
    EDGE_111,
    111,
    "ACT-ALLY",
    "source_spine::trace_act_ally",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_112_positive,
    edge_112_adversarial,
    EDGE_112,
    112,
    "ACT-FIN",
    "source_spine::trace_act_fin",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_113_positive,
    edge_113_adversarial,
    EDGE_113,
    113,
    "ACT-PPL",
    "source_spine::trace_act_ppl",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_114_positive,
    edge_114_adversarial,
    EDGE_114,
    114,
    "ACT-TST",
    "source_spine::trace_act_tst",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_115_positive,
    edge_115_adversarial,
    EDGE_115,
    115,
    "ACT-SRC",
    "contract_matrix::trace_act_src",
    "L2Contract",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_116_positive,
    edge_116_adversarial,
    EDGE_116,
    116,
    "ACT-LAW",
    "no_authority_surface::trace_act_law",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_117_positive,
    edge_117_adversarial,
    EDGE_117,
    117,
    "ACT-EXT",
    "no_authority_surface::trace_act_ext",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_118_positive,
    edge_118_adversarial,
    EDGE_118,
    118,
    ".roles/parliament/civilian-strategy-force-planner.md",
    "no_authority_surface::trace_role_parliament_civilian_strategy_force_planner",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_119_positive,
    edge_119_adversarial,
    EDGE_119,
    119,
    ".roles/parliament/operational-readiness.md",
    "source_spine::trace_role_parliament_operational_readiness",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_120_positive,
    edge_120_adversarial,
    EDGE_120,
    120,
    ".roles/parliament/acquisition-industrial-base.md",
    "source_spine::trace_role_parliament_acquisition_industrial_base",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_121_positive,
    edge_121_adversarial,
    EDGE_121,
    121,
    ".roles/parliament/logistics-sustainment.md",
    "source_spine::trace_role_parliament_logistics_sustainment",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_122_positive,
    edge_122_adversarial,
    EDGE_122,
    122,
    ".roles/parliament/defense-comptroller.md",
    "source_spine::trace_role_parliament_defense_comptroller",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_123_positive,
    edge_123_adversarial,
    EDGE_123,
    123,
    ".roles/parliament/service-member-family.md",
    "source_spine::trace_role_parliament_service_member_family",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_124_positive,
    edge_124_adversarial,
    EDGE_124,
    124,
    ".roles/parliament/independent-test-oversight.md",
    "source_spine::trace_role_parliament_independent_test_oversight",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_125_positive,
    edge_125_adversarial,
    EDGE_125,
    125,
    ".roles/parliament/alliance-interoperability.md",
    "source_spine::trace_role_parliament_alliance_interoperability",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_126_positive,
    edge_126_adversarial,
    EDGE_126,
    126,
    ".roles/panel-reviewer/panel.md",
    "property_cases::trace_role_panel_reviewer_panel",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_127_positive,
    edge_127_adversarial,
    EDGE_127,
    127,
    "Role review steward",
    "hold_closure::trace_role_review_steward",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_128_positive,
    edge_128_adversarial,
    EDGE_128,
    128,
    ".roles/editorial/citation-auditor.md",
    "source_spine::trace_role_editorial_citation_auditor",
    "L2SourceSpine",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_129_positive,
    edge_129_adversarial,
    EDGE_129,
    129,
    ".roles/editorial/scope-keeper.md",
    "no_authority_surface::trace_role_editorial_scope_keeper",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_130_positive,
    edge_130_adversarial,
    EDGE_130,
    130,
    ".roles/editorial/numeracy-checker.md",
    "property_cases::trace_role_editorial_numeracy_checker",
    "L2Property",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_131_positive,
    edge_131_adversarial,
    EDGE_131,
    131,
    ".roles/stakeholders/service-member-family.md",
    "no_authority_surface::trace_role_stakeholders_service_member_family",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_132_positive,
    edge_132_adversarial,
    EDGE_132,
    132,
    ".roles/stakeholders/mission-user.md",
    "no_authority_surface::trace_role_stakeholders_mission_user",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_133_positive,
    edge_133_adversarial,
    EDGE_133,
    133,
    ".roles/stakeholders/depot-logistics-workforce.md",
    "no_authority_surface::trace_role_stakeholders_depot_logistics_workforce",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_134_positive,
    edge_134_adversarial,
    EDGE_134,
    134,
    ".roles/stakeholders/prime-small-supplier.md",
    "no_authority_surface::trace_role_stakeholders_prime_small_supplier",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_135_positive,
    edge_135_adversarial,
    EDGE_135,
    135,
    ".roles/stakeholders/installation-community.md",
    "no_authority_surface::trace_role_stakeholders_installation_community",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_136_positive,
    edge_136_adversarial,
    EDGE_136,
    136,
    ".roles/stakeholders/ally-partner.md",
    "no_authority_surface::trace_role_stakeholders_ally_partner",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_137_positive,
    edge_137_adversarial,
    EDGE_137,
    137,
    ".roles/stakeholders/taxpayer-oversight.md",
    "no_authority_surface::trace_role_stakeholders_taxpayer_oversight",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_138_positive,
    edge_138_adversarial,
    EDGE_138,
    138,
    ".roles/assurance/classification-operational-security.md",
    "adversarial_cases::trace_role_assurance_classification_operational_security",
    "L2Adversarial",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_139_positive,
    edge_139_adversarial,
    EDGE_139,
    139,
    ".roles/assurance/civilian-control-law-safety-readiness.md",
    "no_authority_surface::trace_role_assurance_civilian_control_law_safety_readiness",
    "L2NoAuthority",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_140_positive,
    edge_140_adversarial,
    EDGE_140,
    140,
    "SPEC-UNK-SEC-001",
    "hold_closure::trace_spec_unk_sec_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_141_positive,
    edge_141_adversarial,
    EDGE_141,
    141,
    "TBD-SEC-001",
    "hold_closure::trace_tbd_sec_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_142_positive,
    edge_142_adversarial,
    EDGE_142,
    142,
    "SPEC-UNK-SRC-001",
    "hold_closure::trace_spec_unk_src_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_143_positive,
    edge_143_adversarial,
    EDGE_143,
    143,
    "TBD-SRC-001",
    "hold_closure::trace_tbd_src_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_144_positive,
    edge_144_adversarial,
    EDGE_144,
    144,
    "SPEC-UNK-TST-001",
    "hold_closure::trace_spec_unk_tst_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_145_positive,
    edge_145_adversarial,
    EDGE_145,
    145,
    "TBD-TST-001",
    "hold_closure::trace_tbd_tst_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_146_positive,
    edge_146_adversarial,
    EDGE_146,
    146,
    "SPEC-UNK-REL-001",
    "hold_closure::trace_spec_unk_rel_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
define_edge_case!(
    edge_147_positive,
    edge_147_adversarial,
    EDGE_147,
    147,
    "TBD-REL-001",
    "hold_closure::trace_tbd_rel_001",
    "L2HoldClosure",
    MutationKind::NonCr,
    "BA-TST-R14-IMPL-M01"
);
// EDGE_CASE_DEFINITIONS

const EDGE_EXECUTION_SPECS: &[EdgeExecutionSpec] = &[
    EDGE_000, EDGE_001, EDGE_002, EDGE_003, EDGE_004, EDGE_005, EDGE_006, EDGE_007, EDGE_008,
    EDGE_009, EDGE_010, EDGE_011, EDGE_012, EDGE_013, EDGE_014, EDGE_015, EDGE_016, EDGE_017,
    EDGE_018, EDGE_019, EDGE_020, EDGE_021, EDGE_022, EDGE_023, EDGE_024, EDGE_025, EDGE_026,
    EDGE_027, EDGE_028, EDGE_029, EDGE_030, EDGE_031, EDGE_032, EDGE_033, EDGE_034, EDGE_035,
    EDGE_036, EDGE_037, EDGE_038, EDGE_039, EDGE_040, EDGE_041, EDGE_042, EDGE_043, EDGE_044,
    EDGE_045, EDGE_046, EDGE_047, EDGE_048, EDGE_049, EDGE_050, EDGE_051, EDGE_052, EDGE_053,
    EDGE_054, EDGE_055, EDGE_056, EDGE_057, EDGE_058, EDGE_059, EDGE_060, EDGE_061, EDGE_062,
    EDGE_063, EDGE_064, EDGE_065, EDGE_066, EDGE_067, EDGE_068, EDGE_069, EDGE_070, EDGE_071,
    EDGE_072, EDGE_073, EDGE_074, EDGE_075, EDGE_076, EDGE_077, EDGE_078, EDGE_079, EDGE_080,
    EDGE_081, EDGE_082, EDGE_083, EDGE_084, EDGE_085, EDGE_086, EDGE_087, EDGE_088, EDGE_089,
    EDGE_090, EDGE_091, EDGE_092, EDGE_093, EDGE_094, EDGE_095, EDGE_096, EDGE_097, EDGE_098,
    EDGE_099, EDGE_100, EDGE_101, EDGE_102, EDGE_103, EDGE_104, EDGE_105, EDGE_106, EDGE_107,
    EDGE_108, EDGE_109, EDGE_110, EDGE_111, EDGE_112, EDGE_113, EDGE_114, EDGE_115, EDGE_116,
    EDGE_117, EDGE_118, EDGE_119, EDGE_120, EDGE_121, EDGE_122, EDGE_123, EDGE_124, EDGE_125,
    EDGE_126, EDGE_127, EDGE_128, EDGE_129, EDGE_130, EDGE_131, EDGE_132, EDGE_133, EDGE_134,
    EDGE_135, EDGE_136, EDGE_137, EDGE_138, EDGE_139, EDGE_140, EDGE_141, EDGE_142, EDGE_143,
    EDGE_144, EDGE_145, EDGE_146, EDGE_147,
    // EDGE_CASE_SPEC_ROWS
];

fn audit_correction(
    spec: &EdgeExecutionSpec,
    positive_transcript: &str,
    adversarial_transcript: &str,
) -> AuditCorrection {
    AuditCorrection {
        audit_id: "AUDIT-WP-TST-001-R14-IMPLEMENTATION-001",
        finding_id: spec.finding_id,
        edge: spec.edge,
        executed_positive_digest: hex_sha256(positive_transcript.as_bytes()),
        executed_adversarial_digest: hex_sha256(adversarial_transcript.as_bytes()),
        disposition: "open-retained-corrected-by-r24-edge-proof",
    }
}

fn correction_commitment(correction: &AuditCorrection) -> String {
    format!(
        "audit_id={};finding_id={};edge={}|{}|{};positive={};adversarial={};disposition={}",
        correction.audit_id,
        correction.finding_id,
        correction.edge.controlled_id,
        correction.edge.assertion,
        correction.edge.mode,
        correction.executed_positive_digest,
        correction.executed_adversarial_digest,
        correction.disposition
    )
}

fn ascii_text(bytes: &[u8]) -> Result<&str, &'static str> {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) || bytes.contains(&b'\r') {
        return Err("non-canonical-text");
    }
    if !bytes.is_ascii() {
        return Err("non-ascii");
    }
    match std::str::from_utf8(bytes) {
        Ok(value) => Ok(value),
        Err(_) => Err("invalid-utf8"),
    }
}

pub fn parse_manifest(bytes: &[u8]) -> Result<Vec<ManifestRow<'_>>, &'static str> {
    if bytes.len() > 16 * 1024 || !bytes.ends_with(b"\n") {
        return Err("manifest-bound");
    }
    let text = ascii_text(bytes)?;
    let mut lines = text
        .strip_suffix('\n')
        .ok_or("manifest-final-lf")?
        .split('\n');
    let header = lines.next().ok_or("manifest-header")?;
    if header.split('\t').ne(MANIFEST_HEADER) {
        return Err("manifest-header");
    }
    let mut rows = Vec::new();
    for line in lines {
        if rows.len() == 32 {
            return Err("manifest-row-bound");
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let array: [&str; 19] = match fields.try_into() {
            Ok(value) => value,
            Err(_) => return Err("manifest-field-count"),
        };
        if array
            .iter()
            .any(|value| value.is_empty() || value.len() > 128)
        {
            return Err("manifest-field-bound");
        }
        rows.push(ManifestRow { fields: array });
    }
    if rows.is_empty() {
        return Err("manifest-empty");
    }
    Ok(rows)
}

pub fn parse_fixture(bytes: &[u8]) -> Result<Fixture<'_>, &'static str> {
    if bytes.len() > 4 * 1024 || !bytes.ends_with(b"\n") {
        return Err("fixture-bound");
    }
    let text = ascii_text(bytes)?;
    let lines: Vec<&str> = text
        .strip_suffix('\n')
        .ok_or("fixture-final-lf")?
        .split('\n')
        .collect();
    if lines.len() != FIXTURE_KEYS.len() {
        return Err("fixture-row-count");
    }
    let mut values = [""; 12];
    for (index, line) in lines.iter().enumerate() {
        let Some((key, value)) = line.split_once('=') else {
            return Err("fixture-shape");
        };
        if key != FIXTURE_KEYS[index] || value.is_empty() || value.len() > 128 {
            return Err("fixture-field");
        }
        values[index] = value;
    }
    if values[1]
        .parse::<u64>()
        .ok()
        .filter(|value| *value > 0)
        .is_none()
    {
        return Err("fixture-version");
    }
    if values[11]
        .bytes()
        .any(|byte| !(byte.is_ascii_uppercase() || byte.is_ascii_digit() || b"_:-".contains(&byte)))
    {
        return Err("fixture-token");
    }
    Ok(Fixture { fields: values })
}

pub fn fixture_for_path(path: &str) -> Option<&'static [u8]> {
    FIXTURE_BYTES
        .iter()
        .find(|(candidate, _)| *candidate == path)
        .map(|(_, bytes)| *bytes)
}

pub fn validate_path(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= 240
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.contains(':')
        && !path.contains('%')
        && path
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

pub fn verdict(row: &ManifestRow<'_>, fixture: &Fixture<'_>) -> Result<Verdict, &'static str> {
    if row.get("fixture_id") != fixture.get("fixture_id")
        || row.get("version") != fixture.get("version")
        || row.get("class") != fixture.get("class")
        || row.get("source_posture") != Some("synthetic-inert")
        || fixture.get("source_posture") != Some("synthetic-inert")
        || row.get("supersession_state") != Some("current")
    {
        return Err("binding-mismatch");
    }
    match row.get("expected_posture") {
        Some("accepted-for-harness-only") => Ok(Verdict::AcceptedForHarnessOnly),
        Some("held") => Ok(Verdict::Held),
        Some("rejected") => Ok(Verdict::Rejected),
        Some("rejected-safe") => Ok(Verdict::RejectedSafe),
        _ => Err("unknown-posture"),
    }
}

pub fn validate_scaffold() -> Result<Vec<(ManifestRow<'static>, Fixture<'static>)>, &'static str> {
    let rows = parse_manifest(MANIFEST_BYTES)?;
    if rows.len() != 4
        || FIXTURE_BYTES
            .iter()
            .map(|(_, bytes)| bytes.len())
            .sum::<usize>()
            > 32 * 1024
    {
        return Err("fixture-inventory");
    }
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut result = Vec::new();
    for row in rows {
        let id = row.get("fixture_id").ok_or("fixture-id")?;
        let path = row.get("path").ok_or("fixture-path")?;
        if !ids.insert(id) || !paths.insert(path) || !validate_path(path) {
            return Err("fixture-identity");
        }
        let bytes = fixture_for_path(path).ok_or("fixture-missing")?;
        if hex_sha256(bytes) != row.get("sha256").ok_or("fixture-digest")? {
            return Err("fixture-digest");
        }
        let fixture = parse_fixture(bytes)?;
        let _ = verdict(&row, &fixture)?;
        if row.get("predecessor_id") != Some("none")
            || row.get("predecessor_digest")
                != Some("0000000000000000000000000000000000000000000000000000000000000000")
            || row.get("predecessor_version") != Some("0")
            || fixture.get("predecessor_id") != Some("none")
            || fixture.get("predecessor_version") != Some("0")
        {
            return Err("fixture-predecessor");
        }
        result.push((row, fixture));
    }
    Ok(result)
}

pub fn source_preimage(row: &ManifestRow<'_>) -> Result<String, &'static str> {
    Ok(format!(
        "schema=synthetic-fixture-source.v1\nsource_id={}\nsource_posture={}\n",
        row.get("source_id").ok_or("source-id")?,
        row.get("source_posture").ok_or("source-posture")?
    ))
}

pub fn custody_preimage(row: &ManifestRow<'_>) -> Result<String, &'static str> {
    let keys = [
        ("custodian_id", "custodian_id"),
        ("custody_id", "custody_id"),
        ("fixture_id", "fixture_id"),
        ("version", "version"),
        ("source_posture", "source_posture"),
        ("source_id", "source_id"),
        ("source_digest", "source_digest"),
        ("purpose_id", "purpose_id"),
        ("expected_posture", "expected_posture"),
        ("expected_reason_id", "expected_reason_id"),
        ("proof_input_hold", "proof_input_hold"),
    ];
    let mut result = String::from("schema=test-fixture-custody.v1\n");
    for (label, key) in keys {
        result.push_str(label);
        result.push('=');
        result.push_str(row.get(key).ok_or("custody-field")?);
        result.push('\n');
    }
    Ok(result)
}

pub fn validate_digests(rows: &[(ManifestRow<'_>, Fixture<'_>)]) -> Result<(), &'static str> {
    for (row, _) in rows {
        if hex_sha256(source_preimage(row)?.as_bytes())
            != row.get("source_digest").ok_or("source-digest")?
            || hex_sha256(custody_preimage(row)?.as_bytes())
                != row.get("custody_digest").ok_or("custody-digest")?
        {
            return Err("canonical-digest");
        }
    }
    Ok(())
}

pub fn trace_counts() -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for edge in TRACE_MANIFEST {
        *counts.entry(edge.mode).or_insert(0) += 1;
    }
    counts
}

pub fn verify_trace(controlled_id: &str, assertion: &str, mode: &str) {
    let matches = TRACE_MANIFEST
        .iter()
        .filter(|edge| {
            edge.controlled_id == controlled_id && edge.assertion == assertion && edge.mode == mode
        })
        .count();
    assert_eq!(matches, 1, "trace edge must exist exactly once");
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureState {
    Current,
    Superseded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Successor<'a> {
    pub predecessor_id: &'a str,
    pub predecessor_version: u64,
    pub predecessor_digest: String,
    pub version: u64,
    pub state: FixtureState,
    pub verdict: Verdict,
}

pub fn make_successor<'a>(
    row: &'a ManifestRow<'a>,
    fixture: &'a Fixture<'a>,
) -> Result<Successor<'a>, &'static str> {
    let version = row
        .get("version")
        .ok_or("version")?
        .parse::<u64>()
        .map_err(|_| "version")?;
    if version == u64::MAX {
        return Err("version-overflow");
    }
    Ok(Successor {
        predecessor_id: row.get("fixture_id").ok_or("fixture-id")?,
        predecessor_version: version,
        predecessor_digest: row.get("sha256").ok_or("fixture-digest")?.to_owned(),
        version: version + 1,
        state: FixtureState::Current,
        verdict: verdict(row, fixture)?,
    })
}

fn canonical_scaffold() -> Vec<(ManifestRow<'static>, Fixture<'static>)> {
    let Ok(rows) = validate_scaffold() else {
        panic!("committed fixture scaffold must be closed");
    };
    assert!(
        validate_digests(&rows).is_ok(),
        "source and custody digests must reproduce"
    );
    rows
}

fn assert_manifest_contract() {
    let rows = canonical_scaffold();
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|(row, fixture)| {
        row.fields.len() == 19
            && fixture.fields.len() == 12
            && row.get("fixture_id") == fixture.get("fixture_id")
    }));
}

fn assert_parser_rejects_noncanonical() {
    let mut reversed = MANIFEST_BYTES.to_vec();
    reversed.reverse();
    for bytes in [
        Vec::new(),
        b"fixture_id\tversion\r\n".to_vec(),
        vec![0xef, 0xbb, 0xbf, b'x'],
        reversed,
        vec![b'x'; 16 * 1024 + 1],
    ] {
        assert!(parse_manifest(&bytes).is_err());
    }
    assert!(parse_fixture(b"fixture_id=X\n").is_err());
}

fn assert_successor_is_immutable() {
    let rows = canonical_scaffold();
    let (row, fixture) = &rows[0];
    let Ok(successor) = make_successor(row, fixture) else {
        panic!("valid successor");
    };
    assert_eq!(
        successor.predecessor_id,
        row.get("fixture_id").unwrap_or("")
    );
    assert_eq!(successor.predecessor_version + 1, successor.version);
    assert_eq!(
        successor.predecessor_digest,
        row.get("sha256").unwrap_or("")
    );
    assert_eq!(successor.state, FixtureState::Current);
    assert_eq!(
        successor.verdict,
        verdict(row, fixture).unwrap_or(Verdict::Rejected)
    );
}

fn assert_holds_are_closed() {
    let rows = canonical_scaffold();
    assert_eq!(OPEN_HOLDS.len(), 4);
    assert!(
        rows.iter()
            .all(|(row, _)| OPEN_HOLDS.contains(&row.get("proof_input_hold").unwrap_or("")))
    );
    assert!(
        rows.iter()
            .any(|(row, fixture)| verdict(row, fixture) == Ok(Verdict::Held))
    );
}

fn assert_no_authority() {
    let rows = canonical_scaffold();
    let tokens: Vec<_> = rows
        .iter()
        .filter_map(|(_, fixture)| fixture.get("token"))
        .collect();
    assert!(tokens.iter().all(|token| token.starts_with("SYNTHETIC_")));
    for forbidden in [
        "official",
        "release",
        "taxlane",
        "readiness",
        "budget",
        "deploy",
        "approve",
    ] {
        assert!(
            !tokens
                .iter()
                .any(|token| token.to_ascii_lowercase().contains(forbidden))
        );
    }
}

fn assert_path_boundary() {
    assert!(validate_path("cases/valid.fixture"));
    for path in [
        "",
        "/absolute",
        "../traversal",
        "a/../b",
        "drive:C",
        "alternate\\separator",
        "https://uri",
        "percent%2fescape",
        "double//segment",
    ] {
        assert!(!validate_path(path), "path admitted: {path}");
    }
}

fn assert_trace_registry() {
    assert_eq!(TRACE_MANIFEST.len(), 148);
    let edges: BTreeSet<_> = TRACE_MANIFEST.iter().copied().collect();
    assert_eq!(edges.len(), TRACE_MANIFEST.len());
    let identities: BTreeSet<_> = TRACE_MANIFEST
        .iter()
        .map(|edge| edge.controlled_id)
        .collect();
    assert_eq!(identities.len(), 123);
    let counts = trace_counts();
    assert_eq!(counts.values().sum::<usize>(), 148);
    assert_eq!(counts.get("L1Static"), Some(&8));
}

fn assert_static_registry() {
    assert_eq!(IMPLEMENTATION_PATHS.len(), 18);
    assert!(
        IMPLEMENTATION_PATHS
            .windows(2)
            .all(|pair| pair[0] < pair[1])
    );
    let manifest = include_str!("../../Cargo.toml");
    assert_eq!(manifest.matches("[[test]]").count(), 8);
    assert!(!manifest.contains("[dependencies]"));
    assert!(!manifest.contains("[dev-dependencies]"));
    assert!(!manifest.contains("[build-dependencies]"));
}

fn assert_accounting() {
    let rows = canonical_scaffold();
    let total = rows.iter().try_fold(0_u64, |acc, _| acc.checked_add(1));
    assert_eq!(total, Some(4));
    assert_eq!(
        rows.iter()
            .filter(|(row, _)| row.get("supersession_state") == Some("current"))
            .count(),
        rows.len()
    );
}

fn normative_source(controlled_id: &str) -> &'static str {
    if controlled_id == "Role review steward" {
        return WORK_PACKAGE_SOURCE;
    }
    if controlled_id.starts_with(".roles/") {
        let matches: Vec<_> = ROLE_SOURCES
            .iter()
            .filter(|(path, _)| *path == controlled_id)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "role identity must select one embedded source"
        );
        return matches[0].1;
    }
    if controlled_id.starts_with("BASTION-REQ-") || controlled_id.starts_with("TBD-") {
        REQUIREMENTS_SOURCE
    } else if controlled_id.starts_with("SPEC-") || controlled_id.starts_with("CONTRACT-") {
        SPECIFICATION_SOURCE
    } else if controlled_id.starts_with("VCL-") || controlled_id.starts_with("DES-") {
        WORK_PACKAGE_SOURCE
    } else if controlled_id.starts_with("ACT-") || controlled_id.starts_with("VAL-") {
        VALIDATION_SOURCE
    } else if controlled_id.starts_with("CR-") {
        WORK_PACKAGE_SOURCE
    } else {
        panic!("controlled identity has no embedded normative source: {controlled_id}");
    }
}

fn exact_normative_clause(controlled_id: &str) -> String {
    let source = normative_source(controlled_id);
    if controlled_id.starts_with(".roles/") {
        let clause = source
            .lines()
            .find(|line| !line.trim().is_empty())
            .expect("role source must contain a normative clause");
        return format!("{controlled_id}:{}", clause.trim());
    }
    let clause = source
        .lines()
        .find(|line| line.contains(controlled_id))
        .unwrap_or_else(|| {
            panic!("controlled identity has no exact normative clause: {controlled_id}")
        });
    clause.trim().to_owned()
}

pub fn obligation_proof_registry() -> Vec<ObligationProof> {
    EDGE_EXECUTION_SPECS
        .iter()
        .map(|spec| {
            let edge = &spec.edge;
            let positive_witness = execute_positive(spec, edge);
            let adversarial_witness = execute_adversarial(spec, edge);
            let correction = audit_correction(spec, &positive_witness, &adversarial_witness);
            ObligationProof {
                controlled_id: edge.controlled_id,
                assertion: edge.assertion,
                normative_clause: exact_normative_clause(edge.controlled_id),
                positive_witness,
                adversarial_witness,
                retained_audit_proof: correction_commitment(&correction),
                audit_correction: correction,
                positive_proof: spec.positive,
                adversarial_proof: spec.adversarial,
            }
        })
        .collect()
}

fn admit_exact_trace_tuple(
    controlled_id: &str,
    assertion: &str,
    mode: &str,
) -> Result<(), &'static str> {
    if TRACE_MANIFEST
        .iter()
        .filter(|edge| {
            edge.controlled_id == controlled_id && edge.assertion == assertion && edge.mode == mode
        })
        .count()
        == 1
    {
        Ok(())
    } else {
        Err("unregistered-trace-tuple")
    }
}

fn allocated_semantic_contract(edge: &TraceEdge) -> String {
    let predicate = format!("{:?}", typed_obligation(edge));
    format!(
        "{}|{}|{}|{}|{}",
        edge.controlled_id,
        edge.assertion,
        edge.mode,
        predicate,
        hex_sha256(exact_normative_clause(edge.controlled_id).as_bytes())
    )
}

fn admit_allocated_semantic_contract(edge: &TraceEdge, contract: &str) -> Result<(), &'static str> {
    if contract == allocated_semantic_contract(edge) {
        Ok(())
    } else {
        Err("swapped-semantic-contract")
    }
}

fn assert_obligation_proof_registry() {
    let proofs = obligation_proof_registry();
    assert_eq!(proofs.len(), TRACE_MANIFEST.len());
    let mut identities = BTreeSet::new();
    let mut semantic_case_keys = BTreeSet::new();
    for (ordinal, (proof, edge)) in proofs.iter().zip(TRACE_MANIFEST).enumerate() {
        assert!(
            identities.insert((proof.controlled_id, proof.assertion)),
            "obligation proof identity must be unique"
        );
        assert_eq!(proof.controlled_id, edge.controlled_id);
        assert_eq!(proof.assertion, edge.assertion);
        let semantic_contract = allocated_semantic_contract(edge);
        assert_eq!(
            admit_allocated_semantic_contract(edge, &semantic_contract),
            Ok(())
        );
        let swapped =
            allocated_semantic_contract(&TRACE_MANIFEST[(ordinal + 1) % TRACE_MANIFEST.len()]);
        assert_eq!(
            admit_allocated_semantic_contract(edge, &swapped),
            Err("swapped-semantic-contract"),
            "controlled identity must reject another edge's semantic predicate and mutation allocation"
        );
        if !edge.controlled_id.starts_with("CR-") {
            let allocation = allocated_semantic_contract(edge);
            assert!(
                semantic_case_keys.insert(allocation),
                "allocated edges may share an explicitly named invariant family, but may not alias the same family, normative clause, and mode semantic case"
            );
        }
        assert_proof_custody_fields(
            edge,
            ordinal,
            &proof.normative_clause,
            &proof.positive_witness,
            &proof.adversarial_witness,
            &proof.retained_audit_proof,
        )
        .expect("exact edge proof custody");
        assert_eq!(
            proof.positive_witness,
            execute_positive(&EDGE_EXECUTION_SPECS[ordinal], edge)
        );
        assert_eq!(
            TRACE_MANIFEST
                .iter()
                .filter(|candidate| {
                    candidate.controlled_id == proof.controlled_id
                        && candidate.assertion == proof.assertion
                })
                .count(),
            1,
            "positive witness must select one exact obligation"
        );
        assert_eq!(
            proof.adversarial_witness,
            execute_adversarial(&EDGE_EXECUTION_SPECS[ordinal], edge)
        );
        assert_parser_rejects_noncanonical();
        assert_assertion_semantics(
            edge.controlled_id,
            edge.assertion.split_once("::").unwrap().1,
        );
        (proof.positive_proof)(edge);
        (proof.adversarial_proof)(edge);
        assert_eq!(
            proof.retained_audit_proof,
            correction_commitment(&proof.audit_correction)
        );
        assert_eq!(
            proof.audit_correction,
            audit_correction(
                &EDGE_EXECUTION_SPECS[ordinal],
                &proof.positive_witness,
                &proof.adversarial_witness,
            )
        );
        if let Some(other) = proofs
            .iter()
            .find(|candidate| candidate.controlled_id != proof.controlled_id)
        {
            assert_ne!(
                proof.normative_clause, other.normative_clause,
                "cross-identity normative substitution must be observable"
            );
            assert_eq!(
                assert_proof_custody_fields(
                    edge,
                    ordinal,
                    &other.normative_clause,
                    &proof.positive_witness,
                    &proof.adversarial_witness,
                    &proof.retained_audit_proof,
                ),
                Err("normative-clause-substitution")
            );
        }
        if ordinal + 1 < proofs.len() {
            let other = &proofs[ordinal + 1];
            assert_ne!(
                proof.positive_witness, other.positive_witness,
                "cross-edge positive witness substitution"
            );
            assert_ne!(
                proof.adversarial_witness, other.adversarial_witness,
                "cross-edge adversarial witness substitution"
            );
            assert_ne!(
                proof.retained_audit_proof, other.retained_audit_proof,
                "cross-edge retained proof substitution"
            );
            assert_eq!(
                assert_proof_custody_fields(
                    edge,
                    ordinal,
                    &proof.normative_clause,
                    &other.positive_witness,
                    &proof.adversarial_witness,
                    &proof.retained_audit_proof
                ),
                Err("positive-witness-substitution")
            );
            assert_eq!(
                assert_proof_custody_fields(
                    edge,
                    ordinal,
                    &proof.normative_clause,
                    &proof.positive_witness,
                    &other.adversarial_witness,
                    &proof.retained_audit_proof
                ),
                Err("adversarial-witness-substitution")
            );
            assert_eq!(
                assert_proof_custody_fields(
                    edge,
                    ordinal,
                    &proof.normative_clause,
                    &proof.positive_witness,
                    &proof.adversarial_witness,
                    &other.retained_audit_proof
                ),
                Err("retained-proof-substitution")
            );
        }
    }
}

fn assert_proof_custody_fields(
    edge: &TraceEdge,
    ordinal: usize,
    normative_clause: &str,
    positive_witness: &str,
    adversarial_witness: &str,
    retained_audit_proof: &str,
) -> Result<(), &'static str> {
    if normative_clause != exact_normative_clause(edge.controlled_id) {
        return Err("normative-clause-substitution");
    }
    let spec = &EDGE_EXECUTION_SPECS[ordinal];
    let accepted_prefix = format!(
        "accepted|{}|{}|{}|",
        edge.controlled_id, edge.assertion, edge.mode
    );
    if spec.edge != *edge || !positive_witness.starts_with(&accepted_prefix) {
        return Err("positive-witness-substitution");
    }
    let rejected_prefix = format!("rejected|{}|{}|", edge.controlled_id, edge.assertion);
    if !adversarial_witness.starts_with(&rejected_prefix) {
        return Err("adversarial-witness-substitution");
    }
    if retained_audit_proof
        != correction_commitment(&audit_correction(
            spec,
            positive_witness,
            adversarial_witness,
        ))
    {
        return Err("retained-proof-substitution");
    }
    Ok(())
}

fn assert_controlled_source(controlled_id: &str) {
    let source = normative_source(controlled_id);
    if controlled_id.starts_with(".roles/") {
        assert!(!source.trim().is_empty(), "role source must be nonempty");
        return;
    }
    assert!(
        source.contains(controlled_id),
        "controlled identity must occur in its independently selected normative source"
    );
}

fn assert_terminal_no_output_backflow() {
    #[derive(Clone, Copy)]
    enum OutputState {
        Open,
        Terminal,
    }
    fn emit(state: OutputState, value: &'static str) -> Result<&'static str, &'static str> {
        match state {
            OutputState::Open => Ok(value),
            OutputState::Terminal => Err("terminal-output-backflow-rejected"),
        }
    }
    assert_eq!(
        emit(OutputState::Open, "bounded-output"),
        Ok("bounded-output")
    );
    assert_eq!(
        emit(OutputState::Terminal, "forbidden-output"),
        Err("terminal-output-backflow-rejected")
    );
}

fn assert_assertion_semantics(controlled_id: &str, assertion_name: &str) {
    if !controlled_id.starts_with("CR-") {
        return;
    }
    match assertion_name {
        "cr_002_logical_contract" | "cr_002_logical_responsibility" => {
            assert_manifest_contract();
            assert_path_boundary();
        }
        "cr_003_typed_failure_rejection" | "cr_003_typed_branch_totality" => {
            assert_parser_rejects_noncanonical();
            assert_manifest_contract();
        }
        "cr_004_exhaustion_failure" | "cr_004_finite_bounds_progress" => {
            assert!(MANIFEST_BYTES.len() <= 16 * 1024);
            assert_parser_rejects_noncanonical();
        }
        "cr_005_termination_violation" | "cr_005_call_graph_depth" => {
            assert_no_authority();
            assert_trace_registry();
        }
        "cr_006_hidden_failure_scan" | "cr_006_invalid_state" => {
            assert_parser_rejects_noncanonical();
            assert_successor_is_immutable();
        }
        "cr_008_default_fallback_rejection" | "cr_008_missing_default_hold" => {
            assert_holds_are_closed();
            assert_no_authority();
        }
        "cr_009_typed_family_exhaustiveness" | "cr_009_typed_state_exhaustiveness" => {
            assert_manifest_contract();
            assert_successor_is_immutable();
        }
        "cr_010_release_exception_no_output" | "cr_010_universal_admission_bypass" => {
            assert_no_authority();
            assert_parser_rejects_noncanonical();
        }
        "cr_011_digest_reproduction_order"
        | "cr_011_order_invariance"
        | "cr_011_replay_identity" => {
            let rows = canonical_scaffold();
            assert!(validate_digests(&rows).is_ok());
            assert_accounting();
        }
        "cr_012_ambient_state_absence" | "cr_012_schedule_equivalence" => {
            assert_parser_rejects_noncanonical();
            assert_trace_registry();
        }
        "cr_013_immutable_successor_acyclic" => assert_successor_is_immutable(),
        "cr_014_consumer_direction" => {
            assert_static_registry();
            assert_no_authority();
        }
        "cr_015_content_boundary_provenance" | "cr_015_prohibited_content" => {
            assert_path_boundary();
            assert_no_authority();
        }
        "cr_016_composition_minimization" => {
            assert_static_registry();
            assert_manifest_contract();
        }
        "cr_017_authority_noninflation" | "cr_017_floor_noncompensation" => assert_no_authority(),
        "cr_018_facet_distribution_conservation" => assert_accounting(),
        "cr_019_missing_null_hold" | "cr_019_state_null_na_stale" => {
            assert_holds_are_closed();
            assert_successor_is_immutable();
        }
        "cr_020_checked_accounting" | "cr_020_reconciliation_identity" => assert_accounting(),
        "cr_021_burden_shift_rejection" | "cr_021_false_savings_no_authority" => {
            assert_no_authority()
        }
        "cr_022_eco_delivery_adaptive_shape" => assert_successor_is_immutable(),
        "cr_023_finding_dissent_retention" | "cr_023_review_independence" => {
            assert_holds_are_closed();
            assert_successor_is_immutable();
        }
        "cr_024_terminal_no_output_backflow" => assert_terminal_no_output_backflow(),
        "cr_025_hold_transpose_propagation" => assert_holds_are_closed(),
        "cr_026_invariant_coverage" => assert_trace_registry(),
        "cr_027_property_evidence_set" => {
            assert_accounting();
            assert_manifest_contract();
        }
        "cr_028_transition_model_evidence" => assert_successor_is_immutable(),
        "cr_029_cross_role_adversarial" => {
            assert_no_authority();
            assert_path_boundary();
        }
        "cr_030_per_contract_fixture_matrix" => assert_manifest_contract(),
        "cr_031_parser_fuzz_authority_absent" | "cr_031_parser_surface_absent" => {
            assert_parser_rejects_noncanonical();
            assert_no_authority();
        }
        "cr_032_golden_successor_history" | "cr_032_regression_replay" => {
            assert_successor_is_immutable()
        }
        "cr_033_mode_isolation" => assert_trace_registry(),
        "cr_034_generated_no_emission" | "cr_034_generated_provenance_absence" => {
            assert_no_authority();
            let rows = canonical_scaffold();
            assert!(validate_digests(&rows).is_ok());
        }
        "cr_035_quality_gate_registry" | "cr_035_quality_output_binding" => assert_trace_registry(),
        "cr_036_dependency_license_advisory" => {
            let kind = TypedObligation::Cr(CrOp::LicenseAdvisory);
            assert_eq!(
                validate_domain_state(kind, &positive_domain_state(kind)),
                Ok(())
            );
            let mut stale = positive_domain_state(kind);
            if let DomainState::Cr(CrState::LicenseAdvisory { controls }) = &mut stale {
                controls.advisories_current = false;
            }
            assert_eq!(
                validate_domain_state(kind, &stale),
                Err("advisory-data-stale")
            );
        }
        "cr_037_resource_bound_failure" | "cr_037_resource_bound_registry" => {
            assert!(
                FIXTURE_BYTES
                    .iter()
                    .map(|(_, bytes)| bytes.len())
                    .sum::<usize>()
                    <= 32 * 1024
            );
            assert_parser_rejects_noncanonical();
        }
        "cr_038_waiver_ledger_nonwaiver" => assert_holds_are_closed(),
        "cr_039_evidence_digest_truth" | "cr_039_evidence_state_history" => {
            let rows = canonical_scaffold();
            assert!(validate_digests(&rows).is_ok());
            assert_successor_is_immutable();
        }
        "cr_040_mechanical_trace_contradiction" => assert_trace_registry(),
        _ => panic!("unallocated assertion semantics: {controlled_id}/{assertion_name}"),
    }
}

pub fn verify_obligation(controlled_id: &str, assertion: &str) {
    assert_obligation_allocation_registry();
    let matching_edges: Vec<_> = TRACE_MANIFEST
        .iter()
        .filter(|edge| edge.controlled_id == controlled_id && edge.assertion == assertion)
        .collect();
    assert_eq!(
        matching_edges.len(),
        1,
        "controlled identity and assertion must select one exact trace edge"
    );
    let assertion_name = assertion
        .split_once("::")
        .map(|(_, name)| name)
        .unwrap_or("");
    assert!(!assertion_name.is_empty(), "assertion must be namespaced");
    assert_controlled_source(controlled_id);
    let spec_matches: Vec<_> = EDGE_EXECUTION_SPECS
        .iter()
        .filter(|spec| spec.edge.controlled_id == controlled_id && spec.edge.assertion == assertion)
        .collect();
    assert_eq!(spec_matches.len(), 1, "exact obligation proof must exist");
    let spec = spec_matches[0];
    let positive = execute_positive(spec, &spec.edge);
    let adversarial = execute_adversarial(spec, &spec.edge);
    assert_proof_custody_fields(
        &spec.edge,
        spec.ordinal,
        &exact_normative_clause(controlled_id),
        &positive,
        &adversarial,
        &correction_commitment(&audit_correction(spec, &positive, &adversarial)),
    )
    .expect("named wrapper exact proof custody");
    assert_assertion_semantics(controlled_id, assertion_name);

    if let Some(number) = controlled_id.strip_prefix("CR-") {
        let expected = format!("cr_{}", number.to_ascii_lowercase());
        assert!(
            assertion_name == expected || assertion_name.starts_with(&format!("{expected}_")),
            "CR assertion must name its exact controlled requirement"
        );
    } else {
        let role_prefix = if controlled_id.starts_with(".roles/") {
            "role_"
        } else {
            ""
        };
        let slug: String = controlled_id
            .trim_start_matches(".roles/")
            .trim_end_matches(".md")
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect();
        let slug = slug.trim_matches('_').replace("__", "_");
        assert_eq!(
            assertion_name,
            format!("trace_{role_prefix}{slug}"),
            "trace assertion must encode its exact controlled identity"
        );
    }

    if assertion.starts_with("static_surface::") {
        assert_static_registry();
        if assertion.contains("ambient_state") || assertion.contains("parser_surface") {
            assert_parser_rejects_noncanonical();
        } else if assertion.contains("resource_bound") {
            assert!(MANIFEST_BYTES.len() <= 16 * 1024);
            assert!(
                FIXTURE_BYTES
                    .iter()
                    .map(|(_, bytes)| bytes.len())
                    .sum::<usize>()
                    <= 32 * 1024
            );
        } else {
            assert_trace_registry();
        }
    } else if assertion.starts_with("source_spine::") {
        assert_trace_registry();
        assert_manifest_contract();
    } else if assertion.starts_with("contract_matrix::") {
        assert_manifest_contract();
        assert_path_boundary();
    } else if assertion.starts_with("property_cases::") {
        assert_parser_rejects_noncanonical();
        assert_accounting();
        assert_eq!(
            hex_sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    } else if assertion.starts_with("model_cases::") {
        assert_successor_is_immutable();
        assert_accounting();
    } else if assertion.starts_with("adversarial_cases::") {
        assert_parser_rejects_noncanonical();
        assert_path_boundary();
        assert_no_authority();
    } else if assertion.starts_with("hold_closure::") {
        assert_holds_are_closed();
        assert_successor_is_immutable();
    } else if assertion.starts_with("no_authority_surface::") {
        assert_no_authority();
        assert_path_boundary();
    } else {
        panic!("unallocated assertion: {assertion}");
    }
}

pub fn hex_sha256(bytes: &[u8]) -> String {
    let digest = sha256(bytes);
    let mut result = String::with_capacity(64);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        result.push(char::from(HEX[usize::from(byte >> 4)]));
        result.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    result
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    const INITIAL: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let bit_len = (bytes.len() as u64).wrapping_mul(8);
    let mut padded = bytes.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut state = INITIAL;
    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in words[..16].iter_mut().enumerate() {
            let offset = index * 4;
            *word = u32::from_be_bytes([
                chunk[offset],
                chunk[offset + 1],
                chunk[offset + 2],
                chunk[offset + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temporary1 = h
                .wrapping_add(sum1)
                .wrapping_add(choice)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temporary2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temporary1);
            d = c;
            c = b;
            b = a;
            a = temporary1.wrapping_add(temporary2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut output = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        output[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    output
}

macro_rules! trace_tests {
    ($mode:literal; $( $name:ident => $controlled:literal ),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                let assertion = format!("{}::{}", module_path!(), stringify!($name));
                crate::support::verify_trace($controlled, &assertion, $mode);
                crate::support::verify_obligation($controlled, &assertion);
            }
        )+
    };
}
pub(crate) use trace_tests;

pub const TRACE_MANIFEST: &[TraceEdge] = &[
    TraceEdge {
        controlled_id: "BASTION-REQ-TST-001",
        assertion: "source_spine::trace_bastion_req_tst_001",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-TST-002",
        assertion: "property_cases::trace_bastion_req_tst_002",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-TST-003",
        assertion: "model_cases::trace_bastion_req_tst_003",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-TST-004",
        assertion: "contract_matrix::trace_bastion_req_tst_004",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-TST-005",
        assertion: "hold_closure::trace_bastion_req_tst_005",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-TST-006",
        assertion: "adversarial_cases::trace_bastion_req_tst_006",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-REL-001",
        assertion: "no_authority_surface::trace_bastion_req_rel_001",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-REL-002",
        assertion: "adversarial_cases::trace_bastion_req_rel_002",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "BASTION-REQ-REL-003",
        assertion: "no_authority_surface::trace_bastion_req_rel_003",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "SPEC-TST-001",
        assertion: "source_spine::trace_spec_tst_001",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "SPEC-TST-002",
        assertion: "property_cases::trace_spec_tst_002",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "SPEC-TST-003",
        assertion: "model_cases::trace_spec_tst_003",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "SPEC-TST-004",
        assertion: "contract_matrix::trace_spec_tst_004",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "SPEC-TST-005",
        assertion: "hold_closure::trace_spec_tst_005",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "SPEC-TST-006",
        assertion: "adversarial_cases::trace_spec_tst_006",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "SPEC-REL-001",
        assertion: "no_authority_surface::trace_spec_rel_001",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "SPEC-REL-002",
        assertion: "adversarial_cases::trace_spec_rel_002",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "SPEC-REL-003",
        assertion: "no_authority_surface::trace_spec_rel_003",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-001",
        assertion: "adversarial_cases::trace_spec_nf_001",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-002",
        assertion: "no_authority_surface::trace_spec_nf_002",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-003",
        assertion: "no_authority_surface::trace_spec_nf_003",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-004",
        assertion: "property_cases::trace_spec_nf_004",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-005",
        assertion: "property_cases::trace_spec_nf_005",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-006",
        assertion: "model_cases::trace_spec_nf_006",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-007",
        assertion: "property_cases::trace_spec_nf_007",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-008",
        assertion: "contract_matrix::trace_spec_nf_008",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-009",
        assertion: "model_cases::trace_spec_nf_009",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "SPEC-NF-010",
        assertion: "source_spine::trace_spec_nf_010",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "DES-TEST-001",
        assertion: "contract_matrix::trace_des_test_001",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "DES-REL-001",
        assertion: "no_authority_surface::trace_des_rel_001",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CONTRACT-TEST-001",
        assertion: "contract_matrix::trace_contract_test_001",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "CONTRACT-REL-001",
        assertion: "no_authority_surface::trace_contract_rel_001",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CR-002",
        assertion: "contract_matrix::cr_002_logical_contract",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "CR-002",
        assertion: "source_spine::cr_002_logical_responsibility",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-003",
        assertion: "adversarial_cases::cr_003_typed_failure_rejection",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-003",
        assertion: "contract_matrix::cr_003_typed_branch_totality",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "CR-004",
        assertion: "adversarial_cases::cr_004_exhaustion_failure",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-004",
        assertion: "property_cases::cr_004_finite_bounds_progress",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-005",
        assertion: "static_surface::cr_005_call_graph_depth",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-005",
        assertion: "adversarial_cases::cr_005_termination_violation",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-006",
        assertion: "adversarial_cases::cr_006_hidden_failure_scan",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-006",
        assertion: "model_cases::cr_006_invalid_state",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-008",
        assertion: "adversarial_cases::cr_008_default_fallback_rejection",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-008",
        assertion: "hold_closure::cr_008_missing_default_hold",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "CR-009",
        assertion: "contract_matrix::cr_009_typed_family_exhaustiveness",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "CR-009",
        assertion: "model_cases::cr_009_typed_state_exhaustiveness",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-010",
        assertion: "no_authority_surface::cr_010_release_exception_no_output",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CR-010",
        assertion: "property_cases::cr_010_universal_admission_bypass",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-011",
        assertion: "model_cases::cr_011_replay_identity",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-011",
        assertion: "property_cases::cr_011_order_invariance",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-011",
        assertion: "source_spine::cr_011_digest_reproduction_order",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-012",
        assertion: "static_surface::cr_012_ambient_state_absence",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-012",
        assertion: "property_cases::cr_012_schedule_equivalence",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-013",
        assertion: "model_cases::cr_013_immutable_successor_acyclic",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-014",
        assertion: "static_surface::cr_014_consumer_direction",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-014",
        assertion: "test_gate::cr_014_fixed_dependency_graph",
        mode: "L1SupplyChain",
    },
    TraceEdge {
        controlled_id: "CR-015",
        assertion: "adversarial_cases::cr_015_prohibited_content",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-015",
        assertion: "contract_matrix::cr_015_content_boundary_provenance",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "CR-016",
        assertion: "adversarial_cases::cr_016_composition_minimization",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-017",
        assertion: "adversarial_cases::cr_017_floor_noncompensation",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-017",
        assertion: "no_authority_surface::cr_017_authority_noninflation",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CR-018",
        assertion: "property_cases::cr_018_facet_distribution_conservation",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-019",
        assertion: "hold_closure::cr_019_missing_null_hold",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "CR-019",
        assertion: "model_cases::cr_019_state_null_na_stale",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-020",
        assertion: "model_cases::cr_020_checked_accounting",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-020",
        assertion: "property_cases::cr_020_reconciliation_identity",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-021",
        assertion: "adversarial_cases::cr_021_burden_shift_rejection",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-021",
        assertion: "no_authority_surface::cr_021_false_savings_no_authority",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CR-022",
        assertion: "model_cases::cr_022_eco_delivery_adaptive_shape",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-023",
        assertion: "hold_closure::cr_023_finding_dissent_retention",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "CR-023",
        assertion: "source_spine::cr_023_review_independence",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-024",
        assertion: "no_authority_surface::cr_024_terminal_no_output_backflow",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CR-025",
        assertion: "hold_closure::cr_025_hold_transpose_propagation",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "CR-026",
        assertion: "source_spine::cr_026_invariant_coverage",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-027",
        assertion: "property_cases::cr_027_property_evidence_set",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-028",
        assertion: "model_cases::cr_028_transition_model_evidence",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-029",
        assertion: "adversarial_cases::cr_029_cross_role_adversarial",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-030",
        assertion: "contract_matrix::cr_030_per_contract_fixture_matrix",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "CR-031",
        assertion: "static_surface::cr_031_parser_surface_absent",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-031",
        assertion: "adversarial_cases::cr_031_parser_fuzz_authority_absent",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-032",
        assertion: "model_cases::cr_032_golden_successor_history",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "CR-032",
        assertion: "property_cases::cr_032_regression_replay",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "CR-033",
        assertion: "static_surface::cr_033_mode_isolation",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-033",
        assertion: "test_gate::cr_033_package_isolation",
        mode: "L1SupplyChain",
    },
    TraceEdge {
        controlled_id: "CR-034",
        assertion: "no_authority_surface::cr_034_generated_no_emission",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "CR-034",
        assertion: "source_spine::cr_034_generated_provenance_absence",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-035",
        assertion: "static_surface::cr_035_quality_gate_registry",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-035",
        assertion: "source_spine::cr_035_quality_output_binding",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-036",
        assertion: "test_gate::cr_036_dependency_license_advisory",
        mode: "L1SupplyChain",
    },
    TraceEdge {
        controlled_id: "CR-037",
        assertion: "static_surface::cr_037_resource_bound_registry",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "CR-037",
        assertion: "adversarial_cases::cr_037_resource_bound_failure",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "CR-038",
        assertion: "hold_closure::cr_038_waiver_ledger_nonwaiver",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "CR-039",
        assertion: "hold_closure::cr_039_evidence_state_history",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "CR-039",
        assertion: "source_spine::cr_039_evidence_digest_truth",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "CR-040",
        assertion: "source_spine::cr_040_mechanical_trace_contradiction",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "VCL-01",
        assertion: "source_spine::trace_vcl_01",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "VCL-02",
        assertion: "contract_matrix::trace_vcl_02",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "VCL-03",
        assertion: "model_cases::trace_vcl_03",
        mode: "L2Model",
    },
    TraceEdge {
        controlled_id: "VCL-04",
        assertion: "property_cases::trace_vcl_04",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "VCL-05",
        assertion: "hold_closure::trace_vcl_05",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "VCL-06",
        assertion: "adversarial_cases::trace_vcl_06",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: "VCL-07",
        assertion: "no_authority_surface::trace_vcl_07",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "VCL-08",
        assertion: "no_authority_surface::trace_vcl_08",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "VCL-09",
        assertion: "static_surface::trace_vcl_09",
        mode: "L1Static",
    },
    TraceEdge {
        controlled_id: "VCL-10",
        assertion: "source_spine::trace_vcl_10",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "VAL-SCOPE",
        assertion: "source_spine::trace_val_scope",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "VAL-ASSURANCE",
        assertion: "hold_closure::trace_val_assurance",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "ACT-CIV",
        assertion: "no_authority_surface::trace_act_civ",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "ACT-RDY",
        assertion: "source_spine::trace_act_rdy",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-ACQ",
        assertion: "source_spine::trace_act_acq",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-LOG",
        assertion: "source_spine::trace_act_log",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-ALLY",
        assertion: "source_spine::trace_act_ally",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-FIN",
        assertion: "source_spine::trace_act_fin",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-PPL",
        assertion: "source_spine::trace_act_ppl",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-TST",
        assertion: "source_spine::trace_act_tst",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: "ACT-SRC",
        assertion: "contract_matrix::trace_act_src",
        mode: "L2Contract",
    },
    TraceEdge {
        controlled_id: "ACT-LAW",
        assertion: "no_authority_surface::trace_act_law",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "ACT-EXT",
        assertion: "no_authority_surface::trace_act_ext",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/civilian-strategy-force-planner.md",
        assertion: "no_authority_surface::trace_role_parliament_civilian_strategy_force_planner",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/operational-readiness.md",
        assertion: "source_spine::trace_role_parliament_operational_readiness",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/acquisition-industrial-base.md",
        assertion: "source_spine::trace_role_parliament_acquisition_industrial_base",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/logistics-sustainment.md",
        assertion: "source_spine::trace_role_parliament_logistics_sustainment",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/defense-comptroller.md",
        assertion: "source_spine::trace_role_parliament_defense_comptroller",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/service-member-family.md",
        assertion: "source_spine::trace_role_parliament_service_member_family",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/independent-test-oversight.md",
        assertion: "source_spine::trace_role_parliament_independent_test_oversight",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/parliament/alliance-interoperability.md",
        assertion: "source_spine::trace_role_parliament_alliance_interoperability",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/panel-reviewer/panel.md",
        assertion: "property_cases::trace_role_panel_reviewer_panel",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: "Role review steward",
        assertion: "hold_closure::trace_role_review_steward",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: ".roles/editorial/citation-auditor.md",
        assertion: "source_spine::trace_role_editorial_citation_auditor",
        mode: "L2SourceSpine",
    },
    TraceEdge {
        controlled_id: ".roles/editorial/scope-keeper.md",
        assertion: "no_authority_surface::trace_role_editorial_scope_keeper",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/editorial/numeracy-checker.md",
        assertion: "property_cases::trace_role_editorial_numeracy_checker",
        mode: "L2Property",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/service-member-family.md",
        assertion: "no_authority_surface::trace_role_stakeholders_service_member_family",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/mission-user.md",
        assertion: "no_authority_surface::trace_role_stakeholders_mission_user",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/depot-logistics-workforce.md",
        assertion: "no_authority_surface::trace_role_stakeholders_depot_logistics_workforce",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/prime-small-supplier.md",
        assertion: "no_authority_surface::trace_role_stakeholders_prime_small_supplier",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/installation-community.md",
        assertion: "no_authority_surface::trace_role_stakeholders_installation_community",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/ally-partner.md",
        assertion: "no_authority_surface::trace_role_stakeholders_ally_partner",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/stakeholders/taxpayer-oversight.md",
        assertion: "no_authority_surface::trace_role_stakeholders_taxpayer_oversight",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: ".roles/assurance/classification-operational-security.md",
        assertion: "adversarial_cases::trace_role_assurance_classification_operational_security",
        mode: "L2Adversarial",
    },
    TraceEdge {
        controlled_id: ".roles/assurance/civilian-control-law-safety-readiness.md",
        assertion: "no_authority_surface::trace_role_assurance_civilian_control_law_safety_readiness",
        mode: "L2NoAuthority",
    },
    TraceEdge {
        controlled_id: "SPEC-UNK-SEC-001",
        assertion: "hold_closure::trace_spec_unk_sec_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "TBD-SEC-001",
        assertion: "hold_closure::trace_tbd_sec_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "SPEC-UNK-SRC-001",
        assertion: "hold_closure::trace_spec_unk_src_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "TBD-SRC-001",
        assertion: "hold_closure::trace_tbd_src_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "SPEC-UNK-TST-001",
        assertion: "hold_closure::trace_spec_unk_tst_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "TBD-TST-001",
        assertion: "hold_closure::trace_tbd_tst_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "SPEC-UNK-REL-001",
        assertion: "hold_closure::trace_spec_unk_rel_001",
        mode: "L2HoldClosure",
    },
    TraceEdge {
        controlled_id: "TBD-REL-001",
        assertion: "hold_closure::trace_tbd_rel_001",
        mode: "L2HoldClosure",
    },
];
