# Restart and replaced-cover checks for builds that already have isolation source records.
# Stops only the pid recorded for this label. It does not stop music-mine-desktop by name.
param([string[]]$Labels = @("before", "after"), [switch]$SkipWarm, [switch]$RecoverOnly)
$ErrorActionPreference = "Stop"
$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$origin = (Get-Location).Path
Set-Location $repo
. (Join-Path $repo "scripts\isolation-lib.ps1")

function Stop-Label([string]$Label) {
    if ($Label -notmatch '^[A-Za-z0-9._-]+$') { throw "label must be a simple token" }
    $corpus = Join-Path $env:TEMP "mm-cover-corpus"
    if (-not (Test-Path -LiteralPath $corpus)) { return }
    if (-not (Test-PathUnderRoot $corpus (Get-FinalPath $env:TEMP))) { throw "corpus escapes TEMP" }
    $run = Join-Path $corpus "run-$Label"
    if (-not (Test-PathUnderRoot $run $corpus)) { throw "run directory escapes the corpus" }
    Stop-RecordedPlayer -RecordPath (Join-Path $run "process.json")
}

function Run-Label($Label, [switch]$Keep, $Scenarios) {
    Stop-Label $Label
    $exe = Join-Path $repo "tests\perf\out\bin\cover-$Label.exe"
    $stageArgs = @(
        "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", (Join-Path $PSScriptRoot "stage-cover-run.ps1"),
        "-Label", $Label, "-Exe", $exe
    )
    if ($Keep) { $stageArgs += "-KeepData" }
    & powershell.exe @stageArgs
    if ($LASTEXITCODE -ne 0) { throw "stage-cover-run failed: $LASTEXITCODE" }
    Start-Sleep 6
    Write-Output "== $Label $Scenarios"
    & node (Join-Path $repo "tests\perf\queue-cover-profile.mjs") $Label $Scenarios
    if ($LASTEXITCODE -ne 0) { throw "queue-cover-profile failed: $LASTEXITCODE" }
}

$failed = $null
try {
    if (-not $RecoverOnly) {
        foreach ($label in $Labels) {
            if (-not $SkipWarm) { Run-Label $label -Scenarios "A,C" }
            Run-Label $label -Keep -Scenarios "R,S"
        }
    }
    $env:MM_COVER_CORPUS = Join-Path $env:TEMP "mm-cover-corpus"
    $env:MM_COVER_CORPUS_RECOVER = "1"
    Set-Location (Join-Path $repo "src-tauri")
    & cargo test --release --lib cover_corpus -- --ignored --nocapture
    if ($LASTEXITCODE -ne 0) { throw "cargo test failed: $LASTEXITCODE" }
    Set-Location $repo
    foreach ($label in $Labels) { Run-Label $label -Keep -Scenarios "S" }
} catch {
    $failed = $_
} finally {
    Set-Location $origin
    foreach ($label in $Labels) {
        try { Stop-Label $label } catch { Write-Output "STOP_FAILED $label $($_.Exception.Message)" }
    }
    Remove-Item Env:MM_COVER_CORPUS_RECOVER -ErrorAction SilentlyContinue
}
if ($failed) { throw $failed }
