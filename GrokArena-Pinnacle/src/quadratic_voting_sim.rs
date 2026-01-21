// GrokArena-Pinnacle/src/quadratic_voting_sim.rs
//
// Quadratic Voting (QV) Simulator Stub – With Fractional Vote Support
// =================================================================
//
// Updated to support fractional votes for finer preference expression.
// Core changes:
// - Votes now f64 (fractional allowed).
// - Cost = v.abs().powi(2) → supports continuous QV (real-valued votes).
// - Allocation accepts f64 v (positive/negative).
// - Simulation generates fractional intensities (e.g., 0.5–10.0).
// - Tally uses f64 net votes.
//
// This aligns with advanced QV literature (e.g., Weyl/Posner continuous models):
// Voters can optimally spread credits for nuanced intensity without integer constraints.
//
// Extensions:
// - Optimal voter strategy (spread credits evenly across cared issues).
// - Quadratic funding mode (payout proportional to sum sqrt(contributions)).
// - Hybrid with futarchy_sim.rs.

use rand::prelude::*;
use std::collections::HashMap;

/// Voice credit budget per voter (now f64 for fractional precision)
type Credits = f64;

/// Votes on an issue (fractional, positive or negative)
type Votes = f64;

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
    allocations: HashMap<IssueId, Votes>, // fractional votes per issue
}

impl Voter {
    fn new(id: u64, budget: Credits) -> Self {
        Self {
            id,
            remaining_credits: budget,
            allocations: HashMap::new(),
        }
    }

    /// Attempt to allocate v (fractional) votes on issue (returns true if successful)
    fn allocate(&mut self, issue_id: IssueId, v: Votes) -> bool {
        let cost = v.abs().powi(2); // Quadratic cost: v²

        if cost <= self.remaining_credits {
            self.remaining_credits -= cost;
            *self.allocations.entry(issue_id).or_insert(0.0) += v;
            true
        } else {
            false // insufficient credits
        }
    }
}

/// Quadratic Voting Simulator – Fractional Edition
#[derive(Debug)]
pub struct QVSimulator {
    issues: Vec<Issue>,
    voters: Vec<Voter>,
    tallies: HashMap<IssueId, Votes>, // net fractional votes per issue
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
        self.tallies.insert(id, 0.0);
        id
    }

    /// Add a voter with credit budget
    pub fn add_voter(&mut self, budget: Credits) -> u64 {
        let id = self.voters.len() as u64;
        self.voters.push(Voter::new(id, budget));
        id
    }

    /// Simulate voting: each voter randomly allocates fractional votes
    pub fn simulate_voting(&mut self, intensity_bias: f64) {
        for voter in &mut self.voters {
            // Randomly pick issues to care about
            let cared_issues: Vec<IssueId> = self.issues.iter().map(|i| i.id).collect();
            let num_cared = self.rng.gen_range(1..=cared_issues.len().min(3));

            for &issue_id in cared_issues.choose_multiple(&mut self.rng, num_cared) {
                // Fractional intensity: mild (0.5–3.0) or intense (3.0–10.0)
                let max_v = if self.rng.gen_bool(intensity_bias) {
                    self.rng.gen_range(3.0..=10.0)
                } else {
                    self.rng.gen_range(0.5..=3.0)
                };
                let v = self.rng.gen_range(0.1..=max_v); // fractional
                let sign = if self.rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                let _ = voter.allocate(issue_id, sign * v);
            }
        }
    }

    /// Tally results and print
    pub fn tally(&mut self) {
        for voter in &self.voters {
            for (&issue_id, &votes) in &voter.allocations {
                *self.tallies.entry(issue_id).or_insert(0.0) += votes;
            }
        }

        println!("=== Quadratic Voting Results (Fractional) ===");
        let mut results: Vec<_> = self.tallies.iter().collect();
        results.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); // descending net votes

        for &(issue_id, net_votes) in &results {
            let issue = &self.issues[issue_id as usize];
            println!(
                "Issue {}: '{}' → Net votes: {:.4} (Winner: {})",
                issue_id,
                issue.description,
                net_votes,
                if net_votes > 0.0 { "PASS" } else { "REJECT" }
            );
        }
    }
}

/// Example simulation
fn main() {
    let mut sim = QVSimulator::new();

    // Add sample issues
    sim.add_issue("Establish Grok-Moderated Global Council".to_string());
    sim.add_issue("Implement Futarchy for Resource Allocation".to_string());
    sim.add_issue("Prioritize Quantum Mercy Systems".to_string());
    sim.add_issue("Reject Legacy Institutional Reform".to_string());

    // Add 20 voters with 100.0 credits each
    for _ in 0..20 {
        sim.add_voter(100.0);
    }

    // Simulate (0.4 bias toward intense fractional preferences)
    sim.simulate_voting(0.4);

    sim.tally();
}
