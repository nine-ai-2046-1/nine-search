## Context

Currently config.toml only has `default_provider` and `[[providers]]` with API key. Every CLI invocation requires passing all params as `--key value` flags. This change adds a `[tavily]` section to config.toml with all Tavily API parameter defaults, so users can set their preferred defaults once in config and only override via CLI when needed.

## Goals / Non-Goals

**Goals:**
- All 20 Tavily params in config.toml with sensible defaults and Cantonese emoji comments
- Config defaults are used when CLI argv doesn't provide a value
- CLI argv overrides config values for any param
- Validate merged params against provider schema before API call

**Non-Goals:**
- Interactive config editing
- Config validation beyond type checking
- Multiple provider config sections (v1 only has Tavily)

## Decisions

### 1. Config structure: flat `[tavily]` section

```toml
[tavily]
search_depth = "basic"
max_results = 5
topic = "general"
include_images = false
```

**Why:** Flat TOML table is simple to parse with serde. Users can see all defaults at a glance. No nested structures needed.

### 2. Merge strategy: config defaults → argv override

1. Load config.toml → get `TavilyConfig` as `HashMap<String, String>`
2. Parse CLI argv → get `HashMap<String, String>`
3. Merge: config values + argv values (argv wins on conflict)
4. Validate merged params against provider schema

**Why:** Simple, predictable. Config provides base, CLI provides override. No complex merging logic.

### 3. Cantonese comments with emoji

Each param gets a comment line above it in the config file:
```toml
# 🔍 搜尋深度：basic(快) / advanced(準) / fast / ultra-fast
search_depth = "basic"
```

**Why:** User specifically requested Cantonese descriptions. Emoji makes config more scannable.

### 4. TavilyConfig as Optional fields

```rust
#[derive(Debug, Deserialize, Default)]
pub struct TavilyConfig {
    pub search_depth: Option<String>,
    pub max_results: Option<i64>,
    pub topic: Option<String>,
    // ...
}
```

**Why:** All Tavily params are optional (except query which comes from argv). Using Option allows config to omit params and only set what user cares about. Merge with provider defaults handles missing values.

## Risks / Trade-offs

- **Config file gets long** → Mitigated by clear comments and sensible defaults
- **Type mismatch between config and API** → All values stored as strings in merge HashMap, validated before API call
- **User edits config incorrectly** → Clear error messages on parse failure
