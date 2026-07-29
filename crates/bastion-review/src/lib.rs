//! Dependency-free BASTION review, trace, and convergence controls.
//!
//! The crate validates and carries externally produced digest bonds. It does
//! not parse product content, authenticate external actors, or grant stage,
//! operational, procurement, budget, Taxlane, handoff, or release authority.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const MAX_ID: usize = 128;
const MAX_LARGE_SET: usize = 1_024;
const MAX_ROLES: usize = 128;
const MAX_GATES: usize = 32;
const MAX_CONFLICTS: usize = 256;

/// An opaque controlled identifier with no embedded content semantics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier(String);

impl Identifier {
    /// Constructs an identifier in the exact bounded ASCII grammar.
    ///
    /// # Errors
    ///
    /// Returns [`InputError::InvalidIdentifier`] for an empty, oversized, or
    /// nonconforming value.
    pub fn new(value: impl Into<String>) -> Result<Self, InputError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > MAX_ID
            || !value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':' | b'/')
            })
        {
            return Err(InputError::InvalidIdentifier);
        }
        Ok(Self(value))
    }

    /// Returns the opaque identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A syntactically validated caller-supplied SHA-256 identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digest256(String);

impl Digest256 {
    /// Accepts exactly 64 lower-case hexadecimal ASCII characters.
    ///
    /// # Errors
    ///
    /// Returns [`InputError::InvalidDigest`] for all other representations.
    pub fn new(value: impl Into<String>) -> Result<Self, InputError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(InputError::InvalidDigest);
        }
        Ok(Self(value))
    }
}

/// Exact predecessor in an immutable record chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Predecessor {
    /// Prior record identity.
    pub id: Identifier,
    /// Prior record payload digest.
    pub digest: Digest256,
    /// Prior version.
    pub version: u64,
}

/// Stable record identity, external payload digest, and version lineage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBinding {
    /// Stable identity.
    pub id: Identifier,
    /// Digest of canonical external payload excluding this binding.
    pub digest: Digest256,
    /// Positive version.
    pub version: u64,
    /// Optional exact predecessor.
    pub predecessor: Option<Predecessor>,
}

impl RecordBinding {
    fn valid(&self) -> bool {
        self.version > 0
            && self.predecessor.as_ref().is_none_or(|prior| {
                prior.version > 0 && self.id != prior.id && self.version > prior.version
            })
    }
}

/// One independently digest-bound exact applicability set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredSet {
    /// Set record custody.
    pub binding: RecordBinding,
    /// Exact unique expected identifiers.
    pub ids: Vec<Identifier>,
}

/// The seven independently frozen applicability sets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewPolicy {
    /// Policy record custody.
    pub binding: RecordBinding,
    /// Required role lanes.
    pub roles: RequiredSet,
    /// Required assurance gates.
    pub assurance: RequiredSet,
    /// Expected evidence methods.
    pub evidence_methods: RequiredSet,
    /// Required derivations.
    pub derivations: RequiredSet,
    /// Required negative cases.
    pub negative_cases: RequiredSet,
    /// Required unresolved questions.
    pub unresolved_questions: RequiredSet,
    /// Required trace links.
    pub trace_links: RequiredSet,
    /// Frozen role corpus version.
    pub role_corpus_version: u64,
}

/// Frozen security-admitted review subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenSubject {
    /// Subject identity.
    pub id: Identifier,
    /// Producer identity.
    pub producer_id: Identifier,
    /// Subject digest.
    pub digest: Digest256,
    /// Context digest.
    pub context_digest: Digest256,
    /// Security-admission digest.
    pub admission_digest: Digest256,
    /// Review generation.
    pub generation: u64,
    /// Admitted posture identity.
    pub admitted_posture_id: Identifier,
    /// Policy identity.
    pub policy_id: Identifier,
    /// Policy digest.
    pub policy_digest: Digest256,
    /// Policy version.
    pub policy_version: u64,
}

/// Truthful evidence lifecycle states.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceState {
    /// Planned only.
    Planned,
    /// Absent.
    Absent,
    /// Executed without accepted result.
    Executed,
    /// Current pass.
    Passed,
    /// Failure.
    Failed,
    /// Stale.
    Stale,
    /// Conflicted.
    Conflicted,
    /// Held.
    Held,
    /// Rejected.
    Rejected,
    /// Retained superseded record.
    Superseded,
}

/// One evidence record with all subject bonds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRecord {
    /// Record custody.
    pub binding: RecordBinding,
    /// Method identity.
    pub method_id: Identifier,
    /// Subject identity.
    pub subject_id: Identifier,
    /// Subject digest.
    pub subject_digest: Digest256,
    /// Context digest.
    pub context_digest: Digest256,
    /// Security-admission digest.
    pub admission_digest: Digest256,
    /// Evidence state.
    pub state: EvidenceState,
}

/// A role/assurance vote.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vote {
    /// Pass within the lane's authority.
    Pass,
    /// Hold convergence.
    Hold,
    /// Reject the reviewed branch.
    Reject,
}

/// Context-bound role or assurance decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneDecision {
    /// Decision custody.
    pub binding: RecordBinding,
    /// Role or gate identity.
    pub lane_id: Identifier,
    /// Independent reviewer identity.
    pub reviewer_id: Identifier,
    /// Subject identity.
    pub subject_id: Identifier,
    /// Subject digest.
    pub subject_digest: Digest256,
    /// Context digest.
    pub context_digest: Digest256,
    /// Admission digest.
    pub admission_digest: Digest256,
    /// Policy digest.
    pub policy_digest: Digest256,
    /// Review generation.
    pub generation: u64,
    /// Lane vote.
    pub vote: Vote,
}

/// Finding severity used only for review convergence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    /// Safety, legality, or control-critical defect.
    Critical,
    /// Material defect that prevents convergence.
    Major,
    /// Bounded defect that may remain explicitly dispositioned.
    Minor,
    /// Presentation-only defect.
    Editorial,
}

/// Explicit lifecycle of a retained finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FindingDisposition {
    /// Unresolved finding.
    Open,
    /// Remediation is evidenced externally.
    Remediated,
    /// Resolution is deferred under controlled custody.
    Deferred,
    /// Risk was accepted by external governance.
    AcceptedRisk,
    /// The underlying branch was rejected.
    Rejected,
}

/// Context-bound finding with external closure content referenced by ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finding {
    /// Finding custody.
    pub binding: RecordBinding,
    /// Frozen subject identity.
    pub subject_id: Identifier,
    /// Frozen subject digest.
    pub subject_digest: Digest256,
    /// Frozen context digest.
    pub context_digest: Digest256,
    /// Frozen security-admission digest.
    pub admission_digest: Digest256,
    /// Frozen review generation.
    pub generation: u64,
    /// Frozen policy digest.
    pub policy_digest: Digest256,
    /// Discovering role.
    pub role_id: Identifier,
    /// Finding severity.
    pub severity: Severity,
    /// Affected claim reference.
    pub affected_claim_id: Identifier,
    /// Supporting evidence references.
    pub evidence_ids: Vec<Identifier>,
    /// Explicit disposition.
    pub disposition: FindingDisposition,
    /// Responsible owner, when resolution custody is required.
    pub owner_id: Option<Identifier>,
    /// Controlled destination, when resolution custody is required.
    pub destination_id: Option<Identifier>,
    /// Controlled closure condition reference.
    pub closure_condition_id: Option<Identifier>,
    /// Whether the finding was independently produced.
    pub independent: bool,
    /// Retained dissent references.
    pub dissent_ids: Vec<Identifier>,
}

/// Resolution posture of retained contradictory evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictState {
    /// Conflict remains unresolved.
    Open,
    /// Conflict has an externally evidenced resolution.
    Resolved,
}

/// Explicit conflict between two evidence records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceConflict {
    /// Conflict custody.
    pub binding: RecordBinding,
    /// First evidence identity.
    pub first_evidence_id: Identifier,
    /// First evidence digest.
    pub first_evidence_digest: Digest256,
    /// Second evidence identity.
    pub second_evidence_id: Identifier,
    /// Second evidence digest.
    pub second_evidence_digest: Digest256,
    /// Highest plausible severity while unresolved.
    pub severity: Severity,
    /// Resolution owner.
    pub owner_id: Identifier,
    /// Controlled resolution trigger reference.
    pub resolution_trigger_id: Identifier,
    /// Current conflict posture.
    pub state: ConflictState,
}

/// Gate posture carried by a trace link without granting authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatePosture {
    /// Gate is not yet satisfied.
    Held,
    /// Gate evidence passed for this link only.
    Passed,
    /// Gate rejected this branch.
    Rejected,
}

/// Digest-bound parent/child trace relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceLink {
    /// Trace custody.
    pub binding: RecordBinding,
    /// Parent identity.
    pub parent_id: Identifier,
    /// Parent digest.
    pub parent_digest: Digest256,
    /// Child identity.
    pub child_id: Identifier,
    /// Child digest.
    pub child_digest: Digest256,
    /// Owning stage reference.
    pub stage_id: Identifier,
    /// Gate posture.
    pub gate_posture: GatePosture,
    /// Supporting evidence posture.
    pub evidence_state: EvidenceState,
    /// Optional invalidated or superseded link reference.
    pub supersedes_id: Option<Identifier>,
    /// Explicit downstream stages that this link does not authorize.
    pub next_stage_non_authorizations: Vec<Identifier>,
}

/// Non-authoritative review result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewDisposition {
    /// External acceptance may be considered.
    PassRecommended,
    /// Review is held.
    Hold,
    /// Branch is rejected.
    Reject,
}

/// Input-only externally accepted prior decision bond.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptanceReceipt {
    /// Receipt custody.
    pub binding: RecordBinding,
    /// Exact accepted prior review decision.
    pub decision: RecordBinding,
    /// External stage-controller role.
    pub controller_id: Identifier,
    /// Governance-only posture; this crate cannot create the receipt.
    pub posture: AcceptancePosture,
}

/// Input-only governance acceptance posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcceptancePosture {
    /// External stage governance accepted the bound decision.
    Accepted,
}

/// Complete bounded prior history view; deeper history remains digest-linked.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriorReviewSnapshot {
    /// Prior decision custody.
    pub decision: RecordBinding,
    /// Prior recommendation.
    pub disposition: ReviewDisposition,
    /// Prior subject digest.
    pub subject_digest: Digest256,
    /// Prior context digest.
    pub context_digest: Digest256,
    /// Prior admission digest.
    pub admission_digest: Digest256,
    /// Prior policy digest.
    pub policy_digest: Digest256,
    /// External acceptance when it exists.
    pub acceptance: Option<AcceptanceReceipt>,
    /// Retained findings and defers.
    pub findings: Vec<RecordBinding>,
    /// Retained dissent.
    pub dissents: Vec<RecordBinding>,
    /// Retained negative evidence.
    pub negative_evidence: Vec<RecordBinding>,
    /// Retained conflicts.
    pub conflicts: Vec<RecordBinding>,
    /// Retained trace.
    pub trace: Vec<RecordBinding>,
}

/// Complete frozen packet; payload content remains external.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewPacket {
    /// Packet custody.
    pub binding: RecordBinding,
    /// Frozen subject.
    pub subject: FrozenSubject,
    /// Independently frozen policy.
    pub policy: ReviewPolicy,
    /// Evidence manifest.
    pub evidence: Vec<EvidenceRecord>,
    /// Derivation bindings.
    pub derivations: Vec<RecordBinding>,
    /// Negative-case bindings.
    pub negative_cases: Vec<RecordBinding>,
    /// Unresolved-question bindings.
    pub unresolved_questions: Vec<RecordBinding>,
    /// Trace links.
    pub trace_links: Vec<TraceLink>,
    /// Role decisions.
    pub role_decisions: Vec<LaneDecision>,
    /// Assurance decisions.
    pub assurance_decisions: Vec<LaneDecision>,
    /// Findings and controlled deferrals.
    pub findings: Vec<Finding>,
    /// Dissent bindings.
    pub dissents: Vec<RecordBinding>,
    /// Evidence conflicts.
    pub conflicts: Vec<EvidenceConflict>,
    /// Optional complete prior snapshot.
    pub prior: Option<PriorReviewSnapshot>,
}

/// Deterministic convergence blockers.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BlockerCode {
    /// Cardinality bound exceeded.
    BoundExceeded,
    /// Binding or predecessor invalid.
    InvalidBinding,
    /// Subject/policy/admission/context mismatch.
    StaleBond,
    /// One of the seven required sets differs.
    RequiredSetMismatch,
    /// Duplicate record or required identity.
    DuplicateIdentity,
    /// Reviewer equals producer.
    SelfApproval,
    /// Evidence is non-current but not a rejection.
    EvidenceHeld,
    /// Evidence failed or was rejected.
    EvidenceRejected,
    /// Lane holds.
    VoteHeld,
    /// Lane rejects.
    VoteRejected,
    /// Open conflict.
    OpenConflict,
    /// Critical or major finding remains open.
    OpenSevereFinding,
    /// Finding or deferral lacks controlled custody.
    IncompleteFinding,
    /// Trace is stale, duplicated, or orphaned.
    InvalidTrace,
    /// A finding explicitly rejects the branch.
    FindingRejected,
    /// Prior history/acceptance not exact or append-preserved.
    HistoryMismatch,
}

/// Non-authoritative review recommendation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewDecision {
    /// Prospective decision custody supplied by caller.
    pub binding: RecordBinding,
    /// Recommendation only.
    pub disposition: ReviewDisposition,
    /// Exact blockers.
    pub blockers: Vec<BlockerCode>,
    /// Subject digest.
    pub subject_digest: Digest256,
    /// Context digest.
    pub context_digest: Digest256,
    /// Admission digest.
    pub admission_digest: Digest256,
    /// Policy digest.
    pub policy_digest: Digest256,
    /// Prior acceptance bond, if this recommends a successor.
    pub predecessor_acceptance: Option<RecordBinding>,
    /// Deterministically ordered retained finding bindings.
    pub findings: Vec<RecordBinding>,
    /// Deterministically ordered retained conflict bindings.
    pub conflicts: Vec<RecordBinding>,
    /// Deterministically ordered retained trace bindings.
    pub trace: Vec<RecordBinding>,
    /// Deterministically ordered retained dissent bindings.
    pub dissent: Vec<RecordBinding>,
}

/// Constructor failures before review evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputError {
    /// Identifier violates grammar or size.
    InvalidIdentifier,
    /// Digest violates exact syntax.
    InvalidDigest,
}

impl fmt::Display for InputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidIdentifier => "invalid identifier",
            Self::InvalidDigest => "invalid digest",
        })
    }
}

impl std::error::Error for InputError {}

/// Evaluates the complete packet without mutation or stage authority.
#[must_use]
pub fn evaluate(packet: &ReviewPacket, binding: RecordBinding) -> ReviewDecision {
    let mut blockers = Vec::new();
    validate_bounds(packet, &mut blockers);
    if !blockers.is_empty() {
        return ReviewDecision {
            binding,
            disposition: ReviewDisposition::Hold,
            blockers,
            subject_digest: packet.subject.digest.clone(),
            context_digest: packet.subject.context_digest.clone(),
            admission_digest: packet.subject.admission_digest.clone(),
            policy_digest: packet.policy.binding.digest.clone(),
            predecessor_acceptance: None,
            findings: Vec::new(),
            conflicts: Vec::new(),
            trace: Vec::new(),
            dissent: Vec::new(),
        };
    }
    validate_bindings(packet, &binding, &mut blockers);
    validate_subject(packet, &mut blockers);
    validate_required_sets(packet, &mut blockers);
    validate_evidence(packet, &mut blockers);
    validate_lanes(packet, &mut blockers);
    validate_findings(packet, &mut blockers);
    validate_trace(packet, &mut blockers);
    validate_history(packet, &binding, &mut blockers);
    if packet.conflicts.iter().any(|conflict| {
        conflict.state == ConflictState::Open && !is_historical_conflict(packet, conflict)
    }) {
        blockers.push(BlockerCode::OpenConflict);
    }
    blockers.sort_unstable();
    blockers.dedup();
    let history_valid = !blockers.contains(&BlockerCode::HistoryMismatch);
    let disposition = if blockers.iter().any(|item| {
        matches!(
            item,
            BlockerCode::EvidenceRejected
                | BlockerCode::VoteRejected
                | BlockerCode::FindingRejected
                | BlockerCode::SelfApproval
        )
    }) {
        ReviewDisposition::Reject
    } else if blockers.is_empty() {
        ReviewDisposition::PassRecommended
    } else {
        ReviewDisposition::Hold
    };
    let mut findings = packet
        .findings
        .iter()
        .map(|item| item.binding.clone())
        .collect::<Vec<_>>();
    let mut conflicts = packet
        .conflicts
        .iter()
        .map(|item| item.binding.clone())
        .collect::<Vec<_>>();
    let mut trace = packet
        .trace_links
        .iter()
        .map(|item| item.binding.clone())
        .collect::<Vec<_>>();
    let mut dissent = packet.dissents.clone();
    if let Some(prior) = &packet.prior {
        findings.extend(prior.findings.iter().cloned());
        conflicts.extend(prior.conflicts.iter().cloned());
        trace.extend(prior.trace.iter().cloned());
        dissent.extend(prior.dissents.iter().cloned());
    }
    for values in [&mut findings, &mut conflicts, &mut trace, &mut dissent] {
        values.sort_by(|left, right| left.id.cmp(&right.id));
        values.dedup();
    }
    ReviewDecision {
        binding,
        disposition,
        blockers,
        subject_digest: packet.subject.digest.clone(),
        context_digest: packet.subject.context_digest.clone(),
        admission_digest: packet.subject.admission_digest.clone(),
        policy_digest: packet.policy.binding.digest.clone(),
        predecessor_acceptance: packet
            .prior
            .as_ref()
            .filter(|prior| {
                prior.disposition == ReviewDisposition::PassRecommended && history_valid
            })
            .and_then(|prior| prior.acceptance.as_ref())
            .map(|receipt| receipt.binding.clone()),
        findings,
        conflicts,
        trace,
        dissent,
    }
}

fn validate_bounds(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    let large = [
        packet.evidence.len(),
        packet.derivations.len(),
        packet.negative_cases.len(),
        packet.unresolved_questions.len(),
        packet.trace_links.len(),
        packet.findings.len(),
        packet.dissents.len(),
    ];
    if large.into_iter().any(|count| count > MAX_LARGE_SET)
        || packet.conflicts.len() > MAX_CONFLICTS
        || packet.role_decisions.len() > MAX_ROLES
        || packet.assurance_decisions.len() > MAX_GATES
        || packet.policy.roles.ids.len() > MAX_ROLES
        || packet.policy.assurance.ids.len() > MAX_GATES
        || [
            packet.policy.evidence_methods.ids.len(),
            packet.policy.derivations.ids.len(),
            packet.policy.negative_cases.ids.len(),
            packet.policy.unresolved_questions.ids.len(),
            packet.policy.trace_links.ids.len(),
        ]
        .into_iter()
        .any(|count| count > MAX_LARGE_SET)
        || packet
            .findings
            .iter()
            .any(|item| item.evidence_ids.len() > MAX_ID || item.dissent_ids.len() > MAX_ID)
        || packet
            .trace_links
            .iter()
            .any(|item| item.next_stage_non_authorizations.len() > MAX_ID)
    {
        blockers.push(BlockerCode::BoundExceeded);
    }
    if let Some(prior) = &packet.prior
        && (prior.findings.len() > MAX_LARGE_SET
            || prior.dissents.len() > MAX_LARGE_SET
            || prior.negative_evidence.len() > MAX_LARGE_SET
            || prior.conflicts.len() > MAX_CONFLICTS
            || prior.trace.len() > MAX_LARGE_SET)
    {
        blockers.push(BlockerCode::BoundExceeded);
    }
}

fn validate_bindings(
    packet: &ReviewPacket,
    decision: &RecordBinding,
    blockers: &mut Vec<BlockerCode>,
) {
    let policy_sets = [
        &packet.policy.roles,
        &packet.policy.assurance,
        &packet.policy.evidence_methods,
        &packet.policy.derivations,
        &packet.policy.negative_cases,
        &packet.policy.unresolved_questions,
        &packet.policy.trace_links,
    ];
    let bindings = all_bindings(packet);
    let primary_bindings = [
        &packet.binding,
        &packet.policy.binding,
        &packet.policy.roles.binding,
        &packet.policy.assurance.binding,
        &packet.policy.evidence_methods.binding,
        &packet.policy.derivations.binding,
        &packet.policy.negative_cases.binding,
        &packet.policy.unresolved_questions.binding,
        &packet.policy.trace_links.binding,
        decision,
    ];
    let mut binding_ids = BTreeSet::new();
    let invalid = primary_bindings.iter().any(|item| !item.valid())
        || policy_sets.iter().any(|set| !set.binding.valid())
        || bindings.iter().any(|item| !item.valid());
    if invalid {
        blockers.push(BlockerCode::InvalidBinding);
    }
    if primary_bindings
        .into_iter()
        .chain(bindings)
        .any(|item| !binding_ids.insert(&item.id))
    {
        blockers.push(BlockerCode::DuplicateIdentity);
    }
}

fn validate_subject(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    if packet.subject.generation == 0
        || packet.policy.role_corpus_version == 0
        || packet.subject.policy_id != packet.policy.binding.id
        || packet.subject.policy_digest != packet.policy.binding.digest
        || packet.subject.policy_version != packet.policy.binding.version
    {
        blockers.push(BlockerCode::StaleBond);
    }
}

fn validate_required_sets(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    let actual_roles: Vec<_> = packet
        .role_decisions
        .iter()
        .map(|item| item.lane_id.clone())
        .collect();
    let actual_gates: Vec<_> = packet
        .assurance_decisions
        .iter()
        .map(|item| item.lane_id.clone())
        .collect();
    let actual_methods: Vec<_> = packet
        .evidence
        .iter()
        .map(|item| item.method_id.clone())
        .collect();
    let actual_derivations = binding_ids(&packet.derivations);
    let actual_negative_cases = binding_ids(&packet.negative_cases);
    let actual_questions = binding_ids(&packet.unresolved_questions);
    let actual_trace = packet
        .trace_links
        .iter()
        .map(|item| item.binding.id.clone())
        .collect::<Vec<_>>();
    let checks = [
        (&packet.policy.roles.ids, &actual_roles),
        (&packet.policy.assurance.ids, &actual_gates),
        (&packet.policy.evidence_methods.ids, &actual_methods),
        (&packet.policy.derivations.ids, &actual_derivations),
        (&packet.policy.negative_cases.ids, &actual_negative_cases),
        (&packet.policy.unresolved_questions.ids, &actual_questions),
        (&packet.policy.trace_links.ids, &actual_trace),
    ];
    if checks
        .into_iter()
        .any(|(expected, actual)| exact_policy_set(expected) != exact_set(actual))
    {
        blockers.push(BlockerCode::RequiredSetMismatch);
    }
}

fn validate_findings(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    let evidence_ids = packet
        .evidence
        .iter()
        .map(|item| &item.binding.id)
        .collect::<BTreeSet<_>>();
    let dissent_ids = packet
        .dissents
        .iter()
        .map(|item| &item.id)
        .collect::<BTreeSet<_>>();
    for finding in &packet.findings {
        let historical = is_historical_finding(packet, finding);
        if finding.subject_id != packet.subject.id
            || finding.subject_digest != packet.subject.digest
            || finding.context_digest != packet.subject.context_digest
            || finding.admission_digest != packet.subject.admission_digest
            || finding.generation != packet.subject.generation
            || finding.policy_digest != packet.policy.binding.digest
        {
            blockers.push(BlockerCode::StaleBond);
        }
        if !finding.independent
            || finding.evidence_ids.is_empty()
            || !finding
                .evidence_ids
                .iter()
                .all(|id| evidence_ids.contains(id))
            || !finding
                .dissent_ids
                .iter()
                .all(|id| dissent_ids.contains(id))
            || !packet.policy.roles.ids.contains(&finding.role_id)
        {
            blockers.push(BlockerCode::IncompleteFinding);
        }
        if !historical
            && matches!(finding.severity, Severity::Critical | Severity::Major)
            && finding.disposition != FindingDisposition::Remediated
        {
            blockers.push(BlockerCode::OpenSevereFinding);
        }
        if !historical && finding.disposition == FindingDisposition::Open {
            blockers.push(BlockerCode::IncompleteFinding);
        }
        let needs_closure = matches!(
            finding.disposition,
            FindingDisposition::Remediated
                | FindingDisposition::Deferred
                | FindingDisposition::AcceptedRisk
        );
        if !historical
            && needs_closure
            && (finding.owner_id.is_none()
                || finding.destination_id.is_none()
                || finding.closure_condition_id.is_none())
        {
            blockers.push(BlockerCode::IncompleteFinding);
        }
        if !historical && finding.disposition == FindingDisposition::Rejected {
            blockers.push(BlockerCode::FindingRejected);
        }
    }
}

fn is_historical_finding(packet: &ReviewPacket, finding: &Finding) -> bool {
    let Some(prior) = &packet.prior else {
        return false;
    };
    if !prior
        .findings
        .iter()
        .any(|binding| binding == &finding.binding)
    {
        return false;
    }
    packet.findings.iter().any(|successor| {
        successor.binding.predecessor.as_ref().is_some_and(|link| {
            link.id == finding.binding.id
                && link.digest == finding.binding.digest
                && link.version == finding.binding.version
        }) && successor.subject_id == finding.subject_id
            && successor.subject_digest == finding.subject_digest
            && successor.context_digest == finding.context_digest
            && successor.admission_digest == finding.admission_digest
            && successor.generation == finding.generation
            && successor.policy_digest == finding.policy_digest
            && successor.role_id == finding.role_id
            && successor.severity == finding.severity
            && successor.affected_claim_id == finding.affected_claim_id
    })
}

fn is_historical_conflict(packet: &ReviewPacket, conflict: &EvidenceConflict) -> bool {
    let Some(prior) = &packet.prior else {
        return false;
    };
    if !prior
        .conflicts
        .iter()
        .any(|binding| binding == &conflict.binding)
    {
        return false;
    }
    packet.conflicts.iter().any(|successor| {
        successor.state == ConflictState::Resolved
            && successor.binding.predecessor.as_ref().is_some_and(|link| {
                link.id == conflict.binding.id
                    && link.digest == conflict.binding.digest
                    && link.version == conflict.binding.version
            })
            && successor.first_evidence_id == conflict.first_evidence_id
            && successor.first_evidence_digest == conflict.first_evidence_digest
            && successor.second_evidence_id == conflict.second_evidence_id
            && successor.second_evidence_digest == conflict.second_evidence_digest
    })
}

fn validate_trace(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    let evidence = packet
        .evidence
        .iter()
        .map(|item| (&item.binding.id, item))
        .collect::<BTreeMap<_, _>>();
    let trace_by_id = packet
        .trace_links
        .iter()
        .map(|item| (&item.binding.id, item))
        .collect::<BTreeMap<_, _>>();
    let mut link_ids = BTreeSet::new();
    for link in &packet.trace_links {
        if !link_ids.insert(&link.binding.id)
            || link.parent_id == link.child_id
            || link.parent_id != packet.subject.id
            || link.parent_digest != packet.subject.digest
            || evidence.get(&link.child_id).is_none_or(|row| {
                row.binding.digest != link.child_digest || row.state != link.evidence_state
            })
            || link.gate_posture != GatePosture::Passed
            || link.next_stage_non_authorizations.is_empty()
            || link.supersedes_id.as_ref().is_some_and(|id| {
                id == &link.binding.id
                    || trace_by_id.get(id).is_none_or(|prior| {
                        link.binding.predecessor.as_ref().is_none_or(|predecessor| {
                            predecessor.id != prior.binding.id
                                || predecessor.digest != prior.binding.digest
                                || predecessor.version != prior.binding.version
                        })
                    })
            })
        {
            blockers.push(BlockerCode::InvalidTrace);
        }
    }
    for conflict in &packet.conflicts {
        if conflict.first_evidence_id == conflict.second_evidence_id
            || evidence
                .get(&conflict.first_evidence_id)
                .is_none_or(|row| row.binding.digest != conflict.first_evidence_digest)
            || evidence
                .get(&conflict.second_evidence_id)
                .is_none_or(|row| row.binding.digest != conflict.second_evidence_digest)
            || (conflict.state == ConflictState::Resolved
                && conflict.binding.predecessor.as_ref().is_none_or(|link| {
                    packet.prior.as_ref().is_none_or(|prior| {
                        !prior.conflicts.iter().any(|binding| {
                            binding.id == link.id
                                && binding.digest == link.digest
                                && binding.version == link.version
                        })
                    })
                }))
        {
            blockers.push(BlockerCode::StaleBond);
        }
    }
}

fn validate_evidence(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    let mut methods = BTreeMap::<&Identifier, Vec<&EvidenceRecord>>::new();
    for item in &packet.evidence {
        methods.entry(&item.method_id).or_default().push(item);
        if item.subject_id != packet.subject.id
            || item.subject_digest != packet.subject.digest
            || item.context_digest != packet.subject.context_digest
            || item.admission_digest != packet.subject.admission_digest
        {
            blockers.push(BlockerCode::StaleBond);
        }
    }
    for records in methods.values() {
        if records.len() != 1 {
            blockers.push(BlockerCode::DuplicateIdentity);
        }
        for record in records {
            match record.state {
                EvidenceState::Passed => {}
                EvidenceState::Failed | EvidenceState::Rejected => {
                    blockers.push(BlockerCode::EvidenceRejected);
                }
                EvidenceState::Planned
                | EvidenceState::Absent
                | EvidenceState::Executed
                | EvidenceState::Stale
                | EvidenceState::Conflicted
                | EvidenceState::Held
                | EvidenceState::Superseded => blockers.push(BlockerCode::EvidenceHeld),
            }
        }
    }
}

fn validate_lanes(packet: &ReviewPacket, blockers: &mut Vec<BlockerCode>) {
    for lane in packet
        .role_decisions
        .iter()
        .chain(packet.assurance_decisions.iter())
    {
        if lane.reviewer_id == packet.subject.producer_id {
            blockers.push(BlockerCode::SelfApproval);
        }
        if lane.subject_id != packet.subject.id
            || lane.subject_digest != packet.subject.digest
            || lane.context_digest != packet.subject.context_digest
            || lane.admission_digest != packet.subject.admission_digest
            || lane.policy_digest != packet.policy.binding.digest
            || lane.generation != packet.subject.generation
        {
            blockers.push(BlockerCode::StaleBond);
        }
        match lane.vote {
            Vote::Pass => {}
            Vote::Hold => blockers.push(BlockerCode::VoteHeld),
            Vote::Reject => blockers.push(BlockerCode::VoteRejected),
        }
    }
}

fn validate_history(
    packet: &ReviewPacket,
    decision: &RecordBinding,
    blockers: &mut Vec<BlockerCode>,
) {
    let Some(prior) = &packet.prior else {
        if decision.predecessor.is_some() {
            blockers.push(BlockerCode::HistoryMismatch);
        }
        return;
    };
    let Some(predecessor) = &decision.predecessor else {
        blockers.push(BlockerCode::HistoryMismatch);
        return;
    };
    let prior_bindings = prior
        .findings
        .iter()
        .chain(&prior.dissents)
        .chain(&prior.negative_evidence)
        .chain(&prior.conflicts)
        .chain(&prior.trace);
    let prior_ids = prior_bindings
        .clone()
        .map(|item| item.id.clone())
        .collect::<Vec<_>>();
    let current_ids = all_bindings(packet)
        .into_iter()
        .map(|item| &item.id)
        .collect::<BTreeSet<_>>();
    if !prior.decision.valid()
        || prior_bindings.into_iter().any(|item| !item.valid())
        || exact_set(&prior_ids).is_none()
        || prior_ids.iter().any(|id| current_ids.contains(id))
        || prior
            .acceptance
            .as_ref()
            .is_some_and(|receipt| !receipt.binding.valid() || !receipt.decision.valid())
        || prior.disposition != ReviewDisposition::PassRecommended
        || prior.acceptance.is_none()
        || prior.acceptance.as_ref().is_some_and(|receipt| {
            receipt.controller_id == packet.subject.producer_id
                || packet
                    .role_decisions
                    .iter()
                    .chain(&packet.assurance_decisions)
                    .any(|lane| lane.reviewer_id == receipt.controller_id)
        })
        || predecessor.id != prior.decision.id
        || predecessor.digest != prior.decision.digest
        || predecessor.version != prior.decision.version
        || (prior.disposition == ReviewDisposition::PassRecommended
            && prior
                .acceptance
                .as_ref()
                .is_none_or(|receipt| receipt.decision != prior.decision))
    {
        blockers.push(BlockerCode::HistoryMismatch);
    }
}

fn exact_set(values: &[Identifier]) -> Option<BTreeSet<&Identifier>> {
    let mut result = BTreeSet::new();
    for value in values {
        if !result.insert(value) {
            return None;
        }
    }
    Some(result)
}

fn exact_policy_set(values: &[Identifier]) -> Option<BTreeSet<&Identifier>> {
    if values.is_empty() || !values.windows(2).all(|pair| pair[0] < pair[1]) {
        return None;
    }
    exact_set(values)
}

fn binding_ids(values: &[RecordBinding]) -> Vec<Identifier> {
    values.iter().map(|item| item.id.clone()).collect()
}

fn all_bindings(packet: &ReviewPacket) -> Vec<&RecordBinding> {
    packet
        .evidence
        .iter()
        .map(|item| &item.binding)
        .chain(packet.derivations.iter())
        .chain(packet.negative_cases.iter())
        .chain(packet.unresolved_questions.iter())
        .chain(packet.trace_links.iter().map(|item| &item.binding))
        .chain(packet.role_decisions.iter().map(|item| &item.binding))
        .chain(packet.assurance_decisions.iter().map(|item| &item.binding))
        .chain(packet.findings.iter().map(|item| &item.binding))
        .chain(packet.dissents.iter())
        .chain(packet.conflicts.iter().map(|item| &item.binding))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> Identifier {
        match Identifier::new(value) {
            Ok(item) => item,
            Err(error) => panic!("valid fixture id: {error}"),
        }
    }

    fn digest(character: char) -> Digest256 {
        match Digest256::new(character.to_string().repeat(64)) {
            Ok(item) => item,
            Err(error) => panic!("valid fixture digest: {error}"),
        }
    }

    fn binding(name: &str) -> RecordBinding {
        RecordBinding {
            id: id(name),
            digest: digest('a'),
            version: 1,
            predecessor: None,
        }
    }

    fn set(name: &str, value: &str) -> RequiredSet {
        RequiredSet {
            binding: binding(name),
            ids: vec![id(value)],
        }
    }

    fn packet(state: EvidenceState) -> ReviewPacket {
        let policy = ReviewPolicy {
            binding: binding("policy:1"),
            roles: set("set:roles", "role:test"),
            assurance: set("set:gates", "gate:security"),
            evidence_methods: set("set:evidence", "method:test"),
            derivations: set("set:derivations", "derivation:1"),
            negative_cases: set("set:negative", "negative:1"),
            unresolved_questions: set("set:questions", "question:1"),
            trace_links: set("set:trace", "trace:1"),
            role_corpus_version: 1,
        };
        let subject = FrozenSubject {
            id: id("subject:1"),
            producer_id: id("producer:1"),
            digest: digest('b'),
            context_digest: digest('c'),
            admission_digest: digest('d'),
            generation: 1,
            admitted_posture_id: id("posture:admitted"),
            policy_id: policy.binding.id.clone(),
            policy_digest: policy.binding.digest.clone(),
            policy_version: policy.binding.version,
        };
        let lane = |record: &str, lane_id: &str, reviewer: &str| LaneDecision {
            binding: binding(record),
            lane_id: id(lane_id),
            reviewer_id: id(reviewer),
            subject_id: subject.id.clone(),
            subject_digest: subject.digest.clone(),
            context_digest: subject.context_digest.clone(),
            admission_digest: subject.admission_digest.clone(),
            policy_digest: policy.binding.digest.clone(),
            generation: 1,
            vote: Vote::Pass,
        };
        let role_decision = lane("decision:role", "role:test", "reviewer:1");
        let assurance_decision = lane("decision:gate", "gate:security", "reviewer:2");
        ReviewPacket {
            binding: binding("packet:1"),
            subject: subject.clone(),
            policy,
            evidence: vec![EvidenceRecord {
                binding: binding("evidence:1"),
                method_id: id("method:test"),
                subject_id: subject.id.clone(),
                subject_digest: subject.digest.clone(),
                context_digest: subject.context_digest.clone(),
                admission_digest: subject.admission_digest.clone(),
                state,
            }],
            derivations: vec![binding("derivation:1")],
            negative_cases: vec![binding("negative:1")],
            unresolved_questions: vec![binding("question:1")],
            trace_links: vec![TraceLink {
                binding: binding("trace:1"),
                parent_id: subject.id,
                parent_digest: subject.digest,
                child_id: id("evidence:1"),
                child_digest: digest('a'),
                stage_id: id("stage:review"),
                gate_posture: GatePosture::Passed,
                evidence_state: EvidenceState::Passed,
                supersedes_id: None,
                next_stage_non_authorizations: vec![id("stage:release")],
            }],
            role_decisions: vec![role_decision],
            assurance_decisions: vec![assurance_decision],
            findings: Vec::new(),
            dissents: Vec::new(),
            conflicts: Vec::new(),
            prior: None,
        }
    }

    fn minor_finding_named(name: &str) -> Finding {
        Finding {
            binding: binding(name),
            subject_id: id("subject:1"),
            subject_digest: digest('b'),
            context_digest: digest('c'),
            admission_digest: digest('d'),
            generation: 1,
            policy_digest: digest('a'),
            role_id: id("role:test"),
            severity: Severity::Minor,
            affected_claim_id: id("claim:1"),
            evidence_ids: vec![id("evidence:1")],
            disposition: FindingDisposition::Remediated,
            owner_id: Some(id("owner:1")),
            destination_id: Some(id("destination:1")),
            closure_condition_id: Some(id("closure:1")),
            independent: true,
            dissent_ids: Vec::new(),
        }
    }

    #[test]
    fn complete_packet_recommends_pass_only() {
        let result = evaluate(&packet(EvidenceState::Passed), binding("review:1"));
        assert_eq!(result.disposition, ReviewDisposition::PassRecommended);
        assert!(result.predecessor_acceptance.is_none());
    }

    #[test]
    fn omitted_required_member_holds() {
        let mut candidate = packet(EvidenceState::Passed);
        candidate.negative_cases.clear();
        let result = evaluate(&candidate, binding("review:1"));
        assert_eq!(result.disposition, ReviewDisposition::Hold);
        assert!(result.blockers.contains(&BlockerCode::RequiredSetMismatch));
    }

    #[test]
    fn all_non_pass_states_block() {
        let cases = [
            (EvidenceState::Planned, ReviewDisposition::Hold),
            (EvidenceState::Absent, ReviewDisposition::Hold),
            (EvidenceState::Executed, ReviewDisposition::Hold),
            (EvidenceState::Failed, ReviewDisposition::Reject),
            (EvidenceState::Stale, ReviewDisposition::Hold),
            (EvidenceState::Conflicted, ReviewDisposition::Hold),
            (EvidenceState::Held, ReviewDisposition::Hold),
            (EvidenceState::Rejected, ReviewDisposition::Reject),
            (EvidenceState::Superseded, ReviewDisposition::Hold),
        ];
        for (state, expected) in cases {
            assert_eq!(
                evaluate(&packet(state), binding("review:1")).disposition,
                expected
            );
        }
    }

    #[test]
    fn context_or_admission_replay_holds() {
        let mut candidate = packet(EvidenceState::Passed);
        candidate.role_decisions[0].admission_digest = digest('e');
        assert_eq!(
            evaluate(&candidate, binding("review:1")).disposition,
            ReviewDisposition::Hold
        );
    }

    #[test]
    fn self_approval_rejects() {
        let mut candidate = packet(EvidenceState::Passed);
        candidate.role_decisions[0].reviewer_id = candidate.subject.producer_id.clone();
        assert_eq!(
            evaluate(&candidate, binding("review:1")).disposition,
            ReviewDisposition::Reject
        );
    }

    #[test]
    fn constructors_reject_payload_shapes() {
        assert!(Identifier::new("").is_err());
        assert!(Identifier::new("target payload").is_err());
        assert!(Identifier::new("x".repeat(129)).is_err());
        assert!(Digest256::new("A".repeat(64)).is_err());
        assert!(Digest256::new("a".repeat(63)).is_err());
    }

    #[test]
    fn contract_matrix_exact_sets_and_bonds() {
        let candidate = packet(EvidenceState::Passed);
        let decision = evaluate(&candidate, binding("review:contract"));
        assert_eq!(decision.disposition, ReviewDisposition::PassRecommended);
        assert_eq!(decision.trace, vec![binding("trace:1")]);

        let mut omitted = candidate.clone();
        omitted.assurance_decisions.clear();
        assert!(
            evaluate(&omitted, binding("review:held"))
                .blockers
                .contains(&BlockerCode::RequiredSetMismatch)
        );
    }

    #[test]
    fn model_cases_preserve_accepted_history() {
        let prior_decision = binding("review:prior");
        let retained = minor_finding_named("finding:1");
        let mut candidate = packet(EvidenceState::Passed);
        candidate.prior = Some(PriorReviewSnapshot {
            decision: prior_decision.clone(),
            disposition: ReviewDisposition::PassRecommended,
            subject_digest: digest('b'),
            context_digest: digest('c'),
            admission_digest: digest('d'),
            policy_digest: digest('a'),
            acceptance: Some(AcceptanceReceipt {
                binding: binding("acceptance:prior"),
                decision: prior_decision.clone(),
                controller_id: id("controller:stage"),
                posture: AcceptancePosture::Accepted,
            }),
            findings: vec![retained.binding.clone()],
            dissents: Vec::new(),
            negative_evidence: vec![binding("negative:prior")],
            conflicts: Vec::new(),
            trace: vec![binding("trace:prior")],
        });
        let successor = RecordBinding {
            id: id("review:successor"),
            digest: digest('f'),
            version: 2,
            predecessor: Some(Predecessor {
                id: prior_decision.id.clone(),
                digest: prior_decision.digest.clone(),
                version: prior_decision.version,
            }),
        };
        let retained_decision = evaluate(&candidate, successor);
        assert_eq!(
            retained_decision.disposition,
            ReviewDisposition::PassRecommended
        );
        assert!(retained_decision.findings.contains(&retained.binding));
    }

    #[test]
    fn adversarial_cases_fail_closed() {
        let mut candidate = packet(EvidenceState::Passed);
        let mut finding = minor_finding_named("finding:1");
        finding.severity = Severity::Critical;
        finding.disposition = FindingDisposition::Open;
        candidate.findings.push(finding);
        candidate.trace_links[0].child_digest = digest('f');
        let decision = evaluate(&candidate, binding("review:attack"));
        assert_eq!(decision.disposition, ReviewDisposition::Hold);
        assert!(decision.blockers.contains(&BlockerCode::OpenSevereFinding));
        assert!(decision.blockers.contains(&BlockerCode::InvalidTrace));
    }

    #[test]
    fn contract_matrix_each_required_set_is_exact() {
        for index in 0..7 {
            let mut candidate = packet(EvidenceState::Passed);
            match index {
                0 => candidate.role_decisions.clear(),
                1 => candidate.assurance_decisions.clear(),
                2 => candidate.evidence.clear(),
                3 => candidate.derivations.clear(),
                4 => candidate.negative_cases.clear(),
                5 => candidate.unresolved_questions.clear(),
                6 => candidate.trace_links.clear(),
                _ => unreachable!(),
            }
            assert!(
                evaluate(&candidate, binding("review:required-set"))
                    .blockers
                    .contains(&BlockerCode::RequiredSetMismatch),
                "set {index}"
            );
        }

        let mut oversized = packet(EvidenceState::Passed);
        oversized.evidence = vec![oversized.evidence[0].clone(); MAX_LARGE_SET + 1];
        let decision = evaluate(&oversized, binding("review:oversized"));
        assert_eq!(decision.blockers, vec![BlockerCode::BoundExceeded]);
        assert!(decision.trace.is_empty());
    }

    #[test]
    fn model_cases_canonicalize_retained_output() {
        let mut left = packet(EvidenceState::Passed);
        left.findings = vec![
            minor_finding_named("finding:b"),
            minor_finding_named("finding:a"),
        ];
        let mut right = left.clone();
        right.findings.reverse();
        assert_eq!(
            evaluate(&left, binding("review:canonical")),
            evaluate(&right, binding("review:canonical"))
        );
    }

    #[test]
    fn adversarial_cases_reject_orphan_supersession_and_controller_conflict() {
        let mut candidate = packet(EvidenceState::Passed);
        candidate.trace_links[0].supersedes_id = Some(id("trace:missing"));
        assert!(
            evaluate(&candidate, binding("review:trace-attack"))
                .blockers
                .contains(&BlockerCode::InvalidTrace)
        );

        let prior_decision = binding("review:prior-controller");
        candidate.trace_links[0].supersedes_id = None;
        candidate.prior = Some(PriorReviewSnapshot {
            decision: prior_decision.clone(),
            disposition: ReviewDisposition::PassRecommended,
            subject_digest: digest('b'),
            context_digest: digest('c'),
            admission_digest: digest('d'),
            policy_digest: digest('a'),
            acceptance: Some(AcceptanceReceipt {
                binding: binding("acceptance:controller-conflict"),
                decision: prior_decision.clone(),
                controller_id: id("producer:1"),
                posture: AcceptancePosture::Accepted,
            }),
            findings: Vec::new(),
            dissents: Vec::new(),
            negative_evidence: vec![binding("negative:1")],
            conflicts: Vec::new(),
            trace: vec![binding("trace:1")],
        });
        let successor = RecordBinding {
            id: id("review:controller-successor"),
            digest: digest('f'),
            version: 2,
            predecessor: Some(Predecessor {
                id: prior_decision.id,
                digest: prior_decision.digest,
                version: prior_decision.version,
            }),
        };
        assert!(
            evaluate(&candidate, successor)
                .blockers
                .contains(&BlockerCode::HistoryMismatch)
        );
    }

    #[test]
    fn adversarial_cases_reject_fake_history_and_incomplete_closure() {
        let mut candidate = packet(EvidenceState::Passed);
        let mut open = minor_finding_named("finding:open-critical");
        open.severity = Severity::Critical;
        open.disposition = FindingDisposition::Open;
        let mut fake_successor = minor_finding_named("finding:fake-successor");
        fake_successor.binding = RecordBinding {
            id: id("finding:fake-successor"),
            digest: digest('f'),
            version: 2,
            predecessor: Some(Predecessor {
                id: open.binding.id.clone(),
                digest: open.binding.digest.clone(),
                version: open.binding.version,
            }),
        };
        fake_successor.severity = Severity::Critical;
        candidate.findings = vec![open, fake_successor];
        assert!(
            evaluate(&candidate, binding("review:fake-history"))
                .blockers
                .contains(&BlockerCode::OpenSevereFinding)
        );

        let mut incomplete = packet(EvidenceState::Passed);
        let mut finding = minor_finding_named("finding:incomplete");
        finding.owner_id = None;
        incomplete.findings.push(finding);
        assert!(
            evaluate(&incomplete, binding("review:incomplete"))
                .blockers
                .contains(&BlockerCode::IncompleteFinding)
        );

        let mut self_superseding = packet(EvidenceState::Passed);
        self_superseding.trace_links[0].supersedes_id = Some(id("trace:1"));
        assert!(
            evaluate(&self_superseding, binding("review:self-superseding"))
                .blockers
                .contains(&BlockerCode::InvalidTrace)
        );
    }

    #[test]
    fn model_cases_allow_fully_refreshed_changed_context_successor() {
        let prior_decision = binding("review:prior-context");
        let mut candidate = packet(EvidenceState::Passed);
        candidate.subject.digest = digest('e');
        candidate.subject.context_digest = digest('f');
        candidate.subject.admission_digest = digest('1');
        for evidence in &mut candidate.evidence {
            evidence.subject_digest = digest('e');
            evidence.context_digest = digest('f');
            evidence.admission_digest = digest('1');
        }
        for lane in candidate
            .role_decisions
            .iter_mut()
            .chain(&mut candidate.assurance_decisions)
        {
            lane.subject_digest = digest('e');
            lane.context_digest = digest('f');
            lane.admission_digest = digest('1');
        }
        candidate.trace_links[0].parent_digest = digest('e');
        candidate.prior = Some(PriorReviewSnapshot {
            decision: prior_decision.clone(),
            disposition: ReviewDisposition::PassRecommended,
            subject_digest: digest('b'),
            context_digest: digest('c'),
            admission_digest: digest('d'),
            policy_digest: digest('a'),
            acceptance: Some(AcceptanceReceipt {
                binding: binding("acceptance:prior-context"),
                decision: prior_decision.clone(),
                controller_id: id("controller:context"),
                posture: AcceptancePosture::Accepted,
            }),
            findings: Vec::new(),
            dissents: Vec::new(),
            negative_evidence: vec![binding("negative:old")],
            conflicts: Vec::new(),
            trace: vec![binding("trace:old")],
        });
        let successor = RecordBinding {
            id: id("review:context-successor"),
            digest: digest('2'),
            version: 2,
            predecessor: Some(Predecessor {
                id: prior_decision.id,
                digest: prior_decision.digest,
                version: prior_decision.version,
            }),
        };
        let decision = evaluate(&candidate, successor);
        assert_eq!(decision.disposition, ReviewDisposition::PassRecommended);
        assert!(decision.trace.contains(&binding("trace:old")));
    }

    #[test]
    fn static_surface_has_no_io_or_unsafe_escape() {
        let production = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or("");
        for prohibited in [
            "unsafe {",
            "std::fs",
            "std::net",
            "std::env",
            "std::process",
            "std::thread",
            "unwrap(",
            "expect(",
            "todo!",
            "unimplemented!",
        ] {
            assert!(!production.contains(prohibited), "found {prohibited}");
        }
    }

    #[test]
    fn no_authority_surface_returns_review_only() {
        let decision = evaluate(&packet(EvidenceState::Passed), binding("review:surface"));
        assert_eq!(decision.disposition, ReviewDisposition::PassRecommended);
        assert!(decision.predecessor_acceptance.is_none());
        let production = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or("");
        for prohibited in [
            "pub fn release",
            "pub fn accept",
            "pub fn mutate",
            "pub fn emit",
        ] {
            assert!(!production.contains(prohibited), "found {prohibited}");
        }
    }
}
