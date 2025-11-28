use super::prelude::*;

// State for the Actix-web application
pub struct AppState {
    pub github_client: Arc<CachedGitHubClient>,
    pub code_indexer: Arc<CodeIndexer>, // Add CodeIndexer to AppState
}