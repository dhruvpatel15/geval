# Geval Examples Runner
# This script executes various Geval scenarios to demonstrate its capabilities.

$geval_bin = ".\target\release\geval.exe"

if (-not (Test-Path $geval_bin)) {
    Write-Host "Error: Geval binary not found at $geval_bin. Please run 'cargo build --release' first." -ForegroundColor Red
    exit 1
}

function Run-Scenario($name, $contract, $signals) {
    Write-Host "`n--- Scenario: $name ---" -ForegroundColor Cyan
    Write-Host "Command: geval check --contract $contract --signals $signals" -ForegroundColor Gray
    & $geval_bin check --contract $contract --signals $signals
    & $geval_bin explain --contract $contract --signals $signals
}

Write-Host "===========================" -ForegroundColor Yellow
Write-Host "   GEVAL EXAMPLES RUNNER   " -ForegroundColor Yellow
Write-Host "===========================" -ForegroundColor Yellow

# 1. Multi-Policy Demo
Run-Scenario "Multi-Policy (Quality + Safety)" "examples/contract.yaml" "examples/signals.json"

# 2. Reconciliation Demo (Priority 1 wins)
Run-Scenario "Reconciliation (Manual Override wins over Toxicity Block)" "examples/reconciliation_demo.yaml" "examples/reconciliation_signals.json"

# 3. Edge Cases (Presence-only and First-win)
Run-Scenario "Edge Cases (Presence and First-win)" "examples/edge_case_contract.yaml" "examples/edge_cases.json"

Write-Host "`nDone!" -ForegroundColor Green
