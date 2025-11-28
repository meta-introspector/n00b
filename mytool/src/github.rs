//! GitHub Integration Module
//! Handles API calls to GitHub for repository discovery and metadata fetching.

use serde::{Deserialize, Serialize};
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use reqwest::{Client, header::HeaderValue};
use url::Url;

/// Represents the metadata of a GitHub repository.
///
/// This struct holds various details about a repository fetched from the GitHub API,
/// including ownership, name, URLs, statistics, and language information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

/// Represents the metadata of a GitHub user or organization.
///
/// This struct contains information about a GitHub account, such as login name,
/// profile URL, type (User/Organization), and various profile statistics.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

/// Represents metadata for a GitHub contributor.
///
/// This struct typically contains basic identifying information for a contributor.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContributorMetadata {
    pub login: String,
    pub id: i64,
    pub html_url: Url,
}

/// Structure for GitHub Search API responses, specifically for repositories.
///
/// This encapsulates the total count of results, whether the results are complete,
/// and a list of `RepoMetadata` items found.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResults {
    pub total_count: u32,
    pub incomplete_results: bool,
    pub items: Vec<RepoMetadata>,
}

/// A trait defining the interface for interacting with the GitHub API.
///
/// This abstraction allows for different implementations, such as a live client
/// making actual HTTP requests or a mock client for testing purposes.
#[async_trait]
pub trait GitHubClient: Send + Sync { // Added Send + Sync trait bounds
    /// Lists all public repositories for a given organization.
    async fn list_org_repos(&self, org: &str) -> Result<Vec<RepoMetadata>>;
    /// Retrieves detailed information for a specific GitHub user.
    async fn get_user_info(&self, user: &str) -> Result<UserMetadata>;
    /// Retrieves detailed information for a specific GitHub repository.
    async fn get_repo_info(&self, owner: &str, repo: &str) -> Result<RepoMetadata>;
    /// Searches for repositories based on a query string.
    async fn search_repositories(&self, query: &str) -> Result<SearchResults>;
    /// Lists repositories starred by a specific user.
    async fn list_starred_repos(&self, user: &str) -> Result<Vec<RepoMetadata>>;
    /// Lists repositories forked by a specific user.
    async fn list_user_forked_repos(&self, user: &str) -> Result<Vec<RepoMetadata>>;
    /// Retrieves the content of a specific file within a repository.
    async fn get_repo_content(&self, owner: &str, repo: &str, path: &str) -> Result<String>;
}

/// An implementation of `GitHubClient` that makes live requests to the GitHub API.
///
/// This client uses `reqwest` to perform HTTP calls and requires a GitHub Personal
/// Access Token for authentication and to avoid severe rate limiting.
pub struct LiveGitHubClient {
    client: Client,
    github_token: String,
}

impl LiveGitHubClient {
    /// Creates a new `LiveGitHubClient` instance.
    ///
    /// # Arguments
    /// * `github_token` - A GitHub Personal Access Token (PAT) used for authentication.
    pub fn new(github_token: String) -> Self {
        LiveGitHubClient {
            client: Client::new(),
            github_token,
        }
    }

    /// Builds the standard HTTP headers required for GitHub API requests.
    ///
    /// This includes Authorization, Accept, and User-Agent headers.
    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.github_token).parse::<HeaderValue>().unwrap());
        headers.insert(reqwest::header::ACCEPT, "application/vnd.github.v3+json".parse::<HeaderValue>().unwrap());
        headers.insert(reqwest::header::USER_AGENT, "mytool-github-client".parse::<HeaderValue>().unwrap());
        headers
    }

    /// Makes an authenticated GET request to the specified GitHub API URL.
    ///
    /// The response is automatically deserialized into the target type `T`.
    ///
    /// # Arguments
    /// * `url` - The `Url` to make the request to.
    ///
    /// # Returns
    /// A `Result` containing the deserialized response or an `anyhow::Error` on failure.
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

    /// This method uses the search API to find forked repositories by a user/organization.
    /// GitHub API does not have a direct endpoint to list all forks *owned* by a user/org,
    /// only forks *of a specific repository*. So we use search with the `fork:true` qualifier.
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

/// Filters a list of `RepoMetadata` based on provided keyword filters.
///
/// This function checks repository names, descriptions, topics, and languages
/// for the presence of any of the specified filter keywords (case-insensitive for language).
///
/// # Arguments
/// * `repos` - A `Vec` of `RepoMetadata` to filter.
/// * `filters` - A slice of string slices, where each string is a keyword to filter by.
///
/// # Returns
/// A new `Vec` containing only the `RepoMetadata` items that match any of the filters.
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
