# Contributing to `mytool` (MCP Tycoon Game Submodule)

Welcome, intrepid intern, to the core mechanics of the MCP Tycoon Game! Your contributions to `mytool` are vital for enhancing the game's intelligence layer, optimizing its operations, and laying the groundwork for verifiable gameplay.

## Getting Started

Before diving into contributions, please ensure you have set up your development environment by following the instructions in [`mytool/README.md`](./README.md). Make sure `mytool` builds and all tests pass on your local machine.

## Code of Conduct

Please adhere to the project's [Code of Conduct](LINK_TO_CODE_OF_CONDUCT_HERE) (or a brief statement if none exists). Treat fellow contributors with respect and maintain a positive and collaborative environment.

## Gamified Learning Tracks

We've designed gamified learning tracks to help you understand the codebase and guide your contributions:

*   **Code Cartographer**: Explore the `mytool` codebase, understand its architecture, and map data flows.
*   **Bug Bounty Hunter**: Identify, reproduce, and fix minor issues, improving code stability and robustness.
*   **Feature Architect**: Implement small, defined features, extending `mytool`'s capabilities within the MCP Tycoon Game.

Consult with your mentor to pick up tasks aligned with these tracks.

## How to Contribute

### 1. Finding Tasks

Your mentor will assign tasks, or you can pick them up from the designated issue tracker. Tasks will often be framed within the context of the gamified learning tracks.

### 2. Branching

Always create a new branch for your work. Use a descriptive name, often prefixed with the task type (e.g., `feature/add-health-endpoint`, `bugfix/fix-unused-import`).

```bash
git checkout -b feature/your-feature-name
```

### 3. Coding Style

`mytool` adheres to standard Rust coding conventions. We use `rustfmt` for formatting and `clippy` for linting. Please ensure your code passes these checks before submitting a Pull Request.

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

### 4. Testing

*   **Write Tests**: For new features, write comprehensive unit and/or integration tests. For bug fixes, add a test that reproduces the bug and then passes after your fix.
*   **Run Tests**: Always run all tests before committing to ensure your changes haven't introduced regressions.

```bash
cargo test
```

### 5. Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for clear and consistent commit history.

*   `feat`: A new feature
*   `fix`: A bug fix
*   `docs`: Documentation only changes
*   `style`: Changes that do not affect the meaning of the code (white-space, formatting, missing semicolons, etc)
*   `refactor`: A code change that neither fixes a bug nor adds a feature
*   `test`: Adding missing tests or correcting existing tests
*   `chore`: Changes to the build process or auxiliary tools and libraries suchs as documentation generation

Example:
```
feat: add new /api/mcp/health endpoint

This commit introduces a new health check endpoint for the mytool server
to allow external monitoring.
```

### 6. Pull Request Process

1.  Push your branch to the remote repository.
2.  Open a Pull Request (PR) against the `main` (or designated `dev`) branch.
3.  Provide a clear description of your changes, including:
    *   **What** your change does.
    *   **Why** this change is necessary (motivation).
    *   **How** you tested it.
    *   Reference any related issue numbers (e.g., `Fixes #123`).
4.  Request a review from your mentor or a designated team member.
5.  Address any feedback or suggested changes.

## Reporting Bugs

If you find a bug, please report it via the project's issue tracker or directly to your mentor, providing as much detail as possible to reproduce the issue.

## Asking Questions

Don't hesitate to ask questions! Reach out to your mentor or use the internal communication channels for any queries.

**Thank you for contributing to the MCP Tycoon Game!**
