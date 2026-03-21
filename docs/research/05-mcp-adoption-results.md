# MCP Protocol Adoption — Perplexity Research Results

> Source: Perplexity, 2026-03-21

## Key Findings

### Adoption Scale (Early 2026)
- **110+ official MCP servers** (APITracker)
- **1,065+ total MCP servers** (Gradually AI registry)
- Clients with MCP support: Claude Desktop, Claude Code, Continue, Cursor, some ChatGPT plugins
- Skewed toward dev/infrastructure tools

### Content/Social MCP Tools
- **Postiz**: Ships "Social Media MCP" — schedule from Claude/Cursor
  - "the only scheduler that offers MCP natively" (their claim)
- **fast-agent**: Demo social post generator agent using MCP
- **Content workflow guides**: Claude + Notion + Apify scrapers via MCP
- **ContentForge would be #2** in "social scheduler + MCP" but **#1 as Rust/CLI-first**

### Developer Sentiment
- MCP seen as **"LSP for AI tools"** — more setup than function calling, but more scalable
- Preferred over LangChain abstractions (devs want clearer control)
- Vendor-neutral: works across Claude, Cursor, Continue — unlike OpenAI plugins
- Complexity concern exists but mitigated by better frameworks

### Is MCP a Differentiator? — YES (for dev audience)
- For general creators: means nothing
- For devs using Claude/Cursor: **concrete differentiator**
- "Works out-of-the-box with Claude Code" is a strong hook
- MCP directories/registries drive discovery (official registry, MCP Server Finder, Gradually, APITracker)
- **If two tools are comparable, Claude users pick the MCP one**

### Other Protocols to Consider
| Protocol | Audience | Worth Supporting? |
|---|---|---|
| **MCP** | Claude/Cursor/Continue | Yes — primary |
| **OpenAI function calling** | ChatGPT/Assistants | Yes — secondary |
| **Google A2A** | Enterprise/GCP | Later (enterprise focus) |
| **LangChain tools** | LangChain users | No (devs moving away) |

### Growth Lever
- Tools that surface inside dev workflows (editors, terminals) outperform separate websites
- **MCP-native + CLI-first = primary growth lever** for dev/indie niche
- Design 1-2 hero workflows: "git commit → Claude generates post via MCP → schedule across platforms"
