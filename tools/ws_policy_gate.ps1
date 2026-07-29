param(
    [Parameter(Mandatory = $true)]
    [ValidateSet('L0Format', 'L0Check', 'L0FocusedTest', 'L1WorkspaceCheck', 'L1Lint', 'L1Test', 'L1Doc', 'L1Static', 'L1SupplyChain', 'L2Model', 'L2Adversarial')]
    [string]$Mode,
    [switch]$InternalWorker,
    [string]$RunId,
    [string]$EnvironmentManifestSha256
)

$commandProcess = Get-Process -Id $PID
$commandStarted = [DateTimeOffset]$commandProcess.StartTime.ToUniversalTime()
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$wallLimitSeconds = 60
$memoryLimitBytes = 1GB
$outputLimitBytes = 10MB
$finalRecordReservationBytes = 1MB
$scriptPath = [IO.Path]::GetFullPath($PSCommandPath)
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))

function Get-TextSha256([string]$Text) {
    $bytes = [Text.Encoding]::UTF8.GetBytes($Text)
    return [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
}
function ConvertTo-CanonicalJson($Value, [int]$Depth = 12) {
    return ($Value | ConvertTo-Json -Depth $Depth -Compress)
}
function Get-EnvironmentManifest {
    $entries = @(Get-ChildItem Env: | Sort-Object Name | ForEach-Object {
        [ordered]@{ name = $_.Name; value_sha256 = Get-TextSha256 ([string]$_.Value) }
    })
    return [ordered]@{ entries = $entries; sha256 = Get-TextSha256 (ConvertTo-CanonicalJson $entries) }
}
function Get-ProcessTreeSnapshot([int]$RootPid) {
    $snapshot = @(Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId)
    $ids = [Collections.Generic.HashSet[uint32]]::new()
    [void]$ids.Add([uint32]$RootPid)
    $changed = $true
    while ($changed) {
        $changed = $false
        foreach ($item in $snapshot) {
            if ($ids.Contains([uint32]$item.ParentProcessId) -and $ids.Add([uint32]$item.ProcessId)) { $changed = $true }
        }
    }
    [long]$resident = 0
    foreach ($id in @($ids | Sort-Object)) {
        $process = Get-Process -Id $id -ErrorAction SilentlyContinue
        if ($null -ne $process) { $resident += [long]$process.WorkingSet64 }
    }
    return [ordered]@{ ids = @($ids | Sort-Object); resident_bytes = $resident }
}

Add-Type -TypeDefinition @'
using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;

namespace WorkspacePolicy {
    public sealed class OutputBudget {
        internal long Total;
        internal readonly long Limit;
        internal readonly Process Process;
        public OutputBudget(Process process, long limit) { Process = process; Limit = limit; }
        public long Bytes { get { return Interlocked.Read(ref Total); } }
    }

    public static class BoundedCapture {
        public static async Task<byte[]> ReadAsync(Stream stream, OutputBudget budget) {
            using (var memory = new MemoryStream()) {
                var buffer = new byte[8192];
                while (true) {
                    int count = await stream.ReadAsync(buffer, 0, buffer.Length).ConfigureAwait(false);
                    if (count == 0) break;
                    long total = Interlocked.Add(ref budget.Total, count);
                    if (total > budget.Limit) {
                        try { budget.Process.Kill(true); } catch { }
                        throw new IOException("CAPTURE_OUTPUT_LIMIT");
                    }
                    memory.Write(buffer, 0, count);
                }
                return memory.ToArray();
            }
        }
    }

    public static class NativeJob {
        const UInt32 JOB_OBJECT_LIMIT_JOB_MEMORY = 0x00000200;
        const UInt32 JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE = 0x00002000;

        [StructLayout(LayoutKind.Sequential)]
        struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
            public Int64 PerProcessUserTimeLimit;
            public Int64 PerJobUserTimeLimit;
            public UInt32 LimitFlags;
            public UIntPtr MinimumWorkingSetSize;
            public UIntPtr MaximumWorkingSetSize;
            public UInt32 ActiveProcessLimit;
            public UIntPtr Affinity;
            public UInt32 PriorityClass;
            public UInt32 SchedulingClass;
        }
        [StructLayout(LayoutKind.Sequential)]
        struct IO_COUNTERS {
            public UInt64 ReadOperationCount, WriteOperationCount, OtherOperationCount;
            public UInt64 ReadTransferCount, WriteTransferCount, OtherTransferCount;
        }
        [StructLayout(LayoutKind.Sequential)]
        struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
            public JOBOBJECT_BASIC_LIMIT_INFORMATION BasicLimitInformation;
            public IO_COUNTERS IoInfo;
            public UIntPtr ProcessMemoryLimit;
            public UIntPtr JobMemoryLimit;
            public UIntPtr PeakProcessMemoryUsed;
            public UIntPtr PeakJobMemoryUsed;
        }

        [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
        static extern IntPtr CreateJobObject(IntPtr attributes, string name);
        [DllImport("kernel32.dll", SetLastError=true)]
        static extern bool SetInformationJobObject(IntPtr job, int infoClass, IntPtr info, UInt32 length);
        [DllImport("kernel32.dll", SetLastError=true)]
        static extern bool QueryInformationJobObject(IntPtr job, int infoClass, IntPtr info, UInt32 length, IntPtr returnedLength);
        [DllImport("kernel32.dll", SetLastError=true)]
        static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
        [DllImport("kernel32.dll", SetLastError=true)]
        static extern bool CloseHandle(IntPtr handle);

        public static IntPtr CreateAndAssignCurrent(long memoryLimit) {
            IntPtr job = CreateJobObject(IntPtr.Zero, null);
            if (job == IntPtr.Zero) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error(), "CreateJobObject");
            var limits = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();
            limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_JOB_MEMORY | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            limits.JobMemoryLimit = new UIntPtr((UInt64)memoryLimit);
            int size = Marshal.SizeOf(limits);
            IntPtr ptr = Marshal.AllocHGlobal(size);
            try {
                Marshal.StructureToPtr(limits, ptr, false);
                if (!SetInformationJobObject(job, 9, ptr, (UInt32)size))
                    throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error(), "SetInformationJobObject");
            } finally { Marshal.FreeHGlobal(ptr); }
            if (!AssignProcessToJobObject(job, Process.GetCurrentProcess().Handle))
                throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error(), "AssignProcessToJobObject");
            return job;
        }

        public static long PeakCommittedBytes(IntPtr job) {
            var limits = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();
            int size = Marshal.SizeOf(limits);
            IntPtr ptr = Marshal.AllocHGlobal(size);
            try {
                Marshal.StructureToPtr(limits, ptr, false);
                if (!QueryInformationJobObject(job, 9, ptr, (UInt32)size, IntPtr.Zero))
                    throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error(), "QueryInformationJobObject");
                limits = Marshal.PtrToStructure<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>(ptr);
                return checked((long)limits.PeakJobMemoryUsed.ToUInt64());
            } finally { Marshal.FreeHGlobal(ptr); }
        }
    }
}
'@

$commandProcess.Refresh()
$bootstrapPeakResidentBytes = [long]$commandProcess.PeakWorkingSet64
if ($bootstrapPeakResidentBytes -gt $memoryLimitBytes) { throw 'RUNNER_BOOTSTRAP_MEMORY_LIMIT' }
if (([DateTimeOffset]::UtcNow - $commandStarted).TotalSeconds -ge $wallLimitSeconds) { throw 'RUNNER_BOOTSTRAP_WALL_LIMIT' }

if (-not $InternalWorker) {
    $jobHandle = [WorkspacePolicy.NativeJob]::CreateAndAssignCurrent($memoryLimitBytes)
    $RunId = [Guid]::NewGuid().ToString('D')
    $publicArgv = @('-NoLogo','-NoProfile','-NonInteractive','-File','tools/ws_policy_gate.ps1','-Mode',$Mode)
    $workerArgv = @('-NoLogo','-NoProfile','-NonInteractive','-File',$scriptPath,'-Mode',$Mode,'-InternalWorker','-RunId',$RunId)
    $allowedEnvironmentNames = @('APPDATA','CARGO_HOME','COMSPEC','HOME','LOCALAPPDATA','PATH','PATHEXT','PSModulePath','RUSTUP_HOME','SYSTEMROOT','TEMP','TMP','USERPROFILE','WINDIR')
    $childEnvironment = [ordered]@{}
    foreach ($name in $allowedEnvironmentNames) {
        $value = [Environment]::GetEnvironmentVariable($name)
        if (-not [string]::IsNullOrEmpty($value)) { $childEnvironment[$name] = $value }
    }
    $manifestEntries = @($childEnvironment.Keys | Sort-Object | ForEach-Object {
        [ordered]@{ name = $_; value_sha256 = Get-TextSha256 ([string]$childEnvironment[$_]) }
    })
    $manifestSha = Get-TextSha256 (ConvertTo-CanonicalJson $manifestEntries)
    $workerArgv += @('-EnvironmentManifestSha256',$manifestSha)
    $deniedParentNames = @(Get-ChildItem Env: | Where-Object {
        $_.Name -match '^(CARGO_|RUST|HTTP_PROXY$|HTTPS_PROXY$|ALL_PROXY$|NO_PROXY$)' -and $allowedEnvironmentNames -notcontains $_.Name
    } | Select-Object -ExpandProperty Name | Sort-Object -Unique)

    $psi = [Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = [Diagnostics.Process]::GetCurrentProcess().MainModule.FileName
    foreach ($arg in $workerArgv) { [void]$psi.ArgumentList.Add($arg) }
    $psi.WorkingDirectory = $repoRoot
    $psi.UseShellExecute = $false
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.CreateNoWindow = $true
    $psi.Environment.Clear()
    foreach ($name in $childEnvironment.Keys) { $psi.Environment[$name] = [string]$childEnvironment[$name] }

    $worker = [Diagnostics.Process]::new()
    $worker.StartInfo = $psi
    if (-not $worker.Start()) { throw 'RUNNER_START_FAILED' }
    $captureBudget = [WorkspacePolicy.OutputBudget]::new($worker,$outputLimitBytes)
    $stdoutTask = [WorkspacePolicy.BoundedCapture]::ReadAsync($worker.StandardOutput.BaseStream,$captureBudget)
    $stderrTask = [WorkspacePolicy.BoundedCapture]::ReadAsync($worker.StandardError.BaseStream,$captureBudget)
    [long]$peakTreeBytes = $bootstrapPeakResidentBytes
    $termination = 'none'
    while (-not $worker.HasExited) {
        $tree = Get-ProcessTreeSnapshot $worker.Id
        [Diagnostics.Process]::GetCurrentProcess().Refresh()
        $wholeCommandBytes = $tree.resident_bytes + [Diagnostics.Process]::GetCurrentProcess().WorkingSet64
        if ($wholeCommandBytes -gt $peakTreeBytes) { $peakTreeBytes = $wholeCommandBytes }
        if ($peakTreeBytes -gt $memoryLimitBytes) {
            $termination = 'memory_limit'
            $worker.Kill($true)
            throw 'RUNNER_PROCESS_TREE_MEMORY_LIMIT'
        }
        if (([DateTimeOffset]::UtcNow - $commandStarted).TotalSeconds -ge $wallLimitSeconds) {
            $termination = 'wall_limit'
            $worker.Kill($true)
            throw 'RUNNER_WALL_LIMIT'
        }
        Start-Sleep -Milliseconds 25
    }
    $worker.WaitForExit()
    $stdoutBytes = $stdoutTask.GetAwaiter().GetResult()
    $stderrBytes = $stderrTask.GetAwaiter().GetResult()
    $strictUtf8 = [Text.UTF8Encoding]::new($false,$true)
    $stdout = $strictUtf8.GetString($stdoutBytes)
    $stderr = $strictUtf8.GetString($stderrBytes)
    $workerOutputBytes = $stdoutBytes.Length + $stderrBytes.Length
    if ($workerOutputBytes -gt $outputLimitBytes) { throw 'RUNNER_OUTPUT_LIMIT' }
    if ($worker.ExitCode -ne 0) { throw "RUNNER_WORKER_EXIT_$($worker.ExitCode):$stderr" }
    $subjectText = $stdout.TrimEnd([char]13,[char]10)
    $subject = $subjectText | ConvertFrom-Json -DateKind String
    if ($subject.schema -ne 'workspace-policy-subject.v2' -or $subject.run_id -ne $RunId -or $subject.mode -ne $Mode) { throw 'RUNNER_SUBJECT_IDENTITY_MISMATCH' }

    $supervisorEnded = [DateTimeOffset]::UtcNow
    $jobPeakCommittedBytes = [WorkspacePolicy.NativeJob]::PeakCommittedBytes($jobHandle)
    if ($jobPeakCommittedBytes -gt $memoryLimitBytes) { throw 'RUNNER_JOB_MEMORY_LIMIT' }
    if (([DateTimeOffset]::UtcNow - $commandStarted).TotalSeconds -ge $wallLimitSeconds) { throw 'RUNNER_WALL_LIMIT' }
    if ($peakTreeBytes -le 0) { throw 'RUNNER_PROCESS_TREE_MEASUREMENT_MISSING' }
    $record = [ordered]@{
        schema = 'workspace-policy-evidence.v2'
        record_id = $RunId
        repository = $subject.repository
        work_package = 'WP-WS-001'
        mode = $Mode
        disposition = $subject.disposition
        reason = $subject.reason
        implementation_commit = $subject.implementation_commit
        acceptance_commit = $subject.acceptance_commit
        wp_sha256 = $subject.wp_sha256
        runner_sha256 = $subject.runner_sha256
        invocation = [ordered]@{
            executable = 'pwsh'
            argv = $publicArgv
            cwd = $repoRoot
            canonical = "pwsh $($publicArgv -join ' ')"
        }
        environment = [ordered]@{
            policy = 'clear_then_exact_allowlist'
            allowlisted_names = @($manifestEntries.name)
            manifest = $manifestEntries
            manifest_sha256 = $manifestSha
            denied_parent_names_not_inherited = $deniedParentNames
        }
        started_utc = $commandStarted.ToString('o')
        ended_utc = $supervisorEnded.ToString('o')
        runner = [ordered]@{
            exit_code = $worker.ExitCode
            elapsed_ms = [math]::Round(([DateTimeOffset]::UtcNow - $commandStarted).TotalMilliseconds,3)
            bootstrap_peak_resident_bytes = $bootstrapPeakResidentBytes
            process_tree_peak_resident_bytes = $peakTreeBytes
            captured_stdout_bytes = $stdoutBytes.Length
            captured_stdout_sha256 = Get-TextSha256 $stdout
            captured_stderr_bytes = $stderrBytes.Length
            captured_stderr_sha256 = Get-TextSha256 $stderr
            captured_output_bytes = $workerOutputBytes
            job_peak_committed_bytes = $jobPeakCommittedBytes
            memory_enforcement = 'process-lifetime peak fail-closed through interop bootstrap; then Windows Job Object JOB_OBJECT_LIMIT_JOB_MEMORY over supervisor, worker, and inherited descendants'
            record_output_bytes = 0
            combined_generated_output_bytes = 0
            termination = $termination
            bounds = [ordered]@{ wall_seconds=$wallLimitSeconds; process_tree_resident_bytes=$memoryLimitBytes; combined_output_bytes=$outputLimitBytes; final_record_reservation_bytes=$finalRecordReservationBytes }
            digest_domain = 'captured hashes cover exact internal-worker streams; retained evidence-file hash covers this final record'
        }
        subject = $subject
    }
    $lastSize = -1
    for ($i=0; $i -lt 6; $i++) {
        $supervisorEnded = [DateTimeOffset]::UtcNow
        $record.ended_utc = $supervisorEnded.ToString('o')
        $record.runner.elapsed_ms = [math]::Round(([DateTimeOffset]::UtcNow - $commandStarted).TotalMilliseconds,3)
        $json = ConvertTo-CanonicalJson $record 20
        $size = [Text.Encoding]::UTF8.GetByteCount($json) + 1
        $record.runner.record_output_bytes = $size
        $record.runner.combined_generated_output_bytes = [long]$subject.worker_generated_tool_output_bytes + $workerOutputBytes + $size
        if ($size -eq $lastSize) { break }
        $lastSize = $size
    }
    $json = ConvertTo-CanonicalJson $record 20
    $finalBytes = [Text.Encoding]::UTF8.GetByteCount($json) + 1
    if ($record.runner.record_output_bytes -ne $finalBytes) { throw 'RUNNER_RECORD_SIZE_UNSTABLE' }
    if ($finalBytes -gt $finalRecordReservationBytes) { throw 'RUNNER_RECORD_RESERVATION_EXCEEDED' }
    if (([long]$subject.worker_generated_tool_output_bytes + $workerOutputBytes + $finalBytes) -gt $outputLimitBytes) { throw 'RUNNER_OUTPUT_LIMIT' }
    if (([DateTimeOffset]::UtcNow - $commandStarted).TotalSeconds -ge $wallLimitSeconds) { throw 'RUNNER_WALL_LIMIT' }
    [Console]::Out.Write($json + [char]10)
    exit 0
}

$workerStarted = [DateTimeOffset]::UtcNow
$script:workerGeneratedOutputBytes = 0L
if ([string]::IsNullOrWhiteSpace($RunId)) { throw 'WORKER_RUN_ID_MISSING' }
$actualEnvironment = Get-EnvironmentManifest
if ($actualEnvironment.sha256 -ne $EnvironmentManifestSha256) { throw 'WORKER_ENVIRONMENT_MISMATCH' }
Set-Location -LiteralPath $repoRoot

$expectedToolchain = @'
[toolchain]
channel = "1.95.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
'@
$expectedLock = @'
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4
'@
$wpText = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'docs/vtrace/WP_WS_001.md')
if ($wpText -match '^# ANCHOR ') {
    $repo = 'ANCHOR'
    $entryCommit = '70a204322e164ba7823e9bb8f019d4c870cb4faf'
    $acceptanceCommit = '5fd98be707d5fa8baf40c71adacc6d5167a4f855'
    $acceptancePulsePath = 'context/waves/2026-07-28-anchor-foundation/pulses/pulse-12-wp-ws-001-acceptance.md'
    $expectedManifest = @'
[workspace]
members = []
default-members = []
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.95.0"
publish = false

[workspace.lints.rust]
unsafe_code = "forbid"
warnings = "deny"

[workspace.lints.clippy]
all = "deny"
pedantic = "deny"

[profile.dev]
opt-level = 0
debug = 2
overflow-checks = true
incremental = false

[profile.test]
opt-level = 0
debug = 2
overflow-checks = true
incremental = false

[profile.release]
opt-level = 3
debug = 1
overflow-checks = true
incremental = false
lto = "thin"
codegen-units = 1
panic = "unwind"
'@
    $allowedExact = @(
        'Cargo.toml','Cargo.lock','rust-toolchain.toml','tools/ws_policy_gate.ps1',
        'context/waves/2026-07-28-anchor-foundation/pulses/pulse-13-wp-ws-001-execution.md',
        'context/waves/2026-07-28-anchor-foundation/evidence/wp-ws-001/EVID-WP-WS-001.md',
        'context/waves/2026-07-28-anchor-foundation/evidence/wp-ws-001/TRACE-WP-WS-001.md'
    )
    $allowedPrefixes = @()
} elseif ($wpText -match '^# WP-WS-001-R1') {
    $repo = 'BASTION'
    $entryCommit = '6b0ed64c99da385615295d9cb5573d224fd8662a'
    $acceptanceCommit = 'c79940a91b4fc2fb1cf2dffa4a317f1ae8306ce2'
    $acceptancePulsePath = 'context/waves/2026-07-28-bastion-foundation/pulses/pulse-11-wp-ws-001-acceptance.md'
    $expectedManifest = @'
[workspace]
members = []
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.95.0"

[workspace.lints.rust]
unsafe_code = "forbid"
'@
    $allowedExact = @('Cargo.toml','Cargo.lock','rust-toolchain.toml','tools/ws_policy_gate.ps1')
    $allowedPrefixes = @('context/waves/2026-07-28-bastion-foundation/evidence/wp-ws-001/')
} else { throw 'WP_IDENTITY_UNKNOWN' }

function Normalize-Text([string]$Text) { return (($Text -replace ([char]13+[char]10),[char]10).TrimEnd()) }
function Normalize-RepoPath([string]$Path) { return ($Path.Trim('"').Replace([char]92,[char]47).TrimStart('.','/')) }
function Test-AllowedPath([string]$Path) {
    $normalized = Normalize-RepoPath $Path
    if ($allowedExact -contains $normalized) { return $true }
    foreach ($prefix in $allowedPrefixes) { if ($normalized.StartsWith($prefix,[StringComparison]::Ordinal)) { return $true } }
    return $false
}
function Assert-ManifestCandidate([string]$Candidate) { if ((Normalize-Text $Candidate) -cne (Normalize-Text $expectedManifest)) { throw 'MANIFEST_NOT_EXACT' } }
function Assert-ToolchainCandidate([string]$Candidate) { if ((Normalize-Text $Candidate) -cne (Normalize-Text $expectedToolchain)) { throw 'TOOLCHAIN_NOT_EXACT' } }
function Assert-LockCandidate([string]$Candidate) {
    if ($Candidate.Contains('[[package]]')) { throw 'LOCK_PACKAGE_ENTRY' }
    if ((Normalize-Text $Candidate) -cne (Normalize-Text $expectedLock)) { throw 'LOCK_NOT_EXACT' }
}
function Assert-ChangedPathCandidate([string]$Candidate) { if (-not (Test-AllowedPath $Candidate)) { throw 'PATH_NOT_ALLOWED' } }
function Assert-EnvironmentCandidate([Collections.IDictionary]$Candidate) {
    foreach ($name in $Candidate.Keys) {
        if ($name -in @('RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_TARGET_DIR') -or $name -match '^CARGO_(HTTP|NET|REGISTR|SOURCE)') { throw 'ENV_NOT_ALLOWED' }
    }
}
function Assert-EdgeCandidate([object[]]$Candidate) { if (@($Candidate).Count -ne 0) { throw 'EDGE_SET_NONEMPTY' } }
function Invoke-RejectionCase([string]$Id,[string]$Validator,[string]$ExpectedCode,[string]$CandidateIdentity,[scriptblock]$Action) {
    $actual = $null
    try { & $Action; $actual = 'NOT_REJECTED' } catch { $actual = $_.Exception.Message }
    if ($actual -ne $ExpectedCode) { throw "NEGATIVE_CASE_FAILED:$($Id):$actual" }
    return [ordered]@{ id=$Id; validator=$Validator; expected_code=$ExpectedCode; actual_code=$actual; candidate_sha256=Get-TextSha256 $CandidateIdentity; rejected=$true }
}
function Invoke-CapturedProcess([string]$FileName,[string[]]$Arguments) {
    $psi = [Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = $FileName
    foreach ($arg in $Arguments) { [void]$psi.ArgumentList.Add($arg) }
    $psi.WorkingDirectory = $repoRoot
    $psi.UseShellExecute = $false
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.CreateNoWindow = $true
    $process = [Diagnostics.Process]::new()
    $process.StartInfo = $psi
    $started = [DateTimeOffset]::UtcNow
    if (-not $process.Start()) { throw "PROCESS_START_FAILED:$FileName" }
    $remainingBudget = $outputLimitBytes - $finalRecordReservationBytes - $script:workerGeneratedOutputBytes
    if ($remainingBudget -le 0) { throw 'WORKER_CUMULATIVE_OUTPUT_LIMIT' }
    $captureBudget = [WorkspacePolicy.OutputBudget]::new($process,$remainingBudget)
    $stdoutTask = [WorkspacePolicy.BoundedCapture]::ReadAsync($process.StandardOutput.BaseStream,$captureBudget)
    $stderrTask = [WorkspacePolicy.BoundedCapture]::ReadAsync($process.StandardError.BaseStream,$captureBudget)
    $process.WaitForExit()
    $stdoutBytes = $stdoutTask.GetAwaiter().GetResult()
    $stderrBytes = $stderrTask.GetAwaiter().GetResult()
    $strictUtf8 = [Text.UTF8Encoding]::new($false,$true)
    $stdout = $strictUtf8.GetString($stdoutBytes)
    $stderr = $strictUtf8.GetString($stderrBytes)
    $ended = [DateTimeOffset]::UtcNow
    $outputBytes = $stdoutBytes.Length + $stderrBytes.Length
    $script:workerGeneratedOutputBytes += $outputBytes
    if ($script:workerGeneratedOutputBytes -gt ($outputLimitBytes - $finalRecordReservationBytes)) { throw 'WORKER_CUMULATIVE_OUTPUT_LIMIT' }
    if ($process.ExitCode -ne 0) { throw "PROCESS_EXIT_$($process.ExitCode):$($FileName):$stderr" }
    return [ordered]@{
        executable=$FileName; argv=$Arguments; command="$FileName $($Arguments -join ' ')"
        started_utc=$started.ToString('o'); ended_utc=$ended.ToString('o'); exit_code=$process.ExitCode
        output_bytes=$outputBytes; stdout_bytes=$stdoutBytes.Length; stdout_sha256=Get-TextSha256 $stdout
        stderr_bytes=$stderrBytes.Length; stderr_sha256=Get-TextSha256 $stderr; stdout=$stdout; stderr=$stderr
    }
}
function Get-GitBlobSha256([string]$Spec) {
    $psi=[Diagnostics.ProcessStartInfo]::new(); $psi.FileName='git'
    foreach($arg in @('cat-file','blob',$Spec)){[void]$psi.ArgumentList.Add($arg)}
    $psi.WorkingDirectory=$repoRoot; $psi.UseShellExecute=$false; $psi.RedirectStandardOutput=$true; $psi.RedirectStandardError=$true; $psi.CreateNoWindow=$true
    $process=[Diagnostics.Process]::new(); $process.StartInfo=$psi
    if(-not $process.Start()){throw 'GIT_BLOB_START_FAILED'}
    $remainingBudget=$outputLimitBytes-$finalRecordReservationBytes-$script:workerGeneratedOutputBytes
    if($remainingBudget-le 0){throw 'WORKER_CUMULATIVE_OUTPUT_LIMIT'}
    $captureBudget=[WorkspacePolicy.OutputBudget]::new($process,$remainingBudget)
    $stdoutTask=[WorkspacePolicy.BoundedCapture]::ReadAsync($process.StandardOutput.BaseStream,$captureBudget)
    $stderrTask=[WorkspacePolicy.BoundedCapture]::ReadAsync($process.StandardError.BaseStream,$captureBudget)
    [void]$process.WaitForExit(); $blobBytes=$stdoutTask.GetAwaiter().GetResult(); $stderrBytes=$stderrTask.GetAwaiter().GetResult()
    $stderr=[Text.UTF8Encoding]::new($false,$true).GetString($stderrBytes)
    if($process.ExitCode-ne 0){throw "GIT_BLOB_EXIT_$($process.ExitCode):$stderr"}
    $script:workerGeneratedOutputBytes += $blobBytes.Length + $stderrBytes.Length
    if ($script:workerGeneratedOutputBytes -gt ($outputLimitBytes - $finalRecordReservationBytes)) { throw 'WORKER_CUMULATIVE_OUTPUT_LIMIT' }
    return [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($blobBytes)).ToLowerInvariant()
}

$liveManifest = Get-Content -Raw -LiteralPath Cargo.toml
$liveToolchain = Get-Content -Raw -LiteralPath rust-toolchain.toml
$liveLock = Get-Content -Raw -LiteralPath Cargo.lock
Assert-ManifestCandidate $liveManifest
Assert-ToolchainCandidate $liveToolchain
Assert-LockCandidate $liveLock
Assert-EnvironmentCandidate @{}

$metadataRun = Invoke-CapturedProcess 'cargo' @('+1.95.0','metadata','--format-version','1','--no-deps','--locked','--offline')
$metadata = $metadataRun.stdout | ConvertFrom-Json
if (@($metadata.packages).Count -ne 0) { throw 'METADATA_PACKAGES_NOT_EMPTY' }
if (@($metadata.workspace_members).Count -ne 0) { throw 'METADATA_MEMBERS_NOT_EMPTY' }
if (@($metadata.workspace_default_members).Count -ne 0) { throw 'METADATA_DEFAULT_MEMBERS_NOT_EMPTY' }
if ($null -ne $metadata.resolve) { throw 'METADATA_RESOLVE_NOT_NULL' }
if ([IO.Path]::GetFullPath($metadata.workspace_root) -cne $repoRoot) { throw 'METADATA_ROOT_ESCAPE' }
if (-not [IO.Path]::GetFullPath($metadata.target_directory).StartsWith($repoRoot+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)) { throw 'METADATA_TARGET_ESCAPE' }

$gitDiff = Invoke-CapturedProcess 'git' @('diff','--name-only',"$acceptanceCommit...HEAD")
$gitStatus = Invoke-CapturedProcess 'git' @('status','--porcelain=v1','--untracked-files=all','--ignored=matching')
$changedPaths = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
$gitDiff.stdout.Split([char]10) | Where-Object { $_ } | ForEach-Object { [void]$changedPaths.Add((Normalize-RepoPath $_)) }
$gitStatus.stdout.Split([char]10) | Where-Object { $_.Length -ge 4 } | ForEach-Object {
    $path = $_.Substring(3)
    if ($path -match ' -> ') { $path = ($path -split ' -> ')[-1] }
    $normalized = Normalize-RepoPath $path
    if ($normalized -ne 'target' -and -not $normalized.StartsWith('target/')) { [void]$changedPaths.Add($normalized) }
}
foreach ($path in $changedPaths) { Assert-ChangedPathCandidate $path }

$gitMarker = [IO.Path]::GetFullPath((Join-Path $repoRoot '.git'))
Get-ChildItem -LiteralPath $repoRoot -Force -Recurse | Where-Object {
    $full=[IO.Path]::GetFullPath($_.FullName)
    $full -ne $gitMarker -and -not $full.StartsWith($gitMarker+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)
} | ForEach-Object { if ($null -ne $_.LinkType) { throw "REPARSE_SURFACE:$($_.FullName)" } }

$cases=[Collections.Generic.List[object]]::new()
$manifestAppends=[ordered]@{
    'M-MEMBER-EXPLICIT'='members = ["crates/x"]'; 'M-MEMBER-GLOB'='members = ["crates/*"]'
    'M-PACKAGE'='[package]'+[char]10+'name="x"'+[char]10+'version="0.0.0"'
    'M-TARGET-LIB'='[lib]'+[char]10+'path="src/lib.rs"'; 'M-TARGET-BIN'='[[bin]]'+[char]10+'name="x"'+[char]10+'path="src/main.rs"'
    'M-TARGET-TEST'='[[test]]'+[char]10+'name="x"'+[char]10+'path="tests/x.rs"'; 'M-TARGET-BENCH'='[[bench]]'+[char]10+'name="x"'+[char]10+'path="benches/x.rs"'
    'M-TARGET-EXAMPLE'='[[example]]'+[char]10+'name="x"'+[char]10+'path="examples/x.rs"'
    'M-DEPENDENCY'='[workspace.dependencies]'+[char]10+'serde="1"'; 'M-FEATURE'='[features]'+[char]10+'default=[]'
    'M-PATCH'='[patch.crates-io]'+[char]10+'x={path="vendor/x"}'; 'M-REPLACE'='[replace]'+[char]10+'"x:0.1.0"={path="vendor/x"}'
    'M-REGISTRY-TABLE'='[registries.internal]'+[char]10+'index="https://invalid.example/index"'
    'M-SOURCE-PARENT'='[workspace.dependencies]'+[char]10+'x={path="../x"}'; 'M-SOURCE-SIBLING'='[workspace.dependencies]'+[char]10+'x={path="../sibling/x"}'
    'M-SOURCE-GIT'='[workspace.dependencies]'+[char]10+'x={git="https://invalid.example/x"}'; 'M-SOURCE-REGISTRY'='[workspace.dependencies]'+[char]10+'x={version="1",registry="internal"}'
    'M-BUILD-SCRIPT'='[package]'+[char]10+'build="build.rs"'; 'M-PROC-MACRO'='[lib]'+[char]10+'proc-macro=true'; 'M-NATIVE-LINK'='[package]'+[char]10+'links="native_x"'
}
foreach($entry in $manifestAppends.GetEnumerator()){
    $candidate=(Normalize-Text $expectedManifest)+[char]10+$entry.Value+[char]10
    $cases.Add((Invoke-RejectionCase $entry.Key 'Assert-ManifestCandidate' 'MANIFEST_NOT_EXACT' $candidate { Assert-ManifestCandidate $candidate }))
}
$manifestDrifts=[ordered]@{
    'M-UNSAFE-ALLOW'=@('unsafe_code = "forbid"','unsafe_code = "allow"'); 'M-RESOLVER-DRIFT'=@('resolver = "3"','resolver = "2"')
    'M-EDITION-DRIFT'=@('edition = "2024"','edition = "2021"'); 'M-MSRV-DRIFT'=@('rust-version = "1.95.0"','rust-version = "1.94.0"')
}
if($repo -eq 'ANCHOR'){
    $manifestDrifts['M-DEFAULT-MEMBER-DRIFT']=@('default-members = []','default-members = ["crates/x"]')
    $manifestDrifts['M-PUBLISH-DRIFT']=@('publish = false','publish = true')
    $manifestDrifts['M-PROFILE-DEV']=@('incremental = false','incremental = true')
    $manifestDrifts['M-PROFILE-TEST']=@('overflow-checks = true','overflow-checks = false')
    $manifestDrifts['M-PROFILE-RELEASE']=@('panic = "unwind"','panic = "abort"')
    $manifestDrifts['M-LINT-RUST']=@('warnings = "deny"','warnings = "warn"'); $manifestDrifts['M-LINT-CLIPPY']=@('pedantic = "deny"','pedantic = "warn"')
}
foreach($entry in $manifestDrifts.GetEnumerator()){
    $candidate=$expectedManifest.Replace($entry.Value[0],$entry.Value[1])
    $cases.Add((Invoke-RejectionCase $entry.Key 'Assert-ManifestCandidate' 'MANIFEST_NOT_EXACT' $candidate { Assert-ManifestCandidate $candidate }))
}
$toolchainCandidates=[ordered]@{
    'T-CHANNEL-DRIFT'=$expectedToolchain.Replace('1.95.0','1.94.0'); 'T-PROFILE-DRIFT'=$expectedToolchain.Replace('profile = "minimal"','profile = "default"')
    'T-COMPONENT-EXTRA'=$expectedToolchain.Replace('"clippy"]','"clippy", "rust-src"]'); 'T-COMPONENT-MISSING'=$expectedToolchain.Replace(', "clippy"','')
    'T-TARGET-EXTRA'=(Normalize-Text $expectedToolchain)+[char]10+'targets = ["wasm32-unknown-unknown"]'
}
foreach($entry in $toolchainCandidates.GetEnumerator()){
    $candidate=$entry.Value
    $cases.Add((Invoke-RejectionCase $entry.Key 'Assert-ToolchainCandidate' 'TOOLCHAIN_NOT_EXACT' $candidate { Assert-ToolchainCandidate $candidate }))
}
$environmentNames=@('RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_TARGET_DIR','CARGO_HTTP_PROXY','CARGO_NET_OFFLINE','CARGO_REGISTRIES_INTERNAL_INDEX','CARGO_SOURCE_INTERNAL_REPLACE_WITH')
foreach($name in $environmentNames){
    $candidate=[ordered]@{ $name='synthetic' }
    $cases.Add((Invoke-RejectionCase "E-$name" 'Assert-EnvironmentCandidate' 'ENV_NOT_ALLOWED' $name { Assert-EnvironmentCandidate $candidate }))
}
$lockCases=[ordered]@{
    'L-PACKAGE-ENTRY'=(Normalize-Text $expectedLock)+[char]10+'[[package]]'+[char]10+'name = "x"'
    'L-HAND-EDIT-COMMENT'=$expectedLock.Replace('automatically @generated','manually generated')
    'L-HAND-EDIT-VERSION'=$expectedLock.Replace('version = 4','version = 3'); 'L-EXTRA-TEXT'=(Normalize-Text $expectedLock)+[char]10+'# extra'
}
foreach($entry in $lockCases.GetEnumerator()){
    $candidate=$entry.Value; $code=if($entry.Key -eq 'L-PACKAGE-ENTRY'){'LOCK_PACKAGE_ENTRY'}else{'LOCK_NOT_EXACT'}
    $cases.Add((Invoke-RejectionCase $entry.Key 'Assert-LockCandidate' $code $candidate { Assert-LockCandidate $candidate }))
}
$pathCases=@('.cargo/config','.cargo/config.toml','crates/product/src/lib.rs','tests/example.rs','fixtures/person.json','generated/schema.rs','docs/product.md','.github/workflows/ci.yml','release/manifest.json','TRACKER.md','unexpected.txt','../outside/Cargo.toml')
foreach($candidate in $pathCases){
    $id='P-'+(($candidate-replace'[^A-Za-z0-9]+','-').Trim('-').ToUpperInvariant())
    $cases.Add((Invoke-RejectionCase $id 'Assert-ChangedPathCandidate' 'PATH_NOT_ALLOWED' $candidate { Assert-ChangedPathCandidate $candidate }))
}
$edgeKinds=@('build','runtime','review','generated','documentation','release','XREL','HND','Taxlane','semantic')
foreach($kind in $edgeKinds){
    $candidate=@([ordered]@{kind=$kind;from='WP-WS-001';to='synthetic'}); $identity=ConvertTo-CanonicalJson $candidate
    $cases.Add((Invoke-RejectionCase "G-$($kind.ToUpperInvariant())" 'Assert-EdgeCandidate' 'EDGE_SET_NONEMPTY' $identity { Assert-EdgeCandidate $candidate }))
}

$rustRun=Invoke-CapturedProcess 'rustc' @('+1.95.0','--version','--verbose')
$cargoRun=Invoke-CapturedProcess 'cargo' @('+1.95.0','--version','--verbose')
if($rustRun.stdout -notmatch '(?m)^release: 1[.]95[.]0$'){throw 'RUST_VERSION_MISMATCH'}
if($cargoRun.stdout -notmatch '(?m)^cargo 1[.]95[.]0 '){throw 'CARGO_VERSION_MISMATCH'}
if($PSVersionTable.PSVersion.ToString() -ne '7.6.4'){throw 'POWERSHELL_VERSION_MISMATCH'}
$autoCrlfRun=Invoke-CapturedProcess 'git' @('config','--get','core.autocrlf')
if($autoCrlfRun.stdout.Trim() -ne 'false'){throw 'GIT_AUTOCRLF_NOT_FALSE'}
$headRun=Invoke-CapturedProcess 'git' @('rev-parse','HEAD')
$executionHead=$headRun.stdout.Trim()
$historyRun=Invoke-CapturedProcess 'git' @('rev-list','--ancestry-path','--reverse',"$entryCommit..$executionHead")
$history=@($historyRun.stdout.Split([char]10) | ForEach-Object { $_.Trim() } | Where-Object { $_ })
if($history.Count-lt 1){throw 'IMPLEMENTATION_COMMIT_MISSING'}
$implementationCommit=$history[0]
$parentRun=Invoke-CapturedProcess 'git' @('rev-parse',"$implementationCommit^")
if($parentRun.stdout.Trim()-ne$entryCommit){throw 'IMPLEMENTATION_PARENT_MISMATCH'}
$implementationPaths=@((Invoke-CapturedProcess 'git' @('diff-tree','--no-commit-id','--name-only','-r',$implementationCommit)).stdout.Split([char]10) | ForEach-Object {$_.Trim()} | Where-Object {$_} | Sort-Object)
$expectedImplementationPaths=@('Cargo.lock','Cargo.toml','rust-toolchain.toml','tools/ws_policy_gate.ps1')
if((ConvertTo-CanonicalJson $implementationPaths)-cne(ConvertTo-CanonicalJson $expectedImplementationPaths)){throw 'IMPLEMENTATION_PATH_SET_MISMATCH'}
$implementationTreeCheck=Invoke-CapturedProcess 'git' (@('diff','--exit-code',$implementationCommit,'--')+$expectedImplementationPaths)
foreach($commit in @($history | Select-Object -Skip 1)){
    $laterPaths=@((Invoke-CapturedProcess 'git' @('diff-tree','--no-commit-id','--name-only','-r',$commit)).stdout.Split([char]10) | ForEach-Object {$_.Trim()} | Where-Object {$_})
    foreach($path in $laterPaths){if($expectedImplementationPaths-contains$path){throw 'IMPLEMENTATION_CHANGED_AFTER_FIXED_COMMIT'};Assert-ChangedPathCandidate $path}
}
$pulseSpec=$acceptanceCommit+':'+$acceptancePulsePath
$pulseOidRun=Invoke-CapturedProcess 'git' @('rev-parse',$pulseSpec)
$pulseBlobSha256=Get-GitBlobSha256 $pulseSpec
$naModes=@('L0Format','L1Lint','L1Test','L1Doc')
$disposition=if($naModes -contains $Mode){'not_applicable_empty_workspace_target'}else{'pass'}
$workerEnded=[DateTimeOffset]::UtcNow

$result=[ordered]@{
    schema='workspace-policy-subject.v2'; run_id=$RunId; repository=$repo; mode=$Mode; disposition=$disposition
    reason=if($disposition -eq 'pass'){'empty workspace policy and all assigned assertions passed'}else{'executed metadata proved zero applicable Cargo target; no target pass claimed'}
    implementation_commit=$implementationCommit; execution_head=$executionHead; acceptance_commit=$acceptanceCommit
    acceptance_pulse=[ordered]@{path=$acceptancePulsePath;blob_oid=$pulseOidRun.stdout.Trim();blob_sha256=$pulseBlobSha256;digest_domain='exact immutable Git blob bytes; no checkout conversion or text normalization'}
    wp_sha256=Get-TextSha256 (($wpText-replace([char]13+[char]10),[char]10)); runner_sha256=(Get-FileHash -LiteralPath $scriptPath -Algorithm SHA256).Hash.ToLowerInvariant()
    manifest_sha256=(Get-FileHash -LiteralPath Cargo.toml -Algorithm SHA256).Hash.ToLowerInvariant(); lock_sha256=(Get-FileHash -LiteralPath Cargo.lock -Algorithm SHA256).Hash.ToLowerInvariant()
    toolchain_sha256=(Get-FileHash -LiteralPath rust-toolchain.toml -Algorithm SHA256).Hash.ToLowerInvariant(); worker_started_utc=$workerStarted.ToString('o'); worker_ended_utc=$workerEnded.ToString('o')
    environment_manifest_sha256=$actualEnvironment.sha256; changed_paths=@($changedPaths|Sort-Object)
    tools=[ordered]@{powershell=[ordered]@{version=$PSVersionTable.PSVersion.ToString()};rustc=$rustRun;cargo=$cargoRun;git_core_autocrlf=$autoCrlfRun.stdout.Trim()}
    metadata=$metadataRun; worker_generated_tool_output_bytes=$script:workerGeneratedOutputBytes
    metadata_assertions=[ordered]@{packages=0;workspace_members=0;workspace_default_members=0;resolve=$null;dependency_edges=0;semantic_edges=0}
    negative_cases=@($cases|Sort-Object{$_.id}); negative_case_count=$cases.Count; authority_created=$false; holds_closed=@()
}
$json=ConvertTo-CanonicalJson $result 20
$workerJsonBytes=[Text.Encoding]::UTF8.GetByteCount($json)
if(($script:workerGeneratedOutputBytes+$workerJsonBytes)-gt($outputLimitBytes-$finalRecordReservationBytes)){throw 'WORKER_CUMULATIVE_OUTPUT_LIMIT'}
[Console]::Out.Write($json)
