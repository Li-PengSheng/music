param([switch]$Debug)

$ErrorActionPreference = "Stop"
$repo = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repo

$profileName = "release"
$tauriArgs = @("exec", "tauri", "build", "--no-bundle", "--config", "src-tauri/tauri.portable.conf.json")
if ($Debug) {
    $profileName = "debug"
    $tauriArgs += "--debug"
}

pnpm @tauriArgs
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$candidates = @(
    (Join-Path $repo "src-tauri\target\$profileName\music-mine-desktop.exe")
)
if ($env:CARGO_TARGET_DIR) {
    $candidates = @(
        (Join-Path $env:CARGO_TARGET_DIR "$profileName\music-mine-desktop.exe")
    ) + $candidates
}

$exe = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $exe) {
    throw "找不到便携版 EXE。已查找：$($candidates -join ', ')"
}

$stage = Join-Path $repo "dist-portable\Music Mine"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path $stage | Out-Null
Copy-Item $exe (Join-Path $stage "music-mine-desktop.exe")
Copy-Item (Join-Path $repo "portable\README.txt") (Join-Path $stage "README.txt")

$zip = Join-Path $repo "dist-portable\Music-Mine-Portable-x64.zip"
if (Test-Path $zip) { Remove-Item $zip -Force }
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip
Write-Output "PORTABLE_EXE=$exe"
Write-Output "PORTABLE_ZIP=$zip"
