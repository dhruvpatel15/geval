<p align="center">
  <img src="https://geval.io/white_bg_greenlogo.svg" alt="Geval" width="180" />
</p>

# Geval

<p align="center">
  <strong>One clear decision for every AI change.</strong>
</p>

<p align="center">
  Your numbers in. Your rules. One answer: ship, get approval, or block.
</p>

<p align="center">
  <a href="https://github.com/geval-labs/geval/releases"><img src="https://img.shields.io/github/v/release/geval-labs/geval?label=release" alt="Release"></a>
  <a href="https://github.com/geval-labs/geval/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
  <a href="https://github.com/geval-labs/geval/actions"><img src="https://github.com/geval-labs/geval/workflows/CI/badge.svg" alt="CI"></a>
</p>

---

## Try it in under a minute

**1. Download** (pick your OS):

```bash
# Linux
curl -sSL https://github.com/geval-labs/geval/releases/latest/download/geval-linux-x86_64 -o geval && chmod +x geval

# macOS (Apple Silicon)
curl -sSL https://github.com/geval-labs/geval/releases/latest/download/geval-macos-aarch64 -o geval && chmod +x geval

# Windows (PowerShell) — see note below
Invoke-WebRequest -Uri https://github.com/geval-labs/geval/releases/latest/download/geval-windows-x86_64.exe -OutFile geval.exe
```

> **Windows:** Open **PowerShell as Administrator** (right‑click → *Run as administrator*). Then run the download command and `.\geval.exe demo`.

**2. Run the demo** (no files needed):

```bash
./geval demo          # Linux / macOS
.\geval.exe demo      # Windows (same folder as geval.exe)
```

You get a report and one of three answers: **PASS**, **REQUIRE_APPROVAL**, or **BLOCK**. [Use in CI →](geval/docs/github-actions.md)

**No binary for your OS?** [Build from source](geval/docs/installation.md#build-from-source).

### Updating

Use the same download commands. Replace your old file with the new one. Check version: `geval --version`.

---

## Use Geval with your own rules and data

You need **two files**: one with **your numbers**, one with **your rules**. Geval reads both and gives one answer.

### Step 1: Your numbers (data file)

A list of what you measured. Each item has a name and a value.

Example — save as `mydata.json`:

```json
{
  "signals": [
    { "metric": "accuracy", "value": 0.94 },
    { "metric": "engagement_drop", "value": 0.02 }
  ]
}
```

You can add labels like `component` or `system` if you need them. [Full example →](geval/examples/signals.json)

### Step 2: Your rules (rules file)

A list of rules in order. Geval checks the first rule, then the next, and stops at the first match.

Each rule says: **When** [something about your numbers], **then** [allow / need approval / block].

Example — save as `myrules.yaml`:

```yaml
policy:
  rules:
    - priority: 1
      name: block_bad_engagement
      when:
        metric: engagement_drop
        operator: ">"
        threshold: 0
      then:
        action: block
        reason: "Engagement dropped"

    - priority: 2
      name: allow_good_accuracy
      when:
        metric: accuracy
        operator: ">="
        threshold: 0.9
      then:
        action: pass
```

**Operators:** `>` greater than, `<` less than, `>=` at least, `<=` at most, `==` equal.

**Actions:** `pass` = allow. `block` = don’t allow. `require_approval` = a person must say yes first.

[Full example →](geval/examples/policy.yaml)

### Step 3: Run Geval

Point Geval at your two files:

```bash
./geval check --signals mydata.json --policy myrules.yaml
```

(Windows: `.\geval.exe check --signals mydata.json --policy myrules.yaml`)

### Step 4: Read the answer

- **PASS** — No rule said no. You’re good to go.
- **REQUIRE_APPROVAL** — A rule says someone must approve before you go.
- **BLOCK** — A rule says stop. Fix the issue before going.

To see **why** Geval chose that answer:

```bash
./geval explain --signals mydata.json --policy myrules.yaml
```

To check that your rules file is valid (no run needed):

```bash
./geval validate-policy myrules.yaml
```

---

<p align="center">
  <a href="#the-problem">The problem</a> •
  <a href="#what-geval-does">What Geval does</a> •
  <a href="#cli">Commands</a> •
  <a href="#documentation">Docs</a>
</p>

---

## The problem

You have tests and numbers: accuracy, engagement, safety, reviews. You change a model or a prompt. Then what?

- One report says “better.”
- Another says “worse.”
- Someone asks: “Do we ship?”

Today that decision is in chat or a meeting. Hard to repeat. Hard to audit. Geval puts **your rules in one place** and gives **one answer** every time: ship, get approval first, or block.

---

## What Geval does

You give Geval:

1. **Your numbers** (one file) — e.g. scores, metrics, A/B results.
2. **Your rules** (one file) — e.g. “If engagement drops, block. If accuracy is below X, need approval.”

Geval applies the rules in order and returns:

| Result | Meaning |
|--------|--------|
| **PASS** | No rule said no. Good to go. |
| **REQUIRE_APPROVAL** | A rule says a person must approve first. |
| **BLOCK** | A rule says don’t ship. Fix first. |

Each run is stored: which rules, which numbers, when. So you can always answer: “Why did we ship?” and “Who approved?”

---

## Commands

| Command | What it does |
|--------|----------------|
| `geval demo` | Run the built-in example. Try this first. |
| `geval check` | Run your data + rules → PASS / REQUIRE_APPROVAL / BLOCK |
| `geval explain` | Show why you got that answer |
| `geval approve` / `geval reject` | Record a person’s approval or rejection |
| `geval validate-policy` | Check your rules file is valid |

---

## Documentation

| Guide | Description |
|-------|-------------|
| [**GitHub Actions**](geval/docs/github-actions.md) | Use Geval in CI |
| [**Examples**](geval/examples/README.md) | Sample data and rules files |
| [**Installation**](geval/docs/installation.md) | Install, PATH, build from source |
| [**Developer workflow**](geval/docs/developer-workflow.md) | PRs, check, approve/reject |
| [**Auditing**](geval/docs/auditing.md) | How decisions are recorded |

---

## Contributing

Contributions welcome. [CONTRIBUTING.md](CONTRIBUTING.md). Build from source: [Installation](geval/docs/installation.md#build-from-source).

---

## License

MIT © [Geval Contributors](https://github.com/geval-labs/geval/graphs/contributors)

---

<p align="center">
  <a href="https://geval.io">Website</a> •
  <a href="https://github.com/geval-labs/geval/releases">Releases</a> •
  <a href="https://github.com/geval-labs/geval">GitHub</a>
</p>
