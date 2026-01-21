// GrokArena-Pinnacle/src/quadratic_voting_sim.rs
//
// Quadratic Voting (QV) Simulator Stub – With Stats Collector Module
// =================================================================
//
// Added StatsCollector:
// - New struct StatsCollector to gather post-voting metrics.
// - Tracks:
//   * Votes contributed per strategy type (total positive/negative intensity).
//   * Credits spent per strategy.
//   * Issue outcomes (pass/reject + margin).
//   * Attack success for Malicious/SybilAttack (fraction of targeted issues flipped negative).
//   * Sybil amplification factor (effective power / actual budget for sybil actors).
// - `collect_stats()` method runs after tally, prints structured summary.
//
// Perfect for Eternal Thriving analysis: Quantify coordination power, sybil impact,
// and system robustness under different resistance modes.

use rand::prelude::*;
use std::collections::{HashMap, HashSet};

/// Voice credit budget per voter (f64)
type Credits = f64;

/// Votes on an issue (fractional)
type Votes = f64;

/// Issue identifier
type IssueId = u64;

/// Voter Strategy Enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrategyType {
    Random,
    Optimal,
    Coordinated,
    Malicious,
    SybilAttack,
}

/// Extract simple type for stats grouping
fn strategy_type(strategy: &Strategy) -> StrategyType {
    match strategy {
        Strategy::Random { .. } => StrategyType::Random,
        Strategy::Optimal => StrategyType::Optimal,
        Strategy::Coordinated { .. } => StrategyType::Coordinated,
        Strategy::Malicious { .. } => StrategyType::Malicious,
        Strategy::SybilAttack { .. } => StrategyType::SybilAttack,
    }
}

/// Simple Issue/Proposal
#[derive(Debug, Clone)]
struct Issue {
    id: IssueId,
    description: String,
}

/// Voter state
#[derive(Debug)]
struct Voter {
    id: u64,
    strategy: Strategy,
    original_budget: Credits,        // Track starting budget
    remaining_credits: Credits,
    allocations: HashMap<IssueId, Votes>,
}

impl Voter {
    fn new(id: u64, strategy: Strategy, budget: Credits) -> Self {
        Self {
            id,
            strategy: strategy.clone(),
            original_budget: budget,
            remaining_credits: budget,
            allocations: HashMap::new(),
        }
    }

    // allocate, optimal_allocate, etc. unchanged...
}

/// Stats Collector
#[derive(Debug, Default)]
pub struct StatsCollector {
    pub votes_by_strategy: HashMap<StrategyType, Votes>,       // Total absolute intensity contributed
    pub positive_votes_by_strategy: HashMap<StrategyType, Votes>,
    pub negative_votes_by_strategy: HashMap<StrategyType, Votes>,
    pub credits_spent_by_strategy: HashMap<StrategyType, Credits>,
    pub issue_outcomes: HashMap<IssueId, (Votes, bool)>,        // (net_votes, passed)
    pub malicious_targets_flipped: u32,
    pub malicious_targets_total: u32,
    pub sybil_amplification: f64,                              // Avg effective / actual for sybil actors
    pub sybil_count: u64,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn collect(&mut self, sim: &QVSimulator) {
        self.votes_by_strategy.clear();
        self.positive_votes_by_strategy.clear();
        self.negative_votes_by_strategy.clear();
        self.credits_spent_by_strategy.clear();
        self.issue_outcomes.clear();
        self.malicious_targets_flipped = 0;
        self.malicious_targets_total = 0;
        self.sybil_amplification = 0.0;
        self.sybil_count = 0;

        let mut malicious_targets: HashSet<IssueId> = HashSet::new();
        let mut sybil_effective_sum = 0.0;
        let mut sybil_actual_sum = 0.0;

        // Per-voter aggregation
        for voter in &sim.voters {
            let strat_type = strategy_type(&voter.strategy);
            let spent = voter.original_budget - voter.remaining_credits;
            *self.credits_spent_by_strategy.entry(strat_type.clone()).or_insert(0.0) += spent;

            let mut voter_intensity = 0.0;
            let mut voter_pos = 0.0;
            let mut voter_neg = 0.0;

            for (&_issue_id, &v) in &voter.allocations {
                let abs_v = v.abs();
                voter_intensity += abs_v;
                if v > 0.0 {
                    voter_pos += abs_v;
                } else {
                    voter_neg += abs_v;
                }
            }

            *self.votes_by_strategy.entry(strat_type.clone()).or_insert(0.0) += voter_intensity;
            *self.positive_votes_by_strategy.entry(strat_type.clone()).or_insert(0.0) += voter_pos;
            *self.negative_votes_by_strategy.entry(strat_type.clone()).or_insert(0.0) += voter_neg;

            // Track malicious/sybil specifics
            if let Strategy::Malicious { targets } | Strategy::SybilAttack { targets, .. } = &voter.strategy {
                for &target in targets {
                    malicious_targets.insert(target);
                }
            }

            if let Strategy::SybilAttack { num_sybils, .. } = voter.strategy {
                let effective = voter_intensity; // What they achieved
                let actual = (spent).sqrt();     // Approx without amplification
                sybil_effective_sum += effective;
                sybil_actual_sum += actual;
                self.sybil_count += num_sybils;
            }
        }

        // Issue outcomes
        for (&issue_id, &net_votes) in &sim.tallies {
            let passed = net_votes > 0.0;
            self.issue_outcomes.insert(issue_id, (net_votes, passed));

            if malicious_targets.contains(&issue_id) {
                self.malicious_targets_total += 1;
                if !passed {
                    self.malicious_targets_flipped += 1;
                }
            }
        }

        if self.sybil_count > 0 {
            self.sybil_amplification = sybil_effective_sum / sybil_actual_sum.max(1.0);
        }

        self.print_summary(&sim.issues);
    }

    pub fn print_summary(&self, issues: &[Issue]) {
        println!("\n=== Stats Collector Summary ===");
        println!("Votes Contributed (Absolute Intensity):");
        for (strat, intensity) in &self.votes_by_strategy {
            println!("  {:?}: {:.2}", strat, intensity);
        }
        println!("Credits Spent:");
        for (strat, spent) in &self.credits_spent_by_strategy {
            println!("  {:?}: {:.2}", strat, spent);
        }
        println!("Issue Outcomes:");
        for (&issue_id, &(net, passed)) in &self.issue_outcomes {
            let desc = &issues[issue_id as usize].description;
            println!("  Issue {} ({}): Net {:.2} → {}", issue_id, desc, net, if passed { "PASS" } else { "REJECT" });
        }
        if self.malicious_targets_total > 0 {
            let success_rate = self.malicious_targets_flipped as f64 / self.malicious_targets_total as f64 * 100.0;
            println!("Malicious/Sybil Attack Success: {:.1}% ({}/{}) targets flipped", success_rate, self.malicious_targets_flipped, self.malicious_targets_total);
        }
        if self.sybil_count > 0 {
            println!("Sybil Amplification Factor: {:.2}x (over {} identities)", self.sybil_amplification, self.sybil_count);
        }
    }
}

/// Quadratic Voting Simulator – With StatsCollector
#[derive(Debug)]
pub struct QVSimulator {
    // ... existing fields ...
    pub sybil_resistance_enabled: bool,
    pub stats: StatsCollector,
}

impl QVSimulator {
    pub fn new() -> Self {
        Self {
            // ... existing init ...
            sybil_resistance_enabled: true,
            stats: StatsCollector::new(),
        }
    }

    // After vote_according_to_strategy and tally:
    pub fn collect_and_print_stats(&mut self) {
        self.stats.collect(self);
    }
}

/// Example: Full demo with stats
fn main() {
    let mut sim = QVSimulator::new();

    // Issues...
    // Voters (mixed strategies including sybil)...

    sim.vote_according_to_strategy();
    sim.tally();
    sim.collect_and_print_stats();

    // Reset and run with resistance off
    // ...
}
