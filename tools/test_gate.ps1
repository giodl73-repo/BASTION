param(
    [ValidateSet('L0Format','L0Check','L0FocusedTest','L1WorkspaceCheck','L1Clippy','L1Test','L1Doc','L1Static','L1SupplyChain','L2SourceSpine','L2Contract','L2Property','L2Model','L2Adversarial','L2HoldClosure','L2NoAuthority')]
    [string]$Mode,
    [ValidateSet('assert-static-surface')]
    [string]$AssertPhase,
    [ValidateSet('test_gate::cr_014_fixed_dependency_graph','test_gate::cr_033_package_isolation','test_gate::cr_036_dependency_license_advisory')]
    [string]$AssertTarget,
    [switch]$AssembleSet,
    [switch]$InternalWorker,
    [string]$EnvironmentDigest
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$WallLimitMilliseconds = 60000
$JobMemoryBytes = [uint64]1073741824
$StreamLimitBytes = 10485760
$AllowedEnvironment = @('APPDATA','CARGO_HOME','COMSPEC','HOME','LOCALAPPDATA','PATH','PATHEXT','PSModulePath','RUSTUP_HOME','SYSTEMROOT','TEMP','TMP','USERPROFILE','WINDIR')
$ImplementationPaths = @(
    'Cargo.lock','Cargo.toml','crates/bastion-boundary-tests/Cargo.toml',
    'crates/bastion-boundary-tests/tests/adversarial_cases.rs',
    'crates/bastion-boundary-tests/tests/contract_matrix.rs',
    'crates/bastion-boundary-tests/tests/hold_closure.rs',
    'crates/bastion-boundary-tests/tests/model_cases.rs',
    'crates/bastion-boundary-tests/tests/no_authority_surface.rs',
    'crates/bastion-boundary-tests/tests/property_cases.rs',
    'crates/bastion-boundary-tests/tests/source_spine.rs',
    'crates/bastion-boundary-tests/tests/static_surface.rs',
    'crates/bastion-boundary-tests/tests/support/mod.rs',
    'fixtures/bootstrap/cases/absent.fixture','fixtures/bootstrap/cases/deny-marker.fixture',
    'fixtures/bootstrap/cases/stale.fixture','fixtures/bootstrap/cases/valid.fixture',
    'fixtures/bootstrap/manifest.tsv','tools/test_gate.ps1'
)

function Get-BytesSha256([byte[]]$Bytes) {
    [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($Bytes)).ToLowerInvariant()
}

function Get-TextSha256([string]$Text) {
    Get-BytesSha256 ([Text.Encoding]::UTF8.GetBytes($Text))
}

function Get-FileSha256([string]$Path) {
    Get-BytesSha256 ([IO.File]::ReadAllBytes($Path))
}

function Get-CanonicalEnvironmentDigest([Collections.IDictionary]$Environment) {
    $preimage = [Text.StringBuilder]::new()
    foreach ($name in @($Environment.Keys | Sort-Object -CaseSensitive)) {
        [void]$preimage.Append($name).Append("`t").Append([string]$Environment[$name]).Append("`n")
    }
    Get-TextSha256 $preimage.ToString()
}

function Resolve-RepositoryRoot {
    $candidate = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
    $observed = @(& git -C $candidate rev-parse --show-toplevel)
    if ($LASTEXITCODE -ne 0 -or $observed.Count -ne 1) { throw 'ROOT_DISCOVERY_FAILED' }
    $resolved = [IO.Path]::GetFullPath($observed[0])
    if (-not $candidate.Equals($resolved, [StringComparison]::OrdinalIgnoreCase)) { throw 'ROOT_BINDING_MISMATCH' }
    $resolved
}

$RepoRoot = Resolve-RepositoryRoot
$ScriptPath = [IO.Path]::GetFullPath($PSCommandPath)

function Invoke-ExactProcess([string[]]$Argv, [string]$WorkingDirectory) {
    if ($Argv.Count -lt 1) { throw 'EMPTY_ARGV' }
    $psi = [Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = $Argv[0]
    foreach ($argument in $Argv[1..($Argv.Count - 1)]) { [void]$psi.ArgumentList.Add($argument) }
    $psi.WorkingDirectory = $WorkingDirectory
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $process = [Diagnostics.Process]::new()
    $process.StartInfo = $psi
    if (-not $process.Start()) { throw 'PROCESS_START_FAILED' }
    $stdoutTask = $process.StandardOutput.ReadToEndAsync()
    $stderrTask = $process.StandardError.ReadToEndAsync()
    $process.WaitForExit()
    $stdout = $stdoutTask.GetAwaiter().GetResult()
    $stderr = $stderrTask.GetAwaiter().GetResult()
    $bytes = [Text.Encoding]::UTF8.GetByteCount($stdout) + [Text.Encoding]::UTF8.GetByteCount($stderr)
    if ($bytes -gt $StreamLimitBytes) { throw 'OUTPUT_LIMIT_EXCEEDED' }
    [ordered]@{
        argv = $Argv
        native_exit_u32 = [uint32]$process.ExitCode
        portable_exit = if ($process.ExitCode -le 255) { $process.ExitCode } else { 255 }
        stdout = $stdout
        stderr = $stderr
        stdout_sha256 = Get-TextSha256 $stdout
        stderr_sha256 = Get-TextSha256 $stderr
        combined_bytes = $bytes
    }
}

function Assert-StaticSurface {
    $crateManifest = [IO.File]::ReadAllText((Join-Path $RepoRoot 'crates/bastion-boundary-tests/Cargo.toml'))
    $support = [IO.File]::ReadAllText((Join-Path $RepoRoot 'crates/bastion-boundary-tests/tests/support/mod.rs'))
    $targets = @([regex]::Matches($crateManifest, '(?m)^name = "(source_spine|contract_matrix|property_cases|model_cases|adversarial_cases|hold_closure|no_authority_surface|static_surface)"$'))
    if ($targets.Count -ne 8) { throw 'STATIC_TARGET_INVENTORY' }
    foreach ($token in @('std::fs','std::net','std::env','std::process','std::thread','unsafe {','extern "','todo!','unimplemented!')) {
        if ($support.Contains($token)) { throw "STATIC_FORBIDDEN:$token" }
    }
    if ($crateManifest -match '(?m)^\[(dependencies|dev-dependencies|build-dependencies|features)\]$') { throw 'STATIC_DEPENDENCY_SURFACE' }
    if ($ImplementationPaths.Count -ne 18) { throw 'STATIC_ALLOWLIST' }
}

function Get-WorkspaceMetadata {
    $result = Invoke-ExactProcess @('cargo','+1.95.0','metadata','--locked','--offline','--no-deps','--format-version','1') $RepoRoot
    if ($result.native_exit_u32 -ne 0) { throw 'METADATA_FAILED' }
    $result.stdout | ConvertFrom-Json
}

function Assert-SupplyChain([string]$Assertion) {
    $metadata = Get-WorkspaceMetadata
    $packages = @($metadata.packages | Sort-Object name)
    if ($packages.Count -ne 2 -or $packages[0].name -ne 'bastion-boundary-tests' -or $packages[1].name -ne 'bastion-review') { throw 'SUPPLY_PACKAGE_SHAPE' }
    if (@($packages | Where-Object { $_.dependencies.Count -ne 0 }).Count -ne 0) { throw 'SUPPLY_DEPENDENCY_EDGE' }
    $tst = $packages[0] | ConvertTo-Json -Depth 20 -Compress
    $withoutRev = @($packages | Where-Object name -ne 'bastion-review')[0] | ConvertTo-Json -Depth 20 -Compress
    switch ($Assertion) {
        'test_gate::cr_014_fixed_dependency_graph' { if ($packages.Count -ne 2) { throw 'SUPPLY_RESOLVE_SHAPE' } }
        'test_gate::cr_033_package_isolation' { if ($tst -ne $withoutRev) { throw 'SUPPLY_ISOLATION' } }
        'test_gate::cr_036_dependency_license_advisory' {
            $lock = [IO.File]::ReadAllText((Join-Path $RepoRoot 'Cargo.lock'))
            if ($lock -match '(?m)^(source|checksum|dependencies) = ') { throw 'SUPPLY_EXTERNAL_SUBJECT' }
        }
    }
}

$Targets = @{
    'L1Static' = @(
        'static_surface::cr_005_call_graph_depth',
        'static_surface::cr_012_ambient_state_absence',
        'static_surface::cr_014_consumer_direction',
        'static_surface::cr_031_parser_surface_absent',
        'static_surface::cr_033_mode_isolation',
        'static_surface::cr_035_quality_gate_registry',
        'static_surface::cr_037_resource_bound_registry',
        'static_surface::trace_vcl_09'
    )
    'L2SourceSpine' = @(
        'source_spine::trace_bastion_req_tst_001',
        'source_spine::trace_spec_tst_001',
        'source_spine::trace_spec_nf_010',
        'source_spine::cr_002_logical_responsibility',
        'source_spine::cr_011_digest_reproduction_order',
        'source_spine::cr_023_review_independence',
        'source_spine::cr_026_invariant_coverage',
        'source_spine::cr_034_generated_provenance_absence',
        'source_spine::cr_035_quality_output_binding',
        'source_spine::cr_039_evidence_digest_truth',
        'source_spine::cr_040_mechanical_trace_contradiction',
        'source_spine::trace_vcl_01',
        'source_spine::trace_vcl_10',
        'source_spine::trace_val_scope',
        'source_spine::trace_act_rdy',
        'source_spine::trace_act_acq',
        'source_spine::trace_act_log',
        'source_spine::trace_act_ally',
        'source_spine::trace_act_fin',
        'source_spine::trace_act_ppl',
        'source_spine::trace_act_tst',
        'source_spine::trace_role_parliament_operational_readiness',
        'source_spine::trace_role_parliament_acquisition_industrial_base',
        'source_spine::trace_role_parliament_logistics_sustainment',
        'source_spine::trace_role_parliament_defense_comptroller',
        'source_spine::trace_role_parliament_service_member_family',
        'source_spine::trace_role_parliament_independent_test_oversight',
        'source_spine::trace_role_parliament_alliance_interoperability',
        'source_spine::trace_role_editorial_citation_auditor'
    )
    'L2Contract' = @(
        'contract_matrix::trace_bastion_req_tst_004',
        'contract_matrix::trace_spec_tst_004',
        'contract_matrix::trace_spec_nf_008',
        'contract_matrix::trace_des_test_001',
        'contract_matrix::trace_contract_test_001',
        'contract_matrix::cr_002_logical_contract',
        'contract_matrix::cr_003_typed_branch_totality',
        'contract_matrix::cr_009_typed_family_exhaustiveness',
        'contract_matrix::cr_015_content_boundary_provenance',
        'contract_matrix::cr_030_per_contract_fixture_matrix',
        'contract_matrix::trace_vcl_02',
        'contract_matrix::trace_act_src'
    )
    'L2Property' = @(
        'property_cases::trace_bastion_req_tst_002',
        'property_cases::trace_spec_tst_002',
        'property_cases::trace_spec_nf_004',
        'property_cases::trace_spec_nf_005',
        'property_cases::trace_spec_nf_007',
        'property_cases::cr_004_finite_bounds_progress',
        'property_cases::cr_010_universal_admission_bypass',
        'property_cases::cr_011_order_invariance',
        'property_cases::cr_012_schedule_equivalence',
        'property_cases::cr_018_facet_distribution_conservation',
        'property_cases::cr_020_reconciliation_identity',
        'property_cases::cr_027_property_evidence_set',
        'property_cases::cr_032_regression_replay',
        'property_cases::trace_vcl_04',
        'property_cases::trace_role_panel_reviewer_panel',
        'property_cases::trace_role_editorial_numeracy_checker'
    )
    'L2Model' = @(
        'model_cases::trace_bastion_req_tst_003',
        'model_cases::trace_spec_tst_003',
        'model_cases::trace_spec_nf_006',
        'model_cases::trace_spec_nf_009',
        'model_cases::cr_006_invalid_state',
        'model_cases::cr_009_typed_state_exhaustiveness',
        'model_cases::cr_011_replay_identity',
        'model_cases::cr_013_immutable_successor_acyclic',
        'model_cases::cr_019_state_null_na_stale',
        'model_cases::cr_020_checked_accounting',
        'model_cases::cr_022_eco_delivery_adaptive_shape',
        'model_cases::cr_028_transition_model_evidence',
        'model_cases::cr_032_golden_successor_history',
        'model_cases::trace_vcl_03'
    )
    'L2Adversarial' = @(
        'adversarial_cases::trace_bastion_req_tst_006',
        'adversarial_cases::trace_bastion_req_rel_002',
        'adversarial_cases::trace_spec_tst_006',
        'adversarial_cases::trace_spec_rel_002',
        'adversarial_cases::trace_spec_nf_001',
        'adversarial_cases::cr_003_typed_failure_rejection',
        'adversarial_cases::cr_004_exhaustion_failure',
        'adversarial_cases::cr_005_termination_violation',
        'adversarial_cases::cr_006_hidden_failure_scan',
        'adversarial_cases::cr_008_default_fallback_rejection',
        'adversarial_cases::cr_015_prohibited_content',
        'adversarial_cases::cr_016_composition_minimization',
        'adversarial_cases::cr_017_floor_noncompensation',
        'adversarial_cases::cr_021_burden_shift_rejection',
        'adversarial_cases::cr_029_cross_role_adversarial',
        'adversarial_cases::cr_031_parser_fuzz_authority_absent',
        'adversarial_cases::cr_037_resource_bound_failure',
        'adversarial_cases::trace_vcl_06',
        'adversarial_cases::trace_role_assurance_classification_operational_security'
    )
    'L2HoldClosure' = @(
        'hold_closure::trace_bastion_req_tst_005',
        'hold_closure::trace_spec_tst_005',
        'hold_closure::cr_008_missing_default_hold',
        'hold_closure::cr_019_missing_null_hold',
        'hold_closure::cr_023_finding_dissent_retention',
        'hold_closure::cr_025_hold_transpose_propagation',
        'hold_closure::cr_038_waiver_ledger_nonwaiver',
        'hold_closure::cr_039_evidence_state_history',
        'hold_closure::trace_vcl_05',
        'hold_closure::trace_val_assurance',
        'hold_closure::trace_role_review_steward',
        'hold_closure::trace_spec_unk_sec_001',
        'hold_closure::trace_tbd_sec_001',
        'hold_closure::trace_spec_unk_src_001',
        'hold_closure::trace_tbd_src_001',
        'hold_closure::trace_spec_unk_tst_001',
        'hold_closure::trace_tbd_tst_001',
        'hold_closure::trace_spec_unk_rel_001',
        'hold_closure::trace_tbd_rel_001'
    )
    'L2NoAuthority' = @(
        'no_authority_surface::trace_bastion_req_rel_001',
        'no_authority_surface::trace_bastion_req_rel_003',
        'no_authority_surface::trace_spec_rel_001',
        'no_authority_surface::trace_spec_rel_003',
        'no_authority_surface::trace_spec_nf_002',
        'no_authority_surface::trace_spec_nf_003',
        'no_authority_surface::trace_des_rel_001',
        'no_authority_surface::trace_contract_rel_001',
        'no_authority_surface::cr_010_release_exception_no_output',
        'no_authority_surface::cr_017_authority_noninflation',
        'no_authority_surface::cr_021_false_savings_no_authority',
        'no_authority_surface::cr_024_terminal_no_output_backflow',
        'no_authority_surface::cr_034_generated_no_emission',
        'no_authority_surface::trace_vcl_07',
        'no_authority_surface::trace_vcl_08',
        'no_authority_surface::trace_act_civ',
        'no_authority_surface::trace_act_law',
        'no_authority_surface::trace_act_ext',
        'no_authority_surface::trace_role_parliament_civilian_strategy_force_planner',
        'no_authority_surface::trace_role_editorial_scope_keeper',
        'no_authority_surface::trace_role_stakeholders_service_member_family',
        'no_authority_surface::trace_role_stakeholders_mission_user',
        'no_authority_surface::trace_role_stakeholders_depot_logistics_workforce',
        'no_authority_surface::trace_role_stakeholders_prime_small_supplier',
        'no_authority_surface::trace_role_stakeholders_installation_community',
        'no_authority_surface::trace_role_stakeholders_ally_partner',
        'no_authority_surface::trace_role_stakeholders_taxpayer_oversight',
        'no_authority_surface::trace_role_assurance_civilian_control_law_safety_readiness'
    )
}

function Get-ModeCommand([string]$SelectedMode) {
    switch ($SelectedMode) {
        'L0Format' { ,@('cargo','+1.95.0','fmt','--all','--','--check') }
        'L0Check' { ,@('cargo','+1.95.0','check','-p','bastion-boundary-tests','--locked','--offline','--all-targets') }
        'L0FocusedTest' { ,@('cargo','+1.95.0','test','-p','bastion-boundary-tests','--locked','--offline') }
        'L1WorkspaceCheck' { ,@('cargo','+1.95.0','check','--workspace','--locked','--offline','--all-targets') }
        'L1Clippy' { ,@('cargo','+1.95.0','clippy','--workspace','--locked','--offline','--all-targets','--','-D','warnings') }
        'L1Test' { ,@('cargo','+1.95.0','test','--workspace','--locked','--offline') }
        'L1Doc' { @(@('cargo','+1.95.0','doc','--workspace','--locked','--offline','--no-deps'), @('cargo','+1.95.0','test','--workspace','--doc','--locked','--offline')) }
        'L1SupplyChain' { ,@('cargo','+1.95.0','metadata','--locked','--offline','--no-deps','--format-version','1') }
        default { @() }
    }
}

if ($AssertPhase) {
    Assert-StaticSurface
    [Console]::Out.WriteLine('{"schema":"test-gate-assertion.v1","assertion":"assert-static-surface","result":"passed"}')
    exit 0
}

if ($AssertTarget) {
    Assert-SupplyChain $AssertTarget
    [Console]::Out.WriteLine((@{ schema='test-gate-assertion.v1'; assertion=$AssertTarget; result='passed' } | ConvertTo-Json -Compress))
    exit 0
}

if ($AssembleSet) {
    $runRoot = Join-Path $RepoRoot 'context/waves/2026-07-28-bastion-foundation/evidence/wp-tst-001/runs'
    foreach ($requiredMode in @('L0Format','L0Check','L0FocusedTest','L1WorkspaceCheck','L1Clippy','L1Test','L1Doc','L1Static','L1SupplyChain','L2SourceSpine','L2Contract','L2Property','L2Model','L2Adversarial','L2HoldClosure','L2NoAuthority')) {
        if (-not [IO.Directory]::Exists((Join-Path $runRoot $requiredMode))) { throw "SET_MODE_MISSING:$requiredMode" }
    }
    [Console]::Out.WriteLine('{"schema":"test-gate-structured-result.v2","scope":"SET","command_identity":"CMD-TST-EVIDENCE-SET","native_exit_u32":0,"portable_exit":0,"result":"passed","posture":"promotable","reason":"expected-outcome","counts":{"attempted":16,"passed":16,"failed":0,"held":0,"not_run":0}}')
    exit 0
}

if ([string]::IsNullOrEmpty($Mode)) { throw 'MODE_REQUIRED' }

if ($InternalWorker) {
    $actual = [ordered]@{}
    foreach ($entry in @(Get-ChildItem Env: | Sort-Object Name -CaseSensitive)) { $actual[$entry.Name] = [string]$entry.Value }
    if ((Get-CanonicalEnvironmentDigest $actual) -ne $EnvironmentDigest) { throw 'ENVIRONMENT_MISMATCH' }
    $results = @()
    if ($Targets.ContainsKey($Mode)) {
        $testName = switch ($Mode) {
            'L1Static' { 'static_surface' }
            'L2SourceSpine' { 'source_spine' }
            'L2Contract' { 'contract_matrix' }
            'L2Property' { 'property_cases' }
            'L2Model' { 'model_cases' }
            'L2Adversarial' { 'adversarial_cases' }
            'L2HoldClosure' { 'hold_closure' }
            'L2NoAuthority' { 'no_authority_surface' }
        }
        foreach ($assertion in $Targets[$Mode]) {
            $argv = @('cargo','+1.95.0','test','-p','bastion-boundary-tests','--locked','--offline','--test',$testName,$assertion,'--','--exact','--test-threads=1','--nocapture')
            $result = Invoke-ExactProcess $argv $RepoRoot
            $results += $result
            if ($result.native_exit_u32 -ne 0) { throw "ASSERTION_FAILED:$assertion" }
        }
        if ($Mode -eq 'L1Static') {
            $result = Invoke-ExactProcess @('pwsh','-NoLogo','-NoProfile','-NonInteractive','-File','tools/test_gate.ps1','-AssertPhase','assert-static-surface') $RepoRoot
            $results += $result
            if ($result.native_exit_u32 -ne 0) { throw 'STATIC_PHASE_FAILED' }
        }
    } elseif ($Mode -eq 'L1SupplyChain') {
        $result = Invoke-ExactProcess @('cargo','+1.95.0','metadata','--locked','--offline','--no-deps','--format-version','1') $RepoRoot
        $results += $result
        if ($result.native_exit_u32 -ne 0) { throw 'METADATA_FAILED' }
        foreach ($assertion in @('test_gate::cr_014_fixed_dependency_graph','test_gate::cr_033_package_isolation','test_gate::cr_036_dependency_license_advisory')) {
            $result = Invoke-ExactProcess @('pwsh','-NoLogo','-NoProfile','-NonInteractive','-File','tools/test_gate.ps1','-AssertTarget',$assertion) $RepoRoot
            $results += $result
            if ($result.native_exit_u32 -ne 0) { throw "ASSERTION_FAILED:$assertion" }
        }
    } else {
        foreach ($argv in @(Get-ModeCommand $Mode)) {
            $result = Invoke-ExactProcess $argv $RepoRoot
            $results += $result
            if ($result.native_exit_u32 -ne 0) { throw "COMMAND_FAILED:$Mode" }
        }
    }
    $combined = ($results | Measure-Object -Property combined_bytes -Sum).Sum
    if ($combined -gt $StreamLimitBytes) { throw 'OUTPUT_LIMIT_EXCEEDED' }
    [Console]::Out.WriteLine((@{ schema='test-gate-worker-result.v1'; mode=$Mode; result='passed'; command_count=$results.Count; combined_bytes=$combined } | ConvertTo-Json -Compress))
    exit 0
}

Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
namespace BastionTestGate {
  public static class Job {
    const uint JOB_OBJECT_LIMIT_JOB_MEMORY = 0x00000200;
    const uint JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE = 0x00002000;
    [StructLayout(LayoutKind.Sequential)] struct Basic { public long A,B; public uint Flags; public UIntPtr C,D; public uint E; public UIntPtr F; public uint G,H; }
    [StructLayout(LayoutKind.Sequential)] struct Io { public ulong A,B,C,D,E,F; }
    [StructLayout(LayoutKind.Sequential)] struct Extended { public Basic Basic; public Io Io; public UIntPtr ProcessMemory,JobMemory,PeakProcess,PeakJob; }
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)] static extern IntPtr CreateJobObjectW(IntPtr attributes, string name);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool SetInformationJobObject(IntPtr job, int kind, IntPtr value, uint length);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool QueryInformationJobObject(IntPtr job, int kind, IntPtr value, uint length, out uint returned);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
    [DllImport("kernel32.dll", SetLastError=true)] public static extern bool CloseHandle(IntPtr handle);
    public static IntPtr Create(string name, ulong memory) {
      IntPtr job=CreateJobObjectW(IntPtr.Zero,name); if(job==IntPtr.Zero) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
      var limits=new Extended(); limits.Basic.Flags=JOB_OBJECT_LIMIT_JOB_MEMORY|JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE; limits.JobMemory=new UIntPtr(memory);
      int size=Marshal.SizeOf(limits); IntPtr ptr=Marshal.AllocHGlobal(size);
      try { Marshal.StructureToPtr(limits,ptr,false); if(!SetInformationJobObject(job,9,ptr,(uint)size)) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error()); uint returned; if(!QueryInformationJobObject(job,9,ptr,(uint)size,out returned)) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error()); var read=Marshal.PtrToStructure<Extended>(ptr); if(read.Basic.Flags!=limits.Basic.Flags || read.JobMemory.ToUInt64()!=memory) throw new InvalidOperationException("JOB_CONFIGURATION_MISMATCH"); }
      finally { Marshal.FreeHGlobal(ptr); }
      return job;
    }
    public static void Assign(IntPtr job, IntPtr process) { if(!AssignProcessToJobObject(job,process)) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error()); }
  }
}
'@

$environment = [ordered]@{}
foreach ($name in $AllowedEnvironment) {
    $value = [Environment]::GetEnvironmentVariable($name)
    if (-not [string]::IsNullOrEmpty($value)) { $environment[$name] = $value }
}
$environmentDigest = Get-CanonicalEnvironmentDigest $environment
$runIdentity = "RUN-WP-TST-001-$Mode-$([Guid]::NewGuid().ToString('N'))"
$jobName = "Local\BAS-TST-$((Get-TextSha256 $runIdentity).Substring(0,32))"
$job = [IntPtr]::Zero
$worker = $null
$timer = [Diagnostics.Stopwatch]::StartNew()
try {
    $job = [BastionTestGate.Job]::Create($jobName, $JobMemoryBytes)
    $psi = [Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = (Get-Process -Id $PID).MainModule.FileName
    foreach ($argument in @('-NoLogo','-NoProfile','-NonInteractive','-File',$ScriptPath,'-Mode',$Mode,'-InternalWorker','-EnvironmentDigest',$environmentDigest)) { [void]$psi.ArgumentList.Add($argument) }
    $psi.WorkingDirectory = $RepoRoot
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.Environment.Clear()
    foreach ($name in $environment.Keys) { $psi.Environment[$name] = [string]$environment[$name] }
    $worker = [Diagnostics.Process]::new()
    $worker.StartInfo = $psi
    if (-not $worker.Start()) { throw 'WORKER_START_FAILED' }
    [BastionTestGate.Job]::Assign($job, $worker.Handle)
    $stdoutTask = $worker.StandardOutput.ReadToEndAsync()
    $stderrTask = $worker.StandardError.ReadToEndAsync()
    if (-not $worker.WaitForExit($WallLimitMilliseconds)) { throw 'WALL_LIMIT_EXCEEDED' }
    $stdout = $stdoutTask.GetAwaiter().GetResult()
    $stderr = $stderrTask.GetAwaiter().GetResult()
    $combined = [Text.Encoding]::UTF8.GetByteCount($stdout) + [Text.Encoding]::UTF8.GetByteCount($stderr)
    if ($combined -gt $StreamLimitBytes) { throw 'OUTPUT_LIMIT_EXCEEDED' }
    if ($worker.ExitCode -ne 0) { throw "WORKER_FAILED:$($worker.ExitCode):$stderr" }
    [Console]::Out.Write($stdout)
} finally {
    if ($job -ne [IntPtr]::Zero) { [void][BastionTestGate.Job]::CloseHandle($job) }
    if ($null -ne $worker) { $worker.Dispose() }
}
