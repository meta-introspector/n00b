// Standard library imports
use std::sync::Arc;
use std::path::PathBuf;

// External crate imports
use anyhow::Result;
use actix_web::{web, App}; // Explicitly import App, web
use actix_web::test; // Explicitly import test module
use tempfile::tempdir;

// Crate-level imports
use crate::github::RepoMetadata;
use crate::mock_github_client::MockGitHubClient;
use crate::code_indexer::CodeIndexer;
use crate::github_cache::CachedGitHubClient; // Used in this test

// Internal module imports (relative to parent 'server' module)
use super::super::state::AppState;
use super::helpers::{create_mock_repo};
use super::super::handlers::{
    list_starred_repos_handler
};

#[actix_web::test]
async fn test_list_starred_repos_success() -> Result<()> {
    let mock_gh_client = Arc::new(MockGitHubClient::new());
    let username = "testuser";
    let repo1 = create_mock_repo("starred_owner", "starred_repo_1");
    let repo2 = create_mock_repo("starred_owner", "starred_repo_2");
    mock_gh_client.add_starred_repos(username.to_string(), vec![repo1.clone(), repo2.clone()]);

    let dir = tempdir()?;
    let cache_path = dir.path().to_path_buf();
    let cached_gh_client_mock = Arc::new(crate::github_cache::CachedGitHubClient::new(mock_gh_client, cache_path)?);
    let code_indexer_path = tempdir()?;
    let code_indexer = Arc::new(CodeIndexer::new(code_indexer_path.path().to_path_buf())?);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { github_client: cached_gh_client_mock.clone(), code_indexer }))
            .service(
                web::scope("/api/github")
                    .route("/users/{username}/starred_repos", web::get().to(list_starred_repos_handler))
            ),
    )
    .await;

    let req = test::TestRequest::get().uri(&format!("/api/github/users/{}/starred_repos", username)).to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let response_body: Vec<RepoMetadata> = test::read_body_json(resp).await;
    assert_eq!(response_body.len(), 2);
    assert_eq!(response_body[0].name, "starred_repo_1");
    assert_eq!(response_body[1].name, "starred_repo_2");

    Ok(())
}
