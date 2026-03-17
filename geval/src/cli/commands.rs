//! CLI commands: check, init, demo, approve, reject, explain, validate-policy.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::approval::write_approval;
use crate::artifact::write_decision_artifact;
use crate::cli::{demo_ui::print_demo_report, init::run_init as do_init};
use crate::evaluator::{evaluate, evaluate_with_trace, DecisionOutcome};
use crate::explanation::explain_decision;
use crate::hashing::{hash_policy, hash_signals};
use crate::policy::{parse_policy, parse_policy_str};
use crate::signal_graph::SignalGraph;
use crate::signals::{load_signals, load_signals_from_reader};

/// Geval - decision orchestration engine for AI systems.
#[derive(Parser)]
#[command(name = "geval")]
#[command(version)]
#[command(about = "Decision orchestration and reconciliation for AI system changes", long_about = None)]
pub struct Commands {
    #[command(subcommand)]
    pub sub: Sub,
}

#[derive(Subcommand)]
pub enum Sub {
    /// Evaluate signals against policy; exit 0=PASS, 1=REQUIRE_APPROVAL, 2=BLOCK.
    Check(CheckOpts),
    /// Create a template folder (.geval by default) with sample signals and policy. Edit and run.
    Init(InitOpts),
    /// Run a built-in example (no files needed). Use this to try Geval after downloading.
    Demo(DemoOpts),
    /// Record human approval (for REQUIRE_APPROVAL flow).
    Approve(ApproveOpts),
    /// Record human rejection.
    Reject(RejectOpts),
    /// Print human-readable decision report.
    Explain(ExplainOpts),
    /// Validate policy file syntax.
    ValidatePolicy(ValidatePolicyOpts),
}

/// Built-in demo signals and policy (same as geval/examples/).
const DEMO_SIGNALS_JSON: &str = r#"{
  "signals": [
    {
      "system": "support_agent",
      "component": "retrieval",
      "metric": "context_relevance",
      "value": 0.84
    },
    {
      "system": "support_agent",
      "component": "generator",
      "metric": "hallucination_rate",
      "value": 0.06
    },
    {
      "type": "ab_test",
      "metric": "engagement_drop",
      "value": 0.03
    }
  ]
}"#;

const DEMO_POLICY_YAML: &str = r#"policy:
  environment: prod
  rules:
    - priority: 1
      name: business_block
      when:
        metric: engagement_drop
        operator: ">"
        threshold: 0
      then:
        action: block
        reason: "Business engagement dropped"

    - priority: 2
      name: hallucination_guard
      when:
        component: generator
        metric: hallucination_rate
        operator: ">"
        threshold: 0.05
      then:
        action: block

    - priority: 3
      name: retrieval_quality
      when:
        component: retrieval
        metric: context_relevance
        operator: "<"
        threshold: 0.85
      then:
        action: require_approval
"#;

#[derive(clap::Args)]
pub struct CheckOpts {
    #[arg(long, short = 's')]
    pub signals: PathBuf,
    #[arg(long, short = 'p')]
    pub policy: PathBuf,
    #[arg(long, short = 'e', env = "GEVAL_ENV")]
    pub env: Option<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(clap::Args)]
pub struct ApproveOpts {
    #[arg(long, short = 'r')]
    pub reason: String,
    #[arg(long, short = 'o', default_value = ".geval/approval.json")]
    pub output: PathBuf,
    #[arg(long, env = "USER")]
    pub by: Option<String>,
}

#[derive(clap::Args)]
pub struct RejectOpts {
    #[arg(long, short = 'r')]
    pub reason: String,
    #[arg(long, short = 'o', default_value = ".geval/rejection.json")]
    pub output: PathBuf,
    #[arg(long, env = "USER")]
    pub by: Option<String>,
}

#[derive(clap::Args)]
pub struct ExplainOpts {
    #[arg(long, short = 's')]
    pub signals: PathBuf,
    #[arg(long, short = 'p')]
    pub policy: PathBuf,
    #[arg(long, short = 'e', env = "GEVAL_ENV")]
    pub env: Option<String>,
}

#[derive(clap::Args)]
pub struct ValidatePolicyOpts {
    pub policy: PathBuf,
    #[arg(long)]
    pub json: bool,
}

#[derive(clap::Args)]
pub struct InitOpts {
    /// Directory to create (default: .geval). All template files go here; your project stays unchanged.
    #[arg(default_value = ".geval")]
    pub directory: PathBuf,
    /// Overwrite existing signals.json and policy.yaml if they already exist.
    #[arg(long)]
    pub force: bool,
}

#[derive(clap::Args)]
pub struct DemoOpts {
    #[arg(long)]
    pub json: bool,
}

impl Commands {
    pub fn run(self) -> Result<()> {
        match self.sub {
            Sub::Check(opts) => run_check(&opts),
            Sub::Init(opts) => run_init(&opts),
            Sub::Demo(opts) => run_demo(&opts),
            Sub::Approve(opts) => run_approve(&opts),
            Sub::Reject(opts) => run_reject(&opts),
            Sub::Explain(opts) => run_explain(&opts),
            Sub::ValidatePolicy(opts) => run_validate_policy(&opts),
        }
    }
}

fn run_init(opts: &InitOpts) -> Result<()> {
    do_init(&opts.directory, opts.force).context("init")?;
    println!(
        "Created {} with signals.json, policy.yaml, and README.md.",
        opts.directory.display()
    );
    println!(
        "Edit the files, then run: geval check --signals {}/signals.json --policy {}/policy.yaml",
        opts.directory.display(),
        opts.directory.display()
    );
    Ok(())
}

fn run_demo(opts: &DemoOpts) -> Result<()> {
    let policy = parse_policy_str(DEMO_POLICY_YAML).context("parse built-in policy")?;
    let signals =
        load_signals_from_reader(DEMO_SIGNALS_JSON.as_bytes()).context("parse built-in signals")?;
    let graph = SignalGraph::build(&signals.signals);
    let (decision, trace) = evaluate_with_trace(&policy, &graph);

    if opts.json {
        let out = serde_json::json!({
            "decision": outcome_str(decision.outcome),
            "matched_rule": decision.matched_rule,
            "reason": decision.reason,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        print_demo_report(&policy, &graph, &decision, &trace, Some("prod"));
    }

    let code = match decision.outcome {
        DecisionOutcome::Pass => 0,
        DecisionOutcome::RequireApproval => 1,
        DecisionOutcome::Block => 2,
    };
    std::process::exit(code);
}

fn run_check(opts: &CheckOpts) -> Result<()> {
    let policy = parse_policy(&opts.policy).context("load policy")?;
    let signals = load_signals(&opts.signals).context("load signals")?;
    let graph = SignalGraph::build(&signals.signals);
    let decision = evaluate(&policy, &graph);

    let policy_hash = hash_policy(&policy);
    let signals_hash = hash_signals(&signals);
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let _ = write_decision_artifact(
        &cwd,
        &policy_hash,
        &signals_hash,
        &decision,
        None,
    );

    if opts.json {
        let out = serde_json::json!({
            "decision": outcome_str(decision.outcome),
            "matched_rule": decision.matched_rule,
            "reason": decision.reason,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        let env = opts.env.as_deref().or(policy.environment.as_deref());
        println!("{}", explain_decision(&policy, &graph, &decision, env));
    }

    let code = match decision.outcome {
        DecisionOutcome::Pass => 0,
        DecisionOutcome::RequireApproval => 1,
        DecisionOutcome::Block => 2,
    };
    std::process::exit(code);
}

fn run_approve(opts: &ApproveOpts) -> Result<()> {
    let by = opts
        .by
        .clone()
        .or_else(|| std::env::var("USER").ok())
        .unwrap_or_else(|| "user".to_string());
    write_approval(&opts.output, by, opts.reason.clone(), true)?;
    println!("Approval recorded to {}", opts.output.display());
    Ok(())
}

fn run_reject(opts: &RejectOpts) -> Result<()> {
    let by = opts
        .by
        .clone()
        .or_else(|| std::env::var("USER").ok())
        .unwrap_or_else(|| "user".to_string());
    write_approval(&opts.output, by, opts.reason.clone(), false)?;
    println!("Rejection recorded to {}", opts.output.display());
    Ok(())
}

fn run_explain(opts: &ExplainOpts) -> Result<()> {
    let policy = parse_policy(&opts.policy).context("load policy")?;
    let signals = load_signals(&opts.signals).context("load signals")?;
    let graph = SignalGraph::build(&signals.signals);
    let decision = evaluate(&policy, &graph);
    let env = opts.env.as_deref().or(policy.environment.as_deref());
    println!("{}", explain_decision(&policy, &graph, &decision, env));
    Ok(())
}

fn run_validate_policy(opts: &ValidatePolicyOpts) -> Result<()> {
    let policy = parse_policy(&opts.policy).context("validate policy")?;
    if opts.json {
        println!("{}", serde_json::to_string_pretty(&policy)?);
    } else {
        println!("Policy valid: {} rule(s)", policy.rules.len());
        if let Some(env) = &policy.environment {
            println!("Environment: {}", env);
        }
    }
    Ok(())
}

fn outcome_str(o: DecisionOutcome) -> &'static str {
    match o {
        DecisionOutcome::Pass => "PASS",
        DecisionOutcome::RequireApproval => "REQUIRE_APPROVAL",
        DecisionOutcome::Block => "BLOCK",
    }
}
