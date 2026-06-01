## Context

This is a new Rust CLI project — no existing codebase. We're building a search tool that wraps web search APIs (starting with Tavily) behind a modular provider system. The tool runs as a single binary, reads config from `~/.config/nine-search/config.toml`, and shells out to `curl` for HTTP requests.

Key constraints:
- Rust project, minimal dependencies
- Curl-based HTTP (no reqwest/hyper)
- Config file is TOML, auto-created on first run
- CLI args are loose (`--key value`) and validated per-provider at runtime

## Goals / Non-Goals

**Goals:**
- Self-configuring: first run creates default config
- Modular: adding a new search provider means adding one file + config entry
- Safe: always validate API key exists before making calls
- Transparent: pretty-printed JSON output from API

**Non-Goals:**
- Interactive config setup (just warn and exit)
- Config CLI commands (edit config.toml manually for now)
- Multiple provider support beyond Tavily in v1
- Summary/text output mode (JSON only)
- Caching, retries, or advanced error recovery

## Decisions

### 1. Project structure: flat modules, not deep nesting

```
src/
├── main.rs          ← entry point, arg parsing, orchestration
├── config.rs        ← config load/create/validate
├── providers/
│   ├── mod.rs       ← Provider trait + registry
│   └── tavily.rs    ← Tavily implementation
```

**Why:** Four source files is enough. No need for `cli.rs` or `utils.rs` — main.rs handles arg parsing (via simple iteration), and curl calls are a one-liner in the provider.

**Alternative considered:** Separate cli.rs with clap. Rejected — adds a heavy dependency for something that can be done with 20 lines of argv iteration.

### 2. CLI arg parsing: manual iteration, not clap

Parse `--key value` pairs from `std::env::args()`. Validate keys against provider's param schema at runtime.

**Why:** Keeps dependencies minimal (just `toml` + `serde`/`serde_json`). The loose approach means the CLI doesn't need to know about every provider's params — the provider validates.

**Alternative considered:** clap with derive. Rejected — would require defining all params in the CLI, defeating the modular provider design.

### 3. Config: bundled default template

`config.rs` contains a `DEFAULT_CONFIG` const. If `~/.config/nine-search/config.toml` doesn't exist, write it. If it exists but is malformed, error clearly.

**Why:** Simple, no external template files. The default has an empty API key so the user is forced to fill it in.

### 4. Provider trait: param schema as HashMap

Each provider defines `valid_params()` returning a `HashMap<String, ParamDef>` where `ParamDef` has type info (String, Int, Bool, Enum, Array) and whether it's required.

**Why:** Allows runtime validation of arbitrary `--key value` pairs. Each provider owns its schema — no central registry.

**Alternative considered:** Shared param definitions with provider overrides. Rejected — more complex, and providers have different param sets anyway.

### 5. HTTP: std::process::Command calling curl

Build curl args programmatically: `-X POST`, `-H "Authorization: Bearer ..."`, `-H "Content-Type: application/json"`, `-d '{json_body}'`. Parse stdout as JSON.

**Why:** Matches user requirement. No HTTP client dependency. Curl handles TLS, redirects, timeouts.

**Risk:** Curl might not be installed. Mitigation: check for curl at startup, warn if missing.

### 6. Output: serde_json pretty-print

Deserialize curl stdout into `serde_json::Value`, then pretty-print with `serde_json::to_string_pretty`.

**Why:** Simple, no formatting code needed. JSON is what the API returns.

## Risks / Trade-offs

- **No curl on system** → Check at startup, fail with clear message
- **Config corruption** → Re-read on every invocation, validate structure
- **Provider param drift** → If Tavily adds new params, users can pass them (loose parsing) but they won't be validated until the provider module is updated
- **No retry/rate-limit handling** → v1 limitation, user sees raw API error
