# Geval Examples

Example contracts, policies, and signals for Geval (decision orchestration and reconciliation).

## Files

- **contract.yaml** – Multi-policy demonstration: merges rules from `policies/quality.yaml` and `policies/safety.yaml`.
- **reconciliation_demo.yaml** – Demonstration of priority-based reconciliation (best priority wins).
- **edge_case_contract.yaml** – Testing presence-based rules and first-win signal logic.
- **policy.yaml** – Legacy single-policy example.
- **signals.json** – Standard demo signals for numeric metrics.
- **reconciliation_signals.json** – Overlapping signals to trigger competing rules.
- **edge_cases.json** – Mixed types (strings, objects), presence-only, and out-of-order signals.

## Interactive Runner (Windows)

The simplest way to see Geval in action is to run the PowerShell script:

```powershell
./examples/run_examples.ps1
```

This will execute several scenarios (Multi-policy, Reconciliation, and Edge cases) and show both the decision outcome and the detailed explanation for each.

## Manual Run (from repo root)

```bash
# Ensure the binary is built
cargo build --release --manifest-path geval/Cargo.toml

# 1. Multi-policy evaluation
./geval/target/release/geval check --contract geval/examples/contract.yaml --signals geval/examples/signals.json

# 2. Reconciliation (Priority-based winner)
./geval/target/release/geval explain --contract geval/examples/reconciliation_demo.yaml --signals geval/examples/reconciliation_signals.json

# 3. Edge cases (Presence-only detection)
./geval/target/release/geval explain --contract geval/examples/edge_case_contract.yaml --signals geval/examples/edge_cases.json
```

## Concepts Demonstrated

### Multi-Policy Combinations
Contracts can reference multiple files. Using `combine: all_pass` (default), if any policy blocks, the entire contract blocks.

### Reconciliation
When multiple rules within a policy match the input signals, Geval reconciles them by picking the one with the **lowest priority number** (1 = highest). See `reconciliation_demo.yaml` for a "Manual Override" example.

### Signal Handling
- **Presence-only**: Rules with `operator: presence` match even if the signal has no numeric value (e.g. `manual_override: {}`).
- **First-win**: If the same metric appears multiple times, the first one encountered in the signals file is used for threshold comparisons.
- **Resilience**: Non-numeric signals (strings, objects) are ignored for threshold rules but preserved for audit and reporting.
