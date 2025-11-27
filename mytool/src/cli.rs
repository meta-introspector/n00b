use clap::{Parser, Subcommand};
use log::{info, error};
use std::env;
use crate::github::{LiveGitHubClient, RepoMetadata, filter_repos, GitHubClient}; // Added GitHubClient
use crate::repo_index::RepoIndex;
use crate::trust_metric::calculate_trust_score;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Discover GitHub repositories based on criteria
    Discover {
        #[arg(short, long, default_value = "meta-introspector,jmikedupont2")]
        orgs: String,
        #[arg(short, long, default_value = "rust,terraform,nix,rustc,mcp")]
        filters: String,
        #[arg(short, long, default_value = "true")]
        include_contributors_repos: bool,
    },
    /// List discovered repositories
    List {
        #[arg(short, long, default_value = "50")]
        limit: usize,
        #[arg(short, long)]
        min_trust_score: Option<f64>,
    },
    /// Calculate trust metrics for all discovered repositories
    CalculateTrust,
}

pub async fn run_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let github_token = env::var("GITHUB_TOKEN").map_err(|_| {
        anyhow::anyhow!("GITHUB_TOKEN environment variable not set. Please set it to your GitHub Personal Access Token.")
    })?;
    let github_client = LiveGitHubClient::new(github_token);
    let repo_index = RepoIndex::new("repositories.db")?;

    match &cli.command {
        Commands::Discover { orgs, filters, include_contributors_repos } => {
            info!("Discovering repositories from organizations: {}, with filters: {:?}", orgs, filters);
            info!("Include contributors' repositories: {}", include_contributors_repos);

            let org_names: Vec<&str> = orgs.split(',').map(|s| s.trim()).collect();
            let filter_keywords: Vec<&str> = filters.split(',').map(|s| s.trim()).collect();
            let mut all_discovered_repos: Vec<RepoMetadata> = Vec::new();
            let mut processed_users: std::collections::HashSet<String> = std::collections::HashSet::new();

            for org_name in org_names {
                match github_client.list_org_repos(org_name).await {
                    Ok(repos) => {
                        info!("Found {} repositories in organization {}", repos.len(), org_name);
                        for repo in repos {
                            all_discovered_repos.push(repo);
                        }
                    },
                    Err(e) => error!("Failed to list repositories for organization {}: {}", org_name, e),
                }
            }

            if *include_contributors_repos {
                let current_repos = all_discovered_repos.clone(); // Clone to iterate without mutable borrow issues
                for repo in current_repos {
                    match github_client.get_repo_contributors(&repo.owner, &repo.name).await {
                        Ok(contributors) => {
                            info!("Found {} contributors for {}/{}", contributors.len(), repo.owner, repo.name);
                            for contributor in contributors {
                                if processed_users.insert(contributor.login.clone()) { // Only process user once
                                    match github_client.list_user_repos(&contributor.login).await {
                                        Ok(user_repos) => {
                                            info!("Found {} repositories for user {}", user_repos.len(), contributor.login);
                                            for user_repo in user_repos {
                                                all_discovered_repos.push(user_repo);
                                            }
                                        },
                                        Err(e) => error!("Failed to list repositories for user {}: {}", contributor.login, e),
                                    }
                                }
                            }
                        },
                        Err(e) => error!("Failed to get contributors for {}/{}: {}", repo.owner, repo.name, e),
                    }
                }
            }

            let filtered_repos = filter_repos(all_discovered_repos, &filter_keywords);
            info!("Discovered and filtered {} unique repositories.", filtered_repos.len());

            for repo in filtered_repos {
                repo_index.insert_repo(&repo)?;
            }

            println!("Discovery complete. {} unique repositories indexed.", repo_index.get_all_repos()?.len());
        },
        Commands::List { limit, min_trust_score } => {
            info!("Listing up to {} repositories with minimum trust score: {:?}", limit, min_trust_score);
            let mut repos = repo_index.get_all_repos()?;
            
            // Sort by trust score (descending)
            repos.sort_by(|a, b| b.trust_score.partial_cmp(&a.trust_score).unwrap_or(std::cmp::Ordering::Equal));

            for repo in repos.iter().filter(|r| min_trust_score.map_or(true, |score| r.trust_score >= score)).take(*limit) {
                println!("- {}/{}: {} (Trust Score: {:.2})", repo.owner, repo.name, repo.html_url, repo.trust_score);
            }
            println!("Listed {} repositories.", repos.len());
        },
        Commands::CalculateTrust => {
            info!("Calculating trust metrics for all repositories.");
            let repos_to_update = repo_index.get_all_repos()?;
            for repo in repos_to_update {
                let score = calculate_trust_score(&repo);
                repo_index.update_trust_score(repo.id, score)?;
            }
            println!("Trust metric calculation complete for {} repositories.", repo_index.get_all_repos()?.len());
        },
    }

    Ok(())
}
