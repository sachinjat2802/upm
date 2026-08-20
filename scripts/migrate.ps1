# CPM Automated 1-Command Repository Migration Script (Windows PowerShell)
# Usage: iex (irm https://raw.githubusercontent.com/sachinjat2802/cpm/master/scripts/migrate.ps1)

$ErrorActionPreference = 'Stop'

Write-Host "╭──────────────────────────────────────────────────────╮" -ForegroundColor Cyan
Write-Host "│  ⚡ CPM Automated 1-Command Repository Migration     │" -ForegroundColor Cyan
Write-Host "╰──────────────────────────────────────────────────────╯" -ForegroundColor Cyan
Write-Host ""

# Ensure CPM binary is installed
if (!(Get-Command "cpm" -ErrorAction SilentlyContinue)) {
    Write-Host "  ▶ Installing CPM runtime..." -ForegroundColor Yellow
    iex (irm https://raw.githubusercontent.com/sachinjat2802/cpm/master/install.ps1)
}

# Run cpm migrate command
cpm migrate .
