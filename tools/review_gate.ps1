param(
    [Parameter(Mandatory = $true)]
    [ValidateSet('L0Format','L0Check','L0FocusedTest','L1WorkspaceCheck','L1Clippy','L1Test','L1Doc','L1Static','L1SupplyChain','L2Contract','L2Model','L2Adversarial','L2NoAuthority')]
    [string]$Mode,
    [switch]$InternalWorker,
    [string]$EnvironmentManifestSha256
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$scriptPath = [IO.Path]::GetFullPath($PSCommandPath)
$wallLimitMilliseconds = 60000
$memoryLimitBytes = 1GB
$outputLimitBytes = 10MB

function Get-BytesSha256([byte[]]$Bytes) {
    [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($Bytes)).ToLowerInvariant()
}
function Get-TextSha256([string]$Text) {
    Get-BytesSha256 ([Text.Encoding]::UTF8.GetBytes($Text))
}
function Get-FileSha256([string]$Path) {
    Get-BytesSha256 ([IO.File]::ReadAllBytes($Path))
}
function ConvertTo-CanonicalJson($Value, [int]$Depth = 12) {
    $Value | ConvertTo-Json -Depth $Depth -Compress
}
function New-TempPath {
    $path = [IO.Path]::Combine([IO.Path]::GetTempPath(), "bastion-review-$([Guid]::NewGuid().ToString('N')).tmp")
    [IO.File]::WriteAllBytes($path, [byte[]]::new(0))
    $path
}

Add-Type -TypeDefinition @'
using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
namespace BastionReviewGate {
  public sealed class OutputBudget {
    internal long Total;
    internal readonly long Limit;
    internal Process Process;
    public OutputBudget(Process process, long limit) { Process = process; Limit = limit; }
    public void Attach(Process process) { Process = process; }
    public long Bytes { get { return Interlocked.Read(ref Total); } }
  }
  public static class BoundedCapture {
    public static async Task<byte[]> ReadAsync(Stream stream, OutputBudget budget) {
      using (var memory = new MemoryStream()) {
        var buffer = new byte[8192];
        while (true) {
          int count = await stream.ReadAsync(buffer, 0, buffer.Length).ConfigureAwait(false);
          if (count == 0) break;
          if (Interlocked.Add(ref budget.Total, count) > budget.Limit) {
            try { budget.Process.Kill(true); } catch { }
            throw new IOException("OUTPUT_LIMIT_EXCEEDED");
          }
          memory.Write(buffer, 0, count);
        }
        return memory.ToArray();
      }
    }
  }
  public static class NativeJob {
    const uint JOB_OBJECT_LIMIT_JOB_MEMORY = 0x00000200;
    const uint JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE = 0x00002000;
    [StructLayout(LayoutKind.Sequential)] struct Basic {
      public long PerProcessUserTimeLimit, PerJobUserTimeLimit;
      public uint LimitFlags;
      public UIntPtr MinimumWorkingSetSize, MaximumWorkingSetSize;
      public uint ActiveProcessLimit;
      public UIntPtr Affinity;
      public uint PriorityClass, SchedulingClass;
    }
    [StructLayout(LayoutKind.Sequential)] struct Io {
      public ulong ReadOperationCount, WriteOperationCount, OtherOperationCount;
      public ulong ReadTransferCount, WriteTransferCount, OtherTransferCount;
    }
    [StructLayout(LayoutKind.Sequential)] struct Extended {
      public Basic BasicLimitInformation;
      public Io IoInfo;
      public UIntPtr ProcessMemoryLimit, JobMemoryLimit, PeakProcessMemoryUsed, PeakJobMemoryUsed;
    }
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)] static extern IntPtr CreateJobObject(IntPtr a, string n);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool SetInformationJobObject(IntPtr j, int c, IntPtr i, uint l);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr j, IntPtr p);
    [DllImport("kernel32.dll", SetLastError=true)] public static extern bool CloseHandle(IntPtr h);
    public static IntPtr Create(long memory) {
      IntPtr job = CreateJobObject(IntPtr.Zero, null);
      if (job == IntPtr.Zero) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
      var limits = new Extended();
      limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_JOB_MEMORY | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
      limits.JobMemoryLimit = new UIntPtr((ulong)memory);
      int size = Marshal.SizeOf(limits); IntPtr ptr = Marshal.AllocHGlobal(size);
      try { Marshal.StructureToPtr(limits, ptr, false); if (!SetInformationJobObject(job, 9, ptr, (uint)size)) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error()); }
      finally { Marshal.FreeHGlobal(ptr); }
      return job;
    }
    public static void Assign(IntPtr job, Process process) {
      if (!AssignProcessToJobObject(job, process.Handle)) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
    }
  }
}
'@

if (-not $InternalWorker) {
    $started = [Diagnostics.Stopwatch]::StartNew()
    $supervisedEvidenceDirectory = Join-Path $repoRoot 'context/waves/2026-07-28-bastion-foundation/evidence/wp-rev-001'
    $supervisedEvidencePath = Join-Path $supervisedEvidenceDirectory "$Mode.json"
    [IO.Directory]::CreateDirectory($supervisedEvidenceDirectory) | Out-Null
    if ([IO.File]::Exists($supervisedEvidencePath)) { [IO.File]::Delete($supervisedEvidencePath) }
    $stdoutPath = New-TempPath
    $stderrPath = New-TempPath
    $job = [IntPtr]::Zero
    $manifestSha = ''
    $stdout = ''
    $stderr = ''
    try {
        $job = [BastionReviewGate.NativeJob]::Create($memoryLimitBytes)
        $allowed = @('APPDATA','CARGO_HOME','COMSPEC','HOME','LOCALAPPDATA','PATH','PATHEXT','PSModulePath','RUSTUP_HOME','SYSTEMROOT','TEMP','TMP','USERPROFILE','WINDIR')
        $environment = [ordered]@{}
        foreach ($name in $allowed) {
            $value = [Environment]::GetEnvironmentVariable($name)
            if (-not [string]::IsNullOrEmpty($value)) { $environment[$name] = $value }
        }
        $manifestEntries = @($environment.Keys | Sort-Object | ForEach-Object {
            [ordered]@{ name = $_; value_sha256 = Get-TextSha256 ([string]$environment[$_]) }
        })
        $manifestSha = Get-TextSha256 (ConvertTo-CanonicalJson $manifestEntries)
        $psi = [Diagnostics.ProcessStartInfo]::new()
        $psi.FileName = (Get-Process -Id $PID).MainModule.FileName
        foreach ($argument in @('-NoLogo','-NoProfile','-NonInteractive','-File',$scriptPath,'-Mode',$Mode,'-InternalWorker','-EnvironmentManifestSha256',$manifestSha)) {
            [void]$psi.ArgumentList.Add($argument)
        }
        $psi.WorkingDirectory = $repoRoot
        $psi.UseShellExecute = $false
        $psi.CreateNoWindow = $true
        $psi.RedirectStandardOutput = $false
        $psi.RedirectStandardError = $false
        $psi.Environment.Clear()
        foreach ($name in $environment.Keys) { $psi.Environment[$name] = [string]$environment[$name] }
        $worker = [Diagnostics.Process]::new()
        $worker.StartInfo = $psi
        $worker.StartInfo.RedirectStandardOutput = $true
        $worker.StartInfo.RedirectStandardError = $true
        if (-not $worker.Start()) { throw 'WORKER_START_FAILED' }
        [BastionReviewGate.NativeJob]::Assign($job, $worker)
        $budget = [BastionReviewGate.OutputBudget]::new($worker, $outputLimitBytes)
        $stdoutTask = [BastionReviewGate.BoundedCapture]::ReadAsync($worker.StandardOutput.BaseStream, $budget)
        $stderrTask = [BastionReviewGate.BoundedCapture]::ReadAsync($worker.StandardError.BaseStream, $budget)
        if (-not $worker.WaitForExit($wallLimitMilliseconds)) {
            $worker.Kill($true)
            throw 'WALL_LIMIT_EXCEEDED'
        }
        $stdout = [Text.Encoding]::UTF8.GetString($stdoutTask.GetAwaiter().GetResult())
        $stderr = [Text.Encoding]::UTF8.GetString($stderrTask.GetAwaiter().GetResult())
        if ($worker.ExitCode -ne 0) { throw "WORKER_EXIT_$($worker.ExitCode):$stderr" }
        [IO.File]::WriteAllText($supervisedEvidencePath, $stdout.Trim() + "`n", [Text.UTF8Encoding]::new($false))
        $stdout
    } catch {
        $failure = [ordered]@{
            schema = 'review-gate-evidence.v1'; repository = 'BASTION'; work_package = 'WP-REV-001-R1'; mode = $Mode
            result = 'fail'; exit_code = 1; started_at_utc = [DateTimeOffset]::UtcNow.AddMilliseconds(-$started.ElapsedMilliseconds).ToString('O'); ended_at_utc = [DateTimeOffset]::UtcNow.ToString('O'); duration_ms = $started.ElapsedMilliseconds
            implementation_sha256 = Get-FileSha256 (Join-Path $repoRoot 'crates/bastion-review/src/lib.rs'); work_package_sha256 = Get-FileSha256 (Join-Path $repoRoot 'docs/vtrace/WP_REV_001.md'); acceptance_sha256 = Get-FileSha256 (Join-Path $repoRoot 'context/waves/2026-07-28-bastion-foundation/pulses/pulse-14-wp-rev-001-acceptance.md'); runner_sha256 = Get-FileSha256 $scriptPath
            implementation_commit = (& git -C $repoRoot rev-parse HEAD).Trim(); manifest_sha256 = Get-FileSha256 (Join-Path $repoRoot 'Cargo.toml'); lock_sha256 = Get-FileSha256 (Join-Path $repoRoot 'Cargo.lock'); toolchain = '1.95.0'
            sanitized_environment_sha256 = $manifestSha; failure_sha256 = Get-TextSha256 $_.Exception.Message; argv = @('-NoLogo','-NoProfile','-NonInteractive','-File','tools/review_gate.ps1','-Mode',$Mode)
            captured_streams = [ordered]@{ stdout_sha256 = Get-TextSha256 $stdout; stderr_sha256 = Get-TextSha256 $stderr }
            subject_assertions = @([ordered]@{ id = 'supervised-mode-completion'; result = 'fail' }); assertions = @([ordered]@{ id = 'outer-supervisor-accepted'; result = 'fail' }); negative_cases = @([ordered]@{ id = 'mode-failure-retained'; result = 'pass' })
            bounds = [ordered]@{ wall_seconds = 60; process_tree_bytes = $memoryLimitBytes; output_bytes = $outputLimitBytes }
        }
        $failureJson = ConvertTo-CanonicalJson $failure 12
        $failureDirectory = Join-Path $repoRoot 'context/waves/2026-07-28-bastion-foundation/evidence/wp-rev-001'
        [IO.Directory]::CreateDirectory($failureDirectory) | Out-Null
        [IO.File]::WriteAllText((Join-Path $failureDirectory "$Mode.json"), $failureJson + "`n", [Text.UTF8Encoding]::new($false))
        [Console]::Out.WriteLine($failureJson)
        [Console]::Error.WriteLine($_.Exception.Message)
        exit 1
    } finally {
        if ($job -ne [IntPtr]::Zero) { [void][BastionReviewGate.NativeJob]::CloseHandle($job) }
        foreach ($path in @($stdoutPath,$stderrPath)) { if ([IO.File]::Exists($path)) { [IO.File]::Delete($path) } }
    }
    exit 0
}

$actualEnvironment = @((Get-ChildItem Env: | Sort-Object Name | ForEach-Object {
    [ordered]@{ name = $_.Name; value_sha256 = Get-TextSha256 ([string]$_.Value) }
}))
if ((Get-TextSha256 (ConvertTo-CanonicalJson $actualEnvironment)) -ne $EnvironmentManifestSha256) { throw 'ENVIRONMENT_MISMATCH' }

function Invoke-CommandEvidence([string]$FileName, [string[]]$Arguments) {
    $stdoutPath = New-TempPath
    $stderrPath = New-TempPath
    try {
        $psi = [Diagnostics.ProcessStartInfo]::new()
        $psi.FileName = $FileName
        foreach ($argument in $Arguments) { [void]$psi.ArgumentList.Add($argument) }
        $psi.WorkingDirectory = $repoRoot
        $psi.UseShellExecute = $false
        $psi.CreateNoWindow = $true
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError = $true
        $process = [Diagnostics.Process]::new()
        $process.StartInfo = $psi
        if (-not $process.Start()) { throw 'COMMAND_START_FAILED' }
        if ($null -eq $script:modeBudget) { $script:modeBudget = [BastionReviewGate.OutputBudget]::new($process, $outputLimitBytes) } else { $script:modeBudget.Attach($process) }
        $stdoutTask = [BastionReviewGate.BoundedCapture]::ReadAsync($process.StandardOutput.BaseStream, $script:modeBudget)
        $stderrTask = [BastionReviewGate.BoundedCapture]::ReadAsync($process.StandardError.BaseStream, $script:modeBudget)
        $process.WaitForExit()
        $stdoutBytes = $stdoutTask.GetAwaiter().GetResult()
        $stderrBytes = $stderrTask.GetAwaiter().GetResult()
        $bytes = $stdoutBytes.Length + $stderrBytes.Length
        $script:modeOutputBytes += $bytes
        if ($script:modeOutputBytes -gt $outputLimitBytes) { throw 'MODE_OUTPUT_LIMIT_EXCEEDED' }
        [ordered]@{
            argv = @($FileName) + $Arguments
            exit_code = $process.ExitCode
            stdout_sha256 = Get-BytesSha256 $stdoutBytes
            stderr_sha256 = Get-BytesSha256 $stderrBytes
            output_bytes = $bytes
        }
    } finally {
        foreach ($path in @($stdoutPath,$stderrPath)) { if ([IO.File]::Exists($path)) { [IO.File]::Delete($path) } }
    }
}

$commands = @()
switch ($Mode) {
    'L0Format' { $commands += ,@('cargo','+1.95.0','fmt','--all','--','--check') }
    'L0Check' { $commands += ,@('cargo','+1.95.0','check','-p','bastion-review','--locked','--offline','--all-targets') }
    'L0FocusedTest' { $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--locked','--offline') }
    'L1WorkspaceCheck' { $commands += ,@('cargo','+1.95.0','check','--workspace','--locked','--offline','--all-targets') }
    'L1Clippy' { $commands += ,@('cargo','+1.95.0','clippy','--workspace','--locked','--offline','--all-targets','--','-D','warnings') }
    'L1Test' { $commands += ,@('cargo','+1.95.0','test','--workspace','--locked','--offline') }
    'L1Doc' {
        $commands += ,@('cargo','+1.95.0','doc','--workspace','--locked','--offline','--no-deps')
        $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--doc','--locked','--offline')
    }
    'L1Static' { $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--locked','--offline','static_surface') }
    'L1SupplyChain' { $commands += ,@('cargo','+1.95.0','metadata','--locked','--offline','--no-deps','--format-version','1') }
    'L2Contract' { $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--locked','--offline','contract_matrix') }
    'L2Model' { $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--locked','--offline','model_cases') }
    'L2Adversarial' { $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--locked','--offline','adversarial_cases') }
    'L2NoAuthority' { $commands += ,@('cargo','+1.95.0','test','-p','bastion-review','--locked','--offline','no_authority_surface') }
}
$implementationPath = Join-Path $repoRoot 'crates/bastion-review/src/lib.rs'
$implementationBefore = Get-FileSha256 $implementationPath
$manifestBefore = Get-FileSha256 (Join-Path $repoRoot 'Cargo.toml')
$lockBefore = Get-FileSha256 (Join-Path $repoRoot 'Cargo.lock')
$startedAt = [DateTimeOffset]::UtcNow
$timer = [Diagnostics.Stopwatch]::StartNew()
$script:modeOutputBytes = 0
$script:modeBudget = $null
$results = @()
foreach ($command in $commands) {
    $result = Invoke-CommandEvidence $command[0] $command[1..($command.Count - 1)]
    $results += $result
    if ($result.exit_code -ne 0) { throw "COMMAND_FAILED:$($result.argv -join ' ')" }
}
$manifest = Get-Content (Join-Path $repoRoot 'Cargo.toml') -Raw
$lock = Get-Content (Join-Path $repoRoot 'Cargo.lock') -Raw
if ($implementationBefore -ne (Get-FileSha256 $implementationPath) -or $manifestBefore -ne (Get-FileSha256 (Join-Path $repoRoot 'Cargo.toml')) -or $lockBefore -ne (Get-FileSha256 (Join-Path $repoRoot 'Cargo.lock'))) { throw 'SUBJECT_MUTATED_DURING_GATE' }
if ($Mode -eq 'L1Static') {
    $source = (Get-Content (Join-Path $repoRoot 'crates/bastion-review/src/lib.rs') -Raw).Split('#[cfg(test)]')[0]
    $forbidden = @('unsafe\s*\{','extern\s+"','std::fs','std::net','std::env','std::process','std::thread','\.unwrap\s*\(','\.expect\s*\(','todo!','unimplemented!','pub\s+\w+\s*:\s*(String|Vec<u8>)')
    foreach ($pattern in $forbidden) { if ($source -match $pattern) { throw "STATIC_SURFACE:$pattern" } }
    $allowedPaths = '^(Cargo\.toml|Cargo\.lock|crates/bastion-review/(Cargo\.toml|src/lib\.rs)|tools/review_gate\.ps1|context/waves/2026-07-28-bastion-foundation/evidence/wp-rev-001/[^/]+\.json)$'
    foreach ($line in @(& git -C $repoRoot status --porcelain=v1 -uall)) {
        $path = $line.Substring(3).Replace('\','/')
        if ($path -notmatch $allowedPaths) { throw "FORBIDDEN_CHANGED_PATH:$path" }
    }
    foreach ($path in @(& git -C $repoRoot ls-files --others --ignored --exclude-standard)) {
        $normalized = $path.Replace('\','/')
        if ($normalized -notmatch '^target/' -and $normalized -notmatch $allowedPaths) { throw "FORBIDDEN_IGNORED_PATH:$path" }
    }
    foreach ($path in @(& git -C $repoRoot diff --name-only fe3ac4b..HEAD)) {
        if ($path.Replace('\','/') -notmatch $allowedPaths) { throw "FORBIDDEN_COMMITTED_PATH:$path" }
    }
}
if ($Mode -eq 'L1SupplyChain') {
    $metadataResult = & cargo +1.95.0 metadata --locked --offline --no-deps --format-version 1 | ConvertFrom-Json
    $package = $metadataResult.packages[0]
    $crateManifest = Get-Content (Join-Path $repoRoot 'crates/bastion-review/Cargo.toml') -Raw
    if ($metadataResult.packages.Count -ne 1 -or $package.name -ne 'bastion-review' -or $package.version -ne '0.1.0' -or $package.edition -ne '2024' -or $package.rust_version -ne '1.95.0' -or $package.dependencies.Count -ne 0 -or @($package.features.PSObject.Properties).Count -ne 0 -or $package.targets.Count -ne 1 -or $package.targets[0].kind.Count -ne 1 -or $package.targets[0].kind[0] -ne 'lib' -or $lock -match '(?m)^source = ' -or $manifest -match '(?m)^build\s*=' -or $crateManifest -notmatch '(?m)^publish\s*=\s*false$' -or $crateManifest -notmatch '(?m)^workspace\s*=\s*true$') { throw 'SUPPLY_CHAIN_NOT_CLOSED' }
}
$assertions = @('command-exit-zero','sanitized-environment-exact','wall-limit-60s','job-memory-limit-1GiB','stream-output-limit-10MiB','implementation-and-governance-digests-bound')
$negativeCases = switch ($Mode) {
    'L1Static' { @('executed:static-regex-and-exact-path-allowlist') }
    'L1SupplyChain' { @('executed:cargo-metadata-manifest-lock-closure') }
    { $_ -like 'L2*' } { @("executed:cargo-test-filter:$Mode") }
    default { @() }
}
$record = [ordered]@{
    schema = 'review-gate-evidence.v1'
    repository = 'BASTION'
    work_package = 'WP-REV-001-R1'
    mode = $Mode
    result = 'pass'
    exit_code = 0
    started_at_utc = $startedAt.ToString('O')
    ended_at_utc = [DateTimeOffset]::UtcNow.ToString('O')
    duration_ms = $timer.ElapsedMilliseconds
    implementation_sha256 = $implementationBefore
    implementation_commit = (& git -C $repoRoot rev-parse HEAD).Trim()
    work_package_sha256 = Get-FileSha256 (Join-Path $repoRoot 'docs/vtrace/WP_REV_001.md')
    acceptance_sha256 = Get-FileSha256 (Join-Path $repoRoot 'context/waves/2026-07-28-bastion-foundation/pulses/pulse-14-wp-rev-001-acceptance.md')
    runner_sha256 = Get-FileSha256 $scriptPath
    manifest_sha256 = Get-TextSha256 $manifest
    lock_sha256 = Get-TextSha256 $lock
    toolchain = '1.95.0'
    sanitized_environment_sha256 = $EnvironmentManifestSha256
    bounds = [ordered]@{ wall_seconds = 60; process_tree_bytes = $memoryLimitBytes; output_bytes = $outputLimitBytes }
    commands = $results
    combined_output_bytes = $script:modeBudget.Bytes
    subject_assertions = @([ordered]@{ id = 'implementation-manifest-lock-immutable'; result = 'pass' })
    assertions = @($assertions | ForEach-Object { [ordered]@{ id = $_; result = 'pass' } })
    negative_cases = @($negativeCases | ForEach-Object { [ordered]@{ id = $_; result = 'pass'; command_filter = $Mode } })
}
$json = ConvertTo-CanonicalJson $record 16
$json
