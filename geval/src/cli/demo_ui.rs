//! Elegant, step-by-step CLI output for `geval demo`.
//! Output appears progressively (loading phases, then content streamed line-by-line).
//! Set GEVAL_DEMO_FAST=1 to skip delays (e.g. in CI or when piping).

use crate::evaluator::{Decision, DecisionOutcome, RuleTrace};
use crate::policy::{Action, Operator};
use crate::signal_graph::SignalGraph;
use crate::signals::Signal;
use std::io::{IsTerminal, Write};
use std::time::Duration;

/// Delay in ms; no-op if GEVAL_DEMO_FAST=1 or not a TTY (piped/CI).
fn delay_ms(ms: u64) {
    if std::env::var("GEVAL_DEMO_FAST").is_ok() {
        return;
    }
    if !std::io::stdout().is_terminal() {
        return;
    }
    std::thread::sleep(Duration::from_millis(ms));
}

/// Print line, flush, then optional delay (for progressive "streaming" feel).
fn line<W: Write>(out: &mut W, ms_after: u64, s: &str) {
    let _ = writeln!(out, "{}", s);
    let _ = out.flush();
    delay_ms(ms_after);
}

/// Print "Loading..." then replace with final line after delay (cooking feel).
/// When not a TTY (e.g. piping), just print the done line so output stays clean.
fn loading_then<W: Write>(out: &mut W, loading: &str, done_line: &str, ms: u64) {
    if !std::io::stdout().is_terminal() {
        let _ = writeln!(out, "{}", done_line);
        let _ = out.flush();
        return;
    }
    let _ = write!(out, "{}", loading);
    let _ = out.flush();
    delay_ms(ms);
    let _ = write!(out, "\r{:52}\r{}\n", "", done_line);
    let _ = out.flush();
}

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";

fn dim(s: &str) -> String {
    format!("{}{}{}", DIM, s, RESET)
}

fn bold(s: &str) -> String {
    format!("{}{}{}", BOLD, s, RESET)
}

fn color_outcome(outcome: DecisionOutcome) -> &'static str {
    match outcome {
        DecisionOutcome::Pass => GREEN,
        DecisionOutcome::RequireApproval => YELLOW,
        DecisionOutcome::Block => RED,
    }
}

fn outcome_str(o: DecisionOutcome) -> &'static str {
    match o {
        DecisionOutcome::Pass => "PASS",
        DecisionOutcome::RequireApproval => "REQUIRE_APPROVAL",
        DecisionOutcome::Block => "BLOCK",
    }
}

fn action_str(a: Action) -> &'static str {
    match a {
        Action::Pass => "PASS",
        Action::Block => "BLOCK",
        Action::RequireApproval => "REQUIRE_APPROVAL",
    }
}

fn signal_label(s: &Signal) -> String {
    let parts: Vec<&str> = [s.component.as_deref(), s.metric.as_deref()]
        .into_iter()
        .flatten()
        .collect();
    if parts.is_empty() {
        s.metric.as_deref().unwrap_or("?").to_string()
    } else {
        parts.join(".")
    }
}

fn signal_value_str(s: &Signal) -> String {
    match &s.value {
        Some(v) => {
            if let Some(n) = v.as_f64() {
                format!("{}", n)
            } else if let Some(st) = v.as_str() {
                st.to_string()
            } else {
                v.to_string()
            }
        }
        None => "—".to_string(),
    }
}

fn op_symbol(t: &RuleTrace) -> &'static str {
    match t.operator {
        Operator::GreaterThan => ">",
        Operator::LessThan => "<",
        Operator::GreaterOrEqual => ">=",
        Operator::LessOrEqual => "<=",
        Operator::Equal => "==",
        Operator::Presence => "present?",
    }
}

const DELAY_LOAD: u64 = 380;
const DELAY_LINE: u64 = 68;
const DELAY_BLOCK: u64 = 130;
const DELAY_SECTION: u64 = 280;

/// Print the full demo report: policy, signals, rule-by-rule evaluation, and decision.
/// Output appears progressively (loading phases, then content streamed line-by-line).
pub fn print_demo_report(
    policy: &crate::policy::Policy,
    graph: &SignalGraph,
    decision: &Decision,
    trace: &[RuleTrace],
    environment: Option<&str>,
) {
    let use_color = std::io::stdout().is_terminal();
    let mut out = std::io::stdout().lock();

    let d = |s: &str| if use_color { dim(s) } else { s.to_string() };
    let b = |s: &str| if use_color { bold(s) } else { s.to_string() };
    let green_s = |s: &str| if use_color { format!("{}{}{}", GREEN, s, RESET) } else { s.to_string() };
    let cyan_s = |s: &str| if use_color { format!("{}{}{}", CYAN, s, RESET) } else { s.to_string() };
    let magenta_s = |s: &str| if use_color { format!("{}{}{}", MAGENTA, s, RESET) } else { s.to_string() };
    let yellow_s = |s: &str| if use_color { format!("{}{}{}", YELLOW, s, RESET) } else { s.to_string() };
    let outcome_colored = |o: DecisionOutcome| {
        if use_color {
            format!("{}{}{}", color_outcome(o), outcome_str(o), RESET)
        } else {
            outcome_str(o).to_string()
        }
    };

    let _ = writeln!(out);
    let _ = out.flush();
    line(&mut out, 0, &format!("  {}  {}", cyan_s("╭─────────────────────────────────────────────────────────────╮"), ""));
    line(&mut out, DELAY_LINE, &format!("  {}  {}", cyan_s("│"), b("  GEVAL  ·  Demo")));
    line(&mut out, DELAY_LINE, &format!("  {}  {}", cyan_s("│"), d("  One clear decision for every AI change")));
    line(&mut out, DELAY_LINE, &format!("  {}  {}", cyan_s("╰─────────────────────────────────────────────────────────────╯"), ""));
    line(&mut out, DELAY_SECTION, "");

    // Step 1: Policy — "Loading policy..." then stream rules
    let loading1 = format!("  {}  {}", green_s("▶"), d("Loading policy..."));
    let done1 = format!("  {}  {}", green_s("▶"), b("Step 1: Policy loaded"));
    loading_then(&mut out, &loading1, &done1, DELAY_LOAD);
    line(&mut out, DELAY_LINE, &format!("  {}    {}", d("│"), d("Environment:")));
    line(&mut out, DELAY_LINE, &format!("  {}    {}  {}", d("│"), d("  "), environment.unwrap_or("(not set)")));
    line(&mut out, DELAY_LINE, &format!("  {}    {}", d("│"), d("Rules (evaluated in priority order):")));
    for (i, rule) in policy.sorted_rules().iter().enumerate() {
        line(&mut out, DELAY_LINE, &format!("  {}    {}  {}. {}  {}  {}", d("│"), d("  "), i + 1, magenta_s(&rule.name), d("→"), d(action_str(rule.then.action))));
    }
    line(&mut out, DELAY_LINE, &format!("  {}    {}  {}", d("│"), d("  "), d(&format!("{} rule(s) total", policy.rules.len()))));
    line(&mut out, DELAY_SECTION, "");

    // Step 2: Signals — "Loading signals..." then stream each signal
    let loading2 = format!("  {}  {}", green_s("▶"), d("Loading signals..."));
    let done2 = format!("  {}  {}", green_s("▶"), b("Step 2: Signals loaded"));
    loading_then(&mut out, &loading2, &done2, DELAY_LOAD);
    for s in &graph.signals {
        line(&mut out, DELAY_LINE, &format!("  {}    {}  {}  {}  {}", d("│"), cyan_s("·"), signal_label(s), d("="), signal_value_str(s)));
    }
    line(&mut out, DELAY_LINE, &format!("  {}    {}  {}", d("│"), d("  "), d(&format!("{} signal(s)", graph.signals.len()))));
    line(&mut out, DELAY_SECTION, "");

    // Step 3: Rules — "Evaluating rules..." then stream each rule block
    let loading3 = format!("  {}  {}", green_s("▶"), d("Evaluating rules..."));
    let done3 = format!("  {}  {}", green_s("▶"), b("Step 3: Evaluating rules (first match wins)"));
    loading_then(&mut out, &loading3, &done3, DELAY_LOAD);
    let traced_names: std::collections::HashSet<_> = trace.iter().map(|t| t.rule_name.as_str()).collect();
    let sorted = policy.sorted_rules();
    let mut step = 0;
    for t in trace.iter() {
        step += 1;
        line(&mut out, DELAY_BLOCK, &format!("  {}  {}", d("│"), d("")));
        line(&mut out, DELAY_LINE, &format!("  {}  {}  {}  {}  {}", d("│"), yellow_s(&format!("[{}]", step)), magenta_s(&t.rule_name), d("(priority"), format!("{})", t.priority)));
        line(&mut out, DELAY_LINE, &format!("  {}      {}  {}", d("│"), d("Condition:"), t.condition));
        match (t.signal_value, t.threshold) {
            (Some(v), Some(th)) => {
                line(&mut out, DELAY_LINE, &format!("  {}      {}  {}  {}  {}", d("│"), d("Signal value:"), v, d("  |  Threshold:"), th));
                let op = op_symbol(t);
                if t.matched {
                    line(&mut out, DELAY_LINE, &format!("  {}      {}  {}  {}  {}  {}  {}", d("│"), d(""), format!("{} {} {}", v, op, th), green_s("⇒"), green_s("✓ MATCHED"), green_s("  →  "), green_s(action_str(t.action))));
                } else {
                    line(&mut out, DELAY_LINE, &format!("  {}      {}  {}  {}", d("│"), d(""), format!("{} {} {}  →  false", v, op, th), d("○ No match")));
                }
            }
            (Some(v), None) if matches!(t.operator, Operator::Presence) => {
                line(&mut out, DELAY_LINE, &format!("  {}      {}  {}", d("│"), d("Present:"), v));
                line(&mut out, DELAY_LINE, &format!("  {}      {}", d("│"), if t.matched { green_s("✓ MATCHED") } else { d("○ No match") }));
            }
            _ => {
                line(&mut out, DELAY_LINE, &format!("  {}      {}", d("│"), if t.matched { green_s("✓ MATCHED") } else { d("○ No match (missing value or threshold)") }));
            }
        }
    }
    for rule in sorted.iter() {
        if !traced_names.contains(rule.name.as_str()) {
            step += 1;
            line(&mut out, DELAY_BLOCK, &format!("  {}  {}", d("│"), d("")));
            line(&mut out, DELAY_LINE, &format!("  {}  {}  {}  {}", d("│"), yellow_s(&format!("[{}]", step)), magenta_s(&rule.name), d("(not evaluated — decision already made)")));
        }
    }
    line(&mut out, DELAY_SECTION, "");

    // Step 4: Decision — "Computing decision..." then reveal outcome
    let loading4 = format!("  {}  {}", green_s("▶"), d("Computing decision..."));
    let done4 = format!("  {}  {}", green_s("▶"), b("Step 4: Decision"));
    loading_then(&mut out, &loading4, &done4, DELAY_LOAD);
    line(&mut out, DELAY_LINE, &format!("  {}  {}", d("│"), d("")));
    let oc = outcome_colored(decision.outcome);
    line(&mut out, DELAY_LINE, &format!("  {}    {}", d("│"), cyan_s("╭─────────────────────╮")));
    line(&mut out, DELAY_LINE, &format!("  {}    {}  {}  {}", d("│"), cyan_s("│"), format!("  {}  ", oc), cyan_s("│")));
    line(&mut out, DELAY_LINE, &format!("  {}    {}", d("│"), cyan_s("╰─────────────────────╯")));
    if let Some(ref reason) = decision.reason {
        line(&mut out, DELAY_LINE, &format!("  {}    {}", d("│"), d("")));
        line(&mut out, DELAY_LINE, &format!("  {}    {}  {}", d("│"), d("Reason:"), reason));
    }
    if let Some(ref name) = decision.matched_rule {
        line(&mut out, DELAY_LINE, &format!("  {}    {}  {}", d("│"), d("Matched rule:"), name));
    }
    line(&mut out, 0, "");
    let _ = out.flush();
}
