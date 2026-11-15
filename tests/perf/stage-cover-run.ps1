# Stage a portable exe next to a fresh copy of the generated cover corpus and launch it.
# The exe must have a portable-relative isolation source record. This script does not
# accept an arbitrary build, and it does not stop processes by name.
param([Parameter(Mandatory)][string]$Label, [Parameter(Mandatory)][string]$Exe, [switch]$KeepData, [int]$Port = 9333)
$ErrorActionPreference = "Stop"
$repo = Resolve-Path (Join-Path $PSScriptRoot "..\..")
. (Join-Path $repo "scripts\isolation-lib.ps1")

if ($Label -notmatch '^[A-Za-z0-9._-]+$') { throw "label must be a simple token" }
if (-not (Test-Path -LiteralPath $Exe)) { throw "exe not found: $Exe" }
[void](Assert-PortableLaunchSource -Repo $repo -ExePath $Exe)

$corpus = Join-Path $env:TEMP "mm-cover-corpus"
$tempFinal = Get-FinalPath $env:TEMP
if (-not (Test-Path -LiteralPath $corpus)) { throw "corpus missing: run the cover_corpus test first" }
if (-not (Test-PathUnderRoot $corpus $tempFinal)) { throw "corpus escapes TEMP" }
$pristine = Join-Path $corpus "app-data"
if (-not (Test-Path (Join-Path $pristine "library-tauri-v1.sqlite"))) { throw "corpus missing: run the cover_corpus test first" }

$run = Join-Path $corpus "run-$Label"
if (-not (Test-PathUnderRoot $run $corpus)) { throw "run directory escapes the corpus" }
if (-not $KeepData -and (Test-Path -LiteralPath $run)) {
    Assert-DeletableUnderRoot $run $corpus
    Remove-Item -LiteralPath $run -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $run | Out-Null
$stagedExe = Join-Path $run "music-mine-desktop.exe"
Copy-Item -LiteralPath $Exe -Destination $stagedExe -Force
if ((Get-Sha256Hex $stagedExe) -ne (Get-Sha256Hex $Exe)) { throw "staged exe hash does not match the build source" }
if (-not (Test-Path (Join-Path $run "app-data"))) { Copy-Item $pristine (Join-Path $run "app-data") -Recurse }
if (-not (Test-PathUnderRoot (Join-Path $run "app-data") $corpus)) { throw "staged app-data escapes the corpus" }

$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$Port"
$process = Start-Process $stagedExe -WorkingDirectory $run -PassThru
Write-RunProcessRecord -RecordPath (Join-Path $run "process.json") -ProcessId $process.Id -ExePath $stagedExe
Write-Output "RUN_DIR=$run"
Write-Output "PID=$($process.Id)"
