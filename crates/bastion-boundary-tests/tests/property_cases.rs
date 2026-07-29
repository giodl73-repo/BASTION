mod support;

use support::trace_tests;

trace_tests!(
    "L2Property";
    trace_bastion_req_tst_002 => "BASTION-REQ-TST-002",
    trace_spec_tst_002 => "SPEC-TST-002",
    trace_spec_nf_004 => "SPEC-NF-004",
    trace_spec_nf_005 => "SPEC-NF-005",
    trace_spec_nf_007 => "SPEC-NF-007",
    cr_004_finite_bounds_progress => "CR-004",
    cr_010_universal_admission_bypass => "CR-010",
    cr_011_order_invariance => "CR-011",
    cr_012_schedule_equivalence => "CR-012",
    cr_018_facet_distribution_conservation => "CR-018",
    cr_020_reconciliation_identity => "CR-020",
    cr_027_property_evidence_set => "CR-027",
    cr_032_regression_replay => "CR-032",
    trace_vcl_04 => "VCL-04",
    trace_role_panel_reviewer_panel => ".roles/panel-reviewer/panel.md",
    trace_role_editorial_numeracy_checker => ".roles/editorial/numeracy-checker.md",
);
