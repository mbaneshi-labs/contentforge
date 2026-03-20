# ContentForge Architecture

## System Overview

ContentForge is a layered system where multiple user interfaces share a common core through an internal API layer. The architecture follows a clean separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│                    Interface Layer                       │
│  ┌───────┐  ┌──────────────┐  ┌──────┐  ┌───────────┐  │
│  │  CLI  │  │ Web (Axum +  │  │ TUI  │  │ MCP Server│  │
│  │(clap) │  │  SvelteKit)  │  │(rata-│  │  (rmcp)   │  │
│  │       │  │              │  │ tui) │  │           │  │
│  └───┬───┘  └──────┬───────┘  └──┬───┘  └─────┬─────┘  │
│      │             │             │             │         │
│      └─────────────┼─────────────┼─────────────┘         │
│                    │             │                        │
├────────────────────┼─────────────┼────────────────────────┤
│                 Service Layer    │                        │
│  ┌─────────────────┴─────────────┴──────────────────┐    │
│  │              contentforge-api (Axum)              │    │
│  │    REST endpoints + WebSocket + static files      │    │
│  └──────────────────────┬───────────────────────────┘    │
│                         │                                │
├─────────────────────────┼────────────────────────────────┤
│                   Domain Layer                           │
│  ┌──────────┐  ┌────────┴───────┐  ┌──────────────┐     │
│  │   Core   │  │   AI Agent     │  │   Schedule    │     │
│  │  Types   │  │   Pipeline     │  │    Engine     │     │
│  └────┬─────┘  └────────────────┘  └──────────────┘     │
│       │                                                  │
├───────┼──────────────────────────────────────────────────┤
│       │          Infrastructure Layer                    │
│  ┌────┴────┐  ┌──────────────────────────────────────┐   │
│  │ SQLite  │  │       Platform Adapters               │   │
│  │  (DB)   │  │  ┌───────┐ ┌────────┐ ┌──────┐      │   │
│  │         │  │  │DEV.to │ │Twitter │ │Linked│ ...  │   │
│  │         │  │  │       │ │        │ │In    │      │   │
│  └─────────┘  │  └───────┘ └────────┘ └──────┘      │   │
│               └──────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘
```

## Crate Dependency Graph

```
contentforge-app (binary)
├── contentforge-cli
│   └── contentforge-api
│       ├── contentforge-core
│       ├── contentforge-db
│       │   └── contentforge-core
│       ├── contentforge-publish
│       │   └── contentforge-core
│       ├── contentforge-agent
│       │   └── contentforge-core
│       ├── contentforge-schedule
│       │   └── contentforge-core
│       └── contentforge-analytics
│           └── contentforge-core
├── contentforge-tui
│   └── contentforge-api
├── contentforge-mcp
│   └── contentforge-api
└── contentforge-core (shared by all)
```

The dependency rule is strict: crates depend downward only. `contentforge-core` has zero internal dependencies. No circular dependencies are permitted.

## Data Flow

### Content Lifecycle

```
1. CREATE          2. ADAPT            3. SCHEDULE         4. PUBLISH          5. TRACK
   │                  │                   │                   │                   │
   ▼                  ▼                   ▼                   ▼                   ▼
┌──────┐          ┌──────┐           ┌──────┐           ┌──────┐           ┌──────┐
│Content│  ──────▶│Adapt │  ────────▶│Sched-│  ────────▶│Publi-│  ────────▶│Analy-│
│ .new()│         │for   │           │ule   │           │cation│           │tics  │
│       │         │each  │           │Entry │           │      │           │      │
│ body  │         │plat- │           │      │           │ url  │           │views │
│ title │         │form  │           │cron  │           │ id   │           │likes │
│ type  │         │      │           │time  │           │      │           │share │
└──────┘          └──────┘           └──────┘           └──────┘           └──────┘
  Idea    ──▶    Drafting   ──▶      Ready     ──▶    Scheduled   ──▶    Published
```

1. **Create**: User writes content in Markdown. A `Content` entity is created with status `Idea`.
2. **Adapt**: Content is adapted for each target platform. AI can auto-generate adaptations. Each creates a `PlatformAdaptation` with platform-specific formatting.
3. **Schedule**: A `ScheduleEntry` is created linking content + platform + time. The scheduling engine polls for entries where `scheduled_at <= now`.
4. **Publish**: The `PublisherRegistry` dispatches to the correct adapter. The adapter calls the platform API and returns a `Publication` record with the live URL.
5. **Track**: The analytics module periodically fetches engagement metrics from platform APIs and stores snapshots in the `analytics` table.

## Database Schema

The SQLite database uses WAL mode for concurrent read access. The schema is defined in `crates/contentforge-db/src/migrations/001_initial.sql`.

### Entity Relationship

```
content (1) ──── (N) adaptations
content (1) ──── (N) media
content (1) ──── (N) publications
content (1) ──── (N) schedule
publications (1) ── (N) analytics

platform_accounts (standalone)
recurring_schedules (standalone)
```

### Tables

| Table                | Purpose                                    | Key Columns                          |
|----------------------|--------------------------------------------|--------------------------------------|
| `content`            | Central content entity                     | id, title, body, content_type, status, tags, project |
| `adaptations`        | Platform-specific versions                 | content_id, platform, body, thread_parts, canonical_url |
| `media`              | Attached files (images, videos)            | content_id, path, mime_type, alt_text |
| `platform_accounts`  | Credentials for connected platforms        | platform, display_name, credential (JSON) |
| `publications`       | Records of successful publishes            | content_id, platform, url, platform_post_id |
| `schedule`           | Pending and completed schedule entries     | content_id, platform, scheduled_at, status, retries |
| `recurring_schedules`| Cron-based recurring publication rules     | name, cron_expr, template, platforms |
| `analytics`          | Engagement metric snapshots                | publication_id, views, likes, shares, comments, clicks |
| `_migrations`        | Migration tracking (internal)              | name, applied_at |

### Indexes

- `idx_content_status` -- fast filtering by content status
- `idx_content_project` -- fast filtering by project
- `idx_adaptations_content` -- join adaptations to content
- `idx_publications_content` -- join publications to content
- `idx_schedule_status` -- find pending schedule entries
- `idx_schedule_time` -- find entries due for publishing
- `idx_analytics_publication` -- join analytics to publications

## Platform Adapter Pattern

All platform integrations implement the `Publisher` trait:

```rust
#[async_trait]
pub trait Publisher: Send + Sync {
    fn platform(&self) -> Platform;
    fn validate(&self, adaptation: &PlatformAdaptation) -> Result<(), ContentForgeError>;
    async fn publish(&self, content: &Content, adaptation: &PlatformAdaptation) -> Result<Publication, ContentForgeError>;
    async fn delete(&self, publication: &Publication) -> Result<(), ContentForgeError>;
    async fn health_check(&self) -> Result<(), ContentForgeError>;
}
```

The `validate` method has a default implementation that checks character limits based on `Platform::char_limit()`.

### Adding a New Adapter

1. Add the platform variant to `Platform` enum in `contentforge-core`
2. Create a new module in `contentforge-publish/src/adapters/`
3. Implement the `Publisher` trait
4. Register the module in `adapters/mod.rs`
5. Add configuration support in the CLI/config layer

### Publisher Registry

The `PublisherRegistry` holds all configured adapters and provides:
- `get(platform)` -- retrieve a specific adapter
- `publish_all(content)` -- publish to all platforms that have adaptations

## TUI Architecture

The TUI uses ratatui with crossterm as the backend.

```
┌──────────────────────────────────┐
│          Event Loop              │
│  ┌──────────┐  ┌──────────────┐ │
│  │ Crossterm│  │  Tokio       │ │
│  │  Events  │  │  Channels    │ │
│  │(keyboard,│  │ (API results,│ │
│  │  mouse)  │  │  background) │ │
│  └────┬─────┘  └──────┬───────┘ │
│       └────────┬───────┘         │
│                ▼                 │
│         ┌──────────┐             │
│         │ App State│             │
│         │  update  │             │
│         └────┬─────┘             │
│              ▼                   │
│       ┌──────────────┐           │
│       │   Render     │           │
│       │  (ratatui)   │           │
│       └──────────────┘           │
└──────────────────────────────────┘
```

- **Event loop**: Polls crossterm for terminal events and Tokio channels for async results.
- **App state**: A single `AppState` struct holds all UI state (selected tab, content list, editor state, etc.). State updates are synchronous.
- **Rendering**: Pure function from `AppState` to terminal frames. No side effects during render.

## Web Architecture

```
┌─────────────────────────────────────────┐
│              Browser                     │
│  ┌─────────────────────────────────┐    │
│  │     SvelteKit Frontend          │    │
│  │  (embedded via rust-embed)      │    │
│  └───────────┬─────────────────────┘    │
│              │ HTTP + WebSocket          │
└──────────────┼──────────────────────────┘
               │
┌──────────────┼──────────────────────────┐
│              ▼                           │
│  ┌───────────────────────────┐          │
│  │      Axum Server          │          │
│  ├───────────────────────────┤          │
│  │ GET  /api/content         │          │
│  │ POST /api/content         │          │
│  │ PUT  /api/content/:id     │          │
│  │ POST /api/publish/:id     │          │
│  │ POST /api/schedule/:id    │          │
│  │ GET  /api/analytics       │          │
│  │ WS   /api/ws              │          │
│  ├───────────────────────────┤          │
│  │ GET  /*  (static files)   │ ◀── rust-embed serves SvelteKit build
│  └───────────────────────────┘          │
└──────────────────────────────────────────┘
```

- The SvelteKit app is built at compile time and embedded into the binary using `rust-embed`.
- Axum serves the API endpoints under `/api/` and falls back to the embedded static files for all other routes.
- WebSocket connections at `/api/ws` push real-time updates (new publications, schedule changes, analytics refreshes).

## MCP Integration

ContentForge implements the Model Context Protocol (MCP) using the `rmcp` crate.

### Transport
- **stdio**: Primary mode. Claude Code starts the `contentforge mcp` process and communicates over stdin/stdout.
- **SSE**: Alternative mode for web-based AI clients. Runs as an HTTP endpoint.

### Exposed Tools

| Tool             | Description                                           |
|------------------|-------------------------------------------------------|
| `create_content` | Create a new content piece with title, body, type     |
| `list_content`   | List content filtered by status, project, or type     |
| `get_content`    | Get a specific content piece by ID                    |
| `adapt_content`  | Generate platform-specific adaptation                 |
| `publish`        | Publish content to a specific platform                |
| `schedule`       | Schedule content for future publication               |
| `list_platforms` | List configured platform accounts and their health    |
| `get_analytics`  | Get engagement metrics for published content          |

## Error Handling Strategy

The project uses a two-tier error handling approach:

1. **Library errors** (`ContentForgeError` via `thiserror`): Structured, typed errors that callers can match on. Used in `contentforge-core`, `contentforge-publish`, `contentforge-db`, and other library crates.

2. **Application errors** (`anyhow::Error`): Used at the boundaries -- CLI command handlers, API route handlers, and the binary entry point. These wrap library errors with additional context.

Key error variants in `ContentForgeError`:
- `ContentNotFound` -- 404-equivalent for content lookups
- `PublishFailed` -- platform-specific publish errors with details
- `PlatformNotConfigured` -- adapter not registered
- `RateLimited` -- includes retry-after duration
- `AuthFailed` -- credential issues
- `ContentTooLong` -- content exceeds platform character limit
- `Database` -- SQLite errors
- `Agent` -- AI pipeline errors
- `Schedule` -- scheduling errors

## Testing Strategy

### Unit Tests
Located in `#[cfg(test)]` modules within source files. Test individual functions and type behavior. The `contentforge-db` crate provides `init_memory_db()` for database tests without file I/O.

### Integration Tests
Located in `tests/` directories within each crate. Test interactions between components.

### Mock Testing
Platform adapters are tested with mock HTTP servers. Tests verify:
- Correct API payload construction
- Proper handling of success responses
- Graceful handling of error responses (401, 429, 500)
- Rate limit detection and retry-after parsing

### No Live API Tests
Tests must never call live platform APIs. All network calls in tests use mock servers. This ensures tests are fast, deterministic, and do not require API credentials.
