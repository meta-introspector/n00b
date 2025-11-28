use super::super::super::prelude::*;
use super::super::super::state::AppState;
use super::super::super::handlers::health_check_handler::health_check; // Correct path to health_check

use actix_web::{test, web};
use tempfile::tempdir;
use crate::mock_github_client::MockGitHubClient;
use crate::code_indexer::CodeIndexer;
use crate::github_cache::CachedGitHubClient;

#[actix_web::test]
async fn test_health_check_success() -> Result<()> {
    // Setup a minimal AppState for the test
    let mock_gh_client = Arc::new(MockGitHubClient::new());
    let cache_path = tempdir()?.path().to_path_buf();
    let cached_gh_client = Arc::new(CachedGitHubClient::new(mock_gh_client.clone(), cache_path)?);
    let code_indexer_path = tempdir()?.path().to_path_buf();
    let code_indexer = Arc::new(CodeIndexer::new(code_indexer_path)?);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                github_client: cached_gh_client,
                code_indexer: code_indexer,
            }))
            .service(web::scope("/api").route("/health", web::get().to(health_check))),
    )
    .await;

    // Make a request to the /api/health endpoint
    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert the response status and body
    assert!(resp.status().is_ok());
    let response_body = test::read_body(resp).await;
    assert_eq!(response_body, "OK");

    Ok(())
}
