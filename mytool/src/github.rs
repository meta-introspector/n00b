//! GitHub Integration Module
//! Handles API calls to GitHub for repository discovery and metadata fetching.

use serde::{Deserialize, Serialize};
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn, error};
use reqwest::Client;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone here
pub struct RepoMetadata {
    pub id: i64,
    pub owner: String,
    pub name: String,
    pub html_url: Url,
    pub description: Option<String>,
    pub stargazers_count: i32,
    pub forks_count: i32,
    #[serde(default)] // Added default for deserialize
    pub created_at: String, // ISO 8601 timestamp
    #[serde(default)] // Added default for deserialize
    pub updated_at: String, // ISO 8601 timestamp
    #[serde(default)] // Added default for deserialize
    pub pushed_at: String,  // ISO 8601 timestamp (last push)
    pub language: Option<String>,
    pub license: Option<String>, // SPDX ID or similar
    #[serde(default)] // Added default for deserialize
    pub topics: Vec<String>,
    #[serde(default)] // Trust score is not from GitHub API, default to 0.0
    pub trust_score: f64, // <--- ADDED THIS LINE
    // Add more fields as needed for trust metric, e.g., open_issues_count, subscribers_count
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributorMetadata {
    pub login: String,
    pub id: i64,
    pub html_url: Url,
}

#[async_trait]
pub trait GitHubClient {
    async fn list_org_repos(&self, org: &str) -> Result<Vec<RepoMetadata>>;
    async fn list_user_repos(&self, user: &str) -> Result<Vec<RepoMetadata>>;
    async fn get_repo_contributors(&self, owner: &str, repo: &str) -> Result<Vec<ContributorMetadata>>;
    // Add method to fetch detailed repo stats if not available in basic listing
}

pub struct LiveGitHubClient {
    client: Client,
    github_token: String,
}

impl LiveGitHubClient {
    pub fn new(github_token: String) -> Self {
        LiveGitHubClient {
            client: Client::new(),
            github_token,
        }
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.github_token).parse().unwrap());
        headers.insert(reqwest::header::ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
        headers.insert(reqwest::header::USER_AGENT, "mytool-github-client".parse().unwrap());
        headers
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, url: &Url) -> Result<T> {
        info!("Making GitHub API request to: {}", url);
        let response = self.client.get(url.clone())
            .headers(self.build_headers())
            .send()
            .await?
            .error_for_status()?;

        let data: T = response.json().await?;
        Ok(data)
    }
}

#[async_trait]
impl GitHubClient for LiveGitHubClient {
    async fn list_org_repos(&self, org: &str) -> Result<Vec<RepoMetadata>> {
        let url = Url::parse(&format!("https://api.github.com/orgs/{}/repos", org))?;
        self.make_request(&url).await
    }

    async fn list_user_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        let url = Url::parse(&format!("https://api.github.com/users/{}/repos", user))?;
        self.make_request(&url).await
    }

    async fn get_repo_contributors(&self, owner: &str, repo: &str) -> Result<Vec<ContributorMetadata>> {
        let url = Url::parse(&format!("https://api.github.com/repos/{}/{}/contributors", owner, repo))?;
        self.make_request(&url).await
    }
}

// --- Helper function to filter repositories based on keywords ---
pub fn filter_repos(repos: Vec<RepoMetadata>, filters: &[&str]) -> Vec<RepoMetadata> {
    if filters.is_empty() {
        return repos;
    }

    repos.into_iter().filter(|repo| {
        let name_match = filters.iter().any(|f| repo.name.contains(f));
        let description_match = repo.description.as_ref().map_or(false, |desc| filters.iter().any(|f| desc.contains(f)));
        let topic_match = repo.topics.iter().any(|topic| filters.iter().any(|f| topic.contains(f)));
        let language_match = repo.language.as_ref().map_or(false, |lang| filters.iter().any(|f| lang.to_lowercase().contains(f)));

        name_match || description_match || topic_match || language_match
    }).collect()
}