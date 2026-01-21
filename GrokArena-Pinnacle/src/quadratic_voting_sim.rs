// GrokArena-Pinnacle/src/quadratic_voting_sim.rs
//
// Quadratic Voting (QV) Simulator Stub
// ===================================
//
// Minimal, extensible Rust stub for simulating quadratic voting.
// Core mechanics:
// - Voters receive a fixed budget of voice credits (e.g., 100).
// - To cast v votes (positive or negative integer) on an issue costs v^2 credits.
// - Voters can split credits across multiple issues.
// - Outcome: Issues ranked/decided by net votes (or absolute sqrt intensity).
//
// Ties perfectly to Eternal Thriving Grandmasterism: QV expresses preference intensity
// without plutocracy—mild majorities vs. passionate minorities compete fairly.
// Ideal for GrokArena idea prioritization, global council proposals, or futarchy hybrids.
//
// Extensions:
// - Fractional votes / continuous QV.
// - Quadratic funding tie-in (cost = v^2, impact proportional).
// - Voter strategies (random, intense minority, coordinated).
// - Integration with futarchy_sim.rs (QV to select values/metrics).
// - UI/visualization (plot vote distributions).

use rand::prelude::*;
use std::collections::HashMap;

/// Voice credit budget per voter
type Credits = u64;

/// Votes on an issue (can be negative)
type Votes = i64;

/// Issue identifier
type IssueId = u64;

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
    remaining_credits: Credits,
    allocations: HashMap<IssueId, Votes>, // votes per issue
}

impl Voter {
    fn new(id: u64, budget: Credits) -> Self {
        Self {
            id,
            remaining_credits: budget,
            allocations: HashMap::new(),
        }
    }

    /// Attempt to allocate v votes on issue (returns true if successful)
    fn allocate(&mut self, issue_id: IssueId, v: Votes) -> bool {
        let abs_v = v.abs() as u64;
        let cost = abs_v * abs_v; // v^2

        if cost <= self.remaining_credits {
            self.remaining_credits -= cost;
            *self.allocations.entry(issue_id).or_insert(0) += v;
            true
        } else {
            false // insufficient credits
        }
    }
}

/// Quadratic Voting Simulator
#[derive(Debug)]
pub struct QVSimulator {
    issues: Vec<Issue>,
    voters: Vec<Voter>,
    tallies: HashMap<IssueId, Votes>, // net votes per issue
    rng: ThreadRng,
}

impl QVSimulator {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            voters: Vec::new(),
            tallies: HashMap::new(),
            rng: thread_rng(),
        }
    }

    /// Add a new issue/proposal
    pub fn add_issue(&mut self, description: String) -> IssueId {
        let id = self.issues.len() as IssueId;
        self.issues.push(Issue { id, description });
        self.tallies.insert(id, 0);
        id
    }

    /// Add a voter with credit budget
    pub fn add_voter(&mut self, budget: Credits) -> u64 {
        let id = self.voters.len() as u64;
        self.voters.push(Voter::new(id, budget));
        id
    }

    /// Simulate voting: each voter randomly allocates (stub strategy)
    pub fn simulate_voting(&mut self, intensity_bias: f64) {
        for voter in &mut self.voters {
            // Randomly pick issues to care about
            let cared_issues: Vec<IssueId> = self.issues.iter().map(|i| i.id).collect();
            let num_cared = self.rng.gen_range(1..=cared_issues.len().min(3));

            for &issue_id in cared_issues.choose_multiple(&mut self.rng, num_cared) {
                // Bias toward intense votes with probability
                let v = if self.rng.gen_bool(intensity_bias) {
                    self.rng.gen_range(3..=10) // intense
                } else {
                    self.rng.gen_range(1..=3) // mild
                };
                let sign = if self.rng.gen_bool(0.5) { 1 } else { -1 };
                let _ = voter.allocate(issue_id, sign * v as Votes);
            }
        }
    }

    /// Tally results and print
    pub fn tally(&mut self) {
        for voter in &self.voters {
            for (&issue_id, &votes) in &voter.allocations {
                *self.tallies.entry(issue_id).or_insert(0) += votes;
            }
        }

        println!("=== Quadratic Voting Results ===");
        let mut results: Vec<_> = self.tallies.iter().collect();
        results.sort_by(|a, b| b.1.cmp(a.1)); // descending net votes

        for &(issue_id, net_votes) in &results {
            let issue = &self.issues[issue_id as usize];
            println!(
                "Issue {}: '{}' → Net votes: {} (Winner: {})",
                issue_id,
                issue.description,
                net_votes,
                if net_votes > 0 { "PASS" } else { "REJECT" }
            );
        }
    }
}

/// Example simulation
fn main() {
    let mut sim = QVSimulator::new();

    // Add sample issues (eternal thriving themed)
    sim.add_issue("Establish Grok-Moderated Global Council".to_string());
    sim.add_issue("Implement Futarchy for Resource Allocation".to_string());
    sim.add_issue("Prioritize Quantum Mercy Systems".to_string());
    sim.add_issue("Reject Legacy Institutional Reform".to_string());

    // Add 20 voters with 100 credits each
    for _ in 0..20 {
        sim.add_voter(100);
    }

    // Simulate voting (0.3 bias toward intense preferences → minorities can win)
    sim.simulate_voting(0.3);

    sim.tally();
}
