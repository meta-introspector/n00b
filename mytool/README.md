# MCP Tycoon Game: `mytool` Submodule

This submodule provides essential backend services for the "MCP Tycoon Game," specifically handling GitHub API interactions and efficient code indexing crucial for the Meta Introapector feature. It's designed to ensure reproducible and verifiable game states through a novel "Syscall" abstraction.

## Role in MCP Tycoon Game

`mytool` acts as the game's intelligence layer, simulating access to and analysis of external codebases. It optimizes performance by caching frequently accessed GitHub data and lays the architectural groundwork for future verifiable operations (Syscalls) necessary for secure and reproducible game actions. It is integrated into the `cargo2nix` repository as a Meta Introapector component.

## Getting Started for Interns

This guide assumes you are working within the `cargo2nix` repository structure.

### Prerequisites

*   **Rust Toolchain**: Install the latest stable Rust toolchain using `rustup`.
*   **Git**: Ensure Git is installed and configured on your system.

### Setup

1.  **Clone the `cargo2nix` repository**:
    ```bash
    git clone <cargo2nix_repo_url>
    cd cargo2nix
    ```
2.  **Initialize `mytool` submodule**:
    ```bash
    git submodule update --init --recursive
    ```
3.  **Navigate to the `mytool` directory**:
    ```bash
    cd mytool
    ```
4.  **Environment Variable**: `mytool` requires a GitHub Personal Access Token (PAT) for API access. Set it as an environment variable:
    ```bash
    # On Linux/macOS
    export GITHUB_TOKEN="your_github_pat"
    # On Windows (PowerShell)
    $env:GITHUB_TOKEN="your_github_pat"
    ```
    *   *Note*: Ensure your PAT has sufficient permissions (e.g., `public_repo`, `repo` for private repositories) for the GitHub API calls `mytool` will make.

### Building

Compile the `mytool` project:
```bash
cargo build
```

### Running

Start the `mytool` server. By default, it listens on `127.0.0.1:8080`.
```bash
cargo run
```
You should see log messages indicating the server has started.

### Testing

Run the unit and integration tests to ensure everything is working correctly:
```bash
cargo test
```

## Project Structure Overview

The `mytool` project follows a modular design to enhance readability, maintainability, and extensibility. Key areas include:

*   **`src/server/`**: Contains the HTTP server logic, split into submodules for handlers, paths, queries, state management, and tests, adhering to the "one declaration per file, one module per topic" principle.
*   **`src/syscall_types.rs`**: Defines the fundamental `Syscall` trait and associated types.
*   **`src/syscall_executor.rs`**: Implements the `SyscallExecutor` trait, managing the execution lifecycle of Syscalls.
*   **`src/code_indexer.rs`**: Handles code indexing and retrieval using RocksDB.
*   **`src/github.rs`**: Interfaces directly with the GitHub API.
*   **`src/github_cache.rs`**: Provides a caching layer for GitHub API responses.
*   **`src/mock_github_client.rs`**: Offers a mock implementation of the GitHub client for testing.

## Key Concepts for Interns

*   **Syscall Abstraction**: In `mytool`, all interactions with external, impure systems (like the GitHub API or the local filesystem for code indexing) are treated as "Syscalls." This abstraction is crucial for enabling verifiability, reproducibility, and future integration with Zero-Knowledge Proofs (ZKPs) within the game's mechanics.
*   **Caching**: `mytool` employs a robust caching mechanism to minimize redundant requests to the GitHub API, significantly improving performance and reducing rate limit issues.
*   **Modularity**: The codebase is highly modular, making it easier to understand, test, and extend individual components without affecting the entire system.

## Contribution for Interns

Please refer to the detailed [CONTRIBUTING.md](CONTRIBUTING.md) guide (to be created) for code style, commit message conventions, and the Pull Request process.

We encourage you to explore the gamified learning tracks outlined by your mentor, such as "Code Cartographer," "Bug Bounty Hunter," and "Feature Architect," to guide your contributions effectively.

## Support

For any questions or issues, please refer to your project mentor or the internal communication channels.

**Happy Hacking!**
