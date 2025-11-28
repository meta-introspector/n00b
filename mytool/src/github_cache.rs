use anyhow::{Result, Context};
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

use crate::cache::Cache;
use crate::github::{GitHubClient, RepoMetadata, UserMetadata, SearchResults}; // Import SearchResults

pub struct CachedGitHubClient {
    live_client: Arc<dyn GitHubClient + Send + Sync>, // Use Arc<dyn GitHubClient> for mockability
    cache: Cache,
}

impl CachedGitHubClient {
    pub fn new(live_client: Arc<dyn GitHubClient + Send + Sync>, cache_path: PathBuf) -> Result<Self> {
        let cache = Cache::new(cache_path).context("Failed to initialize GitHub cache")?;
        Ok(Self { live_client, cache })
    }

    fn user_cache_key(user: &str) -> String {
        format!("user:{}", user)
    }

    fn repo_cache_key(owner: &str, repo: &str) -> String {
        format!("repo:{}/{}", owner, repo)
    }

    fn org_repos_cache_key(org: &str) -> String {
        format!("org_repos:{}", org)
    }

    fn search_repos_cache_key(query: &str) -> String {
        format!("search_repos:{}", query)
    }

    fn starred_repos_cache_key(user: &str) -> String {
        format!("starred_repos:{}", user)
    }

    fn user_forked_repos_cache_key(user: &str) -> String {
        format!("user_forked_repos:{}", user)
    }

    fn get_repo_content_cache_key(owner: &str, repo: &str, path: &str) -> String {
        format!("repo_content:{}/{}/{}", owner, repo, path)
    }
}

#[async_trait]
impl GitHubClient for CachedGitHubClient {
    async fn list_org_repos(&self, org: &str) -> Result<Vec<RepoMetadata>> {
        let key = Self::org_repos_cache_key(org);
        if let Some(repos) = self.cache.get::<Vec<RepoMetadata>>(&key)? {
            return Ok(repos);
        }

        let repos = self.live_client.list_org_repos(org).await?;
        self.cache.put(&key, &repos)?;
        Ok(repos)
    }
    async fn get_user_info(&self, user: &str) -> Result<UserMetadata> {
        let key = Self::user_cache_key(user);
        if let Some(user_info) = self.cache.get::<UserMetadata>(&key)? {
            return Ok(user_info);
        }

        let user_info = self.live_client.get_user_info(user).await?;
        self.cache.put(&key, &user_info)?;
        Ok(user_info)
    }

    async fn get_repo_info(&self, owner: &str, repo: &str) -> Result<RepoMetadata> {
        let key = Self::repo_cache_key(owner, repo);
        if let Some(repo_info) = self.cache.get::<RepoMetadata>(&key)? {
            return Ok(repo_info);
        }

        let repo_info = self.live_client.get_repo_info(owner, repo).await?;
        self.cache.put(&key, &repo_info)?;
        Ok(repo_info)
    }

    async fn search_repositories(&self, query: &str) -> Result<SearchResults> {
        let key = Self::search_repos_cache_key(query);
        if let Some(search_results) = self.cache.get::<SearchResults>(&key)? {
            return Ok(search_results);
        }

        let search_results = self.live_client.search_repositories(query).await?;
        self.cache.put(&key, &search_results)?;
        Ok(search_results)
    }

    async fn list_starred_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        let key = Self::starred_repos_cache_key(user);
        if let Some(repos) = self.cache.get::<Vec<RepoMetadata>>(&key)? {
            return Ok(repos);
        }

        let repos = self.live_client.list_starred_repos(user).await?;
        self.cache.put(&key, &repos)?;
        Ok(repos)
    }

    async fn list_user_forked_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        let key = Self::user_forked_repos_cache_key(user);
        if let Some(repos) = self.cache.get::<Vec<RepoMetadata>>(&key)? {
            return Ok(repos);
        }

        let repos = self.live_client.list_user_forked_repos(user).await?;
        self.cache.put(&key, &repos)?;
        Ok(repos)
    }

    async fn get_repo_content(&self, owner: &str, repo: &str, path: &str) -> Result<String> {
        let key = Self::get_repo_content_cache_key(owner, repo, path);
        if let Some(content) = self.cache.get::<String>(&key)? {
            return Ok(content);
        }

        let content = self.live_client.get_repo_content(owner, repo, path).await?;
        self.cache.put(&key, &content)?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_github_client::MockGitHubClient;
    use tempfile::tempdir;
    use url::Url;

    // Helper to create a mock UserMetadata
    fn create_mock_user(login: &str) -> UserMetadata {
        UserMetadata {
            login: login.to_string(),
            id: 1,
            html_url: Url::parse(&format!("https://github.com/{}", login)).unwrap(),
            r#type: "User".to_string(),
            site_admin: false,
            name: Some(login.to_string()),
            company: None,
            blog: None,
            location: None,
            email: None,
            hireable: None,
            bio: None,
            twitter_username: None,
            public_repos: 10,
            followers: 20,
            following: 5,
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        }
    }

    // Helper to create a mock RepoMetadata
    fn create_mock_repo(owner: &str, name: &str) -> RepoMetadata {
        RepoMetadata {
            id: 2,
            owner: owner.to_string(),
            name: name.to_string(),
            html_url: Url::parse(&format!("https://github.com/{}/{}", owner, name)).unwrap(),
            description: Some("A test repo".to_string()),
            stargazers_count: 5,
            forks_count: 1,
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
            pushed_at: "2023-01-01T00:00:00Z".to_string(),
            language: Some("Rust".to_string()),
            license: None,
            topics: vec![],
            trust_score: 0.0,
        }
    }

    #[tokio::test]
    async fn test_cached_get_user_info_caches() -> Result<()> {
        let mock_client = Arc::new(MockGitHubClient::new());
        let user = "testuser";
        let mock_user_data = create_mock_user(user);
        mock_client.add_user(mock_user_data.clone());

        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cached_client = CachedGitHubClient::new(mock_client.clone(), cache_path)?;

        // First call - should hit the live client (mock) and cache
        let fetched_user = cached_client.get_user_info(user).await?;
        assert_eq!(fetched_user, mock_user_data);

        // Remove user from mock client to ensure cache is used
        mock_client.users.lock().unwrap().clear();

        // Second call - should hit the cache
        let fetched_user_from_cache = cached_client.get_user_info(user).await?;
        assert_eq!(fetched_user_from_cache, mock_user_data);

        Ok(())
    }

    #[tokio::test]
    async fn test_cached_get_repo_info_caches() -> Result<()> {
        let mock_client = Arc::new(MockGitHubClient::new());
        let owner = "testowner";
        let repo_name = "testrepo";
        let mock_repo_data = create_mock_repo(owner, repo_name);
        mock_client.add_repo(mock_repo_data.clone());

        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cached_client = CachedGitHubClient::new(mock_client.clone(), cache_path)?;

        // First call - should hit the live client (mock) and cache
        let fetched_repo = cached_client.get_repo_info(owner, repo_name).await?;
        assert_eq!(fetched_repo, mock_repo_data);

        // Remove repo from mock client to ensure cache is used
        mock_client.repos.lock().unwrap().clear();

        // Second call - should hit the cache
        let fetched_repo_from_cache = cached_client.get_repo_info(owner, repo_name).await?;
        assert_eq!(fetched_repo_from_cache, mock_repo_data);

        Ok(())
    }
}
