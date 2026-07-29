mod support;

use support::trace_tests;

trace_tests!(
    "L2Contract";
    trace_bastion_req_tst_004 => "BASTION-REQ-TST-004",
    trace_spec_tst_004 => "SPEC-TST-004",
    trace_spec_nf_008 => "SPEC-NF-008",
    trace_des_test_001 => "DES-TEST-001",
    trace_contract_test_001 => "CONTRACT-TEST-001",
    cr_002_logical_contract => "CR-002",
    cr_003_typed_branch_totality => "CR-003",
    cr_009_typed_family_exhaustiveness => "CR-009",
    cr_015_content_boundary_provenance => "CR-015",
    cr_030_per_contract_fixture_matrix => "CR-030",
    trace_vcl_02 => "VCL-02",
    trace_act_src => "ACT-SRC",
);
