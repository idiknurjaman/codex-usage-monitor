[CmdletBinding()]
param(
    [switch]$RemoveSettings,
    [switch]$Quiet
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$InstallDirectory = Join-Path $env:LOCALAPPDATA 'CodexUsage'
$ExpectedInstallDirectory = [IO.Path]::GetFullPath($InstallDirectory).TrimEnd('\')
$TargetPath = Join-Path $ExpectedInstallDirectory 'codex-usage.exe'
$LegacyInstallDirectory = [IO.Path]::GetFullPath((Join-Path $env:LOCALAPPDATA 'Programs\CodexUsage')).TrimEnd('\')
$ManagedInstallNames = @(
    'codex-usage.exe',
    'uninstall.ps1',
    'codex-usage.exe.new',
    'codex-usage.exe.old'
)
$ShortcutPath = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\Codex Usage.lnk'
$DesktopShortcutPath = Join-Path ([Environment]::GetFolderPath('Desktop')) 'Codex Usage.lnk'
$UninstallKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexUsage'
$RunKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
$SettingsDirectory = Join-Path $env:APPDATA 'CodexUsage'

$AllowedRoot = [IO.Path]::GetFullPath($env:LOCALAPPDATA).TrimEnd('\')
if (-not [String]::Equals($ExpectedInstallDirectory, (Join-Path $AllowedRoot 'CodexUsage'), [StringComparison]::OrdinalIgnoreCase)) {
    throw "Refusing to remove unexpected install directory: $ExpectedInstallDirectory"
}

function Stop-InstalledProcess {
    param(
        [Parameter(Mandatory = $true)][string]$ExecutablePath
    )

    Get-Process -Name 'codex-usage' -ErrorAction SilentlyContinue |
        ForEach-Object {
            try {
                if ([String]::Equals($_.Path, $ExecutablePath, [StringComparison]::OrdinalIgnoreCase)) {
                    Stop-Process -Id $_.Id -Force
                }
            }
            catch {
                # Fall through to the CIM lookup below when Process.Path is unavailable.
            }
        }

    Get-CimInstance Win32_Process -Filter "Name='codex-usage.exe'" -ErrorAction SilentlyContinue |
        Where-Object { [String]::Equals($_.ExecutablePath, $ExecutablePath, [StringComparison]::OrdinalIgnoreCase) } |
        ForEach-Object { Stop-Process -Id $_.ProcessId -Force }
}

function Remove-LegacyManagedInstall {
    if (-not (Test-Path -LiteralPath $LegacyInstallDirectory -PathType Container)) {
        return $true
    }

    $unexpected = @(Get-ChildItem -LiteralPath $LegacyInstallDirectory -Force -ErrorAction Stop |
        Where-Object { $_.Name -notin $ManagedInstallNames })
    if ($unexpected.Count -gt 0) {
        if (-not $Quiet) {
            Write-Warning "Legacy install directory was not removed because it contains unexpected entries: $($unexpected.FullName -join ', ')"
        }
        return $false
    }

    Remove-Item -LiteralPath $LegacyInstallDirectory -Recurse -Force
    return -not (Test-Path -LiteralPath $LegacyInstallDirectory -PathType Container)
}

Stop-InstalledProcess -ExecutablePath $TargetPath
if ($LegacyInstallDirectory -ne $ExpectedInstallDirectory) {
    Stop-InstalledProcess -ExecutablePath (Join-Path $LegacyInstallDirectory 'codex-usage.exe')
}

if (Test-Path -LiteralPath $RunKey) {
    Remove-ItemProperty -Path $RunKey -Name 'CodexUsage' -ErrorAction SilentlyContinue
    Remove-ItemProperty -Path $RunKey -Name 'ClaudeCodeUsageMonitor' -ErrorAction SilentlyContinue
}

Remove-Item -LiteralPath $ShortcutPath -Force -ErrorAction SilentlyContinue
Remove-Item -LiteralPath $DesktopShortcutPath -Force -ErrorAction SilentlyContinue
Remove-Item -LiteralPath $UninstallKey -Recurse -Force -ErrorAction SilentlyContinue

if ($RemoveSettings) {
    $ExpectedSettingsDirectory = [IO.Path]::GetFullPath((Join-Path $env:APPDATA 'CodexUsage')).TrimEnd('\')
    $ResolvedSettingsDirectory = [IO.Path]::GetFullPath($SettingsDirectory).TrimEnd('\')
    if ($ResolvedSettingsDirectory -eq $ExpectedSettingsDirectory) {
        Remove-Item -LiteralPath $ResolvedSettingsDirectory -Recurse -Force -ErrorAction SilentlyContinue
    }
}

foreach ($name in $ManagedInstallNames) {
    Remove-Item -LiteralPath (Join-Path $ExpectedInstallDirectory $name) -Force -ErrorAction SilentlyContinue
}

if (Test-Path -LiteralPath $ExpectedInstallDirectory -PathType Container) {
    $remaining = @(Get-ChildItem -LiteralPath $ExpectedInstallDirectory -Force -ErrorAction Stop)
    if ($remaining.Count -eq 0) {
        Remove-Item -LiteralPath $ExpectedInstallDirectory -Force
    }
}

$LegacyCleanupSucceeded = Remove-LegacyManagedInstall

if (-not $Quiet) {
    Write-Output 'Codex Usage was uninstalled.'
    if (-not $RemoveSettings) {
        Write-Output "Settings were preserved at $SettingsDirectory"
    }
    Write-Output "Legacy managed install cleanup: $LegacyCleanupSucceeded"
}
