use crate::github::{UserMetadata, RepoMetadata};
use url::Url;

// Helper to create a mock UserMetadata (re-used from github_cache tests)
pub fn create_mock_user(login: &str) -> UserMetadata {
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

// Helper to create a mock RepoMetadata (re-used from github_cache tests)
pub fn create_mock_repo(owner: &str, name: &str) -> RepoMetadata {
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
