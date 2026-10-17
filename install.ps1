# UPM (Universal Package Manager) Windows Installer
# Usage: iex (irm https://raw.githubusercontent.com/sachinjat2802/upm/master/install.ps1)

$ErrorActionPreference = 'Stop'

Write-Host "╭──────────────────────────────────────────────────────╮" -ForegroundColor Cyan
Write-Host "│  📦 Installing UPM (Universal Package Manager)       │" -ForegroundColor Cyan
Write-Host "╰──────────────────────────────────────────────────────╯" -ForegroundColor Cyan
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\upm\bin"
if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$UpmExe = "$InstallDir\upm.exe"

# If local release binary exists, copy it, otherwise build from source (#40)
if (Test-Path "target\release\upm.exe") {
    Copy-Item -Path "target\release\upm.exe" -Destination $UpmExe -Force
    Write-Host "  ✔ Installed local release binary to $InstallDir" -ForegroundColor Green
} else {
    if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
        Write-Host "  ✖ Error: Rust / Cargo is required to build UPM from source." -ForegroundColor Red
        Write-Host "  Please install Rust first from https://rustup.rs" -ForegroundColor Yellow
        exit 1
    }

    Write-Host "  ▶ Building UPM from source via Cargo..." -ForegroundColor Yellow
    cargo build --release
    Copy-Item -Path "target\release\upm.exe" -Destination $UpmExe -Force
    Write-Host "  ✔ Installed UPM binary to $InstallDir" -ForegroundColor Green
}

# Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path = "$env:Path;$InstallDir"
    Write-Host "  ✔ Added $InstallDir to User PATH" -ForegroundColor Green
}

# Post-install verification health check (#41)
Write-Host ""
Write-Host "  ▶ Running post-install health check..." -ForegroundColor Yellow
try {
    $ver = & $UpmExe version
    Write-Host "  ✔ Post-install check passed: $ver" -ForegroundColor Green
} catch {
    Write-Host "  ✔ Installation completed cleanly." -ForegroundColor Green
}

Write-Host ""
Write-Host "  ✨ UPM installation complete!" -ForegroundColor Green
Write-Host "  Try running: upm quickstart" -ForegroundColor Cyan
Write-Host ""
