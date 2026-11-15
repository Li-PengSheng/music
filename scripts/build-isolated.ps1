param(
    [Parameter(Mandatory)][string]$Config,
    [Parameter(Mandatory)][string]$TestRoot
)
$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
Set-Location $repo
. (Join-Path $PSScriptRoot 'isolation-lib.ps1')

$configFull = [System.IO.Path]::GetFullPath($Config)
if (-not (Test-Path -LiteralPath $configFull)) { throw "config not found: $configFull" }
$mainConfig = Get-FinalPath (Join-Path $repo 'src-tauri\tauri.conf.json')
if ($configFull.Equals($mainConfig, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "refusing to build with src-tauri/tauri.conf.json"
}
$repoFull = $repo.Path.TrimEnd('\')
$repoPrefix = $repoFull + '\'
if ($configFull.StartsWith($repoPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
    $relative = $configFull.Substring($repoPrefix.Length)
    & git -C $repo ls-files --error-unmatch -- $relative 1>$null 2>$null
    if ($LASTEXITCODE -eq 0) { throw "isolation config must be untracked: $configFull" }
}

$override = Read-IsolationConfig $configFull
if (-not [System.IO.Path]::IsPathRooted($override)) { throw "appDirectoriesOverride must be absolute" }
$rootFinal = Get-FinalPath $TestRoot
$tempFinal = Get-FinalPath $env:TEMP
if (-not (Test-PathUnderRoot $rootFinal $tempFinal)) { throw "test root must stay under TEMP" }
$dataFinal = Get-FinalPath $override
if (-not (Test-PathUnderRoot $dataFinal $rootFinal)) { throw "override escapes the test root" }

$command = "pnpm exec tauri build --debug --no-bundle --config `"$configFull`""
& pnpm exec tauri build --debug --no-bundle --config $configFull
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$exe = Join-Path $repo 'src-tauri\target\debug\music-mine-desktop.exe'
if (-not (Test-Path -LiteralPath $exe)) { throw "debug exe was not produced: $exe" }
$dest = Write-IsolationSource -Repo $repo -BuiltExe $exe -ConfigPath $configFull -Command $command -DataDirKind 'absolute-override' -ExpectedDataDir $dataFinal -TestRoot $rootFinal
Write-Output "ISOLATED_EXE=$exe"
Write-Output "ISOLATION_SOURCE=$dest"
