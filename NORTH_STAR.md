# ContentForge North Star

## Mission

**The Rust-native Postiz/Buffer alternative -- zero-config, single-binary, AI-native.**

ContentForge exists to give developers and small teams a powerful, private, and extensible content publishing platform that runs entirely on their own machine.

## Problem Statement

Content creators who are also developers face a fragmented landscape:

- **Postiz** is a self-hosted social media scheduler, but it requires Docker, PostgreSQL, Redis, and significant configuration. It is a Node.js monolith with heavy infrastructure requirements.
- **Buffer / Hootsuite** are SaaS platforms that hold your data hostage, charge per-channel pricing, and offer no API extensibility for developers.
- **n8n / Zapier** can automate publishing but are workflow tools, not content tools. You still need to write content somewhere else and manage platform-specific formatting manually.
- **Typefully** focuses solely on Twitter/X threads with a polished UI, but lacks multi-platform support and is SaaS-only.
- **Mixpost** is self-hosted but PHP-based, requires MySQL, and has a heavy operational footprint.

**The gap:** There is no single-binary, local-first, AI-augmented content platform built for developers. ContentForge fills this gap.

## Target Users

1. **Solo developers** who blog, tweet, and post on LinkedIn to build their personal brand. They want one tool, not five.
2. **Indie hackers** doing build-in-public who need to cross-post launch announcements, changelogs, and updates across multiple platforms efficiently.
3. **Developer advocates** who create technical content at scale and need to adapt long-form articles into platform-specific formats (threads, short posts, video descriptions).
4. **Small teams (2-10 people)** who want a shared content pipeline without paying per-seat SaaS pricing or running Docker infrastructure.

## Core Principles

### 1. Local-First
All data lives in a local SQLite database. No cloud account required. No data leaves your machine unless you explicitly publish. Works offline for drafting and scheduling.

### 2. Single Binary
One `cargo install` or `brew install` gives you everything: CLI, TUI, Web UI, API server, and MCP server. No Docker, no PostgreSQL, no Redis, no Node.js.

### 3. Privacy-Respecting
No telemetry. No analytics about your usage. Credentials are stored locally. The only network calls are the ones you initiate (publishing, fetching analytics).

### 4. AI-Augmented, Not AI-Dependent
AI features (draft generation, platform adaptation, thread splitting) are opt-in and work with any LLM backend. The tool is fully functional without AI.

### 5. Extensible via MCP
ContentForge exposes itself as an MCP server, so AI assistants like Claude Code can create, adapt, schedule, and publish content on your behalf. This is the "AI-native" differentiator.

### 6. Developer-Grade Ergonomics
Configuration via TOML files. Scriptable CLI with JSON output. Full REST API. Git-friendly content storage. Conventional Rust tooling (cargo build, cargo test, clippy clean).

## Competitive Landscape

| Tool       | Type         | Platforms | Self-Hosted | Single Binary | AI-Native | Local-First |
|------------|--------------|-----------|-------------|---------------|-----------|-------------|
| **ContentForge** | **CLI/TUI/Web** | **9** | **Yes** | **Yes** | **Yes** | **Yes** |
| Postiz     | Web app      | 8+        | Yes         | No (Docker)   | Partial   | No          |
| Mixpost    | Web app      | 6+        | Yes         | No (PHP)      | No        | No          |
| Buffer     | SaaS         | 8+        | No          | N/A           | Partial   | No          |
| Typefully  | SaaS         | 1-2       | No          | N/A           | Yes       | No          |
| n8n        | Automation   | Any       | Yes         | No (Docker)   | No        | No          |

## Product Vision

### v1.0.0 -- "Ship It"
The minimum product that a developer would choose over manually copy-pasting between platforms.

- CLI and TUI for content creation and publishing
- Working adapters for DEV.to, Twitter/X, LinkedIn, Medium
- SQLite persistence with full CRUD
- Scheduling engine with cron support
- Basic AI adaptation (long-form to short-form, thread splitting)
- MCP server for Claude Code integration
- Homebrew and cargo install distribution

**Success metric:** A developer can write one blog post and publish adapted versions to four platforms in under 5 minutes.

### v2.0.0 -- "Team Ready"
Expand from solo use to small team collaboration.

- Web UI (SvelteKit) with visual calendar scheduling
- YouTube and Instagram adapters
- Analytics dashboard pulling engagement metrics from all platforms
- Recurring schedule templates ("weekly ship post")
- AI agent pipeline: generate full content from a one-line prompt
- Content approval workflow (draft -> review -> approved -> scheduled)
- Export/import for content portability

**Success metric:** A 5-person DevRel team uses ContentForge as their primary publishing tool for a quarter.

### v3.0.0 -- "Platform"
Become the extensible platform for content operations.

- Plugin/adapter SDK for community-contributed platforms
- Webhook triggers for CI/CD integration (publish on merge, on release)
- A/B testing for post variants
- SEO optimization suggestions
- RSS feed ingestion for content curation
- Multi-workspace support

## Monetization Ideas (Open Core)

ContentForge is and will remain open source (MIT) for all core functionality. Potential revenue paths:

1. **ContentForge Cloud** -- hosted version with team management, no setup required. Free for solo, paid for teams.
2. **Premium adapters** -- adapters for platforms requiring complex OAuth or partnerships (e.g., Instagram Business, TikTok) could be premium add-ons.
3. **Enterprise features** -- audit logs, SSO, role-based access, priority support.
4. **Managed AI** -- bundled LLM credits for users who do not want to bring their own API key.

The core principle is that a single developer using ContentForge on their own machine should never need to pay for anything.
