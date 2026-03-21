# Platform API Landscape — Perplexity Research Results

> Source: Perplexity, 2026-03-21

## Platform Priority Matrix (based on research)

### Tier 1: Easy + Safe (v1 targets)
| Platform | Auth | Cost | Stability | Notes |
|---|---|---|---|---|
| **DEV.to/Forem** | API key | Free | Very stable | Easiest integration, perfect for devs |
| **Mastodon** | OAuth 2.0 per instance | Free | Stable | Strong in FOSS/dev circles, 1.5M MAU |
| **Bluesky** | JWT token (ATProto) | Free | Evolving | Growing indie/dev audience, no paywalls |

### Tier 2: Medium friction
| Platform | Auth | Cost | Stability | Notes |
|---|---|---|---|---|
| **YouTube** | OAuth 2.0 | Free (quota) | Stable | 10K units/day, ~6 uploads/day max |
| **Threads** | Meta Graph API | Free (review) | New | 500 chars, auto-publish works, needs Meta review |

### Tier 3: High friction / risky
| Platform | Auth | Cost | Stability | Notes |
|---|---|---|---|---|
| **Twitter/X** | OAuth 2.0 | $100+/mo for writes | Volatile | Free tier = read-only, pricing changes repeatedly |
| **LinkedIn** | OAuth 2.0 | Free (partner gate) | Gated | Needs approved LinkedIn Partner status for write |
| **Instagram** | Meta Graph API | Free (heavy review) | Gated | Business/Creator account + Meta verification |
| **Medium** | Deprecated | N/A | Dead | No supported publishing API in 2026 |
| **Substack** | None | N/A | None | No official API exists |

## Revised Platform Strategy

**v0.1 (ship now)**: DEV.to + Mastodon + Bluesky — all free, stable, dev-friendly
**v0.2**: + YouTube (quota-aware) + Threads (if Meta review passes)
**v0.3**: + Twitter/X (user brings own keys) + LinkedIn (if partner approved)
**Defer**: Instagram (too gated), Medium (deprecated), Substack (no API)

## Key Gotcha: Twitter/X

- Free tier has NO write access
- Basic tier costs ~$100/mo minimum
- Each user must bring their own X developer app
- Terms enforce anti-automation — multi-tenant scheduler risk
- **Pricing has changed repeatedly since 2023 — plan for volatility**
