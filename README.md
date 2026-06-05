# nine-search

A modular search CLI for AI agents and developers. Search the web and get clean, structured results.

## Features

- **markdown-search** (default): Free, no API key needed. Returns full Markdown content from web pages. Perfect for RAG and LLM pipelines.
- **tavily**: Paid search API with AI-generated answers, topic filtering, and rich metadata.
- Modular provider system: easy to add new search engines.
- Cantonese-annotated config file with sensible defaults.

## Installation

```bash
# Clone the repo
git clone https://github.com/nine-ai-2046-1/nine-search.git
cd nine-search

# Build and install
cargo build --release
cp target/release/nine-search ~/.local/bin/

# Or use the deploy script
./scripts/build-deploy
```

Requires: `rust` and `curl`.

## Quick Start

```bash
# Search with default provider (markdown-search, free)
nine-search --query "rust programming"

# Get help
nine-search --help
```

## Configuration

Config file: `~/.config/nine-search/config.toml`

Auto-created on first run. Edit it to customize settings.

### markdown-search (default, free)

```toml
default_provider = "markdown-search"

[markdown-search]
n = 5              # Results (1-5)
gl = "hk"          # Region (us, uk, hk, jp...)
hl = "zh"          # Language (en, zh, ja...)
format = "json"    # Response format (don't change)
retain_images = true  # Include images in markdown
```

Rate limits: 30/min, 500/day per IP. No API key needed.

### tavily (paid)

```toml
default_provider = "tavily"

[[providers]]
id = "tavily"
key = "tvly-YOUR_API_KEY"

[tavily]
search_depth = "basic"
max_results = 5
topic = "general"
include_images = false
```

Get API key at [tavily.com](https://tavily.com).

## Switching Providers

Edit `~/.config/nine-search/config.toml`:

```toml
default_provider = "tavily"  # or "markdown-search"
```

## Usage Examples

```bash
# Basic search
nine-search --query "what is rust"

# Search with specific region (override config)
nine-search --query "香港天氣"

# Verbose output for debugging
nine-search --query "test" 2>&1 | less
```

## Response Format

### markdown-search (JSON)

```json
{
  "query": "rust programming",
  "results": [
    {
      "title": "Rust Programming Language",
      "link": "https://www.rust-lang.org",
      "snippet": "A language empowering everyone..."
    }
  ],
  "extracted": [
    {
      "url": "https://www.rust-lang.org",
      "title": "Rust",
      "markdown": "# Full page content in Markdown...",
      "method": "Cloudflare Browser Rendering"
    }
  ]
}
```

## Rate Limits

| Provider | Limits |
|----------|--------|
| markdown-search | 30/min, 500/day per IP |
| tavily | Plan-based (paid) |

## License

MIT
