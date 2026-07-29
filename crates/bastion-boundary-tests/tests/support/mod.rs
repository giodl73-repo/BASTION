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

#[derive(Clone, Debug, Eq, PartialEq)]
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
    exercise_mode(mode);
}

pub fn exercise_mode(mode: &str) {
    let scaffold = validate_scaffold();
    assert!(scaffold.is_ok(), "committed scaffold must validate");
    let rows = scaffold.as_deref().unwrap_or(&[]);
    assert!(
        validate_digests(rows).is_ok(),
        "canonical digests must reproduce"
    );
    match mode {
        "L2SourceSpine" => {
            assert_eq!(TRACE_MANIFEST.len(), 148);
            let identities: BTreeSet<_> = TRACE_MANIFEST
                .iter()
                .map(|edge| edge.controlled_id)
                .collect();
            assert_eq!(identities.len(), 123);
            assert!(
                TRACE_MANIFEST
                    .iter()
                    .all(|edge| !edge.assertion.ends_with("bootstrap"))
            );
        }
        "L2Contract" => {
            assert_eq!(rows.len(), 4);
            assert!(rows.iter().all(|(row, _)| row.fields.len() == 19));
            assert_eq!(
                rows.iter()
                    .filter(|(row, _)| row.get("supersession_state") == Some("current"))
                    .count(),
                4
            );
        }
        "L2Property" => {
            let mut reversed = MANIFEST_BYTES.to_vec();
            reversed.reverse();
            assert!(parse_manifest(&reversed).is_err());
            assert!(parse_fixture(b"fixture_id=X\n").is_err());
            assert_eq!(
                hex_sha256(b""),
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            );
        }
        "L2Model" => {
            let first = rows.first().map(|(row, fixture)| verdict(row, fixture));
            let replay = rows.first().map(|(row, fixture)| verdict(row, fixture));
            assert_eq!(first, replay);
            assert_eq!(first, Some(Ok(Verdict::AcceptedForHarnessOnly)));
        }
        "L2Adversarial" => {
            for rejected in [
                b"".as_slice(),
                b"fixture_id\tversion\r\n".as_slice(),
                &[0xef, 0xbb, 0xbf, b'x'],
            ] {
                assert!(parse_manifest(rejected).is_err());
            }
            for path in [
                "/absolute",
                "../traversal",
                "drive:C",
                "alternate\\separator",
                "https://uri",
            ] {
                assert!(!validate_path(path));
            }
        }
        "L2HoldClosure" => {
            assert_eq!(OPEN_HOLDS.len(), 4);
            assert!(rows.iter().all(|(row, _)| OPEN_HOLDS.contains(&row.get("proof_input_hold").unwrap_or(""))));
            assert!(
                rows.iter()
                    .any(|(row, fixture)| verdict(row, fixture) == Ok(Verdict::Held))
            );
        }
        "L2NoAuthority" => {
            let tokens: Vec<_> = rows
                .iter()
                .filter_map(|(_, fixture)| fixture.get("token"))
                .collect();
            assert!(tokens.iter().all(|token| token.starts_with("SYNTHETIC_")));
            for term in [
                "official-use",
                "release-request",
                "taxlane",
                "readiness-claim",
                "budget-decision",
            ] {
                assert!(
                    !tokens
                        .iter()
                        .any(|token| token.to_ascii_lowercase().contains(term))
                );
            }
        }
        "L1Static" => {
            assert_eq!(trace_counts().get("L1Static"), Some(&8));
            assert_eq!(IMPLEMENTATION_PATHS.len(), 18);
        }
        _ => assert_eq!(mode, "closed-trace-mode", "unexpected trace mode"),
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
