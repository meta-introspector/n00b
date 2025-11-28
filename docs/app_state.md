# Application State (`AppState`)

The `AppState` struct (`src/server/state.rs`) is a central component of `mytool`'s Actix-web server. It serves as a container for shared resources and configuration that need to be accessible across different HTTP request handlers throughout the application's lifecycle.

## Purpose

In Actix-web, `web::Data<AppState>` is typically used to inject shared, immutable (or internally mutable via `Arc<Mutex<T>>`, `Arc<RwLock<T>>`, etc.) application-specific data into handlers. This pattern ensures that expensive-to-create resources (like database connections or API clients) are initialized once and then efficiently reused across all incoming requests.

## Structure of `AppState`

The `AppState` in `mytool` currently holds two primary shared resources:

*   **`github_client: Arc<CachedGitHubClient>`**:
    *   **Role**: This is an instance of our `CachedGitHubClient`, which is responsible for interacting with the GitHub API. It wraps a live GitHub client and adds a caching layer to reduce direct API calls, improve performance, and respect rate limits.
    *   **Why `Arc`?**: `Arc` (Atomically Reference Counted) is used because `CachedGitHubClient` needs to be shared across multiple asynchronous tasks (which Actix-web handlers are) and across potentially multiple worker threads. `Arc` allows multiple owners of the same data, and the data is dropped only when the last owner goes out of scope. The `Send + Sync` bounds on `CachedGitHubClient` (via its inner `GitHubClient` trait) ensure it's safe to share across threads.

*   **`code_indexer: Arc<CodeIndexer>`**:
    *   **Role**: This is an instance of our `CodeIndexer`, which manages the persistent storage and retrieval of code file content and metadata (using RocksDB). It's crucial for the Meta Introapector functionality within the MCP Tycoon Game.
    *   **Why `Arc`?**: Similar to `github_client`, `CodeIndexer` manages a shared resource (the RocksDB database connection) and needs to be accessible from multiple handlers concurrently. `Arc` provides safe, shared ownership across these contexts.

## How Handlers Access `AppState`

Actix-web handlers that require access to `AppState` declare it as a parameter using the `web::Data<AppState>` extractor:

```rust
pub async fn some_handler(data: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    // Access the GitHub client
    let github = &data.github_client;
    // Access the code indexer
    let indexer = &data.code_indexer;

    // ... use github and indexer ...
    Ok(HttpResponse::Ok().body("Hello from handler"))
}
```

This dependency injection pattern makes handlers cleaner, more testable, and centralizes resource management within the `AppState` struct.
