param(
    [string]$Repo = "",
    [string[]]$Exclude = @()
)
$ErrorActionPreference = "Stop"
if (-not $Repo) { $Repo = (Get-Location).Path }
$Repo = (Resolve-Path -LiteralPath $Repo).Path

function Get-GitText([string[]]$GitArgs) {
    $lines = & git -C $Repo -c color.ui=false -c color.diff=false -c core.quotepath=false -c i18n.logOutputEncoding=utf-8 @GitArgs
    if ($LASTEXITCODE -ne 0) { throw "git failed: $($GitArgs -join ' ')" }
    if ($null -eq $lines) { return "" }
    if ($lines -is [string]) { return $lines + "`n" }
    return (($lines | ForEach-Object { $_ }) -join "`n") + "`n"
}

$buffer = New-Object System.IO.MemoryStream
function Add-Bytes([byte[]]$Bytes) {
    if ($null -eq $Bytes -or $Bytes.Length -eq 0) { return }
    $buffer.Write($Bytes, 0, $Bytes.Length)
}
function Add-Text([string]$Text) {
    Add-Bytes ([System.Text.Encoding]::UTF8.GetBytes($Text))
}

$skip = @{}
foreach ($item in @($Exclude)) {
    if ([string]::IsNullOrWhiteSpace($item)) { continue }
    $key = ($item -replace '\\', '/').Trim()
    while ($key.StartsWith('./')) { $key = $key.Substring(2) }
    $key = $key.TrimStart('/')
    $skip[$key] = $true
}

Add-Text "music-mine-worktree-v1`n"
Add-Text "staged`n"
Add-Text (Get-GitText @("diff", "--cached"))
Add-Text "unstaged`n"
Add-Text (Get-GitText @("diff"))
Add-Text "untracked`n"

$listed = & git -C $Repo -c core.quotepath=false ls-files --others --exclude-standard
if ($LASTEXITCODE -ne 0) { throw "git ls-files failed" }
$paths = @()
if ($null -ne $listed) {
    if ($listed -is [string]) { $paths = @($listed) }
    else { $paths = @($listed) }
}
$paths = @($paths | ForEach-Object { ($_ -replace '\\', '/') } | Where-Object { $_ -and -not $skip.ContainsKey($_) } | Sort-Object { $_ })
foreach ($relative in $paths) {
    $full = Join-Path $Repo ($relative -replace '/', '\')
    if (-not (Test-Path -LiteralPath $full -PathType Leaf)) { continue }
    $bytes = [System.IO.File]::ReadAllBytes($full)
    Add-Text ($relative + "`n" + $bytes.Length + "`n")
    Add-Bytes $bytes
    Add-Text "`n"
}

$buffer.Position = 0
$sha = [System.Security.Cryptography.SHA256]::Create()
try {
    $hash = $sha.ComputeHash($buffer)
} finally {
    $sha.Dispose()
    $buffer.Dispose()
}
Write-Output (([System.BitConverter]::ToString($hash)).Replace('-', '').ToLowerInvariant())
