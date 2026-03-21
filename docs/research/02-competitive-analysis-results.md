# Competitive Analysis — Perplexity Research Results

> Source: Perplexity, 2026-03-21
> Prompt: /Users/bm/contentforge/docs/research/prompts/02-competitive-analysis.md

## Competitor Breakdown

### Postiz (Closest Competitor)
- **Stars**: 15K → 27K (growing fast)
- **Revenue**: $14.2K MRR (up from $700/mo in late 2024)
- **Docker downloads**: 584K
- **Community**: 1,115 Discord members
- **Stack**: Node.js/Next.js, Docker, PostgreSQL
- **Strengths**: Broad platform coverage, OSS, team/collab, AI generation, MCP module
- **Weaknesses**:
  - Install/config friction (env vars, Docker issues)
  - Platform-specific glitches (Instagram media fetch, Pinterest uploads)
  - Complex self-hosting (full stack + queue + external integrations)
- **Has MCP**: Yes — "the only scheduler that offers MCP natively"

### Mixpost
- **Stars**: ~3K
- **Revenue**: Unknown (one-time license model)
- **Pricing**: Pro $269 one-time, Enterprise $1,199 one-time
- **Stack**: PHP/Laravel
- **Strengths**: One-time payment, self-hosted, data ownership, SaaS-in-a-box for agencies
- **Weaknesses**: PHP/Laravel stack, config complexity, smaller community

### Shoutify (Dead)
- **Status**: ARCHIVED (May 2023), never became functional
- **Lesson**: Social manager scope + API volatility killed it without revenue model

### Typefully (Gold Standard for UX)
- **Users**: 130,000+ customers
- **Revenue**: ~$1.4M ARR with 3-person team
- **Pricing**: $16-99/mo SaaS
- **Strengths**: Best-in-class thread editor, analytics, AI rewriting, writer-first UX
- **Growth driver**: Word of mouth in crypto/tech Twitter + SEO + affiliates

## Rust/TUI/CLI Landscape — THE GAP

- `queue-social` — simple Rust scheduler, limited
- `tweety-rs` — Rust Twitter API crate
- `perch` — Mastodon/Bluesky TUI (consumption, not publishing)
- `social-poster` — Node.js Puppeteer CLI
- `buffer-cli` — TypeScript CLI wrapping Buffer
- **No tool combines: Rust + TUI + Web + MCP + multi-platform scheduling**

## Differentiation Matrix

| Feature | ContentForge | Postiz | Mixpost | Typefully |
|---|:---:|:---:|:---:|:---:|
| Single binary install | ✅ | ❌ | ❌ | N/A |
| CLI-native | ✅ | ❌ | ❌ | ❌ |
| TUI | ✅ | ❌ | ❌ | ❌ |
| Web UI | ✅ | ✅ | ✅ | ✅ |
| MCP server | ✅ | ✅ | ❌ | ❌ |
| Self-hosted (easy) | ✅ (1 binary) | ⚠️ (Docker) | ⚠️ (Laravel) | ❌ |
| Zero infrastructure | ✅ (SQLite) | ❌ (PG+Redis) | ❌ (MySQL+Redis) | N/A |
| MIT license | ✅ | Apache-2 | MIT (Lite) | Closed |
| Works offline | ✅ | ❌ | ❌ | ❌ |
| Git-native workflows | ✅ | ❌ | ❌ | ❌ |
