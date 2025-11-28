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
    get_repo
};

#[actix_web::test]
async fn test_get_repo_success() -> Result<()> {
    let mock_gh_client = Arc::new(MockGitHubClient::new());
    let owner = "testowner";
    let repo_name = "testrepo";
    let mock_repo_data = create_mock_repo(owner, repo_name);
    mock_gh_client.add_repo(mock_repo_data.clone());

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
                    .route("/repos/{owner}/{repo}", web::get().to(get_repo))
            ),
    )
    .await;

    let req = test::TestRequest::get().uri(&format!("/api/github/repos/{}/{}", owner, repo_name)).to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let response_body: RepoMetadata = test::read_body_json(resp).await;
    assert_eq!(response_body, mock_repo_data);

    Ok(())
}
