<p align="center">
  <img src="https://geval.io/white_bg_greenlogo.svg" alt="Geval" width="180" />
</p>

# Geval

<p align="center">
  <strong>One clear decision for every AI change.</strong>
</p>

<p align="center">
  Evals tell you <em>what happened</em>. Geval tells you <em>whether you can ship.</em>
</p>

<p align="center">
  <a href="https://github.com/geval-labs/geval/releases"><img src="https://img.shields.io/github/v/release/geval-labs/geval?label=release" alt="Release"></a>
  <a href="https://github.com/geval-labs/geval/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
  <a href="https://github.com/geval-labs/geval/actions"><img src="https://github.com/geval-labs/geval/workflows/CI/badge.svg" alt="CI"></a>
</p>

---

## Try it in under a minute

**1. Download the binary** for your OS (no repo clone needed):

```bash
# Linux
curl -sSL https://github.com/geval-labs/geval/releases/latest/download/geval-linux-x86_64 -o geval && chmod +x geval

# macOS (Apple Silicon)
curl -sSL https://github.com/geval-labs/geval/releases/latest/download/geval-macos-aarch64 -o geval && chmod +x geval

# Windows (PowerShell) — see note below
Invoke-WebRequest -Uri https://github.com/geval-labs/geval/releases/latest/download/geval-windows-x86_64.exe -OutFile geval.exe
```

> **Windows:** Run **PowerShell as Administrator** before downloading and running Geval. Right‑click PowerShell → *Run as administrator*, then run the download command and `.\geval.exe demo`. This avoids execution policy and path issues so the demo and CLI work correctly.

**2. Run the built-in demo** (no files needed):

```bash
./geval demo          # Linux / macOS
.\geval.exe demo      # Windows (in the folder where you saved geval.exe)
```

You’ll get a step-by-step decision report and an outcome: **PASS**, **REQUIRE_APPROVAL**, or **BLOCK**. Same binary works in CI — no npm, no pip. [Use in CI →](geval/docs/github-actions.md)

**Using your own files?**

```bash
./geval check --signals path/to/signals.json --policy path/to/policy.yaml --env prod
```

**Download failed or no binary for your OS?** [Build from source](geval/docs/installation.md#build-from-source) (requires [Rust](https://rustup.rs/)).

---

<p align="center">
  <a href="#the-problem">The problem</a> •
  <a href="#what-geval-does">What Geval does</a> •
  <a href="#cli">CLI</a> •
  <a href="#documentation">Docs</a>
</p>

---

## The problem

Your team runs **evals** (accuracy, relevance, safety, hallucinations, latency) plus A/B tests, human review, and business metrics. When you change a model or a prompt, you get a flood of signals:

- **Evals:** “Accuracy improved.”
- **A/B:** “Engagement dropped a bit.”
- **Review:** “Edge case flagged.”

So: **do you ship or not?** Today that call often happens in Slack or a meeting — inconsistent, hard to audit, and easy to forget. Geval gives you **one place to write the rules** and **one clear answer** every time: **ship**, **get approval first**, or **block**.

---

## What Geval does

**Geval is a decision engine for AI releases.** You feed it the outcomes of your evals and other signals (as simple data files). You define your policy in a single file: *“If engagement drops, block. If hallucination rate is above X, block. If retrieval quality is below Y, require human approval.”* Geval applies those rules in a fixed order and returns:

| Outcome | Meaning |
|--------|--------|
| **PASS** | Good to ship. No rule blocked it. |
| **REQUIRE_APPROVAL** | A rule says a human must approve before shipping. |
| **BLOCK** | A rule says do not ship until the issue is fixed. |

Every run is recorded (which policy and signals were used, which rule fired, when). So product managers, engineers, and auditors can always answer: *“Why did we ship this?”* and *“Who approved it?”*

**In short:** evals answer *“What happened?”* Geval answers *“Given what happened, are we allowed to ship?”*

---

## CLI

| Command | What it does |
|--------|----------------|
| `geval demo` | Run built-in example (no files). **Use this first after downloading.** |
| `geval check` | Run your signals + policy → PASS / REQUIRE_APPROVAL / BLOCK (exit 0 / 1 / 2) |
| `geval explain` | Show why (which rule, which signals) |
| `geval approve` / `geval reject` | Record human approval or rejection |
| `geval validate-policy` | Validate policy file |

---

## Documentation

| Guide | Description |
|-------|-------------|
| [**GitHub Actions**](geval/docs/github-actions.md) | Run Geval in CI (workflow YAML, exit codes) |
| [**Examples**](geval/examples/README.md) | Sample `signals.json` and `policy.yaml` |
| [**Installation**](geval/docs/installation.md) | PATH, CI, and build-from-source (for contributors) |
| [**Developer workflow**](geval/docs/developer-workflow.md) | PR → check → approve/reject |
| [**Auditing**](geval/docs/auditing.md) | Decision artifacts, hashes |

---

## Contributing

We welcome contributions. See [CONTRIBUTING.md](CONTRIBUTING.md). To build from source, see [Installation → Build from source](geval/docs/installation.md#build-from-source).

---

## License

MIT © [Geval Contributors](https://github.com/geval-labs/geval/graphs/contributors)

---

<p align="center">
  <a href="https://geval.io">Website</a> •
  <a href="https://github.com/geval-labs/geval/releases">Releases</a> •
  <a href="https://github.com/geval-labs/geval">GitHub</a>
</p>
