$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$origin = (Get-Location).Path
$code = 0
try {
    Set-Location $root
    & pnpm check
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "pnpm check exited $code" }
    & pnpm test
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "pnpm test exited $code" }
    Set-Location (Join-Path $root "src-tauri")
    & cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "cargo fmt exited $code" }
    & cargo clippy --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "cargo clippy exited $code" }
    # GitHub's Windows runner access-violates inside WASAPI when tests open real devices.
    $env:MUSIC_MINE_NULL_OUTPUT = "1"
    & cargo test
    if ($LASTEXITCODE -ne 0) { $code = $LASTEXITCODE; throw "cargo test exited $code" }
} catch {
    if ($code -eq 0) { $code = 1 }
    Write-Host $_.Exception.Message
} finally {
    Set-Location $origin
}
exit $code
