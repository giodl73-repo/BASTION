mod support;

use support::trace_tests;

trace_tests!(
    "L2Model";
    trace_bastion_req_tst_003 => "BASTION-REQ-TST-003",
    trace_spec_tst_003 => "SPEC-TST-003",
    trace_spec_nf_006 => "SPEC-NF-006",
    trace_spec_nf_009 => "SPEC-NF-009",
    cr_006_invalid_state => "CR-006",
    cr_009_typed_state_exhaustiveness => "CR-009",
    cr_011_replay_identity => "CR-011",
    cr_013_immutable_successor_acyclic => "CR-013",
    cr_019_state_null_na_stale => "CR-019",
    cr_020_checked_accounting => "CR-020",
    cr_022_eco_delivery_adaptive_shape => "CR-022",
    cr_028_transition_model_evidence => "CR-028",
    cr_032_golden_successor_history => "CR-032",
    trace_vcl_03 => "VCL-03",
);
