//! `geval init` — create a .geval template in the current directory.
//! Safe for existing codebases: only creates files inside the chosen directory (default .geval).

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const SIGNALS_TEMPLATE: &str = r#"{
  "signals": [
    {
      "system": "my_app",
      "component": "retrieval",
      "metric": "context_relevance",
      "value": 0.85
    },
    {
      "system": "my_app",
      "component": "generator",
      "metric": "hallucination_rate",
      "value": 0.04
    },
    {
      "metric": "engagement_drop",
      "value": 0.01
    },
    {
      "component": "pipeline",
      "step": "validation",
      "metric": "latency_ms",
      "value": 120
    },
    {
      "metric": "human_reviewed"
    }
  ]
}
"#;

const POLICY_TEMPLATE: &str = r#"# Geval policy — your rules. Edit and add your own.
# Rules are evaluated in priority order (lower number first). First match wins.
# No match = PASS (allow).
#
# When: metric (required), optional: component, system, agent, step.
#       operator: ">", "<", ">=", "<=", "==", or "presence" (no threshold; matches if metric exists, even without a value).
#       threshold: number (for comparisons; not used for presence).
# Then: action: pass | block | require_approval. Optional: reason.

policy:
  environment: prod

  rules:
    - priority: 1
      name: block_engagement_drop
      when:
        metric: engagement_drop
        operator: ">"
        threshold: 0
      then:
        action: block
        reason: "Business engagement dropped"

    - priority: 2
      name: block_high_hallucination
      when:
        component: generator
        metric: hallucination_rate
        operator: ">"
        threshold: 0.05
      then:
        action: block

    - priority: 3
      name: require_approval_low_retrieval
      when:
        component: retrieval
        metric: context_relevance
        operator: "<"
        threshold: 0.85
      then:
        action: require_approval
        reason: "Retrieval quality below threshold"

    - priority: 4
      name: pass_high_accuracy
      when:
        metric: context_relevance
        operator: ">="
        threshold: 0.9
      then:
        action: pass
"#;

fn readme_content(dir: &Path) -> String {
    let dir_str = dir.display().to_string();
    format!(
        r#"# Geval workspace

Created by `geval init`. Edit the files in this folder and run Geval from your project root.

## Files

- **signals.json** — Your data (metrics, scores). Add or change entries. Each entry can have: system, agent, component, step, metric, value, type.
- **policy.yaml** — Your rules. Order by priority; first matching rule wins. Actions: pass, block, require_approval.

## Run

From the **project root** (parent of this folder):

```bash
geval check --signals {}/signals.json --policy {}/policy.yaml
```

Explain why you got that result:

```bash
geval explain --signals {}/signals.json --policy {}/policy.yaml
```

Validate your rules file:

```bash
geval validate-policy {}/policy.yaml
```

## Approve / reject

If the result is REQUIRE_APPROVAL, record a decision:

```bash
geval approve --reason "Reviewed and approved" --output {}/approval.json
# or
geval reject --reason "Needs more testing" --output {}/rejection.json
```

Your codebase is unchanged except for this folder. Add these files to version control if you want to share rules with your team.
"#,
        dir_str, dir_str, dir_str, dir_str, dir_str, dir_str, dir_str
    )
}

/// Run `geval init`: create directory and template files.
/// If directory already has signals.json or policy.yaml and force is false, returns error.
pub fn run_init(dir: &Path, force: bool) -> Result<()> {
    let signals_path = dir.join("signals.json");
    let policy_path = dir.join("policy.yaml");
    let readme_path = dir.join("README.md");

    if dir.exists() {
        let has_signals = signals_path.exists();
        let has_policy = policy_path.exists();
        if (has_signals || has_policy) && !force {
            anyhow::bail!(
                "Directory {} already has template files. Use --force to overwrite.",
                dir.display()
            );
        }
    } else {
        fs::create_dir_all(dir).with_context(|| format!("create directory {}", dir.display()))?;
    }

    fs::write(&signals_path, SIGNALS_TEMPLATE)
        .with_context(|| format!("write {}", signals_path.display()))?;
    fs::write(&policy_path, POLICY_TEMPLATE)
        .with_context(|| format!("write {}", policy_path.display()))?;
    fs::write(&readme_path, readme_content(dir))
        .with_context(|| format!("write {}", readme_path.display()))?;

    Ok(())
}
