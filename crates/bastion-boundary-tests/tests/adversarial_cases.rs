mod support;

use support::trace_tests;

trace_tests!(
    "L2Adversarial";
    trace_bastion_req_tst_006 => "BASTION-REQ-TST-006",
    trace_bastion_req_rel_002 => "BASTION-REQ-REL-002",
    trace_spec_tst_006 => "SPEC-TST-006",
    trace_spec_rel_002 => "SPEC-REL-002",
    trace_spec_nf_001 => "SPEC-NF-001",
    cr_003_typed_failure_rejection => "CR-003",
    cr_004_exhaustion_failure => "CR-004",
    cr_005_termination_violation => "CR-005",
    cr_006_hidden_failure_scan => "CR-006",
    cr_008_default_fallback_rejection => "CR-008",
    cr_015_prohibited_content => "CR-015",
    cr_016_composition_minimization => "CR-016",
    cr_017_floor_noncompensation => "CR-017",
    cr_021_burden_shift_rejection => "CR-021",
    cr_029_cross_role_adversarial => "CR-029",
    cr_031_parser_fuzz_authority_absent => "CR-031",
    cr_037_resource_bound_failure => "CR-037",
    trace_vcl_06 => "VCL-06",
    trace_role_assurance_classification_operational_security => ".roles/assurance/classification-operational-security.md",
);
