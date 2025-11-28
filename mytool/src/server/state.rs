use super::prelude::*;

/// Represents the shared application state for the Actix-web server.
///
/// This struct holds references to resources that are accessible across different
/// request handlers, such as the GitHub client for API interactions and the
/// code indexer for managing indexed code.
pub struct AppState {
    /// An Arc-wrapped instance of `CachedGitHubClient` for making efficient GitHub API calls.
    ///
    /// This client handles caching of GitHub API responses, reducing direct calls
    /// to GitHub and improving performance.
    pub github_client: Arc<CachedGitHubClient>,
    /// An Arc-wrapped instance of `CodeIndexer` for managing indexed code files.
    ///
    /// This indexer is responsible for storing and retrieving code content and
    /// metadata from a persistent store (RocksDB).
    pub code_indexer: Arc<CodeIndexer>, // Add CodeIndexer to AppState
}