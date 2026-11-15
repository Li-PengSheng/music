# Dot-source this file. It does not take parameters.
# Build provenance and path checks for isolated desktop runs.

$script:IsolationNativeReady = $false

function Initialize-IsolationNativePath {
    if ($script:IsolationNativeReady) { return }
    if (-not ('MusicMineNativePath' -as [type])) {
        Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
using System.Text;
public static class MusicMineNativePath {
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern IntPtr CreateFile(string name, uint access, uint share, IntPtr security, uint disposition, uint flags, IntPtr templateFile);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern uint GetFinalPathNameByHandle(IntPtr handle, StringBuilder path, uint capacity, uint flags);
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool CloseHandle(IntPtr handle);
}
"@
    }
    $script:IsolationNativeReady = $true
}

function ConvertFrom-ExtendedPath([string]$Path) {
    if ($Path.StartsWith('\\?\UNC\')) { return '\' + $Path.Substring(7) }
    if ($Path.StartsWith('\\?\')) { return $Path.Substring(4) }
    return $Path
}

function Get-FinalExistingPath([string]$Path) {
    Initialize-IsolationNativePath
    $full = [System.IO.Path]::GetFullPath($Path)
    $invalid = New-Object IntPtr -1
    $handle = [MusicMineNativePath]::CreateFile($full, 0, 7, [IntPtr]::Zero, 3, 0x02000000, [IntPtr]::Zero)
    if ($handle -eq $invalid -or $handle -eq [IntPtr]::Zero) {
        throw "cannot open path: $full"
    }
    try {
        $buffer = New-Object System.Text.StringBuilder 2048
        $count = [MusicMineNativePath]::GetFinalPathNameByHandle($handle, $buffer, [uint32]$buffer.Capacity, 0)
        if ($count -eq 0 -or $count -ge $buffer.Capacity) { throw "cannot resolve final path: $full" }
        return (ConvertFrom-ExtendedPath $buffer.ToString()).TrimEnd('\')
    } finally {
        [void][MusicMineNativePath]::CloseHandle($handle)
    }
}

function Get-FinalPath([string]$Path) {
    if ([string]::IsNullOrWhiteSpace($Path)) { throw "empty path" }
    $full = [System.IO.Path]::GetFullPath($Path)
    if (Test-Path -LiteralPath $full) { return (Get-FinalExistingPath $full) }
    $missing = New-Object System.Collections.Generic.List[string]
    $current = $full
    while ($current -and -not (Test-Path -LiteralPath $current)) {
        $name = [System.IO.Path]::GetFileName($current)
        if ($name -eq '.' -or $name -eq '..') { throw "unresolved segment in $Path" }
        $missing.Insert(0, $name)
        $parent = [System.IO.Path]::GetDirectoryName($current)
        if ([string]::IsNullOrEmpty($parent) -or $parent -eq $current) { throw "path does not exist: $Path" }
        $current = $parent
    }
    $base = Get-FinalExistingPath $current
    foreach ($segment in $missing) {
        if ($segment -eq '.' -or $segment -eq '..' -or $segment.IndexOfAny([char[]]@('\', '/')) -ge 0) {
            throw "unsafe path segment: $segment"
        }
        $base = [System.IO.Path]::GetFullPath((Join-Path $base $segment))
    }
    return $base.TrimEnd('\')
}

function Test-PathUnderRoot([string]$Candidate, [string]$Root) {
    $rootFinal = (Get-FinalPath $Root).TrimEnd('\')
    $candidateFinal = (Get-FinalPath $Candidate).TrimEnd('\')
    if ($candidateFinal.Equals($rootFinal, [System.StringComparison]::OrdinalIgnoreCase)) { return $true }
    $prefix = $rootFinal + '\'
    return $candidateFinal.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)
}

function Assert-DeletableUnderRoot([string]$Directory, [string]$Root) {
    if (-not (Test-PathUnderRoot $Directory $Root)) { throw "path escapes test root: $Directory" }
    $pending = New-Object System.Collections.Generic.Queue[string]
    $pending.Enqueue((Get-FinalPath $Directory))
    while ($pending.Count -gt 0) {
        $dir = $pending.Dequeue()
        foreach ($child in @(Get-ChildItem -LiteralPath $dir -Force -ErrorAction Stop)) {
            $reparse = [IO.FileAttributes]::ReparsePoint
            if (($child.Attributes -band $reparse) -eq $reparse) { throw "refusing to delete through a reparse point: $($child.FullName)" }
            if (-not (Test-PathUnderRoot $child.FullName $Root)) { throw "path escapes test root: $($child.FullName)" }
            if ($child.PSIsContainer) { $pending.Enqueue($child.FullName) }
        }
    }
}

function Get-Sha256Hex([string]$Path) {
    $sha = [System.Security.Cryptography.SHA256]::Create()
    $stream = [System.IO.File]::Open($Path, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
    try {
        $hash = $sha.ComputeHash($stream)
        return ([System.BitConverter]::ToString($hash)).Replace('-', '')
    } finally {
        $stream.Dispose()
        $sha.Dispose()
    }
}

function Test-PortableRelativeOverride([string]$Value) {
    if ([string]::IsNullOrWhiteSpace($Value)) { return $false }
    if ([System.IO.Path]::IsPathRooted($Value)) { return $false }
    if ($Value -match '^[A-Za-z]:' -or $Value.StartsWith('\\') -or $Value.StartsWith('/')) { return $false }
    $normal = ($Value -replace '\\', '/').Trim()
    while ($normal.StartsWith('./')) { $normal = $normal.Substring(2) }
    return $normal -eq 'app-data'
}

function Test-AllowlistedBuildPath([string]$Path, [string]$Repo) {
    $repoFull = [System.IO.Path]::GetFullPath($Repo)
    foreach ($relative in @('src-tauri\target', 'dist-portable')) {
        $root = Join-Path $repoFull $relative
        if (Test-Path -LiteralPath $root) {
            if (Test-PathUnderRoot $Path $root) { return $true }
        }
    }
    return $false
}

function Get-IsolationSourceDirectory([string]$Repo) {
    return (Join-Path ([System.IO.Path]::GetFullPath($Repo)) 'tests\perf\out\isolation-sources')
}

function Resolve-RecordedConfigPath([string]$Repo, [string]$ConfigPath) {
    if ([System.IO.Path]::IsPathRooted($ConfigPath)) { return [System.IO.Path]::GetFullPath($ConfigPath) }
    return [System.IO.Path]::GetFullPath((Join-Path $Repo ($ConfigPath -replace '/', '\')))
}

function Read-IsolationConfig([string]$Path) {
    $parsed = Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
    $top = @($parsed.PSObject.Properties.Name)
    if ($top.Count -ne 1 -or $top[0] -ne 'app') { throw "config may only contain app.appDirectoriesOverride" }
    $app = @($parsed.app.PSObject.Properties.Name)
    if ($app.Count -ne 1 -or $app[0] -ne 'appDirectoriesOverride') { throw "config may only contain app.appDirectoriesOverride" }
    return [string]$parsed.app.appDirectoriesOverride
}

function Write-IsolationSource {
    param(
        [Parameter(Mandatory)][string]$Repo,
        [Parameter(Mandatory)][string]$BuiltExe,
        [Parameter(Mandatory)][string]$ConfigPath,
        [Parameter(Mandatory)][string]$Command,
        [Parameter(Mandatory)][ValidateSet('portable-relative', 'absolute-override')][string]$DataDirKind,
        [Parameter(Mandatory)][string]$ExpectedDataDir,
        [string]$TestRoot = ''
    )
    if (-not (Test-AllowlistedBuildPath $BuiltExe $Repo)) {
        throw "build output is not under src-tauri\target or dist-portable: $BuiltExe"
    }
    $builtFull = Get-FinalPath $BuiltExe
    $configFull = Resolve-RecordedConfigPath $Repo $ConfigPath
    if (-not (Test-Path -LiteralPath $configFull)) { throw "config not found: $configFull" }
    $configFull = Get-FinalPath $configFull
    $override = Read-IsolationConfig $configFull
    if ($DataDirKind -eq 'portable-relative') {
        $portable = Get-FinalPath (Join-Path $Repo 'src-tauri\tauri.portable.conf.json')
        if (-not $configFull.Equals($portable, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "portable-relative records must use src-tauri/tauri.portable.conf.json"
        }
        if (-not (Test-PortableRelativeOverride $override)) { throw "portable override must be a relative app-data path" }
        if ($ExpectedDataDir -ne './app-data') { throw "portable expectedDataDir must be ./app-data" }
    } else {
        if (-not [System.IO.Path]::IsPathRooted($override)) { throw "absolute-override config must use an absolute path" }
        if ([string]::IsNullOrWhiteSpace($TestRoot)) { throw "absolute-override requires a test root" }
        $rootFinal = Get-FinalPath $TestRoot
        if ($rootFinal.Length -le 3) { throw "test root must be a dedicated directory, not a drive root" }
        $overrideFinal = Get-FinalPath $override
        $expectedFinal = Get-FinalPath $ExpectedDataDir
        if (-not $overrideFinal.Equals($expectedFinal, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "config override does not match expectedDataDir"
        }
        if (-not (Test-PathUnderRoot $expectedFinal $rootFinal)) { throw "expected data directory escapes the test root" }
    }
    $hash = Get-Sha256Hex $builtFull
    $record = [ordered]@{
        exeSha256 = $hash
        builtExePath = $builtFull
        configPath = $ConfigPath
        configSha256 = (Get-Sha256Hex $configFull)
        command = $Command
        dataDirKind = $DataDirKind
        expectedDataDir = $(if ($DataDirKind -eq 'portable-relative') { './app-data' } else { (Get-FinalPath $ExpectedDataDir) })
        testRoot = $(if ($DataDirKind -eq 'absolute-override') { (Get-FinalPath $TestRoot) } else { '' })
    }
    $dir = Get-IsolationSourceDirectory $Repo
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $dest = Join-Path $dir ($hash + '.json')
    $json = $record | ConvertTo-Json -Compress
    [System.IO.File]::WriteAllText($dest, $json)
    return $dest
}

function Read-IsolationSource([string]$Repo, [string]$ExePath) {
    $hash = Get-Sha256Hex $ExePath
    $path = Join-Path (Get-IsolationSourceDirectory $Repo) ($hash + '.json')
    if (-not (Test-Path -LiteralPath $path)) { throw "MISSING_RECORD $hash" }
    $record = Get-Content -LiteralPath $path -Raw | ConvertFrom-Json
    return $record
}

function Test-IsolationSource {
    param(
        [Parameter(Mandatory)][string]$Repo,
        [Parameter(Mandatory)][string]$ExePath
    )
    $record = Read-IsolationSource $Repo $ExePath
    foreach ($field in @('exeSha256', 'builtExePath', 'configPath', 'configSha256', 'command', 'dataDirKind', 'expectedDataDir')) {
        if ([string]::IsNullOrWhiteSpace([string]$record.$field)) { throw "record missing $field" }
    }
    $launchedHash = Get-Sha256Hex $ExePath
    if ($launchedHash -ne ([string]$record.exeSha256).ToUpperInvariant()) { throw "HASH_MISMATCH" }
    if (-not (Test-AllowlistedBuildPath ([string]$record.builtExePath) $Repo)) { throw "BUILT_PATH_NOT_ALLOWLISTED" }
    if ((Get-Sha256Hex ([string]$record.builtExePath)) -ne $launchedHash) { throw "BUILT_FILE_HASH_MISMATCH" }
    $configFull = Resolve-RecordedConfigPath $Repo ([string]$record.configPath)
    if (-not (Test-Path -LiteralPath $configFull)) { throw "config not found: $configFull" }
    $configFull = Get-FinalPath $configFull
    if ((Get-Sha256Hex $configFull) -ne ([string]$record.configSha256).ToUpperInvariant()) { throw "CONFIG_HASH_MISMATCH" }
    $mainConfig = Get-FinalPath (Join-Path $Repo 'src-tauri\tauri.conf.json')
    if ($configFull.Equals($mainConfig, [System.StringComparison]::OrdinalIgnoreCase)) { throw "REFUSING_MAIN_CONFIG" }
    $override = Read-IsolationConfig $configFull
    if ([string]$record.dataDirKind -eq 'portable-relative') {
        $portable = Get-FinalPath (Join-Path $Repo 'src-tauri\tauri.portable.conf.json')
        if (-not $configFull.Equals($portable, [System.StringComparison]::OrdinalIgnoreCase)) { throw "PORTABLE_CONFIG_PATH" }
        if (-not (Test-PortableRelativeOverride $override)) { throw "PORTABLE_OVERRIDE_NOT_RELATIVE" }
        if ([string]$record.expectedDataDir -ne './app-data') { throw "PORTABLE_EXPECTED_DIR" }
    } elseif ([string]$record.dataDirKind -eq 'absolute-override') {
        if (-not [System.IO.Path]::IsPathRooted($override)) { throw "ABSOLUTE_OVERRIDE_REQUIRED" }
        if ([string]::IsNullOrWhiteSpace([string]$record.testRoot)) { throw "TEST_ROOT_MISSING" }
        $overrideFinal = Get-FinalPath $override
        $expectedFinal = Get-FinalPath ([string]$record.expectedDataDir)
        $rootFinal = Get-FinalPath ([string]$record.testRoot)
        if (-not $overrideFinal.Equals($expectedFinal, [System.StringComparison]::OrdinalIgnoreCase)) { throw "OVERRIDE_EXPECTED_MISMATCH" }
        if (-not (Test-PathUnderRoot $expectedFinal $rootFinal)) { throw "DATA_DIR_ESCAPES_TEST_ROOT" }
    } else {
        throw "UNKNOWN_DATA_DIR_KIND"
    }
    return $record
}

function Assert-PortableLaunchSource {
    param([Parameter(Mandatory)][string]$Repo, [Parameter(Mandatory)][string]$ExePath)
    $record = Test-IsolationSource -Repo $Repo -ExePath $ExePath
    if ([string]$record.dataDirKind -ne 'portable-relative') { throw "stage-cover-run only launches portable-relative builds" }
    return $record
}

function ConvertTo-StartToken($CreationDate) {
    $value = $CreationDate
    if ($value -isnot [datetime]) {
        $value = [System.Management.ManagementDateTimeConverter]::ToDateTime([string]$CreationDate)
    }
    return $value.ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
}

function Get-DescendantIds($Items, [int]$RootPid) {
    $children = @{}
    foreach ($item in @($Items)) {
        $parent = [int]$item.ParentProcessId
        if (-not $children.ContainsKey($parent)) { $children[$parent] = New-Object System.Collections.Generic.List[int] }
        $children[$parent].Add([int]$item.ProcessId)
    }
    $found = New-Object System.Collections.Generic.List[int]
    $queue = New-Object System.Collections.Generic.Queue[int]
    $seen = @{}
    $queue.Enqueue($RootPid)
    $seen[$RootPid] = $true
    while ($queue.Count -gt 0) {
        $id = $queue.Dequeue()
        if (-not $children.ContainsKey($id)) { continue }
        foreach ($child in @($children[$id])) {
            if ($seen.ContainsKey($child)) { continue }
            $seen[$child] = $true
            $found.Add($child)
            $queue.Enqueue($child)
        }
    }
    return @($found)
}

function Test-AncestorOfProcess($Items, [int]$ProcessId, [int]$RootPid) {
    $byId = @{}
    foreach ($item in @($Items)) { $byId[[int]$item.ProcessId] = $item }
    $current = $ProcessId
    for ($guard = 0; $guard -lt 50; $guard++) {
        if (-not $byId.ContainsKey($current)) { return $false }
        $parent = [int]$byId[$current].ParentProcessId
        if ($parent -eq $RootPid) { return $true }
        if ($parent -eq 0 -or $parent -eq $current) { return $false }
        $current = $parent
    }
    return $false
}

function Get-LiveProcessTable {
    return @(Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId, ExecutablePath, CreationDate)
}

function Test-RecordedProcessStillMatches($Record, $Process) {
    if (-not $Process -or -not $Process.ExecutablePath -or -not $Process.CreationDate) { return $false }
    $start = ConvertTo-StartToken $Process.CreationDate
    if ($start -ne [string]$Record.startTime) { return $false }
    $actual = Get-FinalPath ([string]$Process.ExecutablePath)
    $expected = Get-FinalPath ([string]$Record.exePath)
    return $actual.Equals($expected, [System.StringComparison]::OrdinalIgnoreCase)
}

function Stop-RecordedPlayer {
    param([Parameter(Mandatory)][string]$RecordPath)
    if (-not (Test-Path -LiteralPath $RecordPath)) { return }
    $record = Get-Content -LiteralPath $RecordPath -Raw | ConvertFrom-Json
    $processId = [int]$record.pid
    if ($processId -le 0) { throw "process record has no pid" }
    $live = Get-CimInstance Win32_Process -Filter "ProcessId=$processId"
    if (-not $live) { return }
    if (-not (Test-RecordedProcessStillMatches $record $live)) {
        Write-Output "SKIP_IDENTITY"
        return
    }
    $main = Get-Process -Id $processId -ErrorAction SilentlyContinue
    if ($main) { [void]$main.CloseMainWindow() }
    $deadline = (Get-Date).AddSeconds(5)
    do {
        Start-Sleep -Milliseconds 200
        $live = Get-CimInstance Win32_Process -Filter "ProcessId=$processId"
        if (-not $live) { return }
        if (-not (Test-RecordedProcessStillMatches $record $live)) {
            Write-Output "SKIP_IDENTITY"
            return
        }
    } while ((Get-Date) -lt $deadline -and $live)

    $live = Get-CimInstance Win32_Process -Filter "ProcessId=$processId"
    if (-not $live -or -not (Test-RecordedProcessStillMatches $record $live)) {
        Write-Output "SKIP_IDENTITY"
        return
    }
    $table = Get-LiveProcessTable
    $descendants = @(Get-DescendantIds $table $processId)
    [array]::Reverse($descendants)
    foreach ($childId in $descendants) {
        $table = Get-LiveProcessTable
        $root = $null
        foreach ($item in $table) { if ([int]$item.ProcessId -eq $processId) { $root = $item } }
        if (-not (Test-RecordedProcessStillMatches $record $root)) {
            Write-Output "SKIP_IDENTITY"
            return
        }
        if (Test-AncestorOfProcess $table $childId $processId) {
            Stop-Process -Id $childId -Force -ErrorAction SilentlyContinue
        }
    }
    $table = Get-LiveProcessTable
    $root = $null
    foreach ($item in $table) { if ([int]$item.ProcessId -eq $processId) { $root = $item } }
    if ($root -and (Test-RecordedProcessStillMatches $record $root)) {
        Stop-Process -Id $processId -Force -ErrorAction SilentlyContinue
    }
}

function Write-RunProcessRecord {
    param(
        [Parameter(Mandatory)][string]$RecordPath,
        [Parameter(Mandatory)][int]$ProcessId,
        [Parameter(Mandatory)][string]$ExePath
    )
    $live = Get-CimInstance Win32_Process -Filter "ProcessId=$ProcessId"
    if (-not $live -or -not $live.ExecutablePath) {
        $started = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if ($started -and $started.Path -and (Get-FinalPath $started.Path).Equals((Get-FinalPath $ExePath), [System.StringComparison]::OrdinalIgnoreCase)) {
            Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
        }
        throw "cannot record process identity for pid $ProcessId"
    }
    $actual = Get-FinalPath ([string]$live.ExecutablePath)
    $expected = Get-FinalPath $ExePath
    if (-not $actual.Equals($expected, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "refusing to record pid $ProcessId; executable does not match $expected"
    }
    $payload = [ordered]@{
        pid = $ProcessId
        startTime = (ConvertTo-StartToken $live.CreationDate)
        exePath = $actual
    }
    $json = $payload | ConvertTo-Json -Compress
    [System.IO.File]::WriteAllText($RecordPath, $json)
}
