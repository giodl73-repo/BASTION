mod support;

use support::trace_tests;

trace_tests!(
    "L2NoAuthority";
    trace_bastion_req_rel_001 => "BASTION-REQ-REL-001",
    trace_bastion_req_rel_003 => "BASTION-REQ-REL-003",
    trace_spec_rel_001 => "SPEC-REL-001",
    trace_spec_rel_003 => "SPEC-REL-003",
    trace_spec_nf_002 => "SPEC-NF-002",
    trace_spec_nf_003 => "SPEC-NF-003",
    trace_des_rel_001 => "DES-REL-001",
    trace_contract_rel_001 => "CONTRACT-REL-001",
    cr_010_release_exception_no_output => "CR-010",
    cr_017_authority_noninflation => "CR-017",
    cr_021_false_savings_no_authority => "CR-021",
    cr_024_terminal_no_output_backflow => "CR-024",
    cr_034_generated_no_emission => "CR-034",
    trace_vcl_07 => "VCL-07",
    trace_vcl_08 => "VCL-08",
    trace_act_civ => "ACT-CIV",
    trace_act_law => "ACT-LAW",
    trace_act_ext => "ACT-EXT",
    trace_role_parliament_civilian_strategy_force_planner => ".roles/parliament/civilian-strategy-force-planner.md",
    trace_role_editorial_scope_keeper => ".roles/editorial/scope-keeper.md",
    trace_role_stakeholders_service_member_family => ".roles/stakeholders/service-member-family.md",
    trace_role_stakeholders_mission_user => ".roles/stakeholders/mission-user.md",
    trace_role_stakeholders_depot_logistics_workforce => ".roles/stakeholders/depot-logistics-workforce.md",
    trace_role_stakeholders_prime_small_supplier => ".roles/stakeholders/prime-small-supplier.md",
    trace_role_stakeholders_installation_community => ".roles/stakeholders/installation-community.md",
    trace_role_stakeholders_ally_partner => ".roles/stakeholders/ally-partner.md",
    trace_role_stakeholders_taxpayer_oversight => ".roles/stakeholders/taxpayer-oversight.md",
    trace_role_assurance_civilian_control_law_safety_readiness => ".roles/assurance/civilian-control-law-safety-readiness.md",
);
