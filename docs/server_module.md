# The Modular `server` Module

The `mytool` application's HTTP server logic resides within the `src/server/` module. This module has been carefully structured following principles of modularity and separation of concerns to enhance readability, maintainability, and testability. Instead of a single, monolithic file, the server's functionality is distributed across several submodules and individual files.

## Modular Structure

The `src/server/` directory contains the following key submodules:

*   **`mod.rs`**: The main entry point for the `server` module. It declares all public submodules and might contain module-level documentation.
*   **`prelude.rs`**: This module acts as a convenient barrel file, re-exporting commonly used types, traits, and functions that are frequently needed by other modules within `src/server/`. This reduces boilerplate `use` statements in individual files.
*   **`state.rs`**: Defines the `AppState` struct, which holds shared application-level resources (like API clients and indexers) that are passed to request handlers.
*   **`paths/`**: This subdirectory contains individual modules, each defining a specific path parameter struct (e.g., `UserPath`, `RepoPath`). These structs are used by Actix-web's `web::Path` extractor to strongly type and validate URL path segments.
*   **`queries/`**: This subdirectory contains individual modules, each defining a specific query parameter struct (e.g., `FilterQuery`, `SearchQuery`). These structs are used by Actix-web's `web::Query` extractor to strongly type and validate URL query parameters.
*   **`handlers/`**: This is where the core business logic for handling specific HTTP requests resides. Each file typically contains a single `async fn` function that serves a particular API endpoint (e.g., `get_user.rs`, `index_code_handler.rs`). Handlers receive `web::Path`, `web::Query`, and `web::Data<AppState>` extractors to access request data and shared resources.
*   **`runner.rs`**: Contains the `run_server` function, which is responsible for initializing the Actix-web server, configuring its routes, and binding it to a network address. This centralizes the server setup logic.
*   **`tests/`**: This subdirectory holds the unit and integration tests specific to the server's functionality. Individual test functions are often placed in separate files to keep them focused and manageable. Shared test utilities are kept in `helpers.rs`.

## Benefits of this Modular Approach

*   **Clear Separation of Concerns**: Each component has a well-defined responsibility, making the codebase easier to understand and navigate.
*   **Improved Readability**: Smaller files and focused modules reduce cognitive load.
*   **Enhanced Maintainability**: Changes in one part of the system are less likely to inadvertently affect others.
*   **Easier Testing**: Individual units can be tested in isolation, and integration tests can cover specific interaction flows.
*   **Scalability**: The structure supports adding new endpoints, path/query types, or handlers without creating unmanageable monster files.
*   **Intern-Friendly**: New contributors can easily grasp the purpose of each file and module, facilitating onboarding and contributions to the MCP Tycoon Game.

This modular design aligns with Rust's idiomatic practices for organizing larger projects and fosters a collaborative development environment.
