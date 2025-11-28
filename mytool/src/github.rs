//! GitHub Integration Module
//! Handles API calls to GitHub for repository discovery and metadata fetching.

use serde::{Deserialize, Serialize};
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use reqwest::{Client, header::HeaderValue};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] // Added PartialEq here
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] // Added PartialEq here
pub struct UserMetadata {
    pub login: String,
    pub id: i64,
    pub html_url: Url,
    pub r#type: String, // 'type' is a reserved keyword, use r#type
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] // Added Clone and PartialEq here
pub struct ContributorMetadata {
    pub login: String,
    pub id: i64,
    pub html_url: Url,
}

// Structure for GitHub Search API responses
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResults {
    pub total_count: u32,
    pub incomplete_results: bool,
    pub items: Vec<RepoMetadata>,
}

#[async_trait]
pub trait GitHubClient {
    async fn list_org_repos(&self, org: &str) -> Result<Vec<RepoMetadata>>;
    async fn get_user_info(&self, user: &str) -> Result<UserMetadata>;
    async fn get_repo_info(&self, owner: &str, repo: &str) -> Result<RepoMetadata>;
    async fn search_repositories(&self, query: &str) -> Result<SearchResults>;
    async fn list_starred_repos(&self, user: &str) -> Result<Vec<RepoMetadata>>;
    async fn list_user_forked_repos(&self, user: &str) -> Result<Vec<RepoMetadata>>;
    async fn get_repo_content(&self, owner: &str, repo: &str, path: &str) -> Result<String>;
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
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.github_token).parse::<HeaderValue>().unwrap());
        headers.insert(reqwest::header::ACCEPT, "application/vnd.github.v3+json".parse::<HeaderValue>().unwrap());
        headers.insert(reqwest::header::USER_AGENT, "mytool-github-client".parse::<HeaderValue>().unwrap());
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

    async fn get_user_info(&self, user: &str) -> Result<UserMetadata> {
        let url = Url::parse(&format!("https://api.github.com/users/{}", user))?;
        self.make_request(&url).await
    }

    async fn get_repo_info(&self, owner: &str, repo: &str) -> Result<RepoMetadata> {
        let url = Url::parse(&format!("https://api.github.com/repos/{}/{}", owner, repo))?;
        self.make_request(&url).await
    }

    async fn search_repositories(&self, query: &str) -> Result<SearchResults> {
        let url = Url::parse(&format!("https://api.github.com/search/repositories?q={}", query))?;
        self.make_request(&url).await
    }

    async fn list_starred_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        let url = Url::parse(&format!("https://api.github.com/users/{}/starred", user))?;
        self.make_request(&url).await
    }

    // This method uses the search API to find forked repositories by a user/organization
    // GitHub API does not have a direct endpoint to list all forks *owned* by a user/org,
    // only forks *of a specific repository*. So we use search with the fork:true qualifier.
    async fn list_user_forked_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        // Search for repositories owned by the user and are forks
        let query = format!("user:{} fork:true", user);
        let search_results = self.search_repositories(&query).await?;
        Ok(search_results.items)
    }

    async fn get_repo_content(&self, owner: &str, repo: &str, path: &str) -> Result<String> {
        // GitHub API for content can return a JSON object with base64 encoded content
        // or a redirect to the raw file. We'll try to get the raw content directly.
        let url = Url::parse(&format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path))?;

        info!("Making GitHub API request for content to: {}", url);
        let response = self.client.get(url.clone())
            .headers(self.build_headers())
            .header(reqwest::header::ACCEPT, "application/vnd.github.raw".parse::<HeaderValue>().unwrap()) // Request raw content
            .send()
            .await?
            .error_for_status()?;

        // If GitHub responds with a redirect to raw.githubusercontent.com, reqwest follows it
        // If it responds with raw content directly, that's fine too.
        let content = response.text().await?;
        Ok(content)
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
