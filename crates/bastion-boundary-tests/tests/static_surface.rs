mod support;

use support::trace_tests;

trace_tests!(
    "L1Static";
    cr_005_call_graph_depth => "CR-005",
    cr_012_ambient_state_absence => "CR-012",
    cr_014_consumer_direction => "CR-014",
    cr_031_parser_surface_absent => "CR-031",
    cr_033_mode_isolation => "CR-033",
    cr_035_quality_gate_registry => "CR-035",
    cr_037_resource_bound_registry => "CR-037",
    trace_vcl_09 => "VCL-09",
);
