# CLAUDE.md -- AI Agent Instructions for ContentForge

## Project Overview

ContentForge is a Rust-native content creation and multi-platform publishing platform. It is structured as a Cargo workspace with 11 crates that together provide CLI, TUI, Web UI, and MCP server interfaces for creating content, adapting it to platform-specific formats, scheduling publication, and tracking analytics. The database is SQLite with WAL mode. All platform interactions go through the `Publisher` trait in `contentforge-publish`.

## Workspace Structure

```
contentforge/
  Cargo.toml                  # Workspace root with shared dependencies
  crates/
    contentforge-core/        # Domain types: Content, Platform, Schedule, errors
    contentforge-db/          # SQLite persistence, migrations, ContentRepo
    contentforge-publish/     # Publisher trait + platform adapters (DEV.to, Twitter, LinkedIn, Medium)
    contentforge-agent/       # AI agent pipeline (generate, adapt, split, review)
    contentforge-schedule/    # Cron-based scheduling engine
    contentforge-analytics/   # Engagement metrics collection
    contentforge-api/         # Axum REST API + WebSocket server
    contentforge-cli/         # Clap-based CLI
    contentforge-tui/         # Ratatui terminal UI
    contentforge-mcp/         # MCP server (stdio + SSE)
    contentforge-app/         # Binary entry point, wires all crates together
  frontend/                   # SvelteKit web UI (embedded in binary via rust-embed)
  site/                       # MkDocs documentation site
  docs/                       # Architecture and design documents
```

## Build Commands

```bash
# Build the entire workspace
cargo build

# Build in release mode (optimized, stripped)
cargo build --release

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p contentforge-core

# Lint (must pass with zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --check

# Format
cargo fmt
```

## Code Conventions

- **Zero clippy warnings.** The CI enforces `cargo clippy -- -D warnings`. Fix all warnings before committing.
- **Error handling:** Use `thiserror` for library error types (see `ContentForgeError` in `contentforge-core`). Use `anyhow` at application boundaries (CLI, API handlers, binary entry points). Never use `.unwrap()` in library code except in tests.
- **Async everywhere.** All I/O operations are async using Tokio. Platform adapters use `async_trait`. The TUI event loop uses Tokio channels.
- **Serde conventions:** Use `#[serde(rename_all = "snake_case")]` for enums. Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields in API payloads.
- **Documentation:** All public types and functions must have doc comments. Use `///` style.
- **Naming:** Crate names use `contentforge-` prefix. Module files use snake_case. Types use PascalCase.
- **Dependencies:** All dependencies are declared in `[workspace.dependencies]` and referenced with `workspace = true` in crate Cargo.toml files.

## Git Workflow

- Branch from `main` for all work.
- Branch naming: `feat/description`, `fix/description`, `docs/description`, `refactor/description`.
- Commit messages follow Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
- Keep commits focused. One logical change per commit.
- All PRs must pass CI (clippy, tests, format check) before merge.

## Testing Expectations

- Every public function in library crates should have at least one unit test.
- Use `contentforge-db::init_memory_db()` for database tests (no file I/O).
- Platform adapter tests should use mock HTTP servers, not live APIs.
- Integration tests go in `tests/` directories within each crate.
- Test names should describe the behavior being tested: `test_publish_devto_article_with_tags`.

## Key Architectural Decisions

1. **Publisher trait is the extension point.** All platform adapters implement `Publisher`. To add a new platform, create a new struct implementing this trait. Do not bypass the trait for any platform.

2. **Content is the central entity.** Everything revolves around `Content`. Adaptations, publications, schedules, and analytics all reference content by ID.

3. **Database is SQLite only.** Do not introduce PostgreSQL, MySQL, or any other database. SQLite with WAL mode is sufficient and keeps the single-binary promise.

4. **No runtime dependencies.** The binary must work with zero external services. No Redis, no message queues, no Docker. Platform API calls are the only network dependency.

5. **Frontend is embedded.** The SvelteKit build output is embedded into the Rust binary using `rust-embed`. The Axum server serves it directly. There is no separate frontend deployment.

6. **MCP over stdio is the primary AI integration.** The MCP server uses stdio transport for Claude Code integration and SSE transport for web-based AI clients.

7. **Migrations are append-only.** Never modify existing migration files. Add new migration files with incrementing numbers (e.g., `002_add_column.sql`).
