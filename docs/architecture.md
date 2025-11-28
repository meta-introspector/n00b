# Overall Architecture of `mytool`

`mytool` is a Rust-based application designed to serve as a backend service within the broader "MCP Tycoon Game" ecosystem. Its primary role is to manage interactions with external, "impure" systems like the GitHub API and a local persistent data store (RocksDB), providing a robust, verifiable, and modular foundation for game mechanics that rely on external data.

## Core Principles

The architecture of `mytool` is built upon several key principles:

1.  **Modularity**: The codebase is organized into small, self-contained modules, each responsible for a specific concern. This enhances readability, maintainability, and reusability.
2.  **Verifiability**: A central design goal is to enable future integration with Zero-Knowledge Proofs (ZKPs). This is achieved through the "Syscall" abstraction, which ensures that all impure operations are explicit, auditable, and reproducible.
3.  **Performance**: Caching mechanisms are heavily utilized (e.g., for GitHub API responses) to minimize latency and reduce reliance on external services.
4.  **Robustness**: Error handling is a first-class citizen, ensuring the application can gracefully manage failures from external systems.

## Component Overview

`mytool` is structured around several interconnected components:

*   **HTTP Server (`src/server/`)**: Built using Actix-web, this component provides the external API for `mytool`. It exposes endpoints for fetching GitHub data, indexing code, and retrieving indexed information. Its modular design (handlers, paths, queries, state, runner) ensures clear separation of concerns.
*   **GitHub Integration (`src/github.rs`, `src/github_cache.rs`)**: This layer handles all communication with the GitHub API. It includes a `GitHubClient` trait for abstracting API calls, a `LiveGitHubClient` implementation for actual HTTP requests, and a `CachedGitHubClient` that wraps the live client to provide efficient caching of responses.
*   **Code Indexing (`src/code_indexer.rs`)**: Manages the storage and retrieval of code file content and metadata using RocksDB. This is vital for the Meta Introapector feature in the MCP Tycoon Game, allowing for efficient analysis of indexed code.
*   **Syscall Abstraction (`src/syscall_types.rs`, `src/syscall_executor.rs`)**: This is the most critical architectural innovation. All impure operations (interactions with external resources or persistent storage) are modeled as "Syscalls". The `Syscall` trait defines the contract for these operations, and the `SyscallExecutor` handles their generic execution lifecycle, including logging, hashing of inputs/outputs, and preparing for future verifiability.
*   **Application State (`src/server/state.rs`)**: The `AppState` struct holds shared resources like the `CachedGitHubClient` and `CodeIndexer`, making them accessible to all request handlers.

## Data Flow Example (GitHub User Info)

1.  An HTTP request arrives at `/api/github/users/{username}`.
2.  The request is routed to the `get_user` handler in `src/server/handlers/get_user.rs`.
3.  The handler extracts the `username` from the URL path.
4.  It accesses the `CachedGitHubClient` from the `AppState`.
5.  The `CachedGitHubClient` first checks its internal cache for the user's information.
    *   If found, the cached data is returned immediately.
    *   If not found, the `CachedGitHubClient` delegates the request to the `LiveGitHubClient`.
6.  The `LiveGitHubClient` constructs and sends an authenticated HTTP request to the GitHub API.
7.  The GitHub API responds with the user's data.
8.  The `LiveGitHubClient` parses the response.
9.  The `CachedGitHubClient` stores this fresh data in its cache for future use.
10. The `get_user` handler receives the `UserMetadata` and sends it back to the client as a JSON `HttpResponse`.

This modular and layered approach ensures that `mytool` is not only functional but also adaptable and ready for advanced features like the Meta Introapector and ZKP integration within the MCP Tycoon Game.
