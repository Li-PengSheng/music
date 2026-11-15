param(
    [Parameter(Mandatory)][ValidateSet('test-path', 'test-delete', 'check-source', 'write-source', 'descendants')][string]$Action,
    [string]$Candidate = '',
    [string]$Root = '',
    [string]$Repo = '',
    [string]$Exe = '',
    [string]$BuiltExe = '',
    [string]$Config = '',
    [string]$Command = '',
    [string]$Kind = '',
    [string]$Expected = '',
    [string]$TestRoot = '',
    [string]$Table = '',
    [int]$ProcessId = 0
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'isolation-lib.ps1')
try {
    switch ($Action) {
        'test-path' {
            if (Test-PathUnderRoot $Candidate $Root) { Write-Output 'OK'; exit 0 }
            Write-Output 'REJECT'
            exit 1
        }
        'test-delete' {
            Assert-DeletableUnderRoot $Candidate $Root
            Write-Output 'OK'
            exit 0
        }
        'write-source' {
            $dest = Write-IsolationSource -Repo $Repo -BuiltExe $BuiltExe -ConfigPath $Config -Command $Command -DataDirKind $Kind -ExpectedDataDir $Expected -TestRoot $TestRoot
            Write-Output $dest
            exit 0
        }
        'check-source' {
            [void](Test-IsolationSource -Repo $Repo -ExePath $Exe)
            Write-Output 'OK'
            exit 0
        }
        'descendants' {
            $items = Get-Content -LiteralPath $Table -Raw | ConvertFrom-Json
            $ids = @(Get-DescendantIds $items $ProcessId)
            if ($ids.Count -eq 0) { Write-Output '' } else { $ids | ForEach-Object { Write-Output $_ } }
            exit 0
        }
    }
} catch {
    Write-Output $_.Exception.Message
    exit 1
}
