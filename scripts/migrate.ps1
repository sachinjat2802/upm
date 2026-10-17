# UPM Automated 1-Command Repository Migration Script (Windows PowerShell)
# Usage: iex (irm https://raw.githubusercontent.com/sachinjat2802/upm/master/scripts/migrate.ps1)

$ErrorActionPreference = 'Stop'

Write-Host "╭──────────────────────────────────────────────────────╮" -ForegroundColor Cyan
Write-Host "│  ⚡ UPM Automated 1-Command Repository Migration     │" -ForegroundColor Cyan
Write-Host "╰──────────────────────────────────────────────────────╯" -ForegroundColor Cyan
Write-Host ""

# Ensure UPM binary is installed
if (!(Get-Command "upm" -ErrorAction SilentlyContinue)) {
    Write-Host "  ▶ Installing UPM runtime..." -ForegroundColor Yellow
    iex (irm https://raw.githubusercontent.com/sachinjat2802/upm/master/install.ps1)
}

# Run upm migrate command
upm migrate .
