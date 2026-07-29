mod support;

use support::trace_tests;

trace_tests!(
    "L2HoldClosure";
    trace_bastion_req_tst_005 => "BASTION-REQ-TST-005",
    trace_spec_tst_005 => "SPEC-TST-005",
    cr_008_missing_default_hold => "CR-008",
    cr_019_missing_null_hold => "CR-019",
    cr_023_finding_dissent_retention => "CR-023",
    cr_025_hold_transpose_propagation => "CR-025",
    cr_038_waiver_ledger_nonwaiver => "CR-038",
    cr_039_evidence_state_history => "CR-039",
    trace_vcl_05 => "VCL-05",
    trace_val_assurance => "VAL-ASSURANCE",
    trace_role_review_steward => "Role review steward",
    trace_spec_unk_sec_001 => "SPEC-UNK-SEC-001",
    trace_tbd_sec_001 => "TBD-SEC-001",
    trace_spec_unk_src_001 => "SPEC-UNK-SRC-001",
    trace_tbd_src_001 => "TBD-SRC-001",
    trace_spec_unk_tst_001 => "SPEC-UNK-TST-001",
    trace_tbd_tst_001 => "TBD-TST-001",
    trace_spec_unk_rel_001 => "SPEC-UNK-REL-001",
    trace_tbd_rel_001 => "TBD-REL-001",
);
