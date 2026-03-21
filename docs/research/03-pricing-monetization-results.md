# Pricing & Monetization — Perplexity Research Results

> Source: Perplexity, 2026-03-21
> Prompt: /Users/bm/contentforge/docs/research/prompts/03-pricing-monetization.md

## Key Findings

### Open-Core Examples (10 tools)

| Tool | Free | Paid | Price | Revenue |
|---|---|---|---|---|
| GitLab | Git, CI, issues | Security, compliance | $29-99/user/mo | $1B+ ARR |
| Supabase | PG+Auth+Storage | Hosting, scaling, SOC2 | From $25/mo | Tens of M ARR |
| Railway | $5 credits | Higher usage, teams | $5-20/seat/mo | Growing |
| Windmill | OSS <10 users | Enterprise, SAML, audit | From $120/mo | Commercial |
| Tuist | Xcode generator | Cloud features, support | Paid tiers | "Sustainable" |
| Fern | SDK generator CLI | Managed SaaS, publishing | Enterprise | Active |
| OneUptime | Monitoring OSS | Enterprise, SSO | Tiered | Open-core |
| Warp | Free terminal | AI credits, SOC2 | $20-50/mo | Significant users |
| Fig/Amazon Q | Free autocomplete | Team features (pre-acq) | Was paid → now free | Acquired by Amazon |
| Protomaps | OSS map server | Previously gated | Was commercial | Went full OSS |

**Pattern**: CLI/SDK is always free. Money comes from hosting, collaboration, scale, enterprise controls.

### What Gets Gated (Across All 6 Specific Tools)
- **AI credits/usage at scale** (Warp)
- **Team collaboration** (Fig, Railway, Warp)
- **Resource limits** (Supabase, Railway)
- **Compliance/security** — SSO, SOC2, SAML (Warp, Windmill, Supabase)
- **Analytics/audit logs** (Windmill)
- **SLA/priority support** (Windmill, Supabase)
- **Hosted infrastructure** (Supabase, Railway)

### One-Time $49 vs $9/mo — THE VERDICT

- **Recurring wins for tools with ongoing maintenance costs** (API integrations, AI)
- Warp pivoted from multiple tiers to a single $20/mo Build plan — recurring is easier to sustain
- No successful CLI tool uses one-time as the ONLY model
- **Best hybrid**: $9-15/mo recurring + optional $49 one-time "Pro local" license for features that don't incur ongoing costs
- **Developer willingness**: Postman $9/mo, Railway $5-20/mo, Warp $20/mo all have paying users

### Stars → Paid Conversion — REALITY CHECK

- **3-5% of starrers converting is NOT realistic**
- More plausible: **0.1-1% of starrers** become paying customers
- Postiz: 27K stars, 584K Docker pulls, but only $14.2K MRR
- The 24% number in some studies is from **engaged users**, not starrers
- **Model revenue from active deployments/MAUs**, not stars
- **2-10% of active teams** converting is more realistic

### Distribution Channel Ranking (for dev tools)

1. **GitHub trending** — strongest long-term acquisition
2. **Hacker News** — high leverage for technically impressive OSS
3. **Product Hunt** — great launch amplifier
4. **Reddit** — niche engagement (r/selfhosted, r/devtools, r/buildinpublic)
5. **Twitter/X** — build in public narratives
6. **DEV.to** — in-depth articles, slower burn
7. **YouTube** — demo videos, high effort
8. **Homebrew** — distribution enabler, not discovery
9. **crates.io** — if publishing Rust libraries

**Winning combo**: Launch on HN + Product Hunt → push for GitHub trending → content loop on DEV.to/Twitter/YouTube → engage Reddit communities.

## Updated Revenue Model

Based on research, revised from PRODUCT_STRATEGY.md:

### Pricing (Revised)
- **Free**: Full CLI/TUI/Web, 4 platforms, basic scheduling, MCP (stdio)
- **Pro**: $9/mo or $99/year — pipeline automation, cron scheduling, approval flows, encrypted credentials, analytics, premium adapters
- **Team**: $19/user/mo — multi-workspace, shared pipelines, RBAC

### Revenue Projections (Revised, More Conservative)
| Metric | Month 6 | Month 12 |
|---|---|---|
| GitHub stars | 1,000 | 3,000 |
| Active deployments (MAU) | 200 | 800 |
| Paying users (2-5% of MAU) | 10-15 | 30-50 |
| Monthly revenue | $90-135 | $270-450 |
| ARR | ~$1,200 | ~$4,500 |

This is conservative. If ContentForge hits GitHub trending + HN front page, numbers could be 5-10x.
