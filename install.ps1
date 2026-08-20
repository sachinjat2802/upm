# CPM (Cross-language Package Manager) Windows Installer
# Usage: iex (irm https://raw.githubusercontent.com/sachinjat2802/cpm/master/install.ps1)

$ErrorActionPreference = 'Stop'

Write-Host "╭──────────────────────────────────────────────────────╮" -ForegroundColor Cyan
Write-Host "│  📦 Installing CPM (Cross-language Package Manager)  │" -ForegroundColor Cyan
Write-Host "╰──────────────────────────────────────────────────────╯" -ForegroundColor Cyan
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\cpm\bin"
if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$CpmExe = "$InstallDir\cpm.exe"
$UpmExe = "$InstallDir\upm.exe"

# If local release binary exists, copy it, otherwise build/download
if (Test-Path "target\release\cpm.exe") {
    Copy-Item -Path "target\release\cpm.exe" -Destination $CpmExe -Force
    Copy-Item -Path "target\release\upm.exe" -Destination $UpmExe -Force
    Write-Host "  ✔ Installed local release binaries to $InstallDir" -ForegroundColor Green
} else {
    Write-Host "  ▶ Building CPM from source via Cargo..." -ForegroundColor Yellow
    cargo build --release
    Copy-Item -Path "target\release\cpm.exe" -Destination $CpmExe -Force
    Copy-Item -Path "target\release\upm.exe" -Destination $UpmExe -Force
    Write-Host "  ✔ Installed CPM binaries to $InstallDir" -ForegroundColor Green
}

# Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path = "$env:Path;$InstallDir"
    Write-Host "  ✔ Added $InstallDir to User PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "  ✨ CPM installation complete!" -ForegroundColor Green
Write-Host "  Try running: cpm --version" -ForegroundColor Cyan
Write-Host ""
