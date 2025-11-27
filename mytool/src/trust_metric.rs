//! Trust Metric Calculation Module
//! Defines criteria and calculates a trust score for GitHub repositories.

use crate::github::RepoMetadata;
use log::info;

pub fn calculate_trust_score(repo: &RepoMetadata) -> f64 {
    let mut score = 0.0;

    // --- Factors based on discussion ---
    // 1. Stars (more stars = higher trust)
    score += repo.stargazers_count as f64 * 0.1; // Weight 0.1 per star

    // 2. Forks (more forks = higher trust)
    score += repo.forks_count as f64 * 0.2; // Weight 0.2 per fork

    // 3. Activity (recently pushed = higher trust)
    // This would require parsing date strings and comparing,
    // for simplicity, we'll give a small bonus if updated recently (e.g., last year)
    // Real implementation would parse 'pushed_at' and compare to current date.
    // Placeholder for now: assume always active.
    score += 5.0; // Small constant bonus for activity

    // 4. Language match (bonus for specific languages like Rust)
    if let Some(lang) = &repo.language {
        if lang.to_lowercase() == "rust" {
            score += 10.0; // Significant bonus for Rust
        }
    }

    // 5. Topics (bonus for relevant topics)
    for topic in &repo.topics {
        if topic.contains("nix") || topic.contains("terraform") || topic.contains("mcp") || topic.contains("llvm") || topic.contains("lean4") {
            score += 2.0; // Small bonus for relevant topics
        }
    }

    // --- Add other factors based on future needs ---
    // E.g., presence of a license, number of contributors (requires more API calls)

    info!("Calculated trust score for {}/{} is {}", repo.owner, repo.name, score);
    score
}
