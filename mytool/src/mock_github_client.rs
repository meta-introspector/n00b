use anyhow::{Result, Context};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::github::{GitHubClient, RepoMetadata, UserMetadata, SearchResults}; // Import SearchResults

// A simple in-memory mock for GitHubClient
pub struct MockGitHubClient {
    pub users: Arc<Mutex<HashMap<String, UserMetadata>>>,
    pub repos: Arc<Mutex<HashMap<(String, String), RepoMetadata>>>, // (owner, repo_name)
    pub org_repos: Arc<Mutex<HashMap<String, Vec<RepoMetadata>>>>, // org -> Vec<RepoMetadata>
    pub search_results: Arc<Mutex<HashMap<String, SearchResults>>>, // query -> SearchResults
    pub starred_repos: Arc<Mutex<HashMap<String, Vec<RepoMetadata>>>>, // user -> Vec<RepoMetadata>
    pub user_forked_repos: Arc<Mutex<HashMap<String, Vec<RepoMetadata>>>>, // user -> Vec<RepoMetadata>
    pub repo_contents: Arc<Mutex<HashMap<(String, String, String), String>>>, // (owner, repo, path) -> content
}

impl Default for MockGitHubClient {
    fn default() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            repos: Arc::new(Mutex::new(HashMap::new())),
            org_repos: Arc::new(Mutex::new(HashMap::new())),
            search_results: Arc::new(Mutex::new(HashMap::new())),
            starred_repos: Arc::new(Mutex::new(HashMap::new())),
            user_forked_repos: Arc::new(Mutex::new(HashMap::new())),
            repo_contents: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MockGitHubClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(&self, user: UserMetadata) {
        self.users.lock().unwrap().insert(user.login.clone(), user);
    }

    pub fn add_repo(&self, repo: RepoMetadata) {
        self.repos.lock().unwrap().insert((repo.owner.clone(), repo.name.clone()), repo);
    }

    pub fn add_org_repos(&self, org: String, repos: Vec<RepoMetadata>) {
        self.org_repos.lock().unwrap().insert(org, repos);
    }

    pub fn add_search_results(&self, query: String, results: SearchResults) {
        self.search_results.lock().unwrap().insert(query, results);
    }

    pub fn add_starred_repos(&self, user: String, repos: Vec<RepoMetadata>) {
        self.starred_repos.lock().unwrap().insert(user, repos);
    }

    pub fn add_user_forked_repos(&self, user: String, repos: Vec<RepoMetadata>) {
        self.user_forked_repos.lock().unwrap().insert(user, repos);
    }

    pub fn add_repo_content(&self, owner: String, repo: String, path: String, content: String) {
        self.repo_contents.lock().unwrap().insert((owner, repo, path), content);
    }
}

#[async_trait]
impl GitHubClient for MockGitHubClient {
    async fn list_org_repos(&self, org: &str) -> Result<Vec<RepoMetadata>> {
        self.org_repos.lock().unwrap().get(org).cloned().context(format!("Mock org repos not found: {}", org))
    }

    async fn get_user_info(&self, user: &str) -> Result<UserMetadata> {
        self.users.lock().unwrap().get(user).cloned().context(format!("Mock user not found: {}", user))
    }

    async fn get_repo_info(&self, owner: &str, repo: &str) -> Result<RepoMetadata> {
        self.repos.lock().unwrap().get(&(owner.to_string(), repo.to_string())).cloned().context(format!("Mock repo not found: {}/{}", owner, repo))
    }

    async fn search_repositories(&self, query: &str) -> Result<SearchResults> {
        self.search_results.lock().unwrap().get(query).cloned().context(format!("Mock search results not found for query: {}", query))
    }

    async fn list_starred_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        self.starred_repos.lock().unwrap().get(user).cloned().context(format!("Mock starred repos not found for user: {}", user))
    }

    async fn list_user_forked_repos(&self, user: &str) -> Result<Vec<RepoMetadata>> {
        self.user_forked_repos.lock().unwrap().get(user).cloned().context(format!("Mock forked repos not found for user: {}", user))
    }

    async fn get_repo_content(&self, owner: &str, repo: &str, path: &str) -> Result<String> {
        self.repo_contents.lock().unwrap().get(&(owner.to_string(), repo.to_string(), path.to_string())).cloned().context(format!("Mock repo content not found for {}/{}/{}", owner, repo, path))
    }
}
