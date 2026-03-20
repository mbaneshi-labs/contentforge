# ContentForge Roadmap

## Phase 0 -- Foundation (Current)

**Goal:** Establish the architecture, core types, database layer, and first working adapter.

### Deliverables
- [x] Workspace structure with modular crates
- [x] Core domain types: `Content`, `Platform`, `ContentStatus`, `ContentType`, `ScheduleEntry`
- [x] Error handling with `thiserror` (`ContentForgeError`) and `anyhow`
- [x] SQLite database with WAL mode, migrations framework
- [x] Database schema: content, adaptations, media, platform_accounts, publications, schedule, analytics
- [x] Repository pattern (`ContentRepo`) with full CRUD
- [x] `Publisher` trait with validation, publish, delete, health_check
- [x] `PublisherRegistry` for managing multiple adapters
- [x] DEV.to adapter (articles, tags, series, canonical URL)
- [x] Twitter/X adapter (single tweets, threads, rate limit handling)
- [x] LinkedIn adapter (posts via REST API)
- [x] Medium adapter (articles via integration token)
- [ ] Basic CLI with `clap` derive (new, list, publish commands)
- [ ] Project documentation (README, architecture, contributing)

### Success Criteria
- `cargo build` compiles the full workspace with zero warnings
- `cargo test` passes with unit tests for core types, DB operations, and adapter validation
- A developer can programmatically create content and publish to DEV.to via the library API

---

## Phase 1 -- CLI + Scheduling

**Goal:** A fully functional CLI tool with scheduling capabilities.

### Deliverables
- [ ] Complete CLI: `new`, `list`, `show`, `edit`, `adapt`, `publish`, `schedule`, `platforms`
- [ ] JSON output mode for scripting (`--format json`)
- [ ] Configuration file support (`~/.config/contentforge/config.toml`)
- [ ] Credential management (`contentforge platforms add twitter`)
- [ ] Scheduling engine: background task that polls for due entries and publishes
- [ ] Retry logic with exponential backoff for failed publishes
- [ ] `contentforge daemon` for running the scheduler as a background process
- [ ] Integration tests with mock HTTP server

### Success Criteria
- A user can install via `cargo install`, configure a DEV.to API key, and publish an article entirely via CLI
- Scheduled posts publish within 60 seconds of their scheduled time
- All CLI commands have `--help` documentation and examples

---

## Phase 2 -- TUI + Web UI + AI + MCP

**Goal:** Rich interfaces and AI-powered content assistance.

### Deliverables
- [ ] TUI dashboard: content list, status filter, detail view, keyboard navigation
- [ ] TUI editor: inline Markdown editing with live preview
- [ ] TUI schedule view: upcoming publications timeline
- [ ] Axum API server: REST endpoints for all CRUD operations
- [ ] WebSocket support for real-time updates
- [ ] SvelteKit frontend embedded via `rust-embed`
- [ ] Web UI: content editor, platform preview, visual calendar
- [ ] AI agent: content generation from prompts (`contentforge generate "Write about Rust error handling"`)
- [ ] AI agent: automatic platform adaptation (article to tweet thread)
- [ ] AI agent: content review and improvement suggestions
- [ ] MCP server: expose contentforge tools via stdio and SSE transport
- [ ] MCP tools: `create_content`, `list_content`, `adapt`, `publish`, `schedule`

### Success Criteria
- TUI is usable for the complete content workflow without touching the CLI
- Web UI serves from the same binary, no separate frontend build step needed
- Claude Code can create and publish content via MCP with natural language instructions

---

## Phase 3 -- More Platforms + Analytics

**Goal:** Expand platform coverage and close the feedback loop with analytics.

### Deliverables
- [ ] YouTube adapter: video metadata updates, description publishing
- [ ] Instagram adapter: image posts via Graph API
- [ ] Reddit adapter: subreddit posts
- [ ] Hacker News adapter: story submissions
- [ ] Analytics collection: pull views, likes, shares, comments from each platform API
- [ ] Analytics dashboard in TUI and Web UI
- [ ] Recurring schedules: "Every Friday at 9 AM, publish a weekly roundup"
- [ ] Content templates: reusable structures for common post types
- [ ] Bulk operations: adapt and publish to all platforms in one command

### Success Criteria
- At least 6 platforms have working publish + analytics adapters
- Analytics data is collected automatically on a configurable interval
- Recurring schedules work reliably for at least 4 weeks

---

## Phase 4 -- Polish + Distribution

**Goal:** Production-ready release with professional distribution.

### Deliverables
- [ ] Homebrew formula (macOS + Linux)
- [ ] Cross-platform release binaries (Linux x86_64, ARM64; macOS universal; Windows x86_64)
- [ ] GitHub Actions CI/CD: lint, test, build, release
- [ ] release-please integration for automated changelogs and versioning
- [ ] MkDocs documentation site deployed to GitHub Pages
- [ ] Shell completions (bash, zsh, fish)
- [ ] Man pages
- [ ] `contentforge doctor` command for diagnosing configuration issues
- [ ] Comprehensive error messages with suggested fixes
- [ ] Performance benchmarks (publish latency, TUI responsiveness)

### Success Criteria
- A new user can go from `brew install contentforge` to first published post in under 5 minutes
- Documentation covers every CLI command, API endpoint, and configuration option
- Zero `clippy` warnings, all tests pass on CI across all platforms

---

## Phase 5 -- Advanced Features

**Goal:** Power-user features and ecosystem growth.

### Deliverables
- [ ] Thread composer: visual editor for Twitter threads with character count per tweet
- [ ] AI auto-adaptation: when content is created, automatically generate adaptations for all configured platforms
- [ ] Webhook triggers: publish on Git push, GitHub release, or custom webhook
- [ ] Content import: pull existing posts from DEV.to, Medium, etc. into ContentForge
- [ ] A/B testing: publish variants and compare engagement
- [ ] Plugin SDK: Rust trait for community-contributed adapters
- [ ] Multi-workspace support: manage content for multiple brands/projects
- [ ] RSS feed generation for published content
- [ ] Collaborative editing (Web UI): real-time multi-user editing via CRDTs

### Success Criteria
- Community has contributed at least one third-party adapter
- Webhook-triggered publishing works end-to-end with GitHub Actions
- Thread composer handles threads of 20+ tweets with proper character counting
