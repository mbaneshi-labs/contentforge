# Contributing to ContentForge

Thank you for your interest in contributing to ContentForge. This guide covers everything you need to get started.

## Development Environment Setup

### Prerequisites

- **Rust 1.80+** -- install via [rustup](https://rustup.rs/)
- **Node.js 20+** and **pnpm** -- for the SvelteKit frontend (optional, only needed for Web UI changes)
- **SQLite 3** -- bundled via `rusqlite`, no system install needed

### Clone and Build

```bash
git clone https://github.com/mbaneshi/contentforge.git
cd contentforge

# Build the full workspace
cargo build

# Run tests
cargo test

# Run clippy (must pass with zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

### Editor Setup

ContentForge uses standard Rust tooling. Any editor with rust-analyzer support works well. Recommended:

- VS Code with rust-analyzer extension
- Neovim with rust-analyzer LSP
- Zed (built-in Rust support)

If you use VS Code, the workspace settings are configured to run clippy on save.

## Project Structure

The project is a Cargo workspace with 11 crates. See the [Architecture document](docs/architecture/ARCHITECTURE.md) for detailed descriptions of each crate and how they interact.

Key crates for most contributions:

- **`contentforge-core`** -- domain types, start here to understand the data model
- **`contentforge-publish`** -- platform adapters, the most common area for new contributions
- **`contentforge-db`** -- database schema and queries
- **`contentforge-cli`** -- CLI commands

## How to Add a New Platform Adapter

This is the most common type of contribution. Follow these steps:

### Step 1: Add the Platform Variant

If the platform is not already in the `Platform` enum, add it in `crates/contentforge-core/src/platform.rs`:

```rust
pub enum Platform {
    // ... existing variants
    NewPlatform,
}
```

Update the `Display`, `char_limit()`, `supports_markdown()`, `supports_images()`, `supports_threads()`, and `integration_difficulty()` implementations.

### Step 2: Create the Adapter Module

Create a new file at `crates/contentforge-publish/src/adapters/newplatform.rs`:

```rust
use crate::Publisher;
use async_trait::async_trait;
use contentforge_core::{
    Content, ContentForgeError, Platform, PlatformAdaptation, Publication,
};

pub struct NewPlatformPublisher {
    client: reqwest::Client,
    // platform-specific auth fields
}

impl NewPlatformPublisher {
    pub fn new(/* auth params */) -> Self {
        Self {
            client: reqwest::Client::new(),
            // ...
        }
    }
}

#[async_trait]
impl Publisher for NewPlatformPublisher {
    fn platform(&self) -> Platform {
        Platform::NewPlatform
    }

    async fn publish(
        &self,
        content: &Content,
        adaptation: &PlatformAdaptation,
    ) -> Result<Publication, ContentForgeError> {
        self.validate(adaptation)?;
        // Platform-specific publish logic
        todo!()
    }

    async fn delete(&self, publication: &Publication) -> Result<(), ContentForgeError> {
        todo!()
    }

    async fn health_check(&self) -> Result<(), ContentForgeError> {
        todo!()
    }
}
```

### Step 3: Register the Module

Add the module to `crates/contentforge-publish/src/adapters/mod.rs`:

```rust
pub mod newplatform;
```

### Step 4: Write Tests

Add tests that verify:
- Validation rejects content exceeding character limits
- Publish constructs the correct API payload
- Error responses are handled gracefully
- Health check works

Use a mock HTTP server (e.g., `mockito` or `wiremock`) for tests -- never call live APIs in tests.

### Step 5: Update Documentation

- Add the platform to the supported platforms table in `README.md`
- Add a platform guide section in `site/docs/guide/platforms.md`
- Update the `CHANGELOG.md`

## Code Style and Conventions

### Rust
- Run `cargo fmt` before committing.
- Run `cargo clippy --all-targets --all-features -- -D warnings` and fix all warnings.
- All public items must have doc comments (`///`).
- Use `thiserror` for error types in library crates.
- Use `anyhow` only at application boundaries (CLI handlers, API handlers).
- Prefer `impl Into<String>` over `String` for function parameters that construct owned strings.
- Use `serde(rename_all = "snake_case")` for enum serialization.

### Git
- Follow [Conventional Commits](https://www.conventionalcommits.org/):
  - `feat: add Reddit adapter`
  - `fix: handle rate limit on Twitter thread publishing`
  - `docs: add platform setup guide for LinkedIn`
  - `refactor: extract common HTTP client logic`
  - `test: add integration tests for scheduling engine`
- Keep commits atomic -- one logical change per commit.
- Write descriptive commit messages that explain *why*, not just *what*.

## Testing Requirements

- All new code must have tests. PRs that decrease test coverage will be asked to add tests.
- Use `#[cfg(test)]` modules for unit tests within the source file.
- Use `tests/` directories for integration tests.
- Use `contentforge_db::init_memory_db()` for any tests that need a database.
- Never depend on network access or live API keys in tests.

## Pull Request Process

1. **Fork and branch.** Create a feature branch from `main` with a descriptive name (e.g., `feat/reddit-adapter`).

2. **Make your changes.** Follow the code conventions above.

3. **Test locally.** Run the full test suite:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --check
   ```

4. **Open a PR.** Write a clear description of what the PR does and why. Link any related issues.

5. **CI must pass.** The GitHub Actions CI runs clippy, tests, and format checks on every PR.

6. **Code review.** A maintainer will review your PR. Address any feedback and push updates to the same branch.

7. **Merge.** Once approved and CI is green, a maintainer will merge the PR.

## Questions?

Open a [Discussion](https://github.com/mbaneshi/contentforge/discussions) on GitHub for questions about the architecture, design decisions, or contribution ideas.
