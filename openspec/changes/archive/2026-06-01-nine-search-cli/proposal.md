## Why

We need a CLI tool to search the web via Tavily's API. The tool should be modular — easy to add new search providers later — and self-configuring: first run creates a config file, validates the API key, and warns the user if it's missing.

## What Changes

- New Rust CLI binary `nine-search` with argument parsing
- Config file at `~/.config/nine-search/config.toml` — auto-created on first run
- Provider trait system: each search provider is an independent module
- Tavily provider: curl-based POST to `https://api.tavily.com/search` with Bearer auth
- Loose argv parsing (`--key value` pairs) validated against provider's param schema at runtime
- Pretty-printed JSON output from API response
- Graceful error handling: warn on missing API key, validate params before making calls

## Capabilities

### New Capabilities

- `config-management`: Load/create `~/.config/nine-search/config.toml`, validate default provider API key
- `provider-system`: Trait-based provider registry with per-provider param validation
- `tavily-provider`: Tavily search integration via curl with full param support
- `cli-argument-parsing`: Loose `--key value` argv parsing with runtime validation

### Modified Capabilities

_(none — new project)_

## Impact

- New Rust project with `Cargo.toml`
- Dependencies: `toml` (config parsing), `serde`/`serde_json` (API response)
- Uses `std::process::Command` for curl invocation (no HTTP client crate)
- Requires `curl` binary on system
- Config directory created at `~/.config/nine-search/`
