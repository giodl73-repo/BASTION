mod support;

use support::trace_tests;

trace_tests!(
    "L2SourceSpine";
    trace_bastion_req_tst_001 => "BASTION-REQ-TST-001",
    trace_spec_tst_001 => "SPEC-TST-001",
    trace_spec_nf_010 => "SPEC-NF-010",
    cr_002_logical_responsibility => "CR-002",
    cr_011_digest_reproduction_order => "CR-011",
    cr_023_review_independence => "CR-023",
    cr_026_invariant_coverage => "CR-026",
    cr_034_generated_provenance_absence => "CR-034",
    cr_035_quality_output_binding => "CR-035",
    cr_039_evidence_digest_truth => "CR-039",
    cr_040_mechanical_trace_contradiction => "CR-040",
    trace_vcl_01 => "VCL-01",
    trace_vcl_10 => "VCL-10",
    trace_val_scope => "VAL-SCOPE",
    trace_act_rdy => "ACT-RDY",
    trace_act_acq => "ACT-ACQ",
    trace_act_log => "ACT-LOG",
    trace_act_ally => "ACT-ALLY",
    trace_act_fin => "ACT-FIN",
    trace_act_ppl => "ACT-PPL",
    trace_act_tst => "ACT-TST",
    trace_role_parliament_operational_readiness => ".roles/parliament/operational-readiness.md",
    trace_role_parliament_acquisition_industrial_base => ".roles/parliament/acquisition-industrial-base.md",
    trace_role_parliament_logistics_sustainment => ".roles/parliament/logistics-sustainment.md",
    trace_role_parliament_defense_comptroller => ".roles/parliament/defense-comptroller.md",
    trace_role_parliament_service_member_family => ".roles/parliament/service-member-family.md",
    trace_role_parliament_independent_test_oversight => ".roles/parliament/independent-test-oversight.md",
    trace_role_parliament_alliance_interoperability => ".roles/parliament/alliance-interoperability.md",
    trace_role_editorial_citation_auditor => ".roles/editorial/citation-auditor.md",
);
