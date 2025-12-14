// Filename: oxalslien.rs
// Purpose: Minimal ALIEN-inspired claim verifier simulation
// Build & run: `rustc oxalslien.rs && ./oxalslien`

use rand::Rng;
use std::io;

// Struct to hold a claim
struct Claim {
    description: String,
    provers: u32,
    retry_budget: u32,
    success_prob: f64,
}

// ALIEN-inspired constants
const COMPLETENESS: f64 = 0.99; // _c
const VRF_BIAS: f64 = 0.001; // _VRF
const BFT_FAIL: f64 = 0.0001; // _BFT

impl Claim {
    fn new(description: String, provers: u32, retry_budget: u32, success_prob: f64) -> Claim {
        Claim {
            description,
            provers,
            retry_budget,
            success_prob,
        }
    }

    // Compute acceptance probability using ALIEN-style formula
    fn acceptance_probability(&self, rounds: u32) -> f64 {
        let p_retry =
            1.0 - (1.0 - self.success_prob).powf((self.provers * (self.retry_budget + 1)) as f64);
        let mut prob_accept = 1.0 - COMPLETENESS;
        prob_accept *= 1.0 - VRF_BIAS;
        prob_accept *= (1.0 - p_retry).powf(rounds as f64);
        prob_accept = 1.0 - prob_accept - BFT_FAIL;
        prob_accept = prob_accept.clamp(0.0, 1.0);
        prob_accept
    }

    // Simulate provability level (n) and finality
    fn provability_level(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=5) // n = 1..5
    }
}

fn main() {
    println!("Welcome to OxalSLIEN - ALIEN-inspired Claim Simulator");

    let mut input = String::new();
    println!("Enter your claim description:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let description = input.trim().to_string();

    // Parameters (fixed for simplicity)
    let provers = 3;
    let retry_budget = 1;
    let success_prob = 0.95;
    let rounds = 5;

    let claim = Claim::new(description, provers, retry_budget, success_prob);

    println!("\nSimulating ALIEN verification...");
    let prob = claim.acceptance_probability(rounds);
    let level = claim.provability_level();

    println!("Claim: {}", claim.description);
    println!(
        "Acceptance Probability after {} rounds: {:.5}",
        rounds, prob
    );
    println!("Provability Level n: {}", level);

    if prob > 0.99 {
        println!("Status: Highly likely to finalize!");
    } else if prob > 0.7 {
        println!("Status: Likely to finalize");
    } else {
        println!("Status: Low probability of finalization");
    }
}
