// Standard library imports
use std::sync::Arc;
use std::path::PathBuf;

// External crate imports
use anyhow::Result;
use actix_web::{web, App}; // Explicitly import App, web
use actix_web::test; // Explicitly import test module
use tempfile::tempdir;

// Crate-level imports
use crate::github::{RepoMetadata, SearchResults};
use crate::mock_github_client::MockGitHubClient;
use crate::code_indexer::CodeIndexer;
use crate::github_cache::CachedGitHubClient; // Used in this test

// Internal module imports (relative to parent 'server' module)
use super::super::state::AppState;
use super::helpers::{create_mock_repo};
use super::super::handlers::{
    search_repositories_handler
};

#[actix_web::test]
async fn test_search_repositories_success() -> Result<()> {
    let mock_gh_client = Arc::new(MockGitHubClient::new());
    let query = "rust+mcp";
    let repo1 = create_mock_repo("owner1", "rust-mcp-server");
    let repo2 = create_mock_repo("owner2", "another-mcp-project");
    let search_results = SearchResults {
        total_count: 2,
        incomplete_results: false,
        items: vec![repo1.clone(), repo2.clone()],
    };
    mock_gh_client.add_search_results(query.to_string(), search_results.clone());

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
                    .route("/search/repos", web::get().to(search_repositories_handler))
            ),
    )
    .await;

    let req = test::TestRequest::get().uri(&format!("/api/github/search/repos?q={}", query)).to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let response_body: Vec<RepoMetadata> = test::read_body_json(resp).await;
    assert_eq!(response_body.len(), 2);
    assert_eq!(response_body[0].name, "rust-mcp-server");
    assert_eq!(response_body[1].name, "another-mcp-project");

    Ok(())
}
